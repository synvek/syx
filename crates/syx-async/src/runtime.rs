// Phase 134 Track E: bounded Task queue + Abort reject MVP.
// Phase 135 Track E: CallerRuns reject policy MVP.
// Phase 136 Track E: Task pool core / max / keepAlive MVP.
// Phase 137 Track E: DiscardOldest reject policy MVP.
// Phase 138 Track E: Actor shared pool MVP (opt-in N threads drain multiple mailboxes).
// Fingerprint: `runtime-concurrency-v2:bounded-queue-abort-mvp`,
// `runtime-concurrency-v3:caller-runs-mvp`,
// `runtime-concurrency-v4:core-max-keepalive-mvp`,
// `runtime-concurrency-v5:discard-oldest-mvp`,
// `runtime-concurrency-v6:actor-shared-pool-mvp`

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU8, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use parking_lot::{Mutex, RwLock};
use syx_codegen::{CompiledProgram, FunctionUnit};
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_mir::MirOp;
use syx_vm::{JitCompiler, Value, VmInstance, VmRuntime};

use crate::cancellation::CancellationToken;
use crate::task::{TaskGroup, TaskHandle, TaskRegistry};

static NEXT_GROUP_ID: AtomicU64 = AtomicU64::new(1);

/// Rejection policy when the bounded Task queue is full.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskRejectPolicy {
    /// Fail with **C008** (Phase 134 default).
    Abort,
    /// Run the task synchronously on the submitting thread (Phase 135).
    CallerRuns,
    /// Drop the oldest queued task and enqueue the new one (Phase 137).
    DiscardOldest,
}

impl TaskRejectPolicy {
    fn as_code(self) -> u8 {
        match self {
            Self::Abort => 0,
            Self::CallerRuns => 1,
            Self::DiscardOldest => 2,
        }
    }

    fn from_code(code: u8) -> Self {
        match code {
            1 => Self::CallerRuns,
            2 => Self::DiscardOldest,
            _ => Self::Abort,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abort => "abort",
            Self::CallerRuns => "callerRuns",
            Self::DiscardOldest => "discardOldest",
        }
    }
}

fn parse_task_reject_policy(raw: &str) -> Option<TaskRejectPolicy> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "abort" => Some(TaskRejectPolicy::Abort),
        "callerruns" | "caller_runs" | "caller-runs" => Some(TaskRejectPolicy::CallerRuns),
        "discardoldest" | "discard_oldest" | "discard-oldest" => {
            Some(TaskRejectPolicy::DiscardOldest)
        }
        _ => None,
    }
}

/// Process-wide Task worker preference (`0` → resolve via env / num_cpus).
static CONFIGURED_WORKERS: AtomicUsize = AtomicUsize::new(0);
/// Task submission queue capacity (`0` = unbounded; `>0` bounded with reject policy).
static CONFIGURED_TASK_QUEUE_CAPACITY: AtomicI64 = AtomicI64::new(0);
/// Task reject policy when bounded queue is full (`255` = unset; `0` = Abort; `1` = CallerRuns; `2` = DiscardOldest).
static CONFIGURED_TASK_REJECT_POLICY: AtomicU8 = AtomicU8::new(255);
/// Phase 136: core pool size (`0` → resolve via env / workers).
static CONFIGURED_CORE_POOL: AtomicUsize = AtomicUsize::new(0);
/// Phase 136: maximum pool size (`0` → resolve via env / core).
static CONFIGURED_MAX_POOL: AtomicUsize = AtomicUsize::new(0);
/// Phase 136: idle keep-alive ms for non-core workers (`0` → resolve via env / default).
static CONFIGURED_KEEP_ALIVE_MS: AtomicI64 = AtomicI64::new(-1);
/// Max live actor executors (`0` = unlimited).
static MAX_ACTORS: AtomicI64 = AtomicI64::new(0);
/// Default actor mailbox capacity (`0` = unbounded; `>0` bounded).
static MAILBOX_CAPACITY: AtomicI64 = AtomicI64::new(0);
/// Phase 138: opt-in shared actor pool (`false` = one thread per actor, default).
static ACTOR_SHARED_POOL_ENABLED: AtomicBool = AtomicBool::new(false);
/// Phase 138: shared pool worker count (`0` = resolve via env / default 2).
static ACTOR_SHARED_POOL_SIZE: AtomicUsize = AtomicUsize::new(0);
/// ADR-276 / M3-B01: blocking pool size (`0` → default 1).
static CONFIGURED_BLOCKING_POOL_SIZE: AtomicUsize = AtomicUsize::new(0);

fn try_mir_fastpath(
    program: &CompiledProgram,
    func: &FunctionUnit,
    args: &[Value],
    printed: &mut Vec<String>,
) -> Option<SyxResult<Value>> {
    if !JitCompiler::fastpath_enabled() {
        return None;
    }
    let mir_fn = program.mir_functions.get(&func.name)?;
    if mir_fn.blocks.is_empty() {
        return None;
    }
    let has_async = mir_fn.blocks.iter().any(|b| {
        b.instructions.iter().any(|i| {
            matches!(i.op, MirOp::AwaitPoll | MirOp::AwaitSuspend { .. })
        })
    });
    if has_async {
        return None;
    }
    let has_blocking_serve = mir_fn.blocks.iter().any(|b| {
        b.instructions.iter().any(|i| {
            if let MirOp::CallHost(idx) = i.op {
                program
                    .host_fn_names
                    .get(idx as usize)
                    .is_some_and(|n| n == "__syx_http_serve")
            } else {
                false
            }
        })
    });
    if has_blocking_serve {
        return None;
    }
    let has_task_hosts = mir_fn.blocks.iter().any(|b| {
        b.instructions.iter().any(|i| {
            if let MirOp::CallHost(idx) = i.op {
                program
                    .host_fn_names
                    .get(idx as usize)
                    .is_some_and(|n| {
                        n.starts_with("__syx_task_")
                            || n.starts_with("__syx_await_task")
                            || n == "__syx_main_actor_run"
                    })
            } else {
                false
            }
        })
    });
    if has_task_hosts {
        return None;
    }
    if mir_fn.blocks.iter().any(|b| {
        b.instructions
            .iter()
            .any(|i| matches!(i.op, MirOp::Call(_)))
    }) {
        return None;
    }
    let mut jit = JitCompiler::default();
    jit.record_call(func.name.as_str());
    let _ = jit.maybe_compile(func.name.as_str(), program);
    if !jit.fastpath_ready(func.name.as_str()) {
        return None;
    }
    // Phase 14: tier-1 mir-run SSA when blocks are available.
    if std::env::var("SYX_JIT_MIR_RUN")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true)
    {
        let mut rt = syx_mir_run::MirRuntime::new();
        let fns: Vec<_> = program.mir_functions.values().cloned().collect();
        rt.install_functions(fns);
        let result = syx_mir_run::run_mir_function_with_args(mir_fn, &mut rt, &mut [], args);
        printed.extend(rt.printed.clone());
        return Some(result);
    }
    let mut vm = VmInstance::default();
    Some(
        vm.run_function_with_args(func, program, args).map(|v| {
            printed.extend(vm.printed.clone());
            v
        }),
    )
}

struct ThreadPool {
    tx: JobSender,
    shared_rx: Receiver<Box<dyn FnOnce() + Send>>,
    live_workers: Arc<AtomicUsize>,
    core: usize,
    max: usize,
    keep_alive_ms: u64,
}

enum JobSender {
    Unbounded(crossbeam_channel::Sender<Box<dyn FnOnce() + Send>>),
    Bounded(crossbeam_channel::Sender<Box<dyn FnOnce() + Send>>),
}

impl ThreadPool {
    fn new(core: usize, max: usize, keep_alive_ms: u64, queue_capacity: i64) -> Self {
        let core = core.max(1);
        let max = max.max(core);
        let (tx, rx): (
            crossbeam_channel::Sender<Box<dyn FnOnce() + Send>>,
            Receiver<Box<dyn FnOnce() + Send>>,
        ) = if queue_capacity > 0 {
            crossbeam_channel::bounded(queue_capacity as usize)
        } else {
            crossbeam_channel::unbounded()
        };
        let job_tx = if queue_capacity > 0 {
            JobSender::Bounded(tx)
        } else {
            JobSender::Unbounded(tx)
        };
        let live_workers = Arc::new(AtomicUsize::new(0));
        let pool = Self {
            tx: job_tx,
            shared_rx: rx,
            live_workers,
            core,
            max,
            keep_alive_ms,
        };
        for _ in 0..core {
            pool.spawn_worker(true);
        }
        pool
    }

