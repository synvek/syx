//! Phase 98: ORM rusqlite compound bind execution gate.

use std::collections::HashMap;

use syx_vm::{host_orm_find_where_dict_rusqlite, orm_executed_sql_count, orm_last_executed_sql, reset_phase98_state};
use syx_runtime_core::Value;

#[test]
fn p98_orm_rusqlite_bind_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("orm-prepare-bind-v1:rusqlite-row-assert"));
}

#[test]
fn p98_orm_rusqlite_compound_dict_executed() {
    reset_phase98_state();
    let mut map = HashMap::new();
    map.insert("name".into(), Value::Str("alice".into()));
    map.insert("age".into(), Value::I32(30));
    let mut stack = vec![
        Value::Str("users".into()),
        Value::Dict(map),
    ];
    let _ = host_orm_find_where_dict_rusqlite(&mut stack);
    assert_eq!(orm_executed_sql_count(), 1);
    let sql = orm_last_executed_sql().unwrap();
    assert!(sql.contains("name = ?"), "{sql}");
    assert!(sql.contains("alice"), "{sql}");
}
