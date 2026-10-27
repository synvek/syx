//! Phase 98: ORM rusqlite prepare + bind row assert gate.

use std::collections::HashMap;

use syx_runtime_core::Value;
use syx_vm::{host_orm_find_where_dict_prepare, orm_prepared_rows, reset_phase98_state};

#[test]
fn p98_orm_prepare_bind_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("orm-prepare-bind-v1"));
}

#[test]
fn p98_orm_prepare_bind_alice_row() {
    reset_phase98_state();
    std::env::set_var("SYX_ORM_BIND", "1");
    let mut map = HashMap::new();
    map.insert("name".into(), Value::Str("alice".into()));
    let mut stack = vec![Value::Str("users".into()), Value::Dict(map)];
    host_orm_find_where_dict_prepare(&mut stack).unwrap();
    let rows = orm_prepared_rows();
    assert!(rows.iter().any(|r| r == "alice"), "rows: {rows:?}");
    let out = stack.pop().unwrap();
    if let Value::Array(items) = out {
        assert!(items.iter().any(|v| v.as_str() == "alice"), "{items:?}");
    }
    std::env::remove_var("SYX_ORM_BIND");
}
