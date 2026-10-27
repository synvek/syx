use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Once};
use std::thread;
use std::time::Duration;

#[cfg(feature = "async-runtime")]
use crossbeam_channel::{Receiver, Sender};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::MirModule;
use syx_mir_run::{run_mir_function_with_args, MirRuntime};
use syx_runtime_core::{RuntimeBridge, Value, VmRuntime};
use syx_vm::{actor_id_from_value, register_task_hosts};
#[cfg(feature = "async-runtime")]
use syx_vm::{
    platform_int_value, register_actor_reentrancy_policy_host, register_task_group_cancel_host,
    register_task_live_count_host, register_task_local_hosts, register_task_set_deadline_ms_host,
};

#[cfg(feature = "async-runtime")]
use syx_async::{CancellationToken, TaskGroup, TaskHandle, TaskRegistry};

#[cfg(feature = "async-runtime")]
struct ThreadPool {
    tx: Sender<Box<dyn FnOnce() + Send>>,
}

#[cfg(feature = "async-runtime")]
impl ThreadPool {
    fn new(workers: usize) -> Self {
        let (tx, rx): (
            Sender<Box<dyn FnOnce() + Send>>,
            crossbeam_channel::Receiver<Box<dyn FnOnce() + Send>>,
        ) = crossbeam_channel::unbounded();
        for _ in 0..workers.max(1) {
            let rx = rx.clone();
            thread::spawn(move || {
                while let Ok(job) = rx.recv() {
                    job();
                }
            });
        }
        Self { tx }
    }

    fn spawn<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let _ = self.tx.send(Box::new(f));
    }
}

#[cfg(feature = "async-runtime")]
static POOL: std::sync::OnceLock<ThreadPool> = std::sync::OnceLock::new();

#[cfg(feature = "async-runtime")]
fn pool() -> &'static ThreadPool {
    POOL.get_or_init(|| {
        let n = thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4)
            .max(2);
        ThreadPool::new(n)
    })
}

#[cfg(feature = "async-runtime")]
struct NativeActorJob {
    method_name: String,
    args: Vec<Value>,
    reply: Sender<SyxResult<Value>>,
}

/// Per-run native/MIR execution context (isolated for parallel tests).
pub struct NativeRunContext {
    module: Mutex<Option<Arc<MirModule>>>,
    pub printed: Mutex<Vec<String>>,
    gc_promotion_cycles: Mutex<u32>,
    #[cfg(feature = "async-runtime")]
    tasks: TaskRegistry,
    #[cfg(feature = "async-runtime")]
    groups: Mutex<HashMap<u64, Arc<TaskGroup>>>,
    #[cfg(feature = "async-runtime")]
    next_group_id: AtomicU64,
    #[cfg(feature = "async-runtime")]
    actors: Mutex<HashMap<i64, Sender<NativeActorJob>>>,
}

impl Default for NativeRunContext {
    fn default() -> Self {
        Self {
            module: Mutex::new(None),
            printed: Mutex::new(Vec::new()),
            gc_promotion_cycles: Mutex::new(0),
            #[cfg(feature = "async-runtime")]
            tasks: TaskRegistry::default(),
            #[cfg(feature = "async-runtime")]
            groups: Mutex::new(HashMap::new()),
            #[cfg(feature = "async-runtime")]
            next_group_id: AtomicU64::new(1),
            #[cfg(feature = "async-runtime")]
            actors: Mutex::new(HashMap::new()),
        }
    }
}

impl NativeRunContext {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn with_module(module: Arc<MirModule>) -> Arc<Self> {
        let ctx = Self::new();
        *ctx.module.lock().expect("native ctx module lock") = Some(module);
        ctx
    }

    pub fn set_module(&self, module: Arc<MirModule>) {
        *self.module.lock().expect("native ctx module lock") = Some(module);
    }

    pub fn module(&self) -> Option<Arc<MirModule>> {
        self.module.lock().expect("native ctx module lock").clone()
    }

    pub fn clear_ephemeral(&self) {
        self.printed
            .lock()
            .expect("native ctx printed lock")
            .clear();
        #[cfg(feature = "async-runtime")]
        {
            self.tasks.clear();
            self.groups.lock().expect("native ctx groups lock").clear();
            self.actors.lock().expect("native ctx actors lock").clear();
        }
    }

    pub fn printed_snapshot(&self) -> Vec<String> {
        self.printed
            .lock()
            .expect("native ctx printed lock")
            .clone()
    }

