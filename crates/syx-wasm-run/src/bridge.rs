use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, Once};
use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use syx_async::{TaskGroup, TaskHandle, TaskRegistry};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::MirModule;
use syx_mir_run::{run_mir_function_with_args, MirRuntime};
use syx_runtime_core::{platform_int_value, RuntimeBridge, Value, VmRuntime};
use syx_vm::actor_id_from_value;
use syx_vm::{
    register_actor_reentrancy_policy_host, register_bridge_task_group_ops,
    register_bridge_task_live_count, register_task_group_cancel_host, register_task_live_count_host,
    register_task_local_hosts, register_task_set_deadline_ms_host, BridgeTaskGroupOps,
};

struct ActorJob {
    method_name: String,
    args: Vec<Value>,
    reply: Sender<SyxResult<Value>>,
}

/// Per-run WASM/MIR execution context (isolated for parallel tests).
pub struct WasmRunContext {
    module: Mutex<Option<Arc<MirModule>>>,
    pub printed: Mutex<Vec<String>>,
    tasks: TaskRegistry,
    /// Wave A W276 / ADR-620: TaskGroup registry (align NativeRunContext).
    groups: Mutex<HashMap<u64, Arc<TaskGroup>>>,
    next_group_id: AtomicU64,
    actors: Mutex<HashMap<i64, Sender<ActorJob>>>,
}

impl Default for WasmRunContext {
    fn default() -> Self {
        Self {
            module: Mutex::new(None),
            printed: Mutex::new(Vec::new()),
            tasks: TaskRegistry::default(),
            groups: Mutex::new(HashMap::new()),
            next_group_id: AtomicU64::new(1),
            actors: Mutex::new(HashMap::new()),
        }
    }
}

impl WasmRunContext {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn with_module(module: Arc<MirModule>) -> Arc<Self> {
        let ctx = Self::new();
        *ctx.module.lock().expect("wasm ctx module lock") = Some(module);
        ctx
    }

    pub fn set_module(&self, module: Arc<MirModule>) {
        *self.module.lock().expect("wasm ctx module lock") = Some(module);
    }

    pub fn module(&self) -> Option<Arc<MirModule>> {
        self.module.lock().expect("wasm ctx module lock").clone()
    }

    pub fn clear_ephemeral(&self) {
        self.printed.lock().expect("wasm ctx printed lock").clear();
        self.tasks.clear();
        self.groups.lock().expect("wasm ctx groups lock").clear();
        self.actors.lock().expect("wasm ctx actors lock").clear();
    }

    pub fn printed_snapshot(&self) -> Vec<String> {
        self.printed.lock().expect("wasm ctx printed lock").clone()
    }
}

thread_local! {
    static CURRENT_WASM_CTX: RefCell<Option<Arc<WasmRunContext>>> = const { RefCell::new(None) };
}

fn with_current_ctx<F, R>(f: F) -> R
where
    F: FnOnce(&Arc<WasmRunContext>) -> R,
{
    CURRENT_WASM_CTX.with(|cell| {
        let ctx = cell.borrow();
        let ctx = ctx
            .as_ref()
            .expect("wasm run context not installed on this thread");
        f(ctx)
    })
}

fn with_current_ctx_opt<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&Arc<WasmRunContext>) -> R,
{
    CURRENT_WASM_CTX.with(|cell| cell.borrow().as_ref().map(f))
}

fn set_current_ctx(ctx: Arc<WasmRunContext>) {
    CURRENT_WASM_CTX.with(|cell| {
        *cell.borrow_mut() = Some(ctx);
    });
}

fn bridge_live_count_provider() -> Option<usize> {
    with_current_ctx_opt(|ctx| ctx.tasks.live_count())
}

fn wasm_ctx_active() -> bool {
    with_current_ctx_opt(|_| ()).is_some()
}

fn wasm_host_task_local_set(stack: &mut Vec<Value>) -> SyxResult<()> {
    let value = stack.pop().unwrap_or(Value::Nil);
    let key = stack.pop().unwrap_or(Value::Nil).as_str();
    syx_async::task_local_set(key, value);
    stack.push(Value::Nil);
    Ok(())
}

fn wasm_host_task_local_get(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = stack.pop().unwrap_or(Value::Nil).as_str();
    stack.push(syx_async::task_local_get(&key));
    Ok(())
}

fn wasm_host_task_set_deadline_ms(stack: &mut Vec<Value>) -> SyxResult<()> {
    let ms = stack.pop().unwrap_or(Value::I32(0)).as_i64().max(0) as u64;
    syx_async::set_deadline_ms(ms);
    syx_async::apply_deadline_to_cancel();
    stack.push(Value::Nil);
    Ok(())
}

