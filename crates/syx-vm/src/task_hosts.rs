use std::sync::OnceLock;

use syx_diagnostics::SyxResult;

use crate::Value;

type HostFn = fn(&mut Vec<Value>) -> SyxResult<()>;

static TASK_DETACH_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_JOIN_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_CANCEL_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_SLEEP_FN: OnceLock<HostFn> = OnceLock::new();
static MAIN_ACTOR_RUN_FN: OnceLock<HostFn> = OnceLock::new();
static AWAIT_TASK_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_GROUP_CREATE_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_GROUP_ADD_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_GROUP_WAIT_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_GROUP_CANCEL_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_SET_WORKER_COUNT_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_WORKER_COUNT_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_SET_QUEUE_CAPACITY_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_QUEUE_CAPACITY_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_SET_REJECT_POLICY_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_REJECT_POLICY_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_SET_CORE_POOL_SIZE_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_CORE_POOL_SIZE_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_SET_MAX_POOL_SIZE_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_MAX_POOL_SIZE_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_SET_KEEP_ALIVE_MS_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_KEEP_ALIVE_MS_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_LIVE_WORKER_COUNT_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_SET_MAX_ACTORS_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_MAX_ACTORS_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_SET_MAILBOX_CAPACITY_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_MAILBOX_CAPACITY_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_SET_SHARED_POOL_ENABLED_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_SHARED_POOL_ENABLED_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_SET_SHARED_POOL_SIZE_FN: OnceLock<HostFn> = OnceLock::new();
static ACTOR_SHARED_POOL_SIZE_FN: OnceLock<HostFn> = OnceLock::new();

pub fn register_task_hosts(
    detach: HostFn,
    join: HostFn,
    cancel: HostFn,
    sleep: HostFn,
    main_actor_run: HostFn,
    await_task: HostFn,
    group_create: HostFn,
    group_add: HostFn,
    group_wait: HostFn,
) {
    let _ = TASK_DETACH_FN.set(detach);
    let _ = TASK_JOIN_FN.set(join);
    let _ = TASK_CANCEL_FN.set(cancel);
    let _ = TASK_SLEEP_FN.set(sleep);
    let _ = MAIN_ACTOR_RUN_FN.set(main_actor_run);
    let _ = AWAIT_TASK_FN.set(await_task);
    let _ = TASK_GROUP_CREATE_FN.set(group_create);
    let _ = TASK_GROUP_ADD_FN.set(group_add);
    let _ = TASK_GROUP_WAIT_FN.set(group_wait);
}

/// Phase 133 Track E: concurrency control hosts (worker count / max actors / mailbox).
/// Phase 134 Track E: Task queue capacity (Abort reject on overflow).
/// Phase 135 Track E: Task reject policy (Abort / CallerRuns).
pub fn register_concurrency_control_hosts(
    set_worker_count: HostFn,
    worker_count: HostFn,
    set_queue_capacity: HostFn,
    queue_capacity: HostFn,
    set_reject_policy: HostFn,
    reject_policy: HostFn,
    set_max_actors: HostFn,
    max_actors: HostFn,
    set_mailbox_capacity: HostFn,
    mailbox_capacity: HostFn,
) {
    let _ = TASK_SET_WORKER_COUNT_FN.set(set_worker_count);
    let _ = TASK_WORKER_COUNT_FN.set(worker_count);
    let _ = TASK_SET_QUEUE_CAPACITY_FN.set(set_queue_capacity);
    let _ = TASK_QUEUE_CAPACITY_FN.set(queue_capacity);
    let _ = TASK_SET_REJECT_POLICY_FN.set(set_reject_policy);
    let _ = TASK_REJECT_POLICY_FN.set(reject_policy);
    let _ = ACTOR_SET_MAX_ACTORS_FN.set(set_max_actors);
    let _ = ACTOR_MAX_ACTORS_FN.set(max_actors);
    let _ = ACTOR_SET_MAILBOX_CAPACITY_FN.set(set_mailbox_capacity);
    let _ = ACTOR_MAILBOX_CAPACITY_FN.set(mailbox_capacity);
}