    fn spawn_worker(&self, is_core: bool) {
        let rx = self.shared_rx.clone();
        let live = Arc::clone(&self.live_workers);
        let core = self.core;
        let keep_alive_ms = self.keep_alive_ms;
        live.fetch_add(1, Ordering::Relaxed);
        thread::spawn(move || {
            loop {
                let job = if !is_core && keep_alive_ms > 0 {
                    match rx.recv_timeout(Duration::from_millis(keep_alive_ms)) {
                        Ok(job) => job,
                        Err(RecvTimeoutError::Timeout) => {
                            let cur = live.load(Ordering::Relaxed);
                            if cur > core
                                && live
                                    .compare_exchange(
                                        cur,
                                        cur - 1,
                                        Ordering::SeqCst,
                                        Ordering::Relaxed,
                                    )
                                    .is_ok()
                            {
                                return;
                            }
                            continue;
                        }
                        Err(RecvTimeoutError::Disconnected) => {
                            live.fetch_sub(1, Ordering::Relaxed);
                            return;
                        }
                    }
                } else {
                    match rx.recv() {
                        Ok(job) => job,
                        Err(_) => {
                            live.fetch_sub(1, Ordering::Relaxed);
                            return;
                        }
                    }
                };
                job();
            }
        });
    }

    fn try_grow(&self) {
        if self.live_workers.load(Ordering::Relaxed) < self.max {
            self.spawn_worker(false);
        }
    }

    fn try_spawn<F>(&self, f: F) -> SyxResult<()>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        match &self.tx {
            JobSender::Unbounded(tx) => {
                let _ = tx.send(job);
                Ok(())
            }
            JobSender::Bounded(tx) => match tx.try_send(job) {
                Ok(()) => Ok(()),
                Err(crossbeam_channel::TrySendError::Full(job)) => {
                    if resolve_task_reject_policy() == TaskRejectPolicy::DiscardOldest {
                        let _ = self.shared_rx.try_recv();
                        match tx.try_send(job) {
                            Ok(()) => return Ok(()),
                            Err(crossbeam_channel::TrySendError::Full(job)) => {
                                return reject_or_caller_runs(job);
                            }
                            Err(crossbeam_channel::TrySendError::Disconnected(_)) => {
                                return Err(
                                    SyxDiagnostic::error("Task queue closed").with_code("C008")
                                );
                            }
                        }
                    }
                    if self.live_workers.load(Ordering::Relaxed) < self.max {
                        self.try_grow();
                        match tx.try_send(job) {
                            Ok(()) => return Ok(()),
                            Err(crossbeam_channel::TrySendError::Full(job)) => {
                                if resolve_task_reject_policy() == TaskRejectPolicy::DiscardOldest
                                {
                                    let _ = self.shared_rx.try_recv();
                                    match tx.try_send(job) {
                                        Ok(()) => return Ok(()),
                                        Err(crossbeam_channel::TrySendError::Full(job)) => {
                                            return reject_or_caller_runs(job);
                                        }
                                        Err(crossbeam_channel::TrySendError::Disconnected(_)) => {
                                            return Err(
                                                SyxDiagnostic::error("Task queue closed")
                                                    .with_code("C008")
                                            );
                                        }
                                    }
                                }
                                return reject_or_caller_runs(job);
                            }
                            Err(crossbeam_channel::TrySendError::Disconnected(_)) => {
                                return Err(
                                    SyxDiagnostic::error("Task queue closed").with_code("C008")
                                );
                            }
                        }
                    }
                    reject_or_caller_runs(job)
                }
                Err(crossbeam_channel::TrySendError::Disconnected(_)) => {
                    Err(SyxDiagnostic::error("Task queue closed").with_code("C008"))
                }
            },
        }
    }

    fn live_count(&self) -> usize {
        self.live_workers.load(Ordering::Relaxed)
    }

    /// Steal one pending job without blocking (for help-while-waiting on blocking pool).
    fn try_recv_job(&self) -> Option<Box<dyn FnOnce() + Send>> {
        self.shared_rx.try_recv().ok()
    }
}

fn reject_or_caller_runs(job: Box<dyn FnOnce() + Send>) -> SyxResult<()> {
    match resolve_task_reject_policy() {
        TaskRejectPolicy::CallerRuns => {
            job();
            Ok(())
        }
        TaskRejectPolicy::DiscardOldest => Err(SyxDiagnostic::error(
            "Task queue full (DiscardOldest could not enqueue after drop)",
        )
        .with_code("C008")),
        TaskRejectPolicy::Abort => {
            Err(SyxDiagnostic::error("Task queue full (Abort policy)").with_code("C008"))
        }
    }
}

struct ActorMessage {
    method_idx: u32,
    args: Vec<Value>,
    reply: Sender<SyxResult<Value>>,
}

enum ActorDispatch {
    Dedicated(Sender<ActorMessage>),
    Shared,
}

struct SharedActorJob {
    actor_id: i64,
    msg: ActorMessage,
}

pub struct SyxRuntime {
    pub program: Arc<CompiledProgram>,
    pub globals: RwLock<Vec<Value>>,
    pub printed: Mutex<Vec<String>>,
    pub tasks: TaskRegistry,
    thread_pool: Mutex<ThreadPool>,
    /// ADR-275 / M3-B01: dedicated pool for blockingSleep (≠ Task pool).
    blocking_pool: Mutex<ThreadPool>,
    /// ADR-276: configured blocking pool size (core=max=n).
    blocking_pool_size: AtomicUsize,
    worker_count: AtomicUsize,
    task_queue_capacity: AtomicI64,
    core_pool_size: AtomicUsize,
    max_pool_size: AtomicUsize,
    keep_alive_ms: AtomicI64,
    pub groups: Mutex<HashMap<u64, Arc<TaskGroup>>>,
    await_waiters: Mutex<HashMap<u64, Vec<Sender<SyxResult<Value>>>>>,
    actors: Mutex<HashMap<i64, ActorDispatch>>,
    actor_states: Mutex<HashMap<i64, Value>>,
    shared_pool_tx: Mutex<Option<Sender<SharedActorJob>>>,
    pub strict_concurrency: bool,
}

impl SyxRuntime {
    pub fn new(program: CompiledProgram) -> Self {
        let core = resolve_core_pool_size();
        let max = resolve_max_pool_size(core);
        let keep_alive = resolve_keep_alive_ms();
        let queue_capacity = resolve_task_queue_capacity();
        let globals = vec![Value::Nil; program.globals.len()];
        Self {
            program: Arc::new(program),
            globals: RwLock::new(globals),
            printed: Mutex::new(Vec::new()),
            tasks: TaskRegistry::default(),
            thread_pool: Mutex::new(ThreadPool::new(core, max, keep_alive, queue_capacity)),
            // ADR-275/276: dedicated blocking pool (fixed size core=max=n; default 1).
            blocking_pool: Mutex::new(ThreadPool::new(
                resolve_blocking_pool_size(),
                resolve_blocking_pool_size(),
                keep_alive,
                0,
            )),
            blocking_pool_size: AtomicUsize::new(resolve_blocking_pool_size()),
            worker_count: AtomicUsize::new(core),
            task_queue_capacity: AtomicI64::new(queue_capacity),
            core_pool_size: AtomicUsize::new(core),
            max_pool_size: AtomicUsize::new(max),
            keep_alive_ms: AtomicI64::new(keep_alive as i64),
            groups: Mutex::new(HashMap::new()),
            await_waiters: Mutex::new(HashMap::new()),
            actors: Mutex::new(HashMap::new()),
            actor_states: Mutex::new(HashMap::new()),
            shared_pool_tx: Mutex::new(None),
            strict_concurrency: false,
        }
    }

    pub fn with_strict(mut self, strict: bool) -> Self {
        self.strict_concurrency = strict;
        self
    }

    pub fn run_program(self: &Arc<Self>) -> SyxResult<Value> {
        crate::hosts::install_hosts();
        set_current_runtime(Arc::clone(self));
        let result = self.run_on_current_thread()?;
        clear_current_runtime();
        Ok(result)
    }