    /// Phase 35 FL1: run unified GC promotion + collect cycle (syx-rt bridge prod default).
    pub fn run_unified_gc_promotion_cycle(&self) -> i32 {
        if !syx_rt_unified_gc_bridge_enabled() {
            return 0;
        }
        let _ = syx_vm::unified_gc_promote(0);
        let collected = syx_vm::unified_gc_promotion_collect();
        *self.gc_promotion_cycles.lock().expect("native ctx gc lock") += 1;
        collected
    }

    pub fn unified_gc_promotion_cycles(&self) -> u32 {
        *self.gc_promotion_cycles.lock().expect("native ctx gc lock")
    }

    /// Phase 36 FQ6: JIT safepoint — invalidate tier-2 cache before GC promotion cycle.
    pub fn run_jit_safepoint_gc_cycle(&self) -> i32 {
        if !syx_rt_jit_safepoint_enabled() {
            return self.run_unified_gc_promotion_cycle();
        }
        syx_vm::invalidate_all_jit_caches();
        self.run_unified_gc_promotion_cycle()
    }
}

/// Phase 36 FQ6: JIT safepoint registration prod default on.
pub fn syx_rt_jit_safepoint_enabled() -> bool {
    std::env::var("SYX_SYX_RT_JIT_SAFEPOINT")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(syx_rt_unified_gc_bridge_enabled())
}

/// Phase 35 FL1: syx-rt unified GC bridge prod default on.
pub fn syx_rt_unified_gc_bridge_enabled() -> bool {
    std::env::var("SYX_SYX_RT_UNIFIED_GC_BRIDGE")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(syx_vm::unified_gc_promotion_prod_default())
}

thread_local! {
    static CURRENT_NATIVE_CTX: RefCell<Option<Arc<NativeRunContext>>> = const { RefCell::new(None) };
}

fn with_current_ctx<F, R>(f: F) -> R
where
    F: FnOnce(&Arc<NativeRunContext>) -> R,
{
    CURRENT_NATIVE_CTX.with(|cell| {
        let ctx = cell.borrow();
        let ctx = ctx
            .as_ref()
            .expect("native run context not installed on this thread");
        f(ctx)
    })
}

fn set_current_ctx(ctx: Arc<NativeRunContext>) {
    CURRENT_NATIVE_CTX.with(|cell| {
        *cell.borrow_mut() = Some(ctx);
    });
}

/// Install MIR module into an isolated run context (returns context for runtime wiring).
pub fn install_native_module(module: &MirModule) -> Arc<NativeRunContext> {
    install_native_task_hosts();
    install_native_unified_gc_bridge();
    install_native_gui_handler();
    #[cfg(feature = "http")]
    install_native_http_handler();
    let ctx = NativeRunContext::with_module(Arc::new(module.clone()));
    ctx.clear_ephemeral();
    set_current_ctx(Arc::clone(&ctx));
    ctx
}

fn native_gui_click_handler(handler: &str) -> SyxResult<Value> {
    with_current_ctx(|ctx| run_mir_by_name(ctx, handler, vec![]))
}

fn install_native_gui_handler() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        syx_vm::register_native_gui_handler(native_gui_click_handler);
    });
}

#[cfg(feature = "http")]
fn native_http_serve_handler(handler: &str, req: &Value) -> SyxResult<Value> {
    crate::abi::run_native_mir_handler(handler, req)
}

#[cfg(feature = "http")]
fn install_native_http_handler() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        syx_vm::register_native_http_handler(native_http_serve_handler);
    });
}

/// Legacy entry: installs module into thread-local context.
pub fn store_module(module: &MirModule) {
    let _ = install_native_module(module);
}

fn prepare_mir_runtime(rt: &mut MirRuntime, module: &MirModule, ctx: Arc<NativeRunContext>) {
    rt.install_functions(module.functions.clone());
    rt.set_runtime(native_vm_runtime_for(Arc::clone(&ctx)));
    syx_mir_run::install_module_string_pool(rt, module);
}

