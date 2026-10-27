//! Phase 90: winit-style headless GUI, SQL migrations, ORM query GA+, Codable L7.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

use crate::stdlib_phase89_hosts::{host_sql_query_rusqlite, reset_rusqlite_store};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn value_as_map(value: &Value) -> Option<HashMap<String, Value>> {
    match Value::resolve_heap_slot(value) {
        Value::Object(map) | Value::Dict(map) => Some(map),
        _ => None,
    }
}

static MIGRATION_VERSION: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0));

/// Reset Phase 90 SQL/GUI state between tests.
#[cfg(target_arch = "wasm32")]
pub fn reset_phase90_state() {
}

#[cfg(not(target_arch = "wasm32"))]
pub fn reset_phase90_state() {
    reset_rusqlite_store();
    if let Ok(mut v) = MIGRATION_VERSION.lock() {
        *v = 0;
    }
    reset_winit_gui_state();
}

/// Phase 90: apply SQL migration script and bump schema version.
#[cfg(target_arch = "wasm32")]
pub fn host_sql_migrate(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_sql_migrate(stack: &mut Vec<Value>) -> SyxResult<()> {
    let sql = pop_str(stack);
    let mut guard = crate::stdlib_phase89_hosts::rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    let _ =
        conn.execute_batch("CREATE TABLE IF NOT EXISTS _syx_schema (version INTEGER NOT NULL);");
    conn.execute(&sql, [])
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let ver = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM _syx_schema",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let next = ver + 1;
    let _ = conn.execute("INSERT INTO _syx_schema (version) VALUES (?1)", [next]);
    if let Ok(mut g) = MIGRATION_VERSION.lock() {
        *g = next;
    }
    stack.push(Value::I32(next as i32));
    Ok(())
}

/// Phase 90: prepare + bind + step — returns row count changed.
#[cfg(target_arch = "wasm32")]
pub fn host_sql_bind_step(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_sql_bind_step(stack: &mut Vec<Value>) -> SyxResult<()> {
    let bind_val = pop_str(stack);
    let sql = pop_str(stack);
    let mut guard = crate::stdlib_phase89_hosts::rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    let changed = conn
        .execute(&sql, rusqlite::params![bind_val])
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    stack.push(Value::I32(changed as i32));
    Ok(())
}

/// Phase 90: ORM findWhere — SELECT with simple `col = val` WHERE.
#[cfg(target_arch = "wasm32")]
pub fn host_orm_find_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_orm_find_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let val = pop_str(stack);
    let col = pop_str(stack);
    let table = pop_str(stack);
    let mut query_inner = vec![Value::Str(format!(
        "SELECT * FROM {table} WHERE {col} = '{val}'"
    ))];
    host_sql_query_rusqlite(&mut query_inner)?;
    stack.push(query_inner.pop().unwrap_or(Value::Array(vec![])));
    Ok(())
}

/// Phase 90: ORM updateWhere.
#[cfg(target_arch = "wasm32")]
pub fn host_orm_update_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_orm_update_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let new_val = pop_str(stack);
    let new_col = pop_str(stack);
    let where_val = pop_str(stack);
    let where_col = pop_str(stack);
    let table = pop_str(stack);
    let sql = format!("UPDATE {table} SET {new_col} = ? WHERE {where_col} = ?");
    let mut guard = crate::stdlib_phase89_hosts::rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    let changed = conn
        .execute(&sql, rusqlite::params![new_val, where_val])
        .unwrap_or(0);
    stack.push(Value::I32(changed as i32));
    Ok(())
}

/// Phase 90: ORM deleteWhere.
#[cfg(target_arch = "wasm32")]
pub fn host_orm_delete_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_orm_delete_where(stack: &mut Vec<Value>) -> SyxResult<()> {
    let val = pop_str(stack);
    let col = pop_str(stack);
    let table = pop_str(stack);
    let sql = format!("DELETE FROM {table} WHERE {col} = ?");
    let mut guard = crate::stdlib_phase89_hosts::rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    let changed = conn.execute(&sql, rusqlite::params![val]).unwrap_or(0);
    stack.push(Value::I32(changed as i32));
    Ok(())
}

// --- winit-style headless GUI (Phase 90) ---

#[derive(Debug, Clone)]
struct WinitWindow {
    title: String,
    buttons: Vec<String>,
    labels: Vec<String>,
    clicks: Vec<String>,
    last_event: String,
}

static WINIT_WINDOWS: LazyLock<Mutex<HashMap<String, WinitWindow>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub fn reset_winit_gui_state() {
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        g.clear();
    }
}

fn window_id(title: &str) -> String {
    format!("winit-{title}")
}

/// Phase 90: winit headless window create.
pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let title = pop_str(stack);
    let id = window_id(&title);
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        g.insert(
            id.clone(),
            WinitWindow {
                title,
                buttons: Vec::new(),
                labels: Vec::new(),
                clicks: Vec::new(),
                last_event: "init".into(),
            },
        );
    }
    stack.push(Value::Str(id));
    Ok(())
}

/// Phase 90: winit event pump once.
pub fn host_gui_winit_event_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.last_event = format!(
                "pump:{}",
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis()
            );
            stack.push(Value::Str(w.last_event.clone()));
            return Ok(());
        }
    }
    stack.push(Value::Str("no-window".into()));
    Ok(())
}

/// Phase 90: winit canvas draw (softbuffer stub).
pub fn host_gui_winit_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    let color = pop_str(stack);
    let win_id = pop_str(stack);
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.last_event = format!("draw:{color}");
            stack.push(Value::Bool(true));
            return Ok(());
        }
    }
    stack.push(Value::Bool(false));
    Ok(())
}

/// Phase 90: create button on window.
pub fn host_gui_button_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let label = pop_str(stack);
    let win_id = pop_str(stack);
    let btn_id = format!("btn-{label}");
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.buttons.push(btn_id.clone());
            stack.push(Value::Str(btn_id));
            return Ok(());
        }
    }
    stack.push(Value::Str("".into()));
    Ok(())
}

/// Phase 90: create label on window.
pub fn host_gui_label_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = pop_str(stack);
    let win_id = pop_str(stack);
    let lbl_id = format!("lbl-{text}");
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.labels.push(lbl_id.clone());
            stack.push(Value::Str(lbl_id));
            return Ok(());
        }
    }
    stack.push(Value::Str("".into()));
    Ok(())
}

/// Phase 90: register onClick handler name.
pub fn host_gui_on_click(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handler = pop_str(stack);
    let btn_id = pop_str(stack);
    let win_id = pop_str(stack);
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.clicks.push(format!("{btn_id}:{handler}"));
            stack.push(Value::Bool(true));
            return Ok(());
        }
    }
    stack.push(Value::Bool(false));
    Ok(())
}

pub fn winit_window_count() -> usize {
    WINIT_WINDOWS.lock().map(|g| g.len()).unwrap_or(0)
}

pub fn migration_version() -> u32 {
    MIGRATION_VERSION.lock().map(|g| *g).unwrap_or(0)
}
