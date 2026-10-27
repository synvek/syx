//! Phase 96: gui dlopen routing, GC churn SYX host.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

pub use crate::stdlib_phase95_hosts::{
    gc_churn_stdout_lines, host_gui_button_create, host_gui_label_create, host_gui_on_click,
    host_gui_text_field_create, host_gui_touch_event, host_gui_touch_inject, host_orm_delete_where,
    host_orm_find_where, host_orm_find_where_dict_prepare, host_orm_update_where,
    host_sql_begin_transaction, host_sql_bind_multi, host_sql_bind_step,
    host_sql_commit_transaction, host_sql_migrate, host_sql_rollback_transaction, host_xml_xpath,
    orm_bound_value_count, orm_prepared_row_count, orm_prepared_rows,
};

use crate::stdlib_phase91_hosts::{
    host_gui_run_loop_once as hashmap_run_loop_once,
    host_gui_winit_canvas_draw as hashmap_canvas_draw,
    host_gui_winit_event_pump as hashmap_event_pump,
    host_gui_winit_window_create as hashmap_window_create,
};

use crate::stdlib_phase91_hosts::host_gui_pixel_read as hashmap_pixel_read;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

/// Reset Phase 96 state between tests.
pub fn reset_phase96_state() {
    crate::stdlib_phase95_hosts::reset_phase95_state();
    #[cfg(feature = "gui-dlopen")]
    crate::gui_dlopen::reset_dlopen_state();
}

fn gui_visible_env() -> bool {
    std::env::var("SYX_GUI_VISIBLE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn use_dlopen_path() -> bool {
    #[cfg(feature = "gui-dlopen")]
    {
        if gui_visible_env() {
            return crate::gui_dlopen::ensure_loaded();
        }
    }
    false
}

pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_window_create(stack);
    }
    hashmap_window_create(stack)
}

pub fn host_gui_winit_canvas_draw_phase96(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_canvas_draw(stack);
    }
    hashmap_canvas_draw(stack)
}

pub fn host_gui_winit_event_pump_phase96(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_event_pump(stack);
    }
    hashmap_event_pump(stack)
}

pub fn host_gui_winit_run_loop_once_phase96(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_run_loop_once(stack);
    }
    hashmap_run_loop_once(stack)
}

pub fn host_gui_pixel_read_phase96(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_pixel_read(stack);
    }
    hashmap_pixel_read(stack)
}

pub fn visible_window_count() -> usize {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::visible_window_count_dlopen();
    }
    crate::stdlib_phase93_hosts::visible_window_count()
}

pub fn gui_backend_name() -> &'static str {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_backend_name();
    }
    "hashmap-headless"
}

pub fn gui_visible_requested() -> bool {
    gui_visible_env()
}

pub fn gui_dlopen_loaded() -> bool {
    #[cfg(feature = "gui-dlopen")]
    {
        return crate::gui_dlopen::dlopen_loaded();
    }
    #[cfg(not(feature = "gui-dlopen"))]
    {
        false
    }
}

/// Phase 96: trigger managed_heap churn and return stdout line for SYX program print.
pub fn host_gc_churn_heap(stack: &mut Vec<Value>) -> SyxResult<()> {
    let label = pop_str(stack);
    let line = format!("gc:churn:{label}");
    syx_runtime_core::record_gc_churn_stdout(&label);
    let mut roots = vec![Value::Array((0..64).map(|i| Value::I32(i)).collect())];
    for _ in 0..4 {
        let _ = syx_runtime_core::compact_global(&mut roots);
    }
    stack.push(Value::Str(line));
    Ok(())
}

pub fn ensure_gui_loaded() -> bool {
    #[cfg(feature = "gui-dlopen")]
    {
        return crate::gui_dlopen::ensure_loaded();
    }
    #[cfg(not(feature = "gui-dlopen"))]
    {
        true
    }
}