pub(crate) fn run_mir_by_name(
    ctx: &Arc<NativeRunContext>,
    name: &str,
    args: Vec<Value>,
) -> SyxResult<Value> {
    let module = ctx
        .module()
        .ok_or_else(|| SyxDiagnostic::error("MIR module not loaded"))?;
    let func = module
        .functions
        .iter()
        .find(|f| f.name == name)
        .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function '{name}'")))?
        .clone();
    let mut rt = MirRuntime::new();
    prepare_mir_runtime(&mut rt, &module, Arc::clone(ctx));
    run_mir_function_with_args(&func, &mut rt, &mut [], &args)
}

#[cfg(feature = "async-runtime")]
fn ensure_native_actor(ctx: &Arc<NativeRunContext>, actor_id: i64) -> Sender<NativeActorJob> {
    let mut actors = ctx.actors.lock().expect("native ctx actor map");
    if let Some(tx) = actors.get(&actor_id) {
        return tx.clone();
    }
    let ctx_actor = Arc::clone(ctx);
    let (tx, rx): (Sender<NativeActorJob>, Receiver<NativeActorJob>) =
        crossbeam_channel::unbounded();
    thread::spawn(move || {
        set_current_ctx(Arc::clone(&ctx_actor));
        let mut actor_states: HashMap<i64, Value> = HashMap::new();
        while let Ok(job) = rx.recv() {
            let result = (|| {
                let actor_id = job
                    .args
                    .first()
                    .and_then(syx_vm::actor_id_from_value)
                    .unwrap_or(0);
                // ADR-631: enter serial-no-reenter TLS for mailbox dispatch (C009).
                if actor_id != 0 {
                    syx_async::enter_processing_actor(actor_id);
                }
                let mut args = job.args;
                if actor_id != 0 {
                    let state = actor_states
                        .entry(actor_id)
                        .or_insert_with(|| args.first().cloned().unwrap_or(Value::Nil));
                    if !args.is_empty() {
                        args[0] = state.clone();
                    }
                }
                let result = run_mir_by_name(&ctx_actor, &job.method_name, args);
                if actor_id != 0 {
                    syx_async::leave_processing_actor();
                }
                let result = result?;
                if actor_id != 0 && syx_vm::actor_id_from_value(&result).is_some() {
                    actor_states.insert(actor_id, result.clone());
                }
                Ok(result)
            })();
            let _ = job.reply.send(result);
        }
    });
    actors.insert(actor_id, tx.clone());
    tx
}

#[cfg(feature = "async-runtime")]
fn spawn_mir_task(ctx: Arc<NativeRunContext>, name: &str, args: Vec<Value>) -> SyxResult<Value> {
    let module = ctx
        .module()
        .ok_or_else(|| SyxDiagnostic::error("MIR module not loaded"))?;
    let func = module
        .functions
        .iter()
        .find(|f| f.name == name)
        .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function '{name}'")))?
        .clone();
    let module = Arc::clone(&module);
    let (done_tx, done_rx): (Sender<SyxResult<Value>>, Receiver<SyxResult<Value>>) =
        crossbeam_channel::unbounded();
    let handle = Arc::new(TaskHandle::new(done_rx));
    let cancel = handle.cancel.clone();
    ctx.tasks.register(Arc::clone(&handle));
    let bridge = native_vm_runtime_for(Arc::clone(&ctx));
    let ctx_thread = Arc::clone(&ctx);
    pool().spawn(move || {
        // ADR-607: worker must see native ctx + cancel token (sleep / CallHost path).
        set_current_ctx(Arc::clone(&ctx_thread));
        if cancel.is_cancelled() {
            let _ = done_tx.send(Err(SyxDiagnostic::error("Task cancelled").with_code("C004")));
            return;
        }
        syx_async::set_current_task_cancel(cancel.clone());
        let result = (|| {
            if syx_async::task_should_stop() {
                return Err(SyxDiagnostic::error("Task cancelled").with_code("C004"));
            }
            let mut rt = MirRuntime::new();
            prepare_mir_runtime(&mut rt, &module, ctx_thread);
            rt.set_runtime(bridge);
            run_mir_function_with_args(&func, &mut rt, &mut [], &args)
        })();
        syx_async::clear_current_task_cancel();
        let _ = done_tx.send(result);
    });
    Ok(handle.to_value())
}

struct NativeMirBridge {
    ctx: Arc<NativeRunContext>,
}

impl RuntimeBridge for NativeMirBridge {
    fn load_global(&self, _idx: usize) -> Value {
        Value::Nil
    }

    fn store_global(&self, _idx: usize, _value: Value) {}

    fn record_print(&self, text: String) {
        if let Ok(mut printed) = self.ctx.printed.lock() {
            printed.push(text);
        }
    }