fn wasm_host_task_live_count(stack: &mut Vec<Value>) -> SyxResult<()> {
    let n = with_current_ctx(|ctx| ctx.tasks.live_count());
    stack.push(platform_int_value(n as i64));
    Ok(())
}

fn wasm_host_task_group_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let parent = syx_async::current_task_cancel().unwrap_or_default();
    let (id, group) = with_current_ctx(|ctx| {
        let id = ctx.next_group_id.fetch_add(1, Ordering::Relaxed);
        let group = Arc::new(TaskGroup::new(parent));
        ctx.groups
            .lock()
            .expect("wasm ctx groups lock")
            .insert(id, Arc::clone(&group));
        (id, group)
    });
    stack.push(group.to_value_with_id(id));
    Ok(())
}

fn wasm_host_task_group_add(stack: &mut Vec<Value>) -> SyxResult<()> {
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
            .expect("wasm ctx groups lock")
            .get(&group_id)
            .cloned()
            .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
        group.add(handle);
        Ok(())
    })?;
    stack.push(Value::Bool(true));
    Ok(())
}

fn wasm_host_task_group_wait(stack: &mut Vec<Value>) -> SyxResult<()> {
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let group_id = syx_async::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    with_current_ctx(|ctx| {
        let group = ctx
            .groups
            .lock()
            .expect("wasm ctx groups lock")
            .get(&group_id)
            .cloned()
            .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
        group.wait_all()
    })?;
    stack.push(Value::Nil);
    Ok(())
}

fn wasm_host_task_group_cancel(stack: &mut Vec<Value>) -> SyxResult<()> {
    let group_val = stack.pop().unwrap_or(Value::Nil);
    let group_id = syx_async::group_id_from_value(&group_val)
        .ok_or_else(|| SyxDiagnostic::error("Expected task group"))?;
    with_current_ctx(|ctx| {
        let group = ctx
            .groups
            .lock()
            .expect("wasm ctx groups lock")
            .get(&group_id)
            .cloned()
            .ok_or_else(|| SyxDiagnostic::error("Unknown task group"))?;
        group.cancel_all();
        Ok(())
    })?;
    stack.push(Value::Nil);
    Ok(())
}

static WASM_TASK_HOSTS: Once = Once::new();

/// ADR-631 / ADR-264: reentrancy policy fingerprint on wasm CallHost path.
fn wasm_host_actor_reentrancy_policy(stack: &mut Vec<Value>) -> SyxResult<()> {
    stack.push(Value::Str(syx_async::actor_reentrancy_policy().into()));
    Ok(())
}

/// Wave A W275/W276: register Task Local/Deadline/LiveCount + TaskGroup for wasm CallHost.
/// Best-effort OnceLock set; always installs bridge providers for VM-host fallback.
/// Do NOT call `register_task_hosts` with noops — VM may lose the race and break detach/join/sleep.
pub fn install_wasm_task_hosts() {
    WASM_TASK_HOSTS.call_once(|| {
        register_task_local_hosts(wasm_host_task_local_set, wasm_host_task_local_get);
        register_task_set_deadline_ms_host(wasm_host_task_set_deadline_ms);
        register_task_live_count_host(wasm_host_task_live_count);
        register_bridge_task_live_count(bridge_live_count_provider);
        // TaskGroup: prefer bridge ops (host_fn_for / syx-async check active TLS).
        // Best-effort cancel OnceLock when VM has not claimed it yet.
        register_task_group_cancel_host(wasm_host_task_group_cancel);
        register_bridge_task_group_ops(BridgeTaskGroupOps {
            active: wasm_ctx_active,
            create: wasm_host_task_group_create,
            add: wasm_host_task_group_add,
            wait: wasm_host_task_group_wait,
            cancel: wasm_host_task_group_cancel,
        });
        // ADR-631: policy fingerprint for wasm A-04 actor nails.
        register_actor_reentrancy_policy_host(wasm_host_actor_reentrancy_policy);
    });
}

/// Install MIR module into an isolated run context (returns context for runtime wiring).
pub fn install_mir_module(module: Arc<MirModule>) -> Arc<WasmRunContext> {
    install_wasm_task_hosts();
    let ctx = WasmRunContext::with_module(module);
    ctx.clear_ephemeral();
    set_current_ctx(Arc::clone(&ctx));
    ctx
}

fn prepare_mir_runtime(rt: &mut MirRuntime, module: &MirModule, ctx: Arc<WasmRunContext>) {
    rt.install_functions(module.functions.clone());
    rt.set_runtime(wasm_vm_runtime_for(Arc::clone(&ctx)));
    syx_mir_run::install_module_string_pool(rt, module);
}