/// Phase 138 Track E: Actor shared pool opt-in hosts.
pub fn register_actor_shared_pool_hosts(
    set_enabled: HostFn,
    enabled: HostFn,
    set_size: HostFn,
    size: HostFn,
) {
    let _ = ACTOR_SET_SHARED_POOL_ENABLED_FN.set(set_enabled);
    let _ = ACTOR_SHARED_POOL_ENABLED_FN.set(enabled);
    let _ = ACTOR_SET_SHARED_POOL_SIZE_FN.set(set_size);
    let _ = ACTOR_SHARED_POOL_SIZE_FN.set(size);
}

/// Phase 136 Track E: Task pool core / max / keepAlive sizing hosts.
pub fn register_task_pool_sizing_hosts(
    set_core: HostFn,
    core: HostFn,
    set_max: HostFn,
    max: HostFn,
    set_keep_alive: HostFn,
    keep_alive: HostFn,
    live_workers: HostFn,
) {
    let _ = TASK_SET_CORE_POOL_SIZE_FN.set(set_core);
    let _ = TASK_CORE_POOL_SIZE_FN.set(core);
    let _ = TASK_SET_MAX_POOL_SIZE_FN.set(set_max);
    let _ = TASK_MAX_POOL_SIZE_FN.set(max);
    let _ = TASK_SET_KEEP_ALIVE_MS_FN.set(set_keep_alive);
    let _ = TASK_KEEP_ALIVE_MS_FN.set(keep_alive);
    let _ = TASK_LIVE_WORKER_COUNT_FN.set(live_workers);
}

pub fn task_detach_host() -> Option<HostFn> {
    TASK_DETACH_FN.get().copied()
}

pub fn task_join_host() -> Option<HostFn> {
    TASK_JOIN_FN.get().copied()
}

pub fn task_cancel_host() -> Option<HostFn> {
    TASK_CANCEL_FN.get().copied()
}

pub fn task_sleep_host() -> Option<HostFn> {
    TASK_SLEEP_FN.get().copied()
}

pub fn main_actor_run_host() -> Option<HostFn> {
    MAIN_ACTOR_RUN_FN.get().copied()
}

pub fn await_task_host() -> Option<HostFn> {
    AWAIT_TASK_FN.get().copied()
}

pub fn task_group_create_host() -> Option<HostFn> {
    TASK_GROUP_CREATE_FN.get().copied()
}

pub fn task_group_add_host() -> Option<HostFn> {
    TASK_GROUP_ADD_FN.get().copied()
}

pub fn task_group_wait_host() -> Option<HostFn> {
    TASK_GROUP_WAIT_FN.get().copied()
}

/// ADR-260 / M3 W1: register TaskGroup.cancel_all host (separate from register_task_hosts).
pub fn register_task_group_cancel_host(group_cancel: HostFn) {
    let _ = TASK_GROUP_CANCEL_FN.set(group_cancel);
}

pub fn task_group_cancel_host() -> Option<HostFn> {
    TASK_GROUP_CANCEL_FN.get().copied()
}

static TASK_LIVE_COUNT_FN: OnceLock<HostFn> = OnceLock::new();

/// ADR-261 / M3 W2: register TaskRegistry live_count host.
pub fn register_task_live_count_host(live_count: HostFn) {
    let _ = TASK_LIVE_COUNT_FN.set(live_count);
}

pub fn task_live_count_host() -> Option<HostFn> {
    TASK_LIVE_COUNT_FN.get().copied()
}

/// Wave A W275 / ADR-619: optional bridge live_count provider (wasm/native MIR ctx).
/// Checked by VM `host_task_live_count` when OnceLock already holds the VM host
/// (in-process wasm tests cannot overwrite OnceLock).
static BRIDGE_LIVE_COUNT: OnceLock<fn() -> Option<usize>> = OnceLock::new();

pub fn register_bridge_task_live_count(provider: fn() -> Option<usize>) {
    let _ = BRIDGE_LIVE_COUNT.set(provider);
}

pub fn bridge_task_live_count() -> Option<usize> {
    BRIDGE_LIVE_COUNT.get().and_then(|f| f())
}

/// Wave A W276 / ADR-620: optional bridge TaskGroup ops (wasm MIR ctx).
/// When `active()` is true, VM `host_task_group_*` delegates here (OnceLock already held).
pub struct BridgeTaskGroupOps {
    pub active: fn() -> bool,
    pub create: HostFn,
    pub add: HostFn,
    pub wait: HostFn,
    pub cancel: HostFn,
}

