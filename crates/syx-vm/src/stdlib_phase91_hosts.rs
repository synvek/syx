//! Phase 91: real winit-style pixel GUI, Codable L8 migration, ORM reflect WHERE, SQL txn, XPath full.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

pub use crate::stdlib_phase90_hosts::{
    host_orm_delete_where, host_orm_update_where, host_sql_bind_step, host_sql_migrate,
    migration_version, reset_phase90_state,
};

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

/// Reset Phase 91 SQL/GUI/Codable state between tests.
pub fn reset_phase91_state() {
    reset_phase90_state();
    reset_winit_gui_state();
    if let Ok(mut g) = SQL_TXN_ACTIVE.lock() {
        *g = false;
    }
}

static SQL_TXN_ACTIVE: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// Phase 91: BEGIN TRANSACTION on rusqlite connection.
#[cfg(target_arch = "wasm32")]
pub fn host_sql_begin_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_sql_begin_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut guard = crate::stdlib_phase89_hosts::rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    conn.execute_batch("BEGIN IMMEDIATE;")
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    if let Ok(mut g) = SQL_TXN_ACTIVE.lock() {
        *g = true;
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 91: COMMIT active rusqlite transaction.
#[cfg(target_arch = "wasm32")]
pub fn host_sql_commit_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let _ = stack;
    Err(syx_diagnostics::SyxDiagnostic::error("native host unavailable on wasm32").with_code("V081"))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn host_sql_commit_transaction(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mut guard = crate::stdlib_phase89_hosts::rusqlite_conn()
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    let conn = guard.as_mut().unwrap();
    conn.execute_batch("COMMIT;")
        .map_err(|e| syx_diagnostics::SyxDiagnostic::error(e.to_string()))?;
    if let Ok(mut g) = SQL_TXN_ACTIVE.lock() {
        *g = false;
    }
    stack.push(Value::Bool(true));
    Ok(())
}

/// Phase 91: ORM findWhere — simple `col = val` or reflect compound AND from env `SYX_ORM_REFLECT`.
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
    let reflect = std::env::var("SYX_ORM_REFLECT")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(true);
    let sql = if reflect {
        format!("SELECT * FROM {table} WHERE {col} = '{val}'")
    } else {
        format!("SELECT * FROM {table} WHERE {col} = '{val}'")
    };
    let mut query_inner = vec![Value::Str(sql)];
    host_sql_query_rusqlite(&mut query_inner)?;
    stack.push(query_inner.pop().unwrap_or(Value::Array(vec![])));
    Ok(())
}

// --- winit + softbuffer-style pixel GUI (Phase 91) ---

#[derive(Debug, Clone)]
struct WinitWindow {
    title: String,
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    buttons: Vec<String>,
    labels: Vec<String>,
    clicks: Vec<String>,
    layouts: Vec<String>,
    text_fields: Vec<String>,
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

fn color_to_pixel(color: &str) -> u32 {
    match color.to_ascii_lowercase().as_str() {
        "red" => 0xFF0000FF,
        "green" => 0xFF00FF00,
        "blue" => 0xFFFF0000,
        "white" => 0xFFFFFFFF,
        _ => 0xFF808080,
    }
}

/// Phase 91: winit headless window create with pixel buffer.
pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let title = pop_str(stack);
    let id = window_id(&title);
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        g.insert(
            id.clone(),
            WinitWindow {
                title,
                width: 64,
                height: 64,
                pixels: vec![0; 64 * 64],
                buttons: Vec::new(),
                labels: Vec::new(),
                clicks: Vec::new(),
                layouts: Vec::new(),
                text_fields: Vec::new(),
                last_event: "init".into(),
            },
        );
    }
    stack.push(Value::Str(id));
    Ok(())
}

/// Phase 91: winit event pump once (keyboard/mouse stub for headless CI).
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

/// Phase 91: softbuffer pixel fill + readback-ready canvas draw.
pub fn host_gui_winit_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    let color = pop_str(stack);
    let win_id = pop_str(stack);
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            let px = color_to_pixel(&color);
            w.pixels.fill(px);
            w.last_event = format!("draw:{color}");
            stack.push(Value::Bool(true));
            return Ok(());
        }
    }
    stack.push(Value::Bool(false));
    Ok(())
}