fn run_mir_by_name(ctx: &Arc<WasmRunContext>, name: &str, args: Vec<Value>) -> SyxResult<Value> {
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

fn spawn_mir_task(ctx: Arc<WasmRunContext>, name: &str, args: Vec<Value>) -> SyxResult<Value> {
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
    let ctx_thread = Arc::clone(&ctx);
    thread::spawn(move || {
        // ADR-620: worker must see wasm ctx + cancel token (sleep / CallHost path).
        set_current_ctx(Arc::clone(&ctx_thread));
        if cancel.is_cancelled() {
            let _ = done_tx.send(Err(
                SyxDiagnostic::error("Task cancelled").with_code("C004")
            ));
            return;
        }
        syx_async::set_current_task_cancel(cancel.clone());
        let result = (|| {
            if syx_async::task_should_stop() {
                return Err(SyxDiagnostic::error("Task cancelled").with_code("C004"));
            }
            let mut rt = MirRuntime::new();
            prepare_mir_runtime(&mut rt, &module, ctx_thread);
            run_mir_function_with_args(&func, &mut rt, &mut [], &args)
        })();
        syx_async::clear_current_task_cancel();
        let _ = done_tx.send(result);
    });
    Ok(handle.to_value())
}

fn ensure_actor(ctx: &Arc<WasmRunContext>, actor_id: i64) -> Sender<ActorJob> {
    let mut actors = ctx.actors.lock().expect("wasm ctx actors lock");
    if let Some(tx) = actors.get(&actor_id) {
        return tx.clone();
    }
    let ctx_actor = Arc::clone(ctx);
    let (tx, rx): (Sender<ActorJob>, Receiver<ActorJob>) = crossbeam_channel::unbounded();
    thread::spawn(move || {
        set_current_ctx(Arc::clone(&ctx_actor));
        let mut actor_states: HashMap<i64, Value> = HashMap::new();
        while let Ok(job) = rx.recv() {
            let result = (|| {
                let actor_id = job
                    .args
                    .first()
                    .and_then(actor_id_from_value)
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
                if actor_id != 0 && actor_id_from_value(&result).is_some() {
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

struct WasmMirBridge {
    ctx: Arc<WasmRunContext>,
}

impl RuntimeBridge for WasmMirBridge {
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
        spawn_mir_task(Arc::clone(&self.ctx), name, args)
    }

    fn join_task_value(&self, task_value: &Value) -> SyxResult<Value> {
        let id = syx_async::task_id_from_value(task_value)
            .ok_or_else(|| SyxDiagnostic::error("Expected task handle"))?;
        let handle = self
            .ctx
            .tasks
            .get(id)
            .ok_or_else(|| SyxDiagnostic::error(format!("Unknown task id {id}")))?;
        // ADR-261 / ADR-619: join reaps from live_count (align native llvm).
        let result = handle.join();
        handle.mark_reaped();
        result
    }

    fn cancel_task_value(&self, task_value: &Value) -> SyxResult<()> {
        let id = syx_async::task_id_from_value(task_value)
            .ok_or_else(|| SyxDiagnostic::error("Expected task handle"))?;
        if let Some(handle) = self.ctx.tasks.get(id) {
            handle.cancel.cancel();
        }
        Ok(())
    }

    fn task_sleep_ms(&self, ms: i64) -> SyxResult<()> {
        // ADR-607 / ADR-620: cooperative cancel (align native). ≠ Deadline sleep GA.
        const SLICE_MS: u64 = 10;
        let mut remaining = ms.max(0) as u64;
        while remaining > 0 {
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
        let id = syx_async::task_id_from_value(task_value)?;
        let handle = self.ctx.tasks.get(id)?;
        handle.try_join()
    }

    fn actor_send_method(
        &self,
        actor_value: &Value,
        method_name: &str,
        args: Vec<Value>,
    ) -> SyxResult<Value> {
        let actor_id = actor_id_from_value(actor_value)
            .ok_or_else(|| SyxDiagnostic::error("Expected actor reference"))?;
        // ADR-631 / ADR-264: deny sync same-instance reentrancy (anti-deadlock).
        if syx_async::current_processing_actor() == Some(actor_id) {
            return Err(SyxDiagnostic::error("Actor reentrancy denied").with_code("C009"));
        }
        let (reply_tx, reply_rx) = crossbeam_channel::unbounded();
        let tx = ensure_actor(&self.ctx, actor_id);
        tx.send(ActorJob {
            method_name: method_name.to_string(),
            args,
            reply: reply_tx,
        })
        .map_err(|_| SyxDiagnostic::error("Actor mailbox closed").with_code("C006"))?;
        reply_rx
            .recv()
            .unwrap_or(Err(SyxDiagnostic::error("Actor reply channel closed")))
    }
}

pub fn wasm_vm_runtime_for(ctx: Arc<WasmRunContext>) -> VmRuntime {
    set_current_ctx(Arc::clone(&ctx));
    Arc::new(WasmMirBridge { ctx })
}
