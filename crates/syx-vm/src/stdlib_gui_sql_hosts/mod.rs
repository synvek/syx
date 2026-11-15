//! GUI / SQL host ladder (former stdlib_phase90–98 hosts).

mod layer90;
pub(crate) mod layer91;
mod layer92;
mod layer93;
mod layer94;
mod layer95;
mod layer96;
mod layer97;

// hosts.rs: mobile / xpath / touch from layer92
pub use layer92::{
    host_gui_mobile_canvas_draw, host_gui_mobile_event_pump, host_gui_mobile_pixel_read,
    host_gui_mobile_run_loop_once, host_gui_mobile_window_create, host_gui_touch_event,
    host_xml_xpath,
};

// lib.rs re-exports formerly from phase93
pub use layer93::{
    gc_churn_event_count, host_gc_churn_event, host_gui_touch_inject,
    host_orm_find_where_dict_bound, orm_bound_value_count, reset_phase93_state,
};

// lib.rs re-exports formerly from phase94
pub use layer94::{
    gc_churn_stdout_lines, host_gc_churn_stdout, host_gui_winit_canvas_draw_phase94,
    host_gui_winit_event_pump_phase94, host_gui_winit_run_loop_once_phase94,
    host_orm_find_where_dict_rusqlite, orm_executed_sql_count, orm_last_executed_sql,
    reset_phase94_state,
};

pub use layer95::reset_phase95_state;
pub use layer96::reset_phase96_state;

// lib.rs + hosts.rs re-exports formerly from phase97
pub use layer97::{
    ensure_gui_loaded, gui_backend_name, gui_caret_metric_px, gui_caret_supported,
    gui_cjk_subset_char_count, gui_colr_grad_used, gui_colr_radial_or_sweep_used,
    gui_colr_radial_used, gui_colr_sweep_used, gui_dlopen_loaded, gui_doctor_coverage_digest,
    gui_doctor_engine_tag, gui_doctor_present_mode, gui_doctor_widgets, gui_dylib_path_report,
    gui_egui_text_suppressed_used, gui_emoji_atlas_hit, gui_emoji_digest, gui_emoji_engine,
    gui_fallback_hits, gui_font_fallback_digest, gui_font_mgr_bundled_used,
    gui_font_mgr_fallback_used, gui_font_mgr_os_used, gui_glyph_coverage_percent,
    gui_harfbuzz_digest, gui_hb_paint_path_used, gui_ime_commit_supported, gui_ime_preedit_digest,
    gui_ime_preedit_supported, gui_no_double_paint_probe, gui_paint_shaped,
    gui_paragraph_paint_used, gui_sample_center_pixel, gui_shaped, gui_skia_glyph_digest,
    gui_skia_glyph_used, gui_ttf_subset_digest, gui_vertical_ascii_ink_used,
    gui_vertical_hb_vertical_used, gui_vertical_ink_used, gui_vertical_rl_digest,
    gui_vertical_rl_layout_used, gui_visible_requested, gui_wgpu_composite_digest,
    gui_wgpu_composite_used, gui_wgpu_present_used, host_codable_decode_l14,
    host_codable_encode_l14, host_codable_l14_register_migration, host_codable_l14_register_rename,
    host_gc_churn_heap, host_gui_button_create, host_gui_button_take_clicked,
    host_gui_click_inject, host_gui_key_inject, host_gui_label_create, host_gui_layout_configure,
    host_gui_layout_create, host_gui_on_click, host_gui_pixel_read_phase97,
    host_gui_text_field_create, host_gui_widget_get_text, host_gui_widget_set_text,
    host_gui_winit_canvas_draw_phase97, host_gui_winit_event_pump_phase97,
    host_gui_winit_run_loop_once_phase97, host_gui_winit_window_create,
    host_orm_find_where_dict_prepare, host_result_builder_build_array,
    host_result_builder_build_block, host_result_builder_build_either,
    host_result_builder_build_optional, host_weak_alive, host_weak_from, host_weak_load,
    orm_prepared_row_count, orm_prepared_rows, require_gui_dylib_if_visible, reset_phase97_state,
    visible_window_count,
};

/// Compatibility wrapper formerly in `stdlib_phase98_hosts`.
pub fn reset_phase98_state() {
    layer97::reset_phase97_state();
}
