//! Shared lock for tests that mutate SYX_BILLING_* / SYX_FARM_BILLING_PATH env vars.

use std::sync::{Mutex, MutexGuard, OnceLock};

pub fn billing_env_lock() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

pub struct ClearBillingEnv;
impl Drop for ClearBillingEnv {
    fn drop(&mut self) {
        std::env::remove_var("SYX_FARM_BILLING_PATH");
        std::env::remove_var("SYX_BILLING_SETTLEMENT_ACK");
    }
}