    fn run_on_current_thread(self: &Arc<Self>) -> SyxResult<Value> {
        syx_vm::clear_registered_tests();
        syx_vm::set_program_ctx(syx_vm::ProgramCtx {
            program: Arc::clone(&self.program),
            runtime: Some(self.vm_runtime()),
        });
        let mut vm = self.make_vm();
        init_globals(&mut vm, &self.program)?;
        syx_vm::set_active_vm(&mut vm);
        let main = self
            .program
            .functions
            .iter()
            .find(|f| f.name == self.program.entry_point)
            .ok_or_else(|| {
                SyxDiagnostic::error(format!(
                    "No entry function '{}' found",
                    self.program.entry_point
                ))
            })?;
        let result = loop {
            vm.suspended = false;
            let result = if vm.suspend_frame.is_none() {
                try_mir_fastpath(&self.program, main, &[], &mut vm.printed)
                    .unwrap_or_else(|| vm.run_function(main, &self.program))
            } else {
                vm.run_function(main, &self.program)
            };
            if !vm.suspended {
                break result;
            }
            cooperative_yield();
        };
        let outcomes = if result.is_ok() {
            vm.drain_registered_tests(&self.program)
        } else {
            syx_vm::clear_registered_tests();
            Vec::new()
        };
        syx_vm::store_registered_test_outcomes(outcomes);
        syx_vm::clear_active_vm();
        syx_vm::clear_program_ctx();
        for line in vm.printed {
            self.record_print(line);
        }
        result
    }

    pub fn make_vm(self: &Arc<Self>) -> VmInstance {
        let mut vm = VmInstance::new();
        vm.runtime = Some(self.vm_runtime());
        vm
    }

    pub fn vm_runtime(self: &Arc<Self>) -> VmRuntime {
        Arc::new(RuntimeBridgeAdapter {
            inner: Arc::clone(self),
        })
    }

    pub fn spawn_detached(
        self: &Arc<Self>,
        fn_index: u32,
        args: Vec<Value>,
    ) -> SyxResult<Arc<TaskHandle>> {
        let (done_tx, done_rx) = crossbeam_channel::unbounded();
        let parent_cancel = crate::task::current_task_cancel().unwrap_or_default();
        let cancel = CancellationToken::with_parent(parent_cancel);
        let parent_deadline = crate::task::current_task_deadline();
        let parent_locals = crate::task::current_task_locals();
        let handle = Arc::new(TaskHandle::new_with_cancel(done_rx, cancel.clone()));
        let task_id = handle.id;

        let rt = Arc::clone(self);
        self.thread_pool.lock().try_spawn(move || {
            if cancel.is_cancelled() {
                let _ = done_tx.send(Err(SyxDiagnostic::error("Task cancelled").with_code("C004")));
                return;
            }
            set_current_runtime(rt.clone());
            crate::task::set_current_task_cancel(cancel.clone());
            crate::task::set_current_task_deadline(parent_deadline);
            crate::task::set_current_task_locals(parent_locals);
            // ADR-295: bind task id for Trace.contextLine.
            crate::task::set_current_task_id(Some(task_id));
            // Inherit: apply immediately if already past deadline.
            crate::task::apply_deadline_to_cancel();
            if cancel.is_cancelled() {
                crate::task::clear_current_task_id();
                crate::task::clear_current_task_locals();
                crate::task::clear_current_task_deadline();
                crate::task::clear_current_task_cancel();
                clear_current_runtime();
                let _ = done_tx.send(Err(SyxDiagnostic::error("Task cancelled").with_code("C004")));
                return;
            }
            syx_vm::set_program_ctx(syx_vm::ProgramCtx {
                program: Arc::clone(&rt.program),
                runtime: Some(rt.vm_runtime()),
            });
            let result = (|| {
                let mut vm = rt.make_vm();
                let func = rt
                    .program
                    .functions
                    .get(fn_index as usize)
                    .ok_or_else(|| SyxDiagnostic::error("Unknown function index"))?;
                loop {
                    if crate::task::task_should_stop() {
                        return Err(SyxDiagnostic::error("Task cancelled").with_code("C004"));
                    }
                    vm.suspended = false;
                    let result = vm.run_function_with_args(func, &rt.program, &args);
                    if !vm.suspended {
                        return result;
                    }
                    cooperative_yield();
                }
            })();
            syx_vm::clear_program_ctx();
            crate::task::clear_current_task_id();
            crate::task::clear_current_task_locals();
            crate::task::clear_current_task_deadline();
            crate::task::clear_current_task_cancel();
            clear_current_runtime();
            let _ = done_tx.send(result);
        })?;

        self.tasks.register(Arc::clone(&handle));
        Ok(handle)
    }

    pub fn spawn_detached_by_name(
        self: &Arc<Self>,
        name: &str,
        args: Vec<Value>,
    ) -> SyxResult<Arc<TaskHandle>> {
        let idx = self.function_index(name)?;
        self.spawn_detached(idx, args)
    }

    pub fn function_index(&self, name: &str) -> SyxResult<u32> {
        self.program
            .functions
            .iter()
            .position(|f| f.name == name)
            .map(|i| i as u32)
            .ok_or_else(|| SyxDiagnostic::error(format!("Unknown function '{name}'")))
    }

    pub fn join_task(&self, id: u64) -> SyxResult<Value> {
        let handle = self
            .tasks
            .get(id)
            .ok_or_else(|| SyxDiagnostic::error(format!("Unknown task id {id}")))?;
        let result = handle.join();
        // ADR-261: explicit join reaps from live_count (same as taskGroupWait).
        handle.mark_reaped();
        result
    }

    /// ADR-261 / M3 W2: unreaped TaskRegistry size (silent-orphan diagnostic).
    pub fn task_live_count(&self) -> usize {
        self.tasks.live_count()
    }

    pub fn cancel_task(&self, id: u64) -> SyxResult<()> {
        let handle = self
            .tasks
            .get(id)
            .ok_or_else(|| SyxDiagnostic::error(format!("Unknown task id {id}")))?;
        handle.cancel.cancel();
        Ok(())
    }

    pub fn record_print(&self, text: String) {
        self.printed.lock().push(text);
    }

    pub fn load_global(&self, idx: usize) -> Value {
        self.globals
            .read()
            .get(idx)
            .cloned()
            .unwrap_or(Value::Nil)
    }

    pub fn store_global(&self, idx: usize, value: Value) {
        let mut globals = self.globals.write();
        if globals.len() <= idx {
            globals.resize(idx + 1, Value::Nil);
        }
        globals[idx] = value;
    }

    pub fn register_await_waiter(&self, task_id: u64, reply: Sender<SyxResult<Value>>) {
        self.await_waiters
            .lock()
            .entry(task_id)
            .or_default()
            .push(reply);
    }

    pub fn complete_await(&self, task_id: u64, result: SyxResult<Value>) {
        if let Some(waiters) = self.await_waiters.lock().remove(&task_id) {
            for tx in waiters {
                let _ = tx.send(result.clone());
            }
        }
    }

    pub fn create_task_group(&self, parent_cancel: CancellationToken) -> (u64, Arc<TaskGroup>) {
        let id = NEXT_GROUP_ID.fetch_add(1, Ordering::Relaxed);
        let group = Arc::new(TaskGroup::new(parent_cancel));
        self.groups.lock().insert(id, Arc::clone(&group));
        (id, group)
    }

    pub fn actor_send(
        self: &Arc<Self>,
        actor_id: i64,
        method_idx: u32,
        args: Vec<Value>,
    ) -> SyxResult<Receiver<SyxResult<Value>>> {
        // ADR-264 / M3-A03: deny sync same-instance reentrancy (anti-deadlock).
        if current_processing_actor() == Some(actor_id) {
            return Err(SyxDiagnostic::error("Actor reentrancy denied").with_code("C009"));
        }
        let (reply_tx, reply_rx) = crossbeam_channel::unbounded();
        let msg = ActorMessage {
            method_idx,
            args,
            reply: reply_tx,
        };
        self.ensure_actor_registered(actor_id)?;
        if actor_shared_pool_enabled() {
            let pool_tx = self
                .shared_pool_tx
                .lock()
                .clone()
                .ok_or_else(|| SyxDiagnostic::error("Actor shared pool not started"))?;
            pool_tx
                .send(SharedActorJob { actor_id, msg })
                .map_err(|_| SyxDiagnostic::error("Actor shared pool closed").with_code("C006"))?;
            return Ok(reply_rx);
        }
        let tx = match self.actors.lock().get(&actor_id) {
            Some(ActorDispatch::Dedicated(tx)) => tx.clone(),
            _ => return Err(SyxDiagnostic::error("Actor executor missing").with_code("C006")),
        };
        match tx.try_send(msg) {
            Ok(()) => Ok(reply_rx),
            Err(crossbeam_channel::TrySendError::Full(msg)) => {
                // Bounded mailbox full: block like Channel.send (backpressure).
                tx.send(msg).map_err(|_| {
                    SyxDiagnostic::error("Actor mailbox closed").with_code("C006")
                })?;
                Ok(reply_rx)
            }
            Err(crossbeam_channel::TrySendError::Disconnected(_)) => Err(SyxDiagnostic::error(
                "Actor mailbox closed",
            )
            .with_code("C006")),
        }
    }