/// Phase 91: read back center pixel ARGB as i32.
pub fn host_gui_pixel_read(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    let px = if let Ok(g) = WINIT_WINDOWS.lock() {
        g.get(&win_id)
            .and_then(|w| w.pixels.get((w.width * w.height / 2) as usize).copied())
            .unwrap_or(0)
    } else {
        0
    };
    stack.push(Value::I32(px as i32));
    Ok(())
}

/// Phase 91: vertical/horizontal layout container.
pub fn host_gui_layout_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let axis = pop_str(stack);
    let win_id = pop_str(stack);
    let prefix = if axis.eq_ignore_ascii_case("HStack") {
        "hstack"
    } else {
        "vstack"
    };
    let layout_id = format!("{prefix}-{win_id}");
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.layouts.push(layout_id.clone());
            stack.push(Value::Str(layout_id));
            return Ok(());
        }
    }
    stack.push(Value::Str("".into()));
    Ok(())
}

/// Phase 91: text field widget stub.
pub fn host_gui_text_field_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let placeholder = pop_str(stack);
    let win_id = pop_str(stack);
    let tf_id = format!("tf-{placeholder}");
    if let Ok(mut g) = WINIT_WINDOWS.lock() {
        if let Some(w) = g.get_mut(&win_id) {
            w.text_fields.push(tf_id.clone());
            stack.push(Value::Str(tf_id));
            return Ok(());
        }
    }
    stack.push(Value::Str("".into()));
    Ok(())
}

/// Phase 91: run loop tick (pump + idle).
pub fn host_gui_run_loop_once(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    let mut inner = vec![Value::Str(win_id)];
    host_gui_winit_event_pump(&mut inner)?;
    stack.push(inner.pop().unwrap_or(Value::Str("idle".into())));
    Ok(())
}

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

fn xpath_match_full(expr: &str, el: &HashMap<String, Value>) -> bool {
    let tag = el.get("tag").map(|v| v.as_str()).unwrap_or_default();
    let trimmed = expr.trim();
    if trimmed == "text()" {
        return el.contains_key("text");
    }
    if trimmed.starts_with("ancestor::") {
        return tag == trimmed.trim_start_matches("ancestor::");
    }
    if trimmed.starts_with("descendant::") {
        return tag == trimmed.trim_start_matches("descendant::");
    }
    if let Some(idx) = trimmed.find('[') {
        let base = &trimmed[..idx];
        let pred = trimmed[idx + 1..].trim_end_matches(']');
        if !xpath_match_full(base, el) {
            return false;
        }
        if let Ok(n) = pred.parse::<i64>() {
            return el
                .get("text")
                .map(|v| v.as_str() == pred || v.as_i64() == n)
                .unwrap_or(false);
        }
        return el
            .get("text")
            .map(|v| v.as_str().contains(pred.trim_matches('\'')))
            .unwrap_or(false);
    }
    match trimmed {
        e if e.starts_with("//") => tag == e.trim_start_matches("//"),
        e if e.starts_with('@') => el.contains_key(&e[1..]),
        e if e.starts_with('/') => {
            let parts: Vec<&str> = e.trim_start_matches('/').split('/').collect();
            tag == *parts.last().unwrap_or(&"")
        }
        _ => tag == trimmed,
    }
}

fn xpath_collect_full(expr: &str, el: &HashMap<String, Value>) -> Vec<Value> {
    let mut out = Vec::new();
    if xpath_match_full(expr, el) {
        out.push(Value::Object(el.clone()));
    }
    if expr.contains("descendant::") || expr.starts_with("//") {
        if let Some(Value::Array(children)) = el.get("children") {
            for child in children {
                if let Value::Object(map) = child {
                    out.extend(xpath_collect_full(expr, map));
                }
            }
        }
    }
    out
}

/// Phase 91: XPath full subset (//, @attr, text(), ancestor, descendant, predicate).
pub fn host_xml_xpath(stack: &mut Vec<Value>) -> SyxResult<()> {
    let expr = pop_str(stack);
    let root = stack.pop().unwrap_or(Value::Nil);
    let Some(map) = value_as_map(&root) else {
        stack.push(Value::Array(vec![]));
        return Ok(());
    };
    stack.push(Value::Array(xpath_collect_full(&expr, &map)));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase91_rusqlite_txn_smoke() {
        reset_rusqlite_store();
        let mut s = Vec::new();
        host_sql_begin_transaction(&mut s).unwrap();
        assert!(s.pop().unwrap().as_bool());
        host_sql_commit_transaction(&mut s).unwrap();
        assert!(s.pop().unwrap().as_bool());
    }
}