    fn spawn_detached_fn(&self, name: &str, args: Vec<Value>) -> SyxResult<Value> {
        #[cfg(feature = "async-runtime")]
        {
            return spawn_mir_task(Arc::clone(&self.ctx), name, args);
        }
        #[cfg(not(feature = "async-runtime"))]
        {
            run_mir_by_name(&self.ctx, name, args)
        }
    }

    fn join_task_value(&self, task_value: &Value) -> SyxResult<Value> {
        #[cfg(feature = "async-runtime")]
        {
            let id = syx_async::task_id_from_value(task_value)
                .ok_or_else(|| SyxDiagnostic::error("Expected task handle"))?;
            let handle = self
                .ctx
                .tasks
                .get(id)
                .ok_or_else(|| SyxDiagnostic::error(format!("Unknown task id {id}")))?;
            // ADR-261 / ADR-605: join reaps from live_count (align VM join_task).
            let result = handle.join();
            handle.mark_reaped();
            return result;
        }
        #[cfg(not(feature = "async-runtime"))]
        {
            let _ = task_value;
            Ok(Value::Nil)
        }
    }

    fn cancel_task_value(&self, task_value: &Value) -> SyxResult<()> {
        #[cfg(feature = "async-runtime")]
        {
            let id = syx_async::task_id_from_value(task_value)
                .ok_or_else(|| SyxDiagnostic::error("Expected task handle"))?;
            if let Some(handle) = self.ctx.tasks.get(id) {
                handle.cancel.cancel();
            }
            return Ok(());
        }
        #[cfg(not(feature = "async-runtime"))]
        {
            let _ = task_value;
            Ok(())
        }
    }

    fn task_sleep_ms(&self, ms: i64) -> SyxResult<()> {
        // ADR-607: cooperative cancel (align VM await_sleep_ms). ≠ Deadline sleep GA claim.
        const SLICE_MS: u64 = 10;
        let mut remaining = ms.max(0) as u64;
        while remaining > 0 {
            #[cfg(feature = "async-runtime")]
            if syx_async::task_should_stop() {
                return Err(SyxDiagnostic::error("Task was cancelled").with_code("C004"));
            }
            let slice = remaining.min(SLICE_MS);
            thread::sleep(Duration::from_millis(slice));
            remaining = remaining.saturating_sub(slice);
        }
        Ok(())
    }

    fn main_actor_run_fn(&self, name: &str, args: Vec<Value>) -> SyxResult<Value> {
        run_mir_by_name(&self.ctx, name, args)
    }

    fn await_task_value(&self, task_value: &Value) -> SyxResult<Value> {
        self.join_task_value(task_value)
    }

    fn try_await_task_value(&self, task_value: &Value) -> Option<SyxResult<Value>> {
        #[cfg(feature = "async-runtime")]
        {
            let id = syx_async::task_id_from_value(task_value)?;
            let handle = self.ctx.tasks.get(id)?;
            return handle.try_join();
        }
        #[cfg(not(feature = "async-runtime"))]
        {
            let _ = task_value;
            None
        }
    }

    fn actor_send_method(
        &self,
        actor_value: &Value,
        method_name: &str,
        args: Vec<Value>,
    ) -> SyxResult<Value> {
        #[cfg(feature = "async-runtime")]
        {
            if syx_vm::actor_id_from_value(actor_value).is_none() {
                // ADR-631: args already include receiver (mir-run full param drain).
                return run_mir_by_name(&self.ctx, method_name, args);
            }
            let actor_id = syx_vm::actor_id_from_value(actor_value)
                .ok_or_else(|| SyxDiagnostic::error("Expected actor reference"))?;
            // ADR-631 / ADR-264: deny sync same-instance reentrancy (anti-deadlock).
            if syx_async::current_processing_actor() == Some(actor_id) {
                return Err(SyxDiagnostic::error("Actor reentrancy denied").with_code("C009"));
            }
            let (reply_tx, reply_rx) = crossbeam_channel::unbounded();
            let tx = ensure_native_actor(&self.ctx, actor_id);
            tx.send(NativeActorJob {
                method_name: method_name.to_string(),
                args,
                reply: reply_tx,
            })
            .map_err(|_| SyxDiagnostic::error("Actor mailbox closed").with_code("C006"))?;
            return reply_rx
                .recv()
                .unwrap_or(Err(SyxDiagnostic::error("Actor reply channel closed")));
        }
        #[cfg(not(feature = "async-runtime"))]
        {
            run_mir_by_name(&self.ctx, method_name, args)
        }
    }
}

