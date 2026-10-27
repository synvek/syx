use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crossbeam_channel::{Receiver, Sender};
use parking_lot::Mutex;
use syx_diagnostics::{SyxDiagnostic, SyxResult};
use syx_vm::Value;

use crate::cancellation::CancellationToken;

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);

thread_local! {
    static CURRENT_TASK_CANCEL: RefCell<Option<CancellationToken>> =
        const { RefCell::new(None) };
    /// ADR-262 / M3-A02: absolute deadline for current task (inherited by children).
    static CURRENT_TASK_DEADLINE: RefCell<Option<Instant>> = const { RefCell::new(None) };
    /// ADR-263 / M3-A02: task-local key/value map (cloned to children on spawn).
    static CURRENT_TASK_LOCALS: RefCell<HashMap<String, Value>> =
        RefCell::new(HashMap::new());
    /// ADR-295 / M3-D02: current task id for trace correlation (set on spawn).
    static CURRENT_TASK_ID: RefCell<Option<u64>> = const { RefCell::new(None) };
}

pub fn set_current_task_cancel(token: CancellationToken) {
    CURRENT_TASK_CANCEL.with(|c| *c.borrow_mut() = Some(token));
}

pub fn clear_current_task_cancel() {
    CURRENT_TASK_CANCEL.with(|c| *c.borrow_mut() = None);
}

pub fn current_task_cancel() -> Option<CancellationToken> {
    CURRENT_TASK_CANCEL.with(|c| c.borrow().clone())
}

pub fn set_current_task_deadline(deadline: Option<Instant>) {
    CURRENT_TASK_DEADLINE.with(|d| *d.borrow_mut() = deadline);
}

pub fn clear_current_task_deadline() {
    CURRENT_TASK_DEADLINE.with(|d| *d.borrow_mut() = None);
}

pub fn current_task_deadline() -> Option<Instant> {
    CURRENT_TASK_DEADLINE.with(|d| *d.borrow())
}

/// ADR-262: set deadline relative to now on the current task context.
pub fn set_deadline_ms(ms: u64) {
    let deadline = Instant::now() + Duration::from_millis(ms);
    set_current_task_deadline(Some(deadline));
}

/// If the current deadline has passed, cancel the current task token (cooperative C004 path).
pub fn apply_deadline_to_cancel() {
    let Some(deadline) = current_task_deadline() else {
        return;
    };
    if Instant::now() >= deadline {
        if let Some(token) = current_task_cancel() {
            token.cancel();
        }
    }
}

/// True when cancel is set or deadline has expired (side-effect: may cancel token on expiry).
pub fn task_should_stop() -> bool {
    apply_deadline_to_cancel();
    current_task_cancel().is_some_and(|t| t.is_cancelled())
}

pub fn set_current_task_locals(map: HashMap<String, Value>) {
    CURRENT_TASK_LOCALS.with(|l| *l.borrow_mut() = map);
}

pub fn clear_current_task_locals() {
    CURRENT_TASK_LOCALS.with(|l| l.borrow_mut().clear());
}

pub fn current_task_locals() -> HashMap<String, Value> {
    CURRENT_TASK_LOCALS.with(|l| l.borrow().clone())
}

/// ADR-295 / M3-D02: bind current task id for Trace.contextLine correlation.
pub fn set_current_task_id(id: Option<u64>) {
    CURRENT_TASK_ID.with(|c| *c.borrow_mut() = id);
}

pub fn clear_current_task_id() {
    CURRENT_TASK_ID.with(|c| *c.borrow_mut() = None);
}

pub fn current_task_id() -> Option<u64> {
    CURRENT_TASK_ID.with(|c| *c.borrow())
}

/// ADR-263: set a task-local value on the current task context.
pub fn task_local_set(key: String, value: Value) {
    CURRENT_TASK_LOCALS.with(|l| {
        l.borrow_mut().insert(key, value);
    });
}

/// ADR-263: get a task-local value (Nil if missing).
pub fn task_local_get(key: &str) -> Value {
    CURRENT_TASK_LOCALS.with(|l| l.borrow().get(key).cloned().unwrap_or(Value::Nil))
}

#[derive(Debug)]
pub struct TaskHandle {
    pub id: u64,
    pub cancel: CancellationToken,
    done_rx: Receiver<SyxResult<Value>>,
    /// ADR-261: removed from live_count after structured wait/join (handle may remain for late join).
    reaped: AtomicBool,
}

impl TaskHandle {
    pub fn new(done_rx: Receiver<SyxResult<Value>>) -> Self {
        Self::new_with_cancel(done_rx, CancellationToken::default())
    }

    pub fn new_with_cancel(done_rx: Receiver<SyxResult<Value>>, cancel: CancellationToken) -> Self {
        let id = NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed);
        Self {
            id,
            cancel,
            done_rx,
            reaped: AtomicBool::new(false),
        }
    }

    pub fn mark_reaped(&self) {
        self.reaped.store(true, Ordering::SeqCst);
    }

    pub fn is_reaped(&self) -> bool {
        self.reaped.load(Ordering::SeqCst)
    }

    pub fn new_linked(
        done_rx: Receiver<SyxResult<Value>>,
        parent: CancellationToken,
    ) -> Self {
        Self::new_with_cancel(done_rx, CancellationToken::with_parent(parent))
    }

    pub fn try_join(&self) -> Option<SyxResult<Value>> {
        self.done_rx.try_recv().ok()
    }

    pub fn join(&self) -> SyxResult<Value> {
        if self.cancel.is_cancelled() {
            return Err(SyxDiagnostic::error("Task was cancelled").with_code("C004"));
        }
        let result = self
            .done_rx
            .recv()
            .map_err(|_| SyxDiagnostic::error("Task channel closed").with_code("C005"))?;
        // If cancel raced in while waiting, prefer C004 over a late Ok body result.
        if self.cancel.is_cancelled() {
            return Err(SyxDiagnostic::error("Task was cancelled").with_code("C004"));
        }
        result
    }

    pub fn to_value(&self) -> Value {
        let mut map = std::collections::HashMap::new();
        map.insert("__task_id".into(), Value::I64(self.id as i64));
        map.insert(
            "__cancelled".into(),
            Value::Bool(self.cancel.is_cancelled()),
        );
        Value::Object(map)
    }
}

