//! Phase 98: farm billing farm path settlement excess exit non-zero.

use syx_pkg::{enforce_billing_settlement, SyxLockfile};

use crate::billing_env_lock::{billing_env_lock, ClearBillingEnv};

#[test]
fn p98_farm_billing_farm_path_stale_settlement_err() {
    let _g = billing_env_lock();
    let _clear = ClearBillingEnv;
    std::env::set_var("SYX_FARM_BILLING_PATH", "1");
    let mut lock = SyxLockfile::default();
    lock.billing_settlement_id = Some("farm-path-p92".into());
    std::env::remove_var("SYX_BILLING_SETTLEMENT_ACK");
    let err = enforce_billing_settlement(&lock).unwrap_err();
    assert!(err.message.contains("billing settlement stale"), "{err:?}");
}

#[test]
fn p98_farm_billing_farm_path_matching_ack_ok() {
    let _g = billing_env_lock();
    let _clear = ClearBillingEnv;
    std::env::set_var("SYX_FARM_BILLING_PATH", "1");
    let mut lock = SyxLockfile::default();
    lock.billing_settlement_id = Some("farm-ok".into());
    std::env::set_var("SYX_BILLING_SETTLEMENT_ACK", "farm-ok");
    assert!(enforce_billing_settlement(&lock).is_ok());
}

#[test]
fn p98_farm_billing_farm_path_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("farm-billing-farm-path-v1"));
}

#[test]
fn p98_farm_billing_farm_path_worker_surface() {
    assert!(include_str!("../../src/build_syx.rs").contains("farm_worker_run_cycle"));
}

#[test]
fn p100_farm_billing_path_exit_nonzero_tcp_queue() {
    let _g = billing_env_lock();
    let _clear = ClearBillingEnv;
    std::env::set_var("SYX_FARM_BILLING_PATH", "1");
    std::env::remove_var("SYX_BILLING_SETTLEMENT_ACK");
    let err = syx_driver::farm_worker_run_cycle("tcp://127.0.0.1:9/q", "tcp://127.0.0.1:9/x")
        .unwrap_err();
    assert!(
        err.contains("billing") || err.contains("lease"),
        "fail-closed lease may precede billing: {err}"
    );
}