    fn ensure_actor_registered(self: &Arc<Self>, actor_id: i64) -> SyxResult<()> {
        if self.actors.lock().contains_key(&actor_id) {
            return Ok(());
        }
        let max = MAX_ACTORS.load(Ordering::Relaxed);
        if max > 0 && self.actors.lock().len() as i64 >= max {
            return Err(SyxDiagnostic::error(format!(
                "Actor limit exceeded (maxActors={max})"
            ))
            .with_code("C007"));
        }
        if actor_shared_pool_enabled() {
            self.ensure_shared_actor_pool()?;
            self.actors.lock().insert(actor_id, ActorDispatch::Shared);
            return Ok(());
        }
        let capacity = MAILBOX_CAPACITY.load(Ordering::Relaxed);
        let (tx, rx) = if capacity > 0 {
            crossbeam_channel::bounded::<ActorMessage>(capacity as usize)
        } else {
            crossbeam_channel::unbounded::<ActorMessage>()
        };
        let rt = Arc::clone(self);
        thread::spawn(move || {
            set_current_runtime(rt.clone());
            while let Ok(msg) = rx.recv() {
                let result = rt.dispatch_actor_message(actor_id, msg.method_idx, msg.args);
                let _ = msg.reply.send(result);
            }
            clear_current_runtime();
        });
        self.actors
            .lock()
            .insert(actor_id, ActorDispatch::Dedicated(tx));
        Ok(())
    }

    fn ensure_shared_actor_pool(self: &Arc<Self>) -> SyxResult<()> {
        let mut slot = self.shared_pool_tx.lock();
        if slot.is_some() {
            return Ok(());
        }
        let size = resolve_actor_shared_pool_size().max(1);
        let (pool_tx, pool_rx) = crossbeam_channel::unbounded::<SharedActorJob>();
        for _ in 0..size {
            let pool_rx = pool_rx.clone();
            let rt = Arc::clone(self);
            thread::spawn(move || {
                set_current_runtime(rt.clone());
                while let Ok(job) = pool_rx.recv() {
                    let result =
                        rt.dispatch_actor_message(job.actor_id, job.msg.method_idx, job.msg.args);
                    let _ = job.msg.reply.send(result);
                }
                clear_current_runtime();
            });
        }
        *slot = Some(pool_tx);
        Ok(())
    }

    fn dispatch_actor_message(
        self: &Arc<Self>,
        actor_id: i64,
        method_idx: u32,
        args: Vec<Value>,
    ) -> SyxResult<Value> {
        set_current_processing_actor(Some(actor_id));
        let result = self.dispatch_actor_message_inner(actor_id, method_idx, args);
        clear_current_processing_actor();
        result
    }

    fn dispatch_actor_message_inner(
        self: &Arc<Self>,
        actor_id: i64,
        method_idx: u32,
        args: Vec<Value>,
    ) -> SyxResult<Value> {
        let mut vm = self.make_vm();
        let func = self
            .program
            .functions
            .get(method_idx as usize)
            .ok_or_else(|| SyxDiagnostic::error("Unknown actor method"))?;
        let mut args = args;
        if actor_id != 0 {
            let mut states = self.actor_states.lock();
            let state = states
                .entry(actor_id)
                .or_insert_with(|| args.first().cloned().unwrap_or(Value::Nil));
            if !args.is_empty() {
                args[0] = state.clone();
            }
            let result = vm.run_function_with_args(func, &self.program, &args)?;
            if syx_vm::actor_id_from_value(&result).is_some() {
                states.insert(actor_id, result.clone());
            }
            Ok(result)
        } else {
            vm.run_function_with_args(func, &self.program, &args)
        }
    }

    /// Phase 133: set Task ThreadPool size; recreates the pool (also updates core).
    pub fn set_worker_count(self: &Arc<Self>, n: usize) -> usize {
        let n = n.max(1);
        CONFIGURED_WORKERS.store(n, Ordering::Relaxed);
        CONFIGURED_CORE_POOL.store(n, Ordering::Relaxed);
        self.worker_count.store(n, Ordering::Relaxed);
        self.core_pool_size.store(n, Ordering::Relaxed);
        let mut max = self.max_pool_size.load(Ordering::Relaxed);
        if max < n {
            max = n;
            CONFIGURED_MAX_POOL.store(max, Ordering::Relaxed);
            self.max_pool_size.store(max, Ordering::Relaxed);
        }
        let cap = self.task_queue_capacity.load(Ordering::Relaxed);
        let keep = self.keep_alive_ms.load(Ordering::Relaxed).max(0) as u64;
        *self.thread_pool.lock() = ThreadPool::new(n, max, keep, cap);
        n
    }

    /// ADR-275 / M3-B01: sleep on the dedicated `blocking_pool` (≠ Task pool).
    ///
    /// The calling Task worker help-drains pending Task-pool jobs while waiting so a
    /// size=1 Task pool is not starved. Cancel/deadline are observed via captured
    /// token + Instant (TLS is not inherited by blocking workers).
    pub fn blocking_sleep_ms(self: &Arc<Self>, ms: i64) {
        let ms = ms.max(0) as u64;
        let done = Arc::new(AtomicBool::new(false));
        let done_flag = Arc::clone(&done);
        let cancel = crate::task::current_task_cancel();
        let task_deadline = crate::task::current_task_deadline();
        let sleep_deadline = std::time::Instant::now() + Duration::from_millis(ms);
        {
            let pool = self.blocking_pool.lock();
            let _ = pool.try_spawn(move || {
                const SLICE_MS: u64 = 10;
                loop {
                    if cancel.as_ref().is_some_and(|t| t.is_cancelled()) {
                        break;
                    }
                    if task_deadline.is_some_and(|d| std::time::Instant::now() >= d) {
                        break;
                    }
                    let now = std::time::Instant::now();
                    if now >= sleep_deadline {
                        break;
                    }
                    let rem = sleep_deadline.saturating_duration_since(now);
                    let slice = Duration::from_millis(SLICE_MS).min(rem);
                    thread::sleep(slice);
                }
                done_flag.store(true, Ordering::Release);
            });
        }
        while !done.load(Ordering::Acquire) {
            let job = {
                let pool = self.thread_pool.lock();
                pool.try_recv_job()
            };
            if let Some(job) = job {
                // Nested Task jobs clear TLS on exit — restore helper context.
                let saved_rt = current_runtime();
                let saved_cancel = crate::task::current_task_cancel();
                let saved_deadline = crate::task::current_task_deadline();
                let saved_locals = crate::task::current_task_locals();
                let saved_task_id = crate::task::current_task_id();
                let saved_ctx = syx_vm::with_program_ctx(|c| c.clone());
                job();
                if let Some(rt) = saved_rt {
                    set_current_runtime(rt);
                }
                match saved_cancel {
                    Some(c) => crate::task::set_current_task_cancel(c),
                    None => crate::task::clear_current_task_cancel(),
                }
                crate::task::set_current_task_deadline(saved_deadline);
                crate::task::set_current_task_locals(saved_locals);
                crate::task::set_current_task_id(saved_task_id);
                if let Some(ctx) = saved_ctx {
                    syx_vm::set_program_ctx(ctx);
                }
            } else {
                thread::sleep(Duration::from_millis(1));
            }
        }
    }

    /// ADR-276 / M3-B01: set blocking pool size (core=max=n); recreates the pool.
    pub fn set_blocking_pool_size(self: &Arc<Self>, n: usize) -> usize {
        let n = n.max(1);
        CONFIGURED_BLOCKING_POOL_SIZE.store(n, Ordering::Relaxed);
        self.blocking_pool_size.store(n, Ordering::Relaxed);
        let keep = self.keep_alive_ms.load(Ordering::Relaxed).max(0) as u64;
        *self.blocking_pool.lock() = ThreadPool::new(n, n, keep, 0);
        n
    }

    pub fn blocking_pool_size(&self) -> usize {
        self.blocking_pool_size.load(Ordering::Relaxed)
    }

    /// ADR-278 / M3-B02: submit a job onto the dedicated blocking pool.
    pub fn execute_on_blocking_pool(self: &Arc<Self>, f: impl FnOnce() + Send + 'static) {
        let _ = self.blocking_pool.lock().try_spawn(f);
    }

    /// Phase 134: set Task submission queue capacity (`0` = unbounded); recreates the pool.
    pub fn set_task_queue_capacity(self: &Arc<Self>, n: i64) -> i64 {
        let n = n.max(0);
        CONFIGURED_TASK_QUEUE_CAPACITY.store(n, Ordering::Relaxed);
        self.task_queue_capacity.store(n, Ordering::Relaxed);
        let core = self.core_pool_size.load(Ordering::Relaxed);
        let max = self.max_pool_size.load(Ordering::Relaxed).max(core);
        let keep = self.keep_alive_ms.load(Ordering::Relaxed).max(0) as u64;
        *self.thread_pool.lock() = ThreadPool::new(core, max, keep, n);
        n
    }

