//! Phase 98 / C-1d: Host gc_churn_event counter (≠ user-stdout GC KPI).

use syx_vm::{gc_churn_event_count, host_gc_churn_event, reset_phase98_state};

#[test]
fn p98_gc_churn_event_records() {
    reset_phase98_state();
    let mut stack = vec![syx_runtime_core::Value::Str("promote".into())];
    host_gc_churn_event(&mut stack).unwrap();
    assert_eq!(gc_churn_event_count(), 1);
}
