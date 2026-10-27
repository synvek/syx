//! Phase 94: L11 pure protobuf, ORM rusqlite bind execution, winit real routing, GC churn stdout.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

pub use crate::stdlib_phase93_hosts::{
    gc_churn_event_count, host_gui_button_create, host_gui_label_create, host_gui_layout_create,
    host_gui_mobile_canvas_draw, host_gui_mobile_event_pump, host_gui_mobile_pixel_read,
    host_gui_mobile_run_loop_once, host_gui_mobile_window_create, host_gui_on_click,
    host_gui_pixel_read, host_gui_run_loop_once, host_gui_text_field_create, host_gui_touch_event,
    host_gui_touch_inject, host_orm_delete_where, host_orm_find_where, host_orm_update_where,
    host_sql_begin_transaction, host_sql_bind_multi, host_sql_bind_step,
    host_sql_commit_transaction, host_sql_migrate, host_sql_rollback_transaction, host_xml_xpath,
    mobile_touch_event_count, orm_bound_value_count, reset_phase92_state, winit_window_count,
};

use crate::stdlib_phase91_hosts::{
    host_gui_run_loop_once as hashmap_run_loop_once,
    host_gui_winit_canvas_draw as hashmap_canvas_draw,
    host_gui_winit_event_pump as hashmap_event_pump,
    host_gui_winit_window_create as hashmap_window_create,
};

use crate::stdlib_phase89_hosts::host_sql_query_rusqlite;

#[cfg(feature = "gui-winit")]
pub use crate::gui_winit_real::{
    gui_backend_name, gui_visible_requested, real_visible_window_count, reset_gui_winit_real,
};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

static ORM_EXECUTED_SQL: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

static GC_CHURN_STDOUT: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// Reset Phase 94 state between tests.
pub fn reset_phase94_state() {
    crate::stdlib_phase93_hosts::reset_phase93_state();
    if let Ok(mut g) = ORM_EXECUTED_SQL.lock() {
        g.clear();
    }
    if let Ok(mut g) = GC_CHURN_STDOUT.lock() {
        g.clear();
    }
    #[cfg(feature = "gui-winit")]
    reset_gui_winit_real();
}

/// Phase 94: ORM findWhereDict with rusqlite execution record.
pub fn host_orm_find_where_dict_rusqlite(stack: &mut Vec<Value>) -> SyxResult<()> {
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
    let sql = if clauses.is_empty() {
        format!("SELECT * FROM {table}")
    } else {
        format!("SELECT * FROM {table} WHERE {}", clauses.join(" AND "))
    };
    if let Ok(mut g) = ORM_EXECUTED_SQL.lock() {
        g.push(format!("{sql}|{}", binds.join(",")));
    }
    let mut qstack = vec![Value::Str(sql.clone())];
    host_sql_query_rusqlite(&mut qstack)?;
    let rows = qstack.pop().unwrap_or(Value::Array(vec![]));
    stack.push(rows);
    Ok(())
}

pub fn orm_executed_sql_count() -> usize {
    ORM_EXECUTED_SQL.lock().map(|g| g.len()).unwrap_or(0)
}

pub fn orm_last_executed_sql() -> Option<String> {
    ORM_EXECUTED_SQL.lock().ok().and_then(|g| g.last().cloned())
}

/// Phase 94: GC churn with stable stdout line `gc:churn:{reason}`.
pub fn host_gc_churn_stdout(stack: &mut Vec<Value>) -> SyxResult<()> {
    let label = pop_str(stack);
    let line = format!("gc:churn:{label}");
    if let Ok(mut g) = GC_CHURN_STDOUT.lock() {
        g.push(line.clone());
    }
    stack.push(Value::Str(line));
    Ok(())
}

pub fn gc_churn_stdout_lines() -> Vec<String> {
    GC_CHURN_STDOUT
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default()
}

/// Phase 94: route winit window create to real backend when visible requested.
pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-winit")]
    if gui_visible_requested() {
        return crate::gui_winit_real::host_gui_winit_window_create(stack);
    }
    hashmap_window_create(stack)
}

/// Phase 94: route canvas draw.
pub fn host_gui_winit_canvas_draw_phase94(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-winit")]
    if gui_visible_requested() {
        return crate::gui_winit_real::host_gui_winit_canvas_draw(stack);
    }
    hashmap_canvas_draw(stack)
}

/// Phase 94: route event pump.
pub fn host_gui_winit_event_pump_phase94(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-winit")]
    if gui_visible_requested() {
        return crate::gui_winit_real::host_gui_winit_event_pump(stack);
    }
    hashmap_event_pump(stack)
}

/// Phase 94: route run loop once.
pub fn host_gui_winit_run_loop_once_phase94(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-winit")]
    if gui_visible_requested() {
        return crate::gui_winit_real::host_gui_winit_run_loop_once(stack);
    }
    hashmap_run_loop_once(stack)
}

pub fn visible_window_count() -> usize {
    #[cfg(feature = "gui-winit")]
    if gui_visible_requested() {
        return real_visible_window_count();
    }
    crate::stdlib_phase93_hosts::visible_window_count()
}

#[cfg(not(feature = "gui-winit"))]
pub fn gui_backend_name() -> &'static str {
    "hashmap-headless"
}

#[cfg(not(feature = "gui-winit"))]
pub fn gui_visible_requested() -> bool {
    false
}