    pub fn task_queue_capacity(&self) -> i64 {
        self.task_queue_capacity.load(Ordering::Relaxed)
    }

    pub fn worker_count(&self) -> usize {
        self.worker_count.load(Ordering::Relaxed)
    }

    /// Phase 136: set core pool size; recreates the pool.
    pub fn set_core_pool_size(self: &Arc<Self>, n: usize) -> usize {
        let n = n.max(1);
        CONFIGURED_CORE_POOL.store(n, Ordering::Relaxed);
        self.core_pool_size.store(n, Ordering::Relaxed);
        self.worker_count.store(n, Ordering::Relaxed);
        let mut max = self.max_pool_size.load(Ordering::Relaxed);
        if max < n {
            max = n;
            CONFIGURED_MAX_POOL.store(max, Ordering::Relaxed);
            self.max_pool_size.store(max, Ordering::Relaxed);
        }
        let cap = self.task_queue_capacity.load(Ordering::Relaxed);
        let keep = self.keep_alive_ms.load(Ordering::Relaxed).max(0) as u64;
        *self.thread_pool.lock() = ThreadPool::new(n, max, keep, cap);
        n
    }

    pub fn core_pool_size(&self) -> usize {
        self.core_pool_size.load(Ordering::Relaxed)
    }

    /// Phase 136: set maximum pool size; recreates the pool.
    pub fn set_max_pool_size(self: &Arc<Self>, n: usize) -> usize {
        let core = self.core_pool_size.load(Ordering::Relaxed).max(1);
        let n = n.max(core);
        CONFIGURED_MAX_POOL.store(n, Ordering::Relaxed);
        self.max_pool_size.store(n, Ordering::Relaxed);
        let cap = self.task_queue_capacity.load(Ordering::Relaxed);
        let keep = self.keep_alive_ms.load(Ordering::Relaxed).max(0) as u64;
        *self.thread_pool.lock() = ThreadPool::new(core, n, keep, cap);
        n
    }

    pub fn max_pool_size(&self) -> usize {
        self.max_pool_size.load(Ordering::Relaxed)
    }

    /// Phase 136: set idle keep-alive milliseconds for non-core workers.
    pub fn set_keep_alive_ms(self: &Arc<Self>, ms: i64) -> i64 {
        let ms = ms.max(0);
        CONFIGURED_KEEP_ALIVE_MS.store(ms, Ordering::Relaxed);
        self.keep_alive_ms.store(ms, Ordering::Relaxed);
        let core = self.core_pool_size.load(Ordering::Relaxed).max(1);
        let max = self.max_pool_size.load(Ordering::Relaxed).max(core);
        let cap = self.task_queue_capacity.load(Ordering::Relaxed);
        *self.thread_pool.lock() = ThreadPool::new(core, max, ms as u64, cap);
        ms
    }

    pub fn keep_alive_ms(&self) -> i64 {
        self.keep_alive_ms.load(Ordering::Relaxed)
    }

    pub fn live_worker_count(&self) -> usize {
        self.thread_pool.lock().live_count()
    }
}

fn init_globals(vm: &mut VmInstance, program: &CompiledProgram) -> SyxResult<()> {
    vm.globals.resize(program.globals.len(), Value::Nil);
    for func in &program.functions {
        if func.name == "__init_global" {
            vm.run_function(func, program)?;
        }
    }
    Ok(())
}

fn num_cpus() -> usize {
    thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .max(2)
}

/// Resolve Task worker count: `CONFIGURED_WORKERS` → `SYX_TASK_WORKERS` → `num_cpus()`.
/// Invalid / missing env values fall back to `num_cpus()`.
pub fn resolve_worker_count() -> usize {
    let configured = CONFIGURED_WORKERS.load(Ordering::Relaxed);
    if configured > 0 {
        return configured;
    }
    match std::env::var("SYX_TASK_WORKERS") {
        Ok(v) => match v.trim().parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => num_cpus(),
        },
        Err(_) => num_cpus(),
    }
}

/// Phase 136: resolve core pool size — `CONFIGURED_CORE_POOL` → `SYX_TASK_CORE` → workers.
pub fn resolve_core_pool_size() -> usize {
    let configured = CONFIGURED_CORE_POOL.load(Ordering::Relaxed);
    if configured > 0 {
        return configured;
    }
    match std::env::var("SYX_TASK_CORE") {
        Ok(v) => match v.trim().parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => resolve_worker_count(),
        },
        Err(_) => resolve_worker_count(),
    }
}

/// Phase 136: resolve max pool size — `CONFIGURED_MAX_POOL` → `SYX_TASK_MAX` → core.
pub fn resolve_max_pool_size(core: usize) -> usize {
    let configured = CONFIGURED_MAX_POOL.load(Ordering::Relaxed);
    if configured > 0 {
        return configured.max(core);
    }
    match std::env::var("SYX_TASK_MAX") {
        Ok(v) => match v.trim().parse::<usize>() {
            Ok(n) if n > 0 => n.max(core),
            _ => core,
        },
        Err(_) => core,
    }
}

/// Phase 136: resolve keep-alive ms — configured → `SYX_TASK_KEEPALIVE_MS` → 60_000.
pub fn resolve_keep_alive_ms() -> u64 {
    let configured = CONFIGURED_KEEP_ALIVE_MS.load(Ordering::Relaxed);
    if configured >= 0 {
        return configured as u64;
    }
    match std::env::var("SYX_TASK_KEEPALIVE_MS") {
        Ok(v) => match v.trim().parse::<i64>() {
            Ok(n) if n >= 0 => n as u64,
            _ => 60_000,
        },
        Err(_) => 60_000,
    }
}

pub fn set_configured_worker_count(n: usize) -> usize {
    let n = n.max(1);
    CONFIGURED_WORKERS.store(n, Ordering::Relaxed);
    CONFIGURED_CORE_POOL.store(n, Ordering::Relaxed);
    if let Some(rt) = current_runtime() {
        return rt.set_worker_count(n);
    }
    n
}

fn resolve_blocking_pool_size() -> usize {
    let configured = CONFIGURED_BLOCKING_POOL_SIZE.load(Ordering::Relaxed);
    if configured > 0 {
        return configured;
    }
    1
}

/// ADR-276 / M3-B01: set blocking pool size (core=max=n).
pub fn set_configured_blocking_pool_size(n: usize) -> usize {
    let n = n.max(1);
    CONFIGURED_BLOCKING_POOL_SIZE.store(n, Ordering::Relaxed);
    if let Some(rt) = current_runtime() {
        return rt.set_blocking_pool_size(n);
    }
    n
}

pub fn configured_blocking_pool_size() -> usize {
    if let Some(rt) = current_runtime() {
        return rt.blocking_pool_size();
    }
    resolve_blocking_pool_size()
}

pub fn set_configured_core_pool_size(n: usize) -> usize {
    let n = n.max(1);
    CONFIGURED_CORE_POOL.store(n, Ordering::Relaxed);
    if let Some(rt) = current_runtime() {
        return rt.set_core_pool_size(n);
    }
    n
}

pub fn configured_core_pool_size() -> usize {
    if let Some(rt) = current_runtime() {
        return rt.core_pool_size();
    }
    resolve_core_pool_size()
}

pub fn set_configured_max_pool_size(n: usize) -> usize {
    let core = configured_core_pool_size().max(1);
    let n = n.max(core);
    CONFIGURED_MAX_POOL.store(n, Ordering::Relaxed);
    if let Some(rt) = current_runtime() {
        return rt.set_max_pool_size(n);
    }
    n
}

pub fn configured_max_pool_size() -> usize {
    if let Some(rt) = current_runtime() {
        return rt.max_pool_size();
    }
    resolve_max_pool_size(resolve_core_pool_size())
}

pub fn set_configured_keep_alive_ms(ms: i64) -> i64 {
    let ms = ms.max(0);
    CONFIGURED_KEEP_ALIVE_MS.store(ms, Ordering::Relaxed);
    if let Some(rt) = current_runtime() {
        return rt.set_keep_alive_ms(ms);
    }
    ms
}

pub fn configured_keep_alive_ms() -> i64 {
    if let Some(rt) = current_runtime() {
        return rt.keep_alive_ms();
    }
    resolve_keep_alive_ms() as i64
}

pub fn configured_live_worker_count() -> usize {
    if let Some(rt) = current_runtime() {
        return rt.live_worker_count();
    }
    0
}

