//! Serialize permission tests that mutate the process-wide runtime config.
use std::sync::{Mutex, MutexGuard, OnceLock};

pub fn hold() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}