pub fn native_vm_runtime_for(ctx: Arc<NativeRunContext>) -> VmRuntime {
    Arc::new(NativeMirBridge { ctx })
}

/// Returns runtime bound to the current thread's native context.
pub fn default_vm_runtime() -> VmRuntime {
    with_current_ctx(|ctx| native_vm_runtime_for(Arc::clone(ctx)))
}

static NATIVE_TASK_HOSTS: Once = Once::new();

fn parse_task_detach_args(stack: &mut Vec<Value>) -> (String, Vec<Value>) {
    let top = stack.pop().unwrap_or(Value::Nil);
    let bottom = stack.pop();
    match (&top, bottom.as_ref()) {
        (Value::Str(n), Some(Value::Array(items))) => (n.clone(), items.clone()),
        (Value::Str(n), Some(Value::Str(n2))) => (n.clone(), vec![Value::Str(n2.clone())]),
        (Value::Array(items), Some(Value::Str(n))) => (n.clone(), items.clone()),
        (Value::Str(n), _) => (n.clone(), vec![]),
        (_, Some(Value::Str(n))) => (n.clone(), vec![]),
        _ => (String::new(), vec![]),
    }
}

fn native_host_task_detach(stack: &mut Vec<Value>) -> SyxResult<()> {
    let (name, args) = parse_task_detach_args(stack);
    let handle = default_vm_runtime().spawn_detached_fn(&name, args)?;
    stack.push(handle);
    Ok(())
}

fn native_host_task_join(stack: &mut Vec<Value>) -> SyxResult<()> {
    let task = stack.pop().unwrap_or(Value::Nil);
    let value = default_vm_runtime().join_task_value(&task)?;
    let mut map = HashMap::new();
    map.insert("__done".into(), Value::Bool(true));
    map.insert("__value".into(), value);
    stack.push(Value::Object(map));
    Ok(())
}

fn native_host_task_cancel(stack: &mut Vec<Value>) -> SyxResult<()> {
    let task = stack.pop().unwrap_or(Value::Nil);
    default_vm_runtime().cancel_task_value(&task)?;
    stack.push(Value::Bool(true));
    Ok(())
}

fn native_host_task_sleep(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i32() as i64;
    default_vm_runtime().task_sleep_ms(ms)?;
    Ok(())
}

fn native_host_noop(_stack: &mut Vec<Value>) -> SyxResult<()> {
    Ok(())
}

#[cfg(feature = "async-runtime")]
fn native_host_task_group_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let parent = syx_async::current_task_cancel().unwrap_or_default();
    let (id, group) = with_current_ctx(|ctx| {
        let id = ctx.next_group_id.fetch_add(1, Ordering::Relaxed);
        let group = Arc::new(TaskGroup::new(parent));
        ctx.groups
            .lock()
            .expect("native ctx groups lock")
            .insert(id, Arc::clone(&group));
        (id, group)
    });
    stack.push(group.to_value_with_id(id));
    Ok(())
}

#[cfg(feature = "async-runtime")]
fn native_host_task_group_add(stack: &mut Vec<Value>) -> SyxResult<()> {
    let task = stack.pop().unwrap_or(Value::Nil);
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let task_id = syx_async::task_id_from_value(&task)
        .ok_or_else(|| SyxDiagnostic::error("Expected task"))?;
    let group_id = syx_async::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    with_current_ctx(|ctx| {
        let handle = ctx
            .tasks
            .get(task_id)
            .ok_or_else(|| SyxDiagnostic::error("Unknown task"))?;
        let group = ctx
            .groups
            .lock()
            .expect("native ctx groups lock")
            .get(&group_id)
            .cloned()
            .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
        group.add(handle);
        Ok(())
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

#[cfg(feature = "async-runtime")]
fn native_host_task_group_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let group_id = syx_async::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    with_current_ctx(|ctx| {
        let group = ctx
            .groups
            .lock()
            .expect("native ctx groups lock")
            .get(&group_id)
            .cloned()
            .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
        group.wait_all()
    })?;
    stack.push(Value::Nil);
    Ok(())
}

#[cfg(feature = "async-runtime")]
fn native_host_task_group_cancel(stack: &mut Vec<Value>) -> SyxResult<()> {
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let group_id = syx_async::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    with_current_ctx(|ctx| {
        let group = ctx
            .groups
            .lock()
            .expect("native ctx groups lock")
            .get(&group_id)
            .cloned()
            .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
        group.cancel_all();
        Ok(())
    })?;
    stack.push(Value::Nil);
    Ok(())
}

#[cfg(feature = "async-runtime")]
/// ADR-263 / ADR-605: task-local set on native CallHost path.
fn native_host_task_local_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Nil);
    let key = stack.pop().unwrap_or(Value::Nil).as_str();
    syx_async::task_local_set(key, value);
    stack.push(Value::Nil);
    Ok(())
}