/// Resolve Task queue capacity: `CONFIGURED_TASK_QUEUE_CAPACITY` → `SYX_TASK_QUEUE_CAPACITY` → `0` (unbounded).
pub fn resolve_task_queue_capacity() -> i64 {
    let configured = CONFIGURED_TASK_QUEUE_CAPACITY.load(Ordering::Relaxed);
    if configured > 0 {
        return configured;
    }
    match std::env::var("SYX_TASK_QUEUE_CAPACITY") {
        Ok(v) => match v.trim().parse::<i64>() {
            Ok(n) if n > 0 => n,
            _ => 0,
        },
        Err(_) => 0,
    }
}

/// Resolve Task reject policy: `CONFIGURED_TASK_REJECT_POLICY` → `SYX_TASK_REJECT_POLICY` → Abort.
pub fn resolve_task_reject_policy() -> TaskRejectPolicy {
    let configured = CONFIGURED_TASK_REJECT_POLICY.load(Ordering::Relaxed);
    if configured != 255 {
        return TaskRejectPolicy::from_code(configured);
    }
    match std::env::var("SYX_TASK_REJECT_POLICY") {
        Ok(v) => parse_task_reject_policy(&v).unwrap_or(TaskRejectPolicy::Abort),
        Err(_) => TaskRejectPolicy::Abort,
    }
}

pub fn set_configured_task_reject_policy(policy: TaskRejectPolicy) -> TaskRejectPolicy {
    CONFIGURED_TASK_REJECT_POLICY.store(policy.as_code(), Ordering::Relaxed);
    policy
}

pub fn configured_task_reject_policy() -> TaskRejectPolicy {
    resolve_task_reject_policy()
}

pub fn set_configured_task_queue_capacity(n: i64) -> i64 {
    let n = n.max(0);
    CONFIGURED_TASK_QUEUE_CAPACITY.store(n, Ordering::Relaxed);
    if let Some(rt) = current_runtime() {
        return rt.set_task_queue_capacity(n);
    }
    n
}

pub fn configured_task_queue_capacity() -> i64 {
    if let Some(rt) = current_runtime() {
        return rt.task_queue_capacity();
    }
    resolve_task_queue_capacity()
}

/// Phase 134: observable bounded-queue Abort probe for platform tests.
/// Fingerprint: `runtime-concurrency-v2:bounded-queue-abort-mvp`
pub fn probe_bounded_queue_abort_mvp() -> SyxResult<()> {
    let prev = CONFIGURED_TASK_REJECT_POLICY.swap(0, Ordering::Relaxed);
    let result = probe_bounded_queue_flood_mvp();
    if prev == 255 {
        CONFIGURED_TASK_REJECT_POLICY.store(255, Ordering::Relaxed);
    } else {
        CONFIGURED_TASK_REJECT_POLICY.store(prev, Ordering::Relaxed);
    }
    result
}

/// Phase 135: observable bounded-queue CallerRuns probe for platform tests.
/// Fingerprint: `runtime-concurrency-v3:caller-runs-mvp`
pub fn probe_bounded_queue_caller_runs_mvp() -> SyxResult<()> {
    let prev = CONFIGURED_TASK_REJECT_POLICY.swap(1, Ordering::Relaxed);
    let ran = Arc::new(AtomicBool::new(false));
    let ran2 = Arc::clone(&ran);
    let ran3 = Arc::clone(&ran);
    let pool = ThreadPool::new(1, 1, 60_000, 1);
    let started = Arc::new(AtomicBool::new(false));
    let started2 = Arc::clone(&started);
    pool.try_spawn(move || {
        started2.store(true, Ordering::SeqCst);
        thread::sleep(std::time::Duration::from_millis(200));
    })?;
    while !started.load(Ordering::SeqCst) {
        thread::yield_now();
    }
    pool.try_spawn(move || {
        ran2.store(true, Ordering::SeqCst);
    })?;
    pool.try_spawn(move || {
        ran3.store(true, Ordering::SeqCst);
    })?;
    if !ran.load(Ordering::SeqCst) {
        if prev == 255 {
            CONFIGURED_TASK_REJECT_POLICY.store(255, Ordering::Relaxed);
        } else {
            CONFIGURED_TASK_REJECT_POLICY.store(prev, Ordering::Relaxed);
        }
        return Err(SyxDiagnostic::error(
            "CallerRuns did not execute overflow task on submitting thread",
        ));
    }
    if prev == 255 {
        CONFIGURED_TASK_REJECT_POLICY.store(255, Ordering::Relaxed);
    } else {
        CONFIGURED_TASK_REJECT_POLICY.store(prev, Ordering::Relaxed);
    }
    Ok(())
}

/// Phase 137: observable bounded-queue DiscardOldest probe for platform tests.
/// Fingerprint: `runtime-concurrency-v5:discard-oldest-mvp`
pub fn probe_bounded_queue_discard_oldest_mvp() -> SyxResult<()> {
    let prev = CONFIGURED_TASK_REJECT_POLICY.swap(2, Ordering::Relaxed);
    let dropped_ran = Arc::new(AtomicBool::new(false));
    let kept_ran = Arc::new(AtomicBool::new(false));
    let dropped2 = Arc::clone(&dropped_ran);
    let kept2 = Arc::clone(&kept_ran);
    let pool = ThreadPool::new(1, 1, 60_000, 1);
    let started = Arc::new(AtomicBool::new(false));
    let started2 = Arc::clone(&started);
    pool.try_spawn(move || {
        started2.store(true, Ordering::SeqCst);
        thread::sleep(std::time::Duration::from_millis(200));
    })?;
    while !started.load(Ordering::SeqCst) {
        thread::yield_now();
    }
    pool.try_spawn(move || {
        dropped2.store(true, Ordering::SeqCst);
    })?;
    pool.try_spawn(move || {
        kept2.store(true, Ordering::SeqCst);
    })?;
    thread::sleep(std::time::Duration::from_millis(400));
    let dropped = dropped_ran.load(Ordering::SeqCst);
    let kept = kept_ran.load(Ordering::SeqCst);
    if prev == 255 {
        CONFIGURED_TASK_REJECT_POLICY.store(255, Ordering::Relaxed);
    } else {
        CONFIGURED_TASK_REJECT_POLICY.store(prev, Ordering::Relaxed);
    }
    if dropped {
        return Err(SyxDiagnostic::error(
            "DiscardOldest should drop the oldest queued task, not run it",
        ));
    }
    if !kept {
        return Err(SyxDiagnostic::error(
            "DiscardOldest should enqueue and run the newest task",
        ));
    }
    Ok(())
}

fn probe_bounded_queue_flood_mvp() -> SyxResult<()> {
    let pool = ThreadPool::new(1, 1, 60_000, 1);
    let started = Arc::new(AtomicBool::new(false));
    let started2 = Arc::clone(&started);
    pool.try_spawn(move || {
        started2.store(true, Ordering::SeqCst);
        thread::sleep(std::time::Duration::from_millis(200));
    })?;
    while !started.load(Ordering::SeqCst) {
        thread::yield_now();
    }
    pool.try_spawn(|| thread::sleep(std::time::Duration::from_millis(50)))?;
    pool.try_spawn(|| ())
}

/// Phase 136: observable core/max/keepAlive pool sizing probe.
/// Fingerprint: `runtime-concurrency-v4:core-max-keepalive-mvp`
pub fn probe_core_max_keepalive_mvp() -> SyxResult<()> {
    let prev_core = CONFIGURED_CORE_POOL.swap(1, Ordering::Relaxed);
    let prev_max = CONFIGURED_MAX_POOL.swap(3, Ordering::Relaxed);
    let prev_keep = CONFIGURED_KEEP_ALIVE_MS.swap(50, Ordering::Relaxed);
    let pool = ThreadPool::new(1, 3, 50, 1);
    let started = Arc::new(AtomicBool::new(false));
    let started2 = Arc::clone(&started);
    pool.try_spawn(move || {
        started2.store(true, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(150));
    })?;
    while !started.load(Ordering::SeqCst) {
        thread::yield_now();
    }
    let _ = pool.try_spawn(|| thread::sleep(Duration::from_millis(80)));
    let _ = pool.try_spawn(|| ());
    let live_after_grow = pool.live_count();
    if live_after_grow < 2 {
        CONFIGURED_CORE_POOL.store(prev_core, Ordering::Relaxed);
        CONFIGURED_MAX_POOL.store(prev_max, Ordering::Relaxed);
        CONFIGURED_KEEP_ALIVE_MS.store(prev_keep, Ordering::Relaxed);
        return Err(SyxDiagnostic::error(
            "core/max pool did not grow live workers above core",
        ));
    }
    thread::sleep(Duration::from_millis(250));
    let live_after_shrink = pool.live_count();
    CONFIGURED_CORE_POOL.store(prev_core, Ordering::Relaxed);
    CONFIGURED_MAX_POOL.store(prev_max, Ordering::Relaxed);
    CONFIGURED_KEEP_ALIVE_MS.store(prev_keep, Ordering::Relaxed);
    if live_after_shrink > live_after_grow {
        return Err(SyxDiagnostic::error(
            "keepAlive idle shrink did not reduce or hold live workers",
        ));
    }
    let _ = set_configured_core_pool_size(2);
    let _ = set_configured_max_pool_size(4);
    let _ = set_configured_keep_alive_ms(100);
    if configured_core_pool_size() != 2
        || configured_max_pool_size() != 4
        || configured_keep_alive_ms() != 100
    {
        return Err(SyxDiagnostic::error(
            "core/max/keepAlive size query not observable",
        ));
    }
    Ok(())
}