static BRIDGE_TASK_GROUP: OnceLock<BridgeTaskGroupOps> = OnceLock::new();

pub fn register_bridge_task_group_ops(ops: BridgeTaskGroupOps) {
    let _ = BRIDGE_TASK_GROUP.set(ops);
}

pub fn bridge_task_group_ops() -> Option<&'static BridgeTaskGroupOps> {
    BRIDGE_TASK_GROUP.get()
}

static TASK_SET_DEADLINE_MS_FN: OnceLock<HostFn> = OnceLock::new();

/// ADR-262 / M3 W3: register task deadline host.
pub fn register_task_set_deadline_ms_host(set_deadline: HostFn) {
    let _ = TASK_SET_DEADLINE_MS_FN.set(set_deadline);
}

pub fn task_set_deadline_ms_host() -> Option<HostFn> {
    TASK_SET_DEADLINE_MS_FN.get().copied()
}

static TASK_LOCAL_SET_FN: OnceLock<HostFn> = OnceLock::new();
static TASK_LOCAL_GET_FN: OnceLock<HostFn> = OnceLock::new();

/// ADR-263 / M3 W4: register task-local set/get hosts.
pub fn register_task_local_hosts(set: HostFn, get: HostFn) {
    let _ = TASK_LOCAL_SET_FN.set(set);
    let _ = TASK_LOCAL_GET_FN.set(get);
}

pub fn task_local_set_host() -> Option<HostFn> {
    TASK_LOCAL_SET_FN.get().copied()
}

pub fn task_local_get_host() -> Option<HostFn> {
    TASK_LOCAL_GET_FN.get().copied()
}

static ACTOR_REENTRANCY_POLICY_FN: OnceLock<HostFn> = OnceLock::new();

/// ADR-264 / M3 W5: register actor reentrancy policy host.
pub fn register_actor_reentrancy_policy_host(policy: HostFn) {
    let _ = ACTOR_REENTRANCY_POLICY_FN.set(policy);
}

pub fn actor_reentrancy_policy_host() -> Option<HostFn> {
    ACTOR_REENTRANCY_POLICY_FN.get().copied()
}

static BLOCKING_SLEEP_FN: OnceLock<HostFn> = OnceLock::new();
static ISOLATION_POLICY_FN: OnceLock<HostFn> = OnceLock::new();

/// ADR-275 / M3 W16: register blocking-pool sleep + isolation policy hosts.
pub fn register_blocking_isolation_hosts(sleep: HostFn, policy: HostFn) {
    let _ = BLOCKING_SLEEP_FN.set(sleep);
    let _ = ISOLATION_POLICY_FN.set(policy);
}

pub fn blocking_sleep_host() -> Option<HostFn> {
    BLOCKING_SLEEP_FN.get().copied()
}

pub fn isolation_policy_host() -> Option<HostFn> {
    ISOLATION_POLICY_FN.get().copied()
}

static BLOCKING_POOL_SET_SIZE_FN: OnceLock<HostFn> = OnceLock::new();
static BLOCKING_POOL_SIZE_FN: OnceLock<HostFn> = OnceLock::new();

/// ADR-276 / M3 W17: register blocking pool size get/set hosts.
pub fn register_blocking_pool_sizing_hosts(set_size: HostFn, size: HostFn) {
    let _ = BLOCKING_POOL_SET_SIZE_FN.set(set_size);
    let _ = BLOCKING_POOL_SIZE_FN.set(size);
}

pub fn blocking_pool_set_size_host() -> Option<HostFn> {
    BLOCKING_POOL_SET_SIZE_FN.get().copied()
}

pub fn blocking_pool_size_host() -> Option<HostFn> {
    BLOCKING_POOL_SIZE_FN.get().copied()
}

static IO_ADAPTER_POLICY_FN: OnceLock<HostFn> = OnceLock::new();

/// ADR-278 / M3 W19: register I/O adapter policy fingerprint host.
pub fn register_io_adapter_policy_host(policy: HostFn) {
    let _ = IO_ADAPTER_POLICY_FN.set(policy);
}