#[cfg(feature = "async-runtime")]
/// ADR-263 / ADR-605: task-local get (Nil if missing).
fn native_host_task_local_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = stack.pop().unwrap_or(Value::Nil).as_str();
    stack.push(syx_async::task_local_get(&key));
    Ok(())
}

#[cfg(feature = "async-runtime")]
/// ADR-262 / ADR-605: set task-scoped deadline (ms from now); no sleep/C004 claim.
fn native_host_task_set_deadline_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64().max(0) as u64;
    syx_async::set_deadline_ms(ms);
    syx_async::apply_deadline_to_cancel();
    stack.push(Value::Nil);
    Ok(())
}

#[cfg(feature = "async-runtime")]
/// ADR-261 / ADR-605: unreaped TaskRegistry live_count via native run context.
/// Must not call `syx_async::task_live_count()` — that path is VM-runtime-scoped and is 0 here.
fn native_host_task_live_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = with_current_ctx(|ctx| ctx.tasks.live_count());
    stack.push(platform_int_value(n as i64));
    Ok(())
}

#[cfg(feature = "async-runtime")]
/// ADR-631 / ADR-264: reentrancy policy fingerprint on native CallHost path.
fn native_host_actor_reentrancy_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str(syx_async::actor_reentrancy_policy().into()));
    Ok(())
}

/// Register __syx_task_* host handlers for native MIR `CallHost` paths.
pub fn install_native_task_hosts() {
    NATIVE_TASK_HOSTS.call_once(|| {
        #[cfg(feature = "async-runtime")]
        {
            register_task_hosts(
                native_host_task_detach,
                native_host_task_join,
                native_host_task_cancel,
                native_host_task_sleep,
                native_host_noop, // main_actor_run
                native_host_noop, // await_task
                native_host_task_group_create,
                native_host_task_group_add,
                native_host_task_group_wait,
            );
            register_task_group_cancel_host(native_host_task_group_cancel);
            // ADR-605: TaskLocal*/Deadline/LiveCount llvm=real narrow4.
            register_task_local_hosts(native_host_task_local_set, native_host_task_local_get);
            register_task_set_deadline_ms_host(native_host_task_set_deadline_ms);
            register_task_live_count_host(native_host_task_live_count);
            // ADR-631: policy fingerprint for native_mir A-04 actor nails.
            register_actor_reentrancy_policy_host(native_host_actor_reentrancy_policy);
        }
        #[cfg(not(feature = "async-runtime"))]
        {
            register_task_hosts(
                native_host_task_detach,
                native_host_task_join,
                native_host_task_cancel,
                native_host_task_sleep,
                native_host_noop,
                native_host_noop,
                native_host_noop,
                native_host_noop,
                native_host_noop,
            );
        }
    });
}

static NATIVE_UNIFIED_GC_HOSTS: Once = Once::new();

fn native_host_unified_gc_promote(stack: &mut Vec<Value>) -> SyxResult<()> {
    let slot = stack.pop().map(|v| v.as_i32()).unwrap_or(0);
    stack.push(Value::I32(syx_vm::unified_gc_promote(slot)));
    Ok(())
}

fn native_host_unified_gc_promotion_collect(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::I32(syx_vm::unified_gc_promotion_collect()));
    Ok(())
}

/// Phase 35 FL1: wire unified GC promotion host fns into native MIR bridge.
pub fn install_native_unified_gc_bridge() {
    NATIVE_UNIFIED_GC_HOSTS.call_once(|| {
        if syx_rt_unified_gc_bridge_enabled() {
            let _ = (
                native_host_unified_gc_promote,
                native_host_unified_gc_promotion_collect,
            );
        }
    });
}