pub fn configured_worker_count() -> usize {
    if let Some(rt) = current_runtime() {
        return rt.worker_count();
    }
    resolve_worker_count()
}

pub fn set_max_actors(n: i64) -> i64 {
    let n = n.max(0);
    MAX_ACTORS.store(n, Ordering::Relaxed);
    n
}

pub fn max_actors() -> i64 {
    MAX_ACTORS.load(Ordering::Relaxed)
}

pub fn set_mailbox_capacity(n: i64) -> i64 {
    let n = n.max(0);
    MAILBOX_CAPACITY.store(n, Ordering::Relaxed);
    n
}

pub fn mailbox_capacity() -> i64 {
    MAILBOX_CAPACITY.load(Ordering::Relaxed)
}

/// Phase 138: whether the shared actor pool is enabled (default `false` = one thread per actor).
pub fn actor_shared_pool_enabled() -> bool {
    if ACTOR_SHARED_POOL_ENABLED.load(Ordering::Relaxed) {
        return true;
    }
    match std::env::var("SYX_ACTOR_SHARED_POOL") {
        Ok(v) => {
            let v = v.trim();
            !(v.is_empty() || v == "0" || v.eq_ignore_ascii_case("false"))
        }
        Err(_) => false,
    }
}

/// Phase 138: resolve shared pool worker count — configured → `SYX_ACTOR_SHARED_POOL` numeric → `2`.
pub fn resolve_actor_shared_pool_size() -> usize {
    let configured = ACTOR_SHARED_POOL_SIZE.load(Ordering::Relaxed);
    if configured > 0 {
        return configured;
    }
    match std::env::var("SYX_ACTOR_SHARED_POOL") {
        Ok(v) => match v.trim().parse::<usize>() {
            Ok(n) if n > 0 => n,
            _ => 2,
        },
        Err(_) => 2,
    }
}

pub fn set_actor_shared_pool_enabled(enabled: bool) -> bool {
    ACTOR_SHARED_POOL_ENABLED.store(enabled, Ordering::Relaxed);
    enabled
}

pub fn set_actor_shared_pool_size(n: usize) -> usize {
    let n = n.max(1);
    ACTOR_SHARED_POOL_SIZE.store(n, Ordering::Relaxed);
    n
}

pub fn configured_actor_shared_pool_size() -> usize {
    resolve_actor_shared_pool_size()
}

/// Phase 138: observable shared actor pool config probe for platform tests.
/// Fingerprint: `runtime-concurrency-v6:actor-shared-pool-mvp`
pub fn probe_actor_shared_pool_mvp() -> SyxResult<()> {
    let prev_en = ACTOR_SHARED_POOL_ENABLED.swap(true, Ordering::Relaxed);
    let prev_sz = ACTOR_SHARED_POOL_SIZE.swap(2, Ordering::Relaxed);
    if !actor_shared_pool_enabled() {
        ACTOR_SHARED_POOL_ENABLED.store(prev_en, Ordering::Relaxed);
        ACTOR_SHARED_POOL_SIZE.store(prev_sz, Ordering::Relaxed);
        return Err(SyxDiagnostic::error(
            "actor shared pool enabled flag not observable",
        ));
    }
    if resolve_actor_shared_pool_size() != 2 {
        ACTOR_SHARED_POOL_ENABLED.store(prev_en, Ordering::Relaxed);
        ACTOR_SHARED_POOL_SIZE.store(prev_sz, Ordering::Relaxed);
        return Err(SyxDiagnostic::error(
            "actor shared pool size not observable",
        ));
    }
    let ran = Arc::new(AtomicUsize::new(0));
    let (tx, rx) = crossbeam_channel::unbounded::<()>();
    for _ in 0..resolve_actor_shared_pool_size() {
        let rx = rx.clone();
        let ran = Arc::clone(&ran);
        thread::spawn(move || {
            while rx.recv().is_ok() {
                ran.fetch_add(1, Ordering::SeqCst);
            }
        });
    }
    for _ in 0..4 {
        let _ = tx.send(());
    }
    drop(tx);
    thread::sleep(Duration::from_millis(100));
    let jobs = ran.load(Ordering::SeqCst);
    ACTOR_SHARED_POOL_ENABLED.store(prev_en, Ordering::Relaxed);
    ACTOR_SHARED_POOL_SIZE.store(prev_sz, Ordering::Relaxed);
    if jobs != 4 {
        return Err(SyxDiagnostic::error(
            "shared pool workers did not drain queued jobs",
        ));
    }
    set_actor_shared_pool_enabled(true);
    set_actor_shared_pool_size(3);
    if !actor_shared_pool_enabled() || configured_actor_shared_pool_size() != 3 {
        return Err(SyxDiagnostic::error(
            "actor shared pool host config roundtrip not observable",
        ));
    }
    set_actor_shared_pool_enabled(false);
    Ok(())
}

thread_local! {
    static CURRENT_RUNTIME: std::cell::RefCell<Option<Arc<SyxRuntime>>> =
        const { std::cell::RefCell::new(None) };
    /// ADR-264 / M3-A03: actor id currently executing a mailbox handler on this thread.
    static CURRENT_PROCESSING_ACTOR: std::cell::RefCell<Option<i64>> =
        const { std::cell::RefCell::new(None) };
}

pub fn set_current_runtime(rt: Arc<SyxRuntime>) {
    CURRENT_RUNTIME.with(|c| *c.borrow_mut() = Some(rt));
}

pub fn clear_current_runtime() {
    CURRENT_RUNTIME.with(|c| *c.borrow_mut() = None);
}

pub fn current_runtime() -> Option<Arc<SyxRuntime>> {
    CURRENT_RUNTIME.with(|c| c.borrow().clone())
}

fn set_current_processing_actor(id: Option<i64>) {
    CURRENT_PROCESSING_ACTOR.with(|c| *c.borrow_mut() = id);
}

fn clear_current_processing_actor() {
    CURRENT_PROCESSING_ACTOR.with(|c| *c.borrow_mut() = None);
}

/// ADR-631: mir/wasm mailbox bridges enter/leave the same TLS as VM async runtime.
pub fn enter_processing_actor(actor_id: i64) {
    set_current_processing_actor(Some(actor_id));
}

/// ADR-631: clear processing-actor TLS after mailbox dispatch.
pub fn leave_processing_actor() {
    clear_current_processing_actor();
}

/// ADR-295 / M3-D02: current actor id while processing a mailbox message (for Trace).
pub fn current_processing_actor() -> Option<i64> {
    CURRENT_PROCESSING_ACTOR.with(|c| *c.borrow())
}

/// ADR-264 / ADR-265 / M3-A03: product-visible reentrancy policy fingerprint.
pub fn actor_reentrancy_policy() -> &'static str {
    "serial-no-reenter;await-hold"
}

/// ADR-275 / ADR-276 / M3-B01: product-visible Task vs blocking pool isolation fingerprint.
pub fn isolation_policy() -> &'static str {
    "task-pool+blocking-pool-v1-sized"
}

/// ADR-278/279 / M3-B02: product-visible File/HTTP/Tcp async adapter fingerprint (≠ reactor GA).
pub fn io_adapter_policy() -> &'static str {
    "blocking-pool-async-adapter-v1-tcp"
}

pub fn with_runtime<R>(f: impl FnOnce(&SyxRuntime) -> R) -> Option<R> {
    CURRENT_RUNTIME.with(|c| c.borrow().as_ref().map(|rt| f(rt.as_ref())))
}

/// ADR-261 / M3 W2: unreaped TaskRegistry size (0 if no current runtime).
pub fn task_live_count() -> usize {
    current_runtime()
        .map(|rt| rt.task_live_count())
        .unwrap_or(0)
}

pub fn cooperative_yield() {
    thread::yield_now();
}

