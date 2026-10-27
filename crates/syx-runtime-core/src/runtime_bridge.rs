use std::sync::Arc;

use syx_diagnostics::SyxResult;

use crate::Value;

/// Bridge to shared runtime state (globals, print aggregation) without depending on `syx-async`.
pub trait RuntimeBridge: Send + Sync {
    fn load_global(&self, idx: usize) -> Value;
    fn store_global(&self, idx: usize, value: Value);
    fn record_print(&self, text: String);
    fn spawn_detached_fn(&self, name: &str, args: Vec<Value>) -> SyxResult<Value>;
    fn join_task_value(&self, task_value: &Value) -> SyxResult<Value>;
    fn cancel_task_value(&self, task_value: &Value) -> SyxResult<()>;
    /// Sleep cooperatively; return `C004` when the current task should stop.
    fn task_sleep_ms(&self, ms: i64) -> SyxResult<()>;
    fn main_actor_run_fn(&self, name: &str, args: Vec<Value>) -> SyxResult<Value>;
    fn await_task_value(&self, task_value: &Value) -> SyxResult<Value>;
    /// Non-blocking poll for async state machine; `None` if task still running.
    fn try_await_task_value(&self, task_value: &Value) -> Option<SyxResult<Value>> {
        let _ = task_value;
        None
    }
    fn actor_send_method(
        &self,
        actor_value: &Value,
        method_name: &str,
        args: Vec<Value>,
    ) -> SyxResult<Value>;
}

pub type VmRuntime = Arc<dyn RuntimeBridge>;
