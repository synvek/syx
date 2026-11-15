//! Phase 90: winit-style headless GUI, SQL migrations, ORM query GA+, Codable L7.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use syx_diagnostics::SyxResult;

use syx_runtime_core::Value;

#[cfg(not(all(feature = "sql", not(target_arch = "wasm32"))))]
pub use crate::sql_stubs::{
    host_orm_delete_where, host_orm_find_where, host_orm_update_where, host_sql_bind_step,
    host_sql_migrate, migration_version,
};
#[cfg(all(feature = "sql", not(target_arch = "wasm32")))]
pub use syx_sql_impl::{
    host_orm_delete_where, host_orm_find_where_l90 as host_orm_find_where, host_orm_update_where,
    host_sql_bind_step, host_sql_migrate, migration_version,
};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

/// Reset Phase 90 SQL/GUI state between tests.
#[cfg(target_arch = "wasm32")]
pub fn reset_phase90_state() {}

#[cfg(all(not(target_arch = "wasm32"), feature = "sql"))]
pub fn reset_phase90_state() {
    syx_sql_impl::reset_rusqlite_store();
    syx_sql_impl::reset_migration_version();
    reset_winit_gui_state();
}

#[cfg(all(not(target_arch = "wasm32"), not(feature = "sql")))]
pub fn reset_phase90_state() {
    reset_winit_gui_state();
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