pub fn await_sleep_ms(ms: u64) {
    // Cooperative cancel + deadline: slice sleep so cancel/deadline can interrupt.
    const SLICE_MS: u64 = 10;
    let mut remaining = ms;
    while remaining > 0 {
        if crate::task::task_should_stop() {
            return;
        }
        let slice = remaining.min(SLICE_MS);
        thread::sleep(std::time::Duration::from_millis(slice));
        remaining = remaining.saturating_sub(slice);
    }
}

struct RuntimeBridgeAdapter {
    inner: Arc<SyxRuntime>,
}

impl syx_vm::RuntimeBridge for RuntimeBridgeAdapter {
    fn load_global(&self, idx: usize) -> Value {
        self.inner.load_global(idx)
    }

    fn store_global(&self, idx: usize, value: Value) {
        self.inner.store_global(idx, value);
    }

    fn record_print(&self, text: String) {
        self.inner.record_print(text);
    }

    fn spawn_detached_fn(&self, name: &str, args: Vec<Value>) -> SyxResult<Value> {
        let handle = self.inner.spawn_detached_by_name(name, args)?;
        Ok(handle.to_value())
    }

    fn join_task_value(&self, task_value: &Value) -> SyxResult<Value> {
        let id = crate::task::task_id_from_value(task_value)
            .ok_or_else(|| SyxDiagnostic::error("Expected task handle"))?;
        self.inner.join_task(id)
    }

    fn cancel_task_value(&self, task_value: &Value) -> SyxResult<()> {
        let id = crate::task::task_id_from_value(task_value)
            .ok_or_else(|| SyxDiagnostic::error("Expected task handle"))?;
        self.inner.cancel_task(id)
    }

    fn task_sleep_ms(&self, ms: i64) -> SyxResult<()> {
        await_sleep_ms(ms.max(0) as u64);
        if crate::task::task_should_stop() {
            return Err(SyxDiagnostic::error("Task was cancelled").with_code("C004"));
        }
        Ok(())
    }

    fn main_actor_run_fn(&self, name: &str, args: Vec<Value>) -> SyxResult<Value> {
        let idx = self.inner.function_index(name)?;
        let mut vm = self.inner.make_vm();
        let func = self
            .inner
            .program
            .functions
            .get(idx as usize)
            .ok_or_else(|| SyxDiagnostic::error("Unknown function"))?;
        vm.run_function_with_args(func, &self.inner.program, &args)
    }

    fn await_task_value(&self, task_value: &Value) -> SyxResult<Value> {
        if let Some(id) = crate::task::task_id_from_value(task_value) {
            return self.inner.join_task(id);
        }
        if let Value::Object(map) = task_value {
            if let Some(v) = map.get("__await_value") {
                return Ok(v.clone());
            }
        }
        Ok(task_value.clone())
    }

    fn try_await_task_value(&self, task_value: &Value) -> Option<SyxResult<Value>> {
        if let Value::Object(map) = task_value {
            if let Some(v) = map.get("__await_value") {
                return Some(Ok(v.clone()));
            }
        }
        if let Some(result) = syx_vm::try_poll_channel_recv(task_value) {
            return Some(result);
        }
        if let Some(result) = syx_vm::try_poll_http_request(task_value) {
            return Some(result);
        }
        if let Some(result) = syx_vm::try_poll_file_read(task_value) {
            return Some(result);
        }
        if let Some(result) = syx_vm::try_poll_tcp_read(task_value) {
            return Some(result);
        }
        let id = crate::task::task_id_from_value(task_value)?;
        let handle = self.inner.tasks.get(id)?;
        handle.try_join()
    }

    fn actor_send_method(
        &self,
        actor_value: &Value,
        method_name: &str,
        args: Vec<Value>,
    ) -> SyxResult<Value> {
        let actor_id = syx_vm::actor_id_from_value(actor_value)
            .ok_or_else(|| SyxDiagnostic::error("Expected actor instance"))?;
        let method_idx = crate::hosts::resolve_actor_method(&self.inner, method_name, actor_value)?;
        let rx = self.inner.actor_send(actor_id, method_idx, args)?;
        rx.recv()
            .map_err(|_| SyxDiagnostic::error("Actor reply channel closed").with_code("C006"))?
    }
}

// --- Phase 23 DD3: in-process distributed Actor MVP (route table + serialize smoke) ---

static DIST_ACTOR_ROUTES: OnceLock<Mutex<Vec<(i64, i64, String)>>> = OnceLock::new();

fn dist_routes() -> &'static Mutex<Vec<(i64, i64, String)>> {
    DIST_ACTOR_ROUTES.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn distributed_actor_mvp_enabled() -> bool {
    std::env::var("SYX_DISTRIBUTED_ACTOR_MVP")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(true)
}

pub fn dist_actor_route(from: i64, to: i64, payload: impl Into<String>) -> bool {
    if !distributed_actor_mvp_enabled() {
        return false;
    }
    let payload = payload.into();
    dist_routes().lock().push((from, to, payload));
    true
}

pub fn dist_actor_route_count() -> usize {
    dist_routes().lock().len()
}

pub fn dist_actor_last_route() -> Option<(i64, i64, String)> {
    dist_routes().lock().last().cloned()
}

pub fn dist_actor_clear_routes() {
    dist_routes().lock().clear();
}

/// Phase 26 DS2 / Phase 27 DX1: cross-process distributed Actor (UDS/pipe when enabled).
pub fn dist_actor_process_enabled() -> bool {
    std::env::var("SYX_DIST_ACTOR_PROCESS")
        .map(|v| v != "0" && !v.eq_ignore_ascii_case("false"))
        .unwrap_or(distributed_actor_mvp_enabled())
}

pub fn dist_actor_send_process(from: i64, to: i64, payload: impl Into<String>) -> bool {
    if !dist_actor_process_enabled() {
        return false;
    }
    let ok = crate::dist_transport::transport_send_process(from, to, payload);
    if ok {
        dist_actor_route(from, to, "uds-frame");
    }
    ok
}

pub fn dist_actor_recv_process() -> Option<(i64, i64, String)> {
    if !dist_actor_process_enabled() {
        return None;
    }
    crate::dist_transport::transport_recv_process()
}

pub fn dist_actor_process_count() -> usize {
    crate::dist_transport::transport_process_count()
}

pub fn dist_actor_clear_process_routes() {
    crate::dist_transport::transport_clear_process();
}

/// Phase 27 DX1: UDS transport gate.
pub fn dist_actor_uds_enabled() -> bool {
    crate::dist_transport::dist_actor_uds_enabled()
}

/// Phase 28 EC1: UDS production mode (no memory fallback).
pub fn dist_actor_uds_prod_enabled() -> bool {
    crate::dist_transport::dist_actor_uds_prod_enabled()
}

/// Phase 27 DX2: TLS cross-machine MVP gate.
pub fn dist_actor_tls_enabled() -> bool {
    crate::dist_transport::dist_actor_tls_enabled()
}

/// Phase 28 EC2: TLS real handshake gate.
pub fn dist_actor_tls_real_enabled() -> bool {
    crate::dist_transport::dist_actor_tls_real_enabled()
}

pub fn dist_actor_send_tls(from: i64, to: i64, payload: impl Into<String>) -> bool {
    if !dist_actor_tls_enabled() {
        return false;
    }
    crate::dist_transport::transport_send_tls(from, to, payload)
}

pub fn dist_actor_recv_tls() -> Option<(String, i64, i64, String)> {
    if !dist_actor_tls_enabled() {
        return None;
    }
    crate::dist_transport::transport_recv_tls()
}

pub fn dist_actor_tls_count() -> usize {
    crate::dist_transport::transport_tls_count()
}

pub fn dist_actor_tls_real_count() -> usize {
    crate::dist_transport::transport_tls_real_count()
}

/// Phase 29 EH3: rustls TLS handshake count.
pub fn dist_actor_tls_rustls_enabled() -> bool {
    crate::dist_transport::dist_actor_tls_rustls_enabled()
}

pub fn dist_actor_tls_rustls_count() -> usize {
    crate::dist_transport::transport_tls_rustls_count()
}

/// Phase 29 EH2: Windows named pipe UDS gate.
pub fn dist_actor_win_pipe_enabled() -> bool {
    crate::dist_transport::dist_actor_win_pipe_enabled()
}

/// Phase 30 EM2: Windows named pipe prod gate.
pub fn dist_actor_win_pipe_prod_enabled() -> bool {
    crate::dist_transport::dist_actor_win_pipe_prod_enabled()
}

/// Phase 30 EM3: TLS rustls prod gate.
pub fn dist_actor_tls_rustls_prod_enabled() -> bool {
    crate::dist_transport::dist_actor_tls_rustls_prod_enabled()
}
