//! Phase 98: ORM compound parameter binding gate.

use std::collections::HashMap;

use syx_vm::{host_orm_find_where_dict_bound, orm_bound_value_count, reset_phase98_state};
use syx_runtime_core::Value;

#[test]
fn p98_orm_bind_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("orm-bind-compound-v1"));
}

#[test]
fn p98_orm_compound_binds_values() {
    reset_phase98_state();
    std::env::set_var("SYX_ORM_BIND", "1");
    let mut map = HashMap::new();
    map.insert("name".into(), Value::Str("alice".into()));
    map.insert("age".into(), Value::Str("30".into()));
    let mut stack = vec![Value::Dict(map), Value::Str("users".into())];
    host_orm_find_where_dict_bound(&mut stack).unwrap();
    assert!(orm_bound_value_count() >= 2);
    std::env::remove_var("SYX_ORM_BIND");
}
