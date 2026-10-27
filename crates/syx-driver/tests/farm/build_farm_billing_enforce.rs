//! Phase 91: farm billing settlement enforce exit non-zero on stale ack.

use syx_pkg::{enforce_billing_settlement, SyxLockfile};

use crate::billing_env_lock::{billing_env_lock, ClearBillingEnv};

#[test]
fn p98_farm_billing_enforce_stale_settlement_err() {
    let _g = billing_env_lock();
    let _clear = ClearBillingEnv;
    let mut lock = SyxLockfile::default();
    lock.billing_settlement_id = Some("settle-p91".into());
    std::env::remove_var("SYX_BILLING_SETTLEMENT_ACK");
    let err = enforce_billing_settlement(&lock).unwrap_err();
    assert!(
        err.message.contains("billing settlement stale"),
        "{err:?}"
    );
}

#[test]
fn p98_farm_billing_enforce_matching_ack_ok() {
    let _g = billing_env_lock();
    let _clear = ClearBillingEnv;
    let mut lock = SyxLockfile::default();
    lock.billing_settlement_id = Some("settle-ok".into());
    std::env::set_var("SYX_BILLING_SETTLEMENT_ACK", "settle-ok");
    assert!(enforce_billing_settlement(&lock).is_ok());
}

#[test]
fn p98_farm_billing_enforce_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-billing-farm-path-v1"));
}