pub fn io_adapter_policy_host() -> Option<HostFn> {
    IO_ADAPTER_POLICY_FN.get().copied()
}

pub fn task_set_worker_count_host() -> Option<HostFn> {
    TASK_SET_WORKER_COUNT_FN.get().copied()
}

pub fn task_worker_count_host() -> Option<HostFn> {
    TASK_WORKER_COUNT_FN.get().copied()
}

pub fn task_set_queue_capacity_host() -> Option<HostFn> {
    TASK_SET_QUEUE_CAPACITY_FN.get().copied()
}

pub fn task_queue_capacity_host() -> Option<HostFn> {
    TASK_QUEUE_CAPACITY_FN.get().copied()
}

pub fn task_set_reject_policy_host() -> Option<HostFn> {
    TASK_SET_REJECT_POLICY_FN.get().copied()
}

pub fn task_reject_policy_host() -> Option<HostFn> {
    TASK_REJECT_POLICY_FN.get().copied()
}

pub fn task_set_core_pool_size_host() -> Option<HostFn> {
    TASK_SET_CORE_POOL_SIZE_FN.get().copied()
}

pub fn task_core_pool_size_host() -> Option<HostFn> {
    TASK_CORE_POOL_SIZE_FN.get().copied()
}

pub fn task_set_max_pool_size_host() -> Option<HostFn> {
    TASK_SET_MAX_POOL_SIZE_FN.get().copied()
}

pub fn task_max_pool_size_host() -> Option<HostFn> {
    TASK_MAX_POOL_SIZE_FN.get().copied()
}

pub fn task_set_keep_alive_ms_host() -> Option<HostFn> {
    TASK_SET_KEEP_ALIVE_MS_FN.get().copied()
}

pub fn task_keep_alive_ms_host() -> Option<HostFn> {
    TASK_KEEP_ALIVE_MS_FN.get().copied()
}

pub fn task_live_worker_count_host() -> Option<HostFn> {
    TASK_LIVE_WORKER_COUNT_FN.get().copied()
}

pub fn actor_set_max_actors_host() -> Option<HostFn> {
    ACTOR_SET_MAX_ACTORS_FN.get().copied()
}

pub fn actor_max_actors_host() -> Option<HostFn> {
    ACTOR_MAX_ACTORS_FN.get().copied()
}

pub fn actor_set_mailbox_capacity_host() -> Option<HostFn> {
    ACTOR_SET_MAILBOX_CAPACITY_FN.get().copied()
}

pub fn actor_mailbox_capacity_host() -> Option<HostFn> {
    ACTOR_MAILBOX_CAPACITY_FN.get().copied()
}

pub fn actor_set_shared_pool_enabled_host() -> Option<HostFn> {
    ACTOR_SET_SHARED_POOL_ENABLED_FN.get().copied()
}

pub fn actor_shared_pool_enabled_host() -> Option<HostFn> {
    ACTOR_SHARED_POOL_ENABLED_FN.get().copied()
}

pub fn actor_set_shared_pool_size_host() -> Option<HostFn> {
    ACTOR_SET_SHARED_POOL_SIZE_FN.get().copied()
}

pub fn actor_shared_pool_size_host() -> Option<HostFn> {
    ACTOR_SHARED_POOL_SIZE_FN.get().copied()
}

// --- ADR-295 / M3-D02: Trace context providers (task/actor ids from syx-async) ---

static TRACE_TASK_ID_FN: OnceLock<fn() -> Option<u64>> = OnceLock::new();
static TRACE_ACTOR_ID_FN: OnceLock<fn() -> Option<i64>> = OnceLock::new();

/// Register optional task/actor id getters for `Trace.contextLine`.
pub fn register_trace_context_providers(
    task_id: fn() -> Option<u64>,
    actor_id: fn() -> Option<i64>,
) {
    let _ = TRACE_TASK_ID_FN.set(task_id);
    let _ = TRACE_ACTOR_ID_FN.set(actor_id);
}

pub fn trace_current_task_id() -> Option<u64> {
    TRACE_TASK_ID_FN.get().and_then(|f| f())
}

pub fn trace_current_actor_id() -> Option<i64> {
    TRACE_ACTOR_ID_FN.get().and_then(|f| f())
}
