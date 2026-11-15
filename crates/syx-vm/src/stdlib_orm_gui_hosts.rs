//! Phase 89: XML XPath, Codable L6 schema, GUI loopback hosts.
//! SQL/ORM rusqlite hosts live in `syx-sql-impl` when the `sql` feature is on.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

#[cfg(all(feature = "sql", not(target_arch = "wasm32")))]
pub use syx_sql_impl::{
    host_orm_delete_where_on, host_orm_find, host_orm_find_where_on, host_orm_save,
    host_orm_save_on, host_orm_update_where_on, host_sql_begin_transaction_on,
    host_sql_bind_prepared_on, host_sql_bind_step_on, host_sql_close,
    host_sql_commit_transaction_on, host_sql_exec_on, host_sql_exec_rusqlite, host_sql_open,
    host_sql_pool_acquire, host_sql_pool_close, host_sql_pool_create, host_sql_pool_release,
    host_sql_prepare, host_sql_prepare_on, host_sql_query_on, host_sql_query_rusqlite,
    host_sql_rollback_transaction_on, host_sql_row_column_names, host_sql_row_get_blob,
    host_sql_row_get_float, host_sql_row_get_int, host_sql_row_get_string, host_sql_row_is_null,
    reset_rusqlite_store, rusqlite_conn, sql_conn_force_close, sql_conn_live_count,
};

#[cfg(not(all(feature = "sql", not(target_arch = "wasm32"))))]
pub use crate::sql_stubs::{
    host_orm_delete_where_on, host_orm_find, host_orm_find_where_on, host_orm_save,
    host_orm_save_on, host_orm_update_where_on, host_sql_begin_transaction_on,
    host_sql_bind_prepared_on, host_sql_bind_step_on, host_sql_close,
    host_sql_commit_transaction_on, host_sql_exec_on, host_sql_exec_rusqlite, host_sql_open,
    host_sql_pool_acquire, host_sql_pool_close, host_sql_pool_create, host_sql_pool_release,
    host_sql_prepare, host_sql_prepare_on, host_sql_query_on, host_sql_query_rusqlite,
    host_sql_rollback_transaction_on, host_sql_row_column_names, host_sql_row_get_blob,
    host_sql_row_get_float, host_sql_row_get_int, host_sql_row_get_string, host_sql_row_is_null,
    reset_rusqlite_store, sql_conn_force_close, sql_conn_live_count,
};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn value_as_map(value: &Value) -> Option<HashMap<String, Value>> {
    match Value::resolve_heap_slot(value) {
        Value::Object(map) | Value::Dict(map) => Some(map),
        _ => None,
    }
}

static CODABLE_SCHEMAS: LazyLock<Mutex<HashMap<String, u32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Phase 89: register wire schema id for a type name.
pub fn host_codable_schema_register(stack: &mut Vec<Value>) -> SyxResult<()> {
    let wire_id = stack.pop().unwrap_or(Value::I32(0)).as_i64() as u32;
    let type_name = pop_str(stack);
    if let Ok(mut reg) = CODABLE_SCHEMAS.lock() {
        reg.insert(type_name, wire_id);
    }
    stack.push(Value::Bool(true));
    Ok(())
}

fn xpath_match(expr: &str, el: &HashMap<String, Value>) -> bool {
    let tag = el.get("tag").map(|v| v.as_str()).unwrap_or_default();
    match expr.trim() {
        e if e.starts_with("//") => tag == e.trim_start_matches("//"),
        e if e.starts_with('/') => {
            let parts: Vec<&str> = e.trim_start_matches('/').split('/').collect();
            tag == *parts.last().unwrap_or(&"")
        }
        e if e.starts_with('@') => el.contains_key(&e[1..]),
        _ => tag == expr,
    }
}

fn xpath_collect(expr: &str, el: &HashMap<String, Value>) -> Vec<Value> {
    let mut out = Vec::new();
    if xpath_match(expr, el) {
        out.push(Value::Object(el.clone()));
    }
    if let Some(Value::Array(children)) = el.get("children") {
        for child in children {
            if let Value::Object(map) = child {
                out.extend(xpath_collect(expr, map));
            }
        }
    }
    out
}

/// Phase 89: XPath subset query on parsed XML element tree.
/// Call shape: `__syx_xml_xpath(expr, root)` — last arg is on top of the VM stack.
pub fn host_xml_xpath(stack: &mut Vec<Value>) -> SyxResult<()> {
    let root = stack.pop().unwrap_or(Value::Nil);
    let expr = pop_str(stack);
    let Some(map) = value_as_map(&root) else {
        stack.push(Value::Array(vec![]));
        return Ok(());
    };
    stack.push(Value::Array(xpath_collect(&expr, &map)));
    Ok(())
}

static GUI_WINDOWS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Phase 89: create GUI window (loopback stub).
pub fn host_gui_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let title = pop_str(stack);
    let id = format!("win:{title}");
    if let Ok(mut wins) = GUI_WINDOWS.lock() {
        wins.push(id.clone());
    }
    stack.push(Value::Str(id));
    Ok(())
}

/// Phase 89: pump GUI event loop once (loopback).
pub fn host_gui_event_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _win = pop_str(stack);
    stack.push(Value::Str("event:idle".into()));
    Ok(())
}

/// Phase 89: canvas clear/draw stub.
pub fn host_gui_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _color = pop_str(stack);
    let _win = pop_str(stack);
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn gui_window_count() -> usize {
    GUI_WINDOWS.lock().map(|g| g.len()).unwrap_or(0)
}

pub fn reset_gui_state() {
    if let Ok(mut wins) = GUI_WINDOWS.lock() {
        wins.clear();
    }
}
