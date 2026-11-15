//! Phase 92: L9 Codable protobuf wire, ORM compound reflect, mobile touch GUI, SQL rollback/multi-bind.

use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;

use syx_runtime_core::Value;

pub use crate::stdlib_phase91_hosts::{
    host_gui_button_create, host_gui_label_create, host_gui_layout_create, host_gui_on_click,
    host_gui_pixel_read, host_gui_run_loop_once, host_gui_text_field_create,
    host_gui_winit_canvas_draw, host_gui_winit_event_pump, host_gui_winit_window_create,
    host_orm_delete_where, host_orm_update_where, host_sql_begin_transaction, host_sql_bind_step,
    host_sql_commit_transaction, host_sql_migrate, host_xml_xpath, reset_phase91_state,
    winit_window_count,
};

#[cfg(not(all(feature = "sql", not(target_arch = "wasm32"))))]
pub use crate::sql_stubs::{
    host_orm_find_where, host_orm_find_where_dict, host_sql_bind_multi,
    host_sql_rollback_transaction,
};
#[cfg(all(feature = "sql", not(target_arch = "wasm32")))]
pub use syx_sql_impl::{
    host_orm_find_where, host_orm_find_where_dict, host_sql_bind_multi,
    host_sql_rollback_transaction,
};

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
