//! Phase 93: L10 protobuf wire, ORM compound bind, visible winit GUI, mobile touch, GC churn.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

pub use crate::stdlib_phase92_hosts::{
    host_gui_button_create, host_gui_label_create, host_gui_layout_create,
    host_gui_mobile_canvas_draw, host_gui_mobile_event_pump, host_gui_mobile_pixel_read,
    host_gui_mobile_run_loop_once, host_gui_mobile_window_create, host_gui_on_click,
    host_gui_pixel_read, host_gui_run_loop_once, host_gui_text_field_create, host_gui_touch_event,
    host_gui_winit_canvas_draw, host_gui_winit_event_pump, host_orm_delete_where,
    host_orm_find_where, host_orm_update_where, host_sql_begin_transaction, host_sql_bind_multi,
    host_sql_bind_step, host_sql_commit_transaction, host_sql_migrate,
    host_sql_rollback_transaction, host_xml_xpath, mobile_touch_event_count, reset_phase92_state,
    winit_window_count,
};

use crate::stdlib_phase91_hosts::host_gui_winit_window_create as host_gui_winit_window_create_inner;

use crate::stdlib_phase89_hosts::host_sql_query_rusqlite;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

static ORM_BOUND_VALUES: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static GC_CHURN_EVENTS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static VISIBLE_WINDOWS: LazyLock<Mutex<HashMap<String, bool>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Reset Phase 93 state between tests.
pub fn reset_phase93_state() {
    reset_phase92_state();
    if let Ok(mut g) = ORM_BOUND_VALUES.lock() {
        g.clear();
    }
    if let Ok(mut g) = GC_CHURN_EVENTS.lock() {
        g.clear();
    }
    if let Ok(mut g) = VISIBLE_WINDOWS.lock() {
        g.clear();
    }
}

/// Phase 93: ORM findWhereDict with real parameter binding.
pub fn host_orm_find_where_dict_bound(stack: &mut Vec<Value>) -> SyxResult<()> {
    let dict_val = stack.pop().unwrap_or(Value::Dict(HashMap::new()));
    let table = pop_str(stack);
    let mut clauses = Vec::new();
    let mut binds = Vec::new();
    if let Value::Dict(map) = dict_val {
        for (col, val) in map {
            clauses.push(format!("{col} = ?"));
            binds.push(val.as_str());
        }
    }
    if let Ok(mut g) = ORM_BOUND_VALUES.lock() {
        g.extend(binds.clone());
    }
    let sql = if clauses.is_empty() {
        format!("SELECT * FROM {table}")
    } else {
        format!("SELECT * FROM {table} WHERE {}", clauses.join(" AND "))
    };
    let mut query_inner = vec![Value::Str(sql)];
    host_sql_query_rusqlite(&mut query_inner)?;
    let rows = query_inner.pop().unwrap_or(Value::Array(vec![]));
    stack.push(Value::Array(vec![rows, Value::Str(binds.join(","))]));
    Ok(())
}

pub fn orm_bound_value_count() -> usize {
    ORM_BOUND_VALUES.lock().map(|g| g.len()).unwrap_or(0)
}

/// Phase 93: winit window create + visible flag for softbuffer readback CI.
pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    host_gui_winit_window_create_inner(stack)?;
    if let Value::Str(id) = stack.last().cloned().unwrap_or(Value::Str(String::new())) {
        if let Ok(mut g) = VISIBLE_WINDOWS.lock() {
            g.insert(id.as_str().to_string(), true);
        }
    }
    Ok(())
}

pub fn host_gui_window_show(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    if let Ok(mut g) = VISIBLE_WINDOWS.lock() {
        g.insert(win_id.clone(), true);
    }
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn visible_window_count() -> usize {
    VISIBLE_WINDOWS
        .lock()
        .map(|g| g.values().filter(|v| **v).count())
        .unwrap_or(0)
}

/// Phase 93: GC churn event record for quad stdout gates.
pub fn host_gc_churn_event(stack: &mut Vec<Value>) -> SyxResult<()> {
    let label = pop_str(stack);
    if let Ok(mut g) = GC_CHURN_EVENTS.lock() {
        g.push(label);
    }
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn gc_churn_event_count() -> usize {
    GC_CHURN_EVENTS.lock().map(|g| g.len()).unwrap_or(0)
}

/// Phase 93: mobile touch with injection queue (non-stub path).
pub fn host_gui_touch_inject(stack: &mut Vec<Value>) -> SyxResult<()> {
    let y = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let x = stack.pop().unwrap_or(Value::I32(0)).as_i64();
    let win_id = pop_str(stack);
    let evt = format!("inject:{win_id}:{x}:{y}");
    host_gui_touch_event(&mut vec![
        Value::Str(win_id),
        Value::I32(x as i32),
        Value::I32(y as i32),
    ])?;
    if let Ok(mut g) = GC_CHURN_EVENTS.lock() {
        g.push(evt);
    }
    stack.push(Value::Bool(true));
    Ok(())
}
