//! Phase 98: GC churn four-end stdout gate.

use syx_vm::{gc_churn_event_count, host_gc_churn_event, reset_phase98_state};

#[test]
fn p98_gc_churn_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gc-churn-quad-v1"));
}

#[test]
fn p98_gc_churn_event_records() {
    reset_phase98_state();
    let mut stack = vec![syx_runtime_core::Value::Str("promote".into())];
    host_gc_churn_event(&mut stack).unwrap();
    assert_eq!(gc_churn_event_count(), 1);
}
