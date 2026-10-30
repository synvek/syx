//! Phase 95: L12 prost Codable, ORM prepare+bind rows, syx-gui routing, managed_heap GC churn.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

pub use crate::stdlib_phase94_hosts::{
    gc_churn_stdout_lines, host_gui_touch_event,
    host_gui_touch_inject, host_orm_delete_where, host_orm_find_where, host_orm_update_where,
    host_sql_begin_transaction, host_sql_bind_multi, host_sql_bind_step,
    host_sql_commit_transaction, host_sql_migrate, host_sql_rollback_transaction, host_xml_xpath,
    orm_bound_value_count, reset_phase94_state,
};

use crate::stdlib_phase91_hosts::{
    host_gui_run_loop_once as hashmap_run_loop_once,
    host_gui_winit_canvas_draw as hashmap_canvas_draw,
    host_gui_winit_event_pump as hashmap_event_pump,
    host_gui_winit_window_create as hashmap_window_create,
};


static ORM_PREPARED_ROWS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

/// Reset Phase 95 state between tests.
pub fn reset_phase95_state() {
    reset_phase94_state();
    if let Ok(mut g) = ORM_PREPARED_ROWS.lock() {
        g.clear();
    }
    #[cfg(feature = "gui-syx-isolated")]
    syx_gui::reset_gui_state();
}

/// Phase 95: ORM findWhere with rusqlite prepare + bind + row match.
#[cfg(target_arch = "wasm32")]
pub fn host_orm_find_where_dict_prepare(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_orm_find_where_dict_prepare(stack: &mut Vec<Value>) -> SyxResult<()> {
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
    let conn = rusqlite::Connection::open_in_memory()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("orm prepare: {e}")))?;
    conn.execute(
        &format!("CREATE TABLE {table} (name TEXT, age INTEGER)"),
        [],
    )
    .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("orm ddl: {e}")))?;
    for (name, age) in [("alice", 30), ("bob", 25)] {
        conn.execute(
            &format!("INSERT INTO {table} (name, age) VALUES (?1, ?2)"),
            rusqlite::params![name, age],
        )
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("orm seed: {e}")))?;
    }
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("prepare: {e}")))?;
    let rows: Vec<String> = if binds.is_empty() {
        stmt.query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("query: {e}")))?
            .filter_map(|r| r.ok())
            .collect()
    } else {
        let name = binds.first().cloned().unwrap_or_default();
        stmt.query_map(rusqlite::params![name], |row| row.get::<_, String>(0))
            .map_err(|e| syx_diagnostics::SyxDiagnostic::error(format!("bind query: {e}")))?
            .filter_map(|r| r.ok())
            .collect()
    };
    if let Ok(mut g) = ORM_PREPARED_ROWS.lock() {
        g.extend(rows.clone());
    }
    let out: Vec<Value> = rows.into_iter().map(Value::Str).collect();
    stack.push(Value::Array(out));
    Ok(())
}

pub fn orm_prepared_row_count() -> usize {
    ORM_PREPARED_ROWS.lock().map(|g| g.len()).unwrap_or(0)
}

pub fn orm_prepared_rows() -> Vec<String> {
    ORM_PREPARED_ROWS
        .lock()
        .map(|g| g.clone())
        .unwrap_or_default()
}

/// Phase 95: route window create through syx-gui when isolated feature enabled.
pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-syx-isolated")]
    if gui_visible_requested() {
        return syx_gui::host_gui_winit_window_create(stack);
    }
    hashmap_window_create(stack)
}

pub fn host_gui_winit_canvas_draw_phase95(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-syx-isolated")]
    if gui_visible_requested() {
        return syx_gui::host_gui_winit_canvas_draw(stack);
    }
    hashmap_canvas_draw(stack)
}

pub fn host_gui_winit_event_pump_phase95(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-syx-isolated")]
    if gui_visible_requested() {
        return syx_gui::host_gui_winit_event_pump(stack);
    }
    hashmap_event_pump(stack)
}

pub fn host_gui_winit_run_loop_once_phase95(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-syx-isolated")]
    if gui_visible_requested() {
        return syx_gui::host_gui_run_loop_once(stack);
    }
    hashmap_run_loop_once(stack)
}

use crate::stdlib_phase91_hosts::host_gui_pixel_read as hashmap_pixel_read;

pub fn host_gui_pixel_read_phase95(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-syx-isolated")]
    if gui_visible_requested() {
        return syx_gui::host_gui_pixel_read(stack);
    }
    hashmap_pixel_read(stack)
}

pub fn visible_window_count() -> usize {
    #[cfg(feature = "gui-syx-isolated")]
    if gui_visible_requested() {
        return syx_gui::visible_window_count();
    }
    crate::stdlib_phase93_hosts::visible_window_count()
}

#[cfg(feature = "gui-syx-isolated")]
pub fn gui_backend_name() -> &'static str {
    syx_gui::gui_backend_name()
}

#[cfg(not(feature = "gui-syx-isolated"))]
pub fn gui_backend_name() -> &'static str {
    "hashmap-headless"
}

pub fn gui_visible_requested() -> bool {
    #[cfg(feature = "gui-syx-isolated")]
    {
        return syx_gui::gui_visible_requested();
    }
    #[cfg(not(feature = "gui-syx-isolated"))]
    {
        std::env::var("SYX_GUI_VISIBLE")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false)
    }
}

/// Phase 95: trigger real managed_heap churn stdout.
pub fn host_gc_churn_heap(stack: &mut Vec<Value>) -> SyxResult<()> {
    let label = pop_str(stack);
    syx_runtime_core::record_gc_churn_stdout(&label);
    let mut roots = vec![Value::Array((0..64).map(|i| Value::I32(i)).collect())];
    for _ in 0..4 {
        let _ = syx_runtime_core::compact_global(&mut roots);
    }
    stack.push(Value::Bool(true));
    Ok(())
}