#[derive(Debug, Default)]
pub struct TaskRegistry {
    tasks: Mutex<std::collections::HashMap<u64, Arc<TaskHandle>>>,
}

impl TaskRegistry {
    pub fn register(&self, handle: Arc<TaskHandle>) {
        self.tasks.lock().insert(handle.id, handle);
    }

    pub fn get(&self, id: u64) -> Option<Arc<TaskHandle>> {
        self.tasks.lock().get(&id).cloned()
    }

    pub fn remove(&self, id: u64) {
        self.tasks.lock().remove(&id);
    }

    pub fn clear(&self) {
        self.tasks.lock().clear();
    }

    /// ADR-261 / M3-A01: unreaped registered tasks (silent-orphan diagnostic).
    pub fn live_count(&self) -> usize {
        self.tasks
            .lock()
            .values()
            .filter(|h| !h.is_reaped())
            .count()
    }

    pub fn mark_reaped(&self, id: u64) {
        if let Some(h) = self.tasks.lock().get(&id) {
            h.mark_reaped();
        }
    }
}

pub struct TaskGroup {
    pub parent_cancel: CancellationToken,
    children: Mutex<Vec<Arc<TaskHandle>>>,
    done_tx: Mutex<Option<Sender<()>>>,
    done_rx: Receiver<()>,
    /// Phase 21 CT4: structured child error aggregation.
    pub errors: Mutex<Vec<String>>,
}

impl TaskGroup {
    pub fn new(parent_cancel: CancellationToken) -> Self {
        let (done_tx, done_rx) = crossbeam_channel::unbounded();
        Self {
            parent_cancel,
            children: Mutex::new(Vec::new()),
            done_tx: Mutex::new(Some(done_tx)),
            done_rx,
            errors: Mutex::new(Vec::new()),
        }
    }

    pub fn add(&self, handle: Arc<TaskHandle>) {
        if self.parent_cancel.is_cancelled() {
            handle.cancel.cancel();
        }
        self.children.lock().push(handle);
    }

    pub fn cancel_all(&self) {
        self.parent_cancel.cancel();
        for child in self.children.lock().iter() {
            child.cancel.cancel();
        }
    }

    pub fn wait_all(&self) -> SyxResult<()> {
        // Always cancel children when the group token is cancelled, then join so
        // cancelled handles surface C004 promptly (no hang on unreaped sleepers).
        if self.parent_cancel.is_cancelled() {
            self.cancel_all();
        }
        let children: Vec<_> = self.children.lock().clone();
        let mut aggregated: Vec<String> = Vec::new();
        for child in children {
            if self.parent_cancel.is_cancelled() {
                self.cancel_all();
            }
            match child.join() {
                Ok(_) => {}
                Err(e) if e.code.as_deref() == Some("C004") => {}
                Err(e) => {
                    let msg = e.message.clone();
                    aggregated.push(msg);
                }
            }
            // Structured wait reaps child from live_count (ADR-261); handle stays joinable.
            child.mark_reaped();
        }
        if !aggregated.is_empty() {
            self.errors.lock().extend(aggregated.clone());
            return Err(
                SyxDiagnostic::error(format!(
                    "TaskGroup failed with {} error(s): {}",
                    aggregated.len(),
                    aggregated.join("; ")
                ))
                .with_code("C007"),
            );
        }
        Ok(())
    }

    pub fn to_value(&self) -> Value {
        let mut map = std::collections::HashMap::new();
        map.insert("__task_group".into(), Value::Bool(true));
        map.insert(
            "__child_count".into(),
            Value::I64(self.children.lock().len() as i64),
        );
        Value::Object(map)
    }

    pub fn to_value_with_id(&self, id: u64) -> Value {
        let mut map = std::collections::HashMap::new();
        map.insert("__task_group".into(), Value::Bool(true));
        map.insert("__group_id".into(), Value::I64(id as i64));
        map.insert(
            "__child_count".into(),
            Value::I64(self.children.lock().len() as i64),
        );
        Value::Object(map)
    }
}

pub fn group_id_from_value(v: &Value) -> Option<u64> {
    if let Value::Object(map) = v {
        map.get("__group_id").and_then(|id| match id {
            Value::I64(n) => Some(*n as u64),
            Value::I32(n) => Some(*n as u64),
            _ => None,
        })
    } else {
        None
    }
}

pub fn task_id_from_value(v: &Value) -> Option<u64> {
    if let Value::Object(map) = v {
        map.get("__task_id").and_then(|id| match id {
            Value::I64(n) => Some(*n as u64),
            Value::I32(n) => Some(*n as u64),
            _ => None,
        })
    } else {
        None
    }
}
