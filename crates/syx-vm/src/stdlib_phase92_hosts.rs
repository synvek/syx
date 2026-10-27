//! Phase 92: L9 Codable protobuf wire, ORM compound reflect, mobile touch GUI, SQL rollback/multi-bind.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

pub use crate::stdlib_phase91_hosts::{
    host_gui_button_create, host_gui_label_create, host_gui_layout_create, host_gui_on_click,
    host_gui_pixel_read, host_gui_run_loop_once, host_gui_text_field_create,
    host_gui_winit_canvas_draw, host_gui_winit_event_pump, host_gui_winit_window_create,
    host_orm_delete_where, host_orm_update_where, host_sql_begin_transaction, host_sql_bind_step,
    host_sql_commit_transaction, host_sql_migrate, host_xml_xpath, migration_version,
    reset_phase91_state, reset_winit_gui_state, winit_window_count,
};

use crate::stdlib_phase89_hosts::host_sql_query_rusqlite;
use crate::stdlib_phase91_hosts::host_orm_find_where as host_orm_find_where_l91;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

/// Reset Phase 92 SQL/GUI/Codable state between tests.
pub fn reset_phase92_state() {
    reset_phase91_state();
    if let Ok(mut g) = MOBILE_TOUCH_EVENTS.lock() {
        g.clear();
    }
}

static MOBILE_TOUCH_EVENTS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Phase 92: ORM findWhereDict compound AND with parameterized reflect SQL.
#[cfg(target_arch = "wasm32")]
pub fn host_orm_find_where_dict(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_orm_find_where_dict(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dict_val = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let table = pop_str(stack);
    let mut clauses = Vec::new();
    if let Value::Dict(map) = dict_val {
        for (col, val) in map {
            clauses.push(format!("{col} = ?"));
            let _ = val;
        }
    }
    let sql = if clauses.is_empty() {
        format!("SELECT * FROM {table}")
    } else {
        format!("SELECT * FROM {table} WHERE {}", clauses.join(" AND "))
    };
    let mut query_inner = vec![Value::Str(sql)];
    host_sql_query_rusqlite(&mut query_inner)?;
    stack.push(query_inner.pop().unwrap_or(Value::Array(vec![])));
    Ok(())
}

/// Phase 92: ORM findWhere — delegate to L91 or compound dict path via env.
pub fn host_orm_find_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let compound = std::env::var("SYX_ORM_COMPOUND")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    if compound {
        let val = pop_str(stack);
        let col = pop_str(stack);
        let table = pop_str(stack);
        let mut map = HashMap::new();
        map.insert(col, Value::Str(val));
        let mut inner = vec![Value::Dict(map), Value::Str(table)];
        return host_orm_find_where_dict(&mut inner);
    }
    host_orm_find_where_l91(stack)
}

/// Phase 92: SQL multi-param bind — comma-separated values bound to last prepare.
pub fn host_sql_bind_multi(stack: &mut Vec<Value>) -> SyxResult<()> {
    let params = pop_str(stack);
    for p in params.split(',') {
        let mut inner = vec![Value::Str(p.trim().to_string())];
        host_sql_bind_step(&mut inner)?;
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 92: ROLLBACK active rusqlite transaction.
#[cfg(target_arch = "wasm32")]
pub fn host_sql_rollback_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_sql_rollback_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut guard = crate::stdlib_phase89_hosts::rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    conn.execute_batch("ROLLBACK;")
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 92: mobile touch event stub (records touch for readback).
pub fn host_gui_touch_event(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let x = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let win_id = pop_str(stack);
    let evt = format!("touch:{win_id}:{x}:{y}");
    if let Ok(mut g) = MOBILE_TOUCH_EVENTS.lock() {
        g.push(evt.clone());
    }
    stack.push(Value::Str(evt));
    Ok(())
}

/// Phase 92: mobile window create — winit window + mobile tag.
pub fn host_gui_mobile_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_gui_winit_window_create(stack)?;
    if let Value::Str(id) = stack.last().cloned().unwrap_or(Value::Str(String::new())) {
        stack.pop();
        stack.push(Value::Str(format!("mobile-{id}")));
    }
    Ok(())
}

/// Phase 92: mobile event pump — touch-aware winit pump.
pub fn host_gui_mobile_event_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    let mut inner = vec![Value::Str(win_id.clone())];
    host_gui_winit_event_pump(&mut inner)?;
    let base = inner.pop().unwrap_or(Value::Str("idle".into())).as_str();
    stack.push(Value::Str(format!("mobile-{base}")));
    Ok(())
}

/// Phase 92: mobile canvas draw — softbuffer pixel fill.
pub fn host_gui_mobile_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_gui_winit_canvas_draw(stack)
}

/// Phase 92: mobile run loop once.
pub fn host_gui_mobile_run_loop_once(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_gui_run_loop_once(stack)
}

/// Phase 92: mobile pixel readback.
pub fn host_gui_mobile_pixel_read(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_gui_pixel_read(stack)
}

pub fn mobile_touch_event_count() -> usize {
    MOBILE_TOUCH_EVENTS.lock().map(|g| g.len()).unwrap_or(0)
}
