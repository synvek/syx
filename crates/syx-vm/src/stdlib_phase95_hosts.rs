//! Phase 95: L12 prost Codable, ORM prepare+bind rows, syx-gui routing, managed_heap GC churn.

use syx_diagnostics::SyxResult;

use syx_runtime_core::Value;

#[cfg(not(all(feature = "sql", not(target_arch = "wasm32"))))]
pub use crate::sql_stubs::{
    host_orm_find_where_dict_prepare, orm_prepared_row_count, orm_prepared_rows,
};
#[cfg(all(feature = "sql", not(target_arch = "wasm32")))]
pub use syx_sql_impl::{
    host_orm_find_where_dict_prepare, orm_prepared_row_count, orm_prepared_rows,
};

pub use crate::stdlib_phase94_hosts::{
    gc_churn_stdout_lines, host_gui_touch_event, host_gui_touch_inject, host_orm_delete_where,
    host_orm_find_where, host_orm_update_where, host_sql_begin_transaction, host_sql_bind_multi,
    host_sql_bind_step, host_sql_commit_transaction, host_sql_migrate,
    host_sql_rollback_transaction, host_xml_xpath, orm_bound_value_count, reset_phase94_state,
};

use crate::stdlib_phase91_hosts::{
    host_gui_run_loop_once as hashmap_run_loop_once,
    host_gui_winit_canvas_draw as hashmap_canvas_draw,
    host_gui_winit_event_pump as hashmap_event_pump,
    host_gui_winit_window_create as hashmap_window_create,
};

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

/// Reset Phase 95 state between tests.
pub fn reset_phase95_state() {
    reset_phase94_state();
    #[cfg(all(feature = "sql", not(target_arch = "wasm32")))]
    syx_sql_impl::reset_orm_prepared_rows();
    #[cfg(feature = "gui-syx-isolated")]
    syx_gui::reset_gui_state();
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
