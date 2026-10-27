//! Phase 101: C ABI exports for `libloading` dlopen (interactive hit-test + layout).

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use crate::{
    drain_pending_handlers, host_gui_button_create, host_gui_button_clicked,
    host_gui_button_take_clicked, host_gui_click_inject, host_gui_key_inject,
    host_gui_label_create, host_gui_layout_create, host_gui_layout_configure,
    host_gui_on_click_register,
    host_gui_pixel_read, host_gui_run_loop_once, host_gui_text_field_create,
    host_gui_widget_get_text, host_gui_widget_set_text, host_gui_winit_canvas_draw,
    host_gui_winit_event_pump, host_gui_winit_window_create, reset_gui_state,
};
use syx_runtime_core::Value;

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn push_c_string(out: *mut *mut c_char, s: &str) -> bool {
    if out.is_null() {
        return false;
    }
    match CString::new(s) {
        Ok(c) => {
            unsafe { *out = c.into_raw() };
            true
        }
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_backend_name() -> *const c_char {
    b"syx-gui-callback-ga-v1:dlopen\0".as_ptr() as *const c_char
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_reset() {
    reset_gui_state();
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe {
            let _ = CString::from_raw(ptr);
        }
    }
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_window_create(
    title: *const c_char,
    out_id: *mut *mut c_char,
) -> bool {
    let title = unsafe {
        if title.is_null() {
            String::new()
        } else {
            CStr::from_ptr(title).to_string_lossy().into_owned()
        }
    };
    let mut stack = vec![Value::Str(title)];
    if host_gui_winit_window_create(&mut stack).is_err() {
        return false;
    }
    let id = pop_str(&mut stack);
    push_c_string(out_id, &id)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_canvas_draw(win_id: *const c_char, color: *const c_char) -> bool {
    let win = unsafe {
        if win_id.is_null() {
            String::new()
        } else {
            CStr::from_ptr(win_id).to_string_lossy().into_owned()
        }
    };
    let color = unsafe {
        if color.is_null() {
            String::new()
        } else {
            CStr::from_ptr(color).to_string_lossy().into_owned()
        }
    };
    let mut stack = vec![Value::Str(win), Value::Str(color)];
    host_gui_winit_canvas_draw(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_pixel_read(win_id: *const c_char) -> i32 {
    let win = unsafe {
        if win_id.is_null() {
            String::new()
        } else {
            CStr::from_ptr(win_id).to_string_lossy().into_owned()
        }
    };
    let mut stack = vec![Value::Str(win)];
    host_gui_pixel_read(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_i32())
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_run_loop_once(
    win_id: *const c_char,
    out_event: *mut *mut c_char,
) -> bool {
    let win = unsafe {
        if win_id.is_null() {
            String::new()
        } else {
            CStr::from_ptr(win_id).to_string_lossy().into_owned()
        }
    };
    let mut stack = vec![Value::Str(win)];
    if host_gui_run_loop_once(&mut stack).is_err() {
        return false;
    }
    let ev = pop_str(&mut stack);
    push_c_string(out_event, &ev)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_event_pump(
    win_id: *const c_char,
    out_event: *mut *mut c_char,
) -> bool {
    let win = unsafe {
        if win_id.is_null() {
            String::new()
        } else {
            CStr::from_ptr(win_id).to_string_lossy().into_owned()
        }
    };
    let mut stack = vec![Value::Str(win)];
    if host_gui_winit_event_pump(&mut stack).is_err() {
        return false;
    }
    let ev = pop_str(&mut stack);
    push_c_string(out_event, &ev)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_visible_window_count() -> usize {
    crate::visible_window_count()
}

fn read_c_str(p: *const c_char) -> String {
    unsafe {
        if p.is_null() {
            String::new()
        } else {
            CStr::from_ptr(p).to_string_lossy().into_owned()
        }
    }
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_label_create(
    win_id: *const c_char,
    text: *const c_char,
    out_id: *mut *mut c_char,
) -> bool {
    let mut stack = vec![Value::Str(read_c_str(win_id)), Value::Str(read_c_str(text))];
    if host_gui_label_create(&mut stack).is_err() {
        return false;
    }
    let id = pop_str(&mut stack);
    push_c_string(out_id, &id)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_button_create(
    win_id: *const c_char,
    text: *const c_char,
    out_id: *mut *mut c_char,
) -> bool {
    let mut stack = vec![Value::Str(read_c_str(win_id)), Value::Str(read_c_str(text))];
    if host_gui_button_create(&mut stack).is_err() {
        return false;
    }
    let id = pop_str(&mut stack);
    push_c_string(out_id, &id)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_text_field_create(
    win_id: *const c_char,
    text: *const c_char,
    out_id: *mut *mut c_char,
) -> bool {
    let mut stack = vec![Value::Str(read_c_str(win_id)), Value::Str(read_c_str(text))];
    if host_gui_text_field_create(&mut stack).is_err() {
        return false;
    }
    let id = pop_str(&mut stack);
    push_c_string(out_id, &id)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_layout_create(
    win_id: *const c_char,
    axis: *const c_char,
    out_id: *mut *mut c_char,
) -> bool {
    let mut stack = vec![
        Value::Str(read_c_str(win_id)),
        Value::Str(read_c_str(axis)),
    ];
    if host_gui_layout_create(&mut stack).is_err() {
        return false;
    }
    let id = pop_str(&mut stack);
    push_c_string(out_id, &id)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_layout_configure(
    win_id: *const c_char,
    spacing: i32,
    align: *const c_char,
    out_tag: *mut *mut c_char,
) -> bool {
    let mut stack = vec![
        Value::Str(read_c_str(win_id)),
        Value::I32(spacing),
        Value::Str(read_c_str(align)),
    ];
    if host_gui_layout_configure(&mut stack).is_err() {
        return false;
    }
    let tag = pop_str(&mut stack);
    push_c_string(out_tag, &tag)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_on_click_register(
    win_id: *const c_char,
    btn_id: *const c_char,
    handler: *const c_char,
) -> bool {
    let mut stack = vec![
        Value::Str(read_c_str(win_id)),
        Value::Str(read_c_str(btn_id)),
        Value::Str(read_c_str(handler)),
    ];
    host_gui_on_click_register(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_click_inject(win_id: *const c_char, btn_id: *const c_char) -> bool {
    let mut stack = vec![Value::Str(read_c_str(win_id)), Value::Str(read_c_str(btn_id))];
    host_gui_click_inject(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_button_take_clicked(
    win_id: *const c_char,
    widget_id: *const c_char,
) -> bool {
    let mut stack = vec![
        Value::Str(read_c_str(win_id)),
        Value::Str(read_c_str(widget_id)),
    ];
    host_gui_button_take_clicked(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_widget_set_text(
    win_id: *const c_char,
    widget_id: *const c_char,
    text: *const c_char,
) -> bool {
    let mut stack = vec![
        Value::Str(read_c_str(win_id)),
        Value::Str(read_c_str(widget_id)),
        Value::Str(read_c_str(text)),
    ];
    host_gui_widget_set_text(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_button_clicked(win_id: *const c_char, widget_id: *const c_char) -> bool {
    let mut stack = vec![
        Value::Str(read_c_str(win_id)),
        Value::Str(read_c_str(widget_id)),
    ];
    host_gui_button_clicked(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_widget_get_text(
    win_id: *const c_char,
    widget_id: *const c_char,
    out_text: *mut *mut c_char,
) -> bool {
    let mut stack = vec![
        Value::Str(read_c_str(win_id)),
        Value::Str(read_c_str(widget_id)),
    ];
    if host_gui_widget_get_text(&mut stack).is_err() {
        return false;
    }
    let text = pop_str(&mut stack);
    push_c_string(out_text, &text)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_key_inject(win_id: *const c_char, key: *const c_char) -> bool {
    let mut stack = vec![Value::Str(read_c_str(win_id)), Value::Str(read_c_str(key))];
    host_gui_key_inject(&mut stack)
        .ok()
        .and_then(|_| stack.pop())
        .map(|v| v.as_bool())
        .unwrap_or(false)
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_drain_pending_handlers(
    win_id: *const c_char,
    out_handlers: *mut *mut c_char,
) -> bool {
    let handlers = drain_pending_handlers(&read_c_str(win_id));
    push_c_string(out_handlers, &handlers.join(","))
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ttf_fingerprint() -> *const c_char {
    c"gui-ttf-expand-v1:cjk64-plus".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ime_preedit_fingerprint() -> *const c_char {
    c"gui-ime-highlight-v1:preedit-invert".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_doctor_coverage_fingerprint() -> *const c_char {
    c"gui-doctor-engine-v6:phase118".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_egui_wgpu_fingerprint() -> *const c_char {
    c"gui-egui-wgpu-v1:winit-present".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_hb_paint_fingerprint() -> *const c_char {
    c"gui-hb-paint-v1:string-shape".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_emoji_fingerprint() -> *const c_char {
    c"gui-emoji-v9:font-radial-and-sweep".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_features_fingerprint() -> *const c_char {
    c"gui-ot-features-v7:os-fontmgr-default-paint".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_vf_export_parity_fingerprint() -> *const c_char {
    c"gui-dylib-parity-v1:ot-vf-export".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_wght_fingerprint() -> *const c_char {
    c"gui-font-vf-v1:wght-mvp".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_multi_axis_fingerprint() -> *const c_char {
    c"gui-font-vf-v2:multi-axis-mvp".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_axes_ge3_fingerprint() -> *const c_char {
    c"gui-font-vf-v4:paint-variations-mvp".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_paint_variations_fingerprint() -> *const c_char {
    c"gui-font-vf-v4:paint-variations-mvp".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_ot_substitution() -> bool {
    crate::gui_prove_ot_substitution()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_subst_proven() -> bool {
    crate::gui_ot_subst_proven()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_ot_script_subset() -> bool {
    crate::gui_prove_ot_script_subset()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_script_subst_proven() -> bool {
    crate::gui_ot_script_subst_proven()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_ot_feature_matrix() -> bool {
    crate::gui_prove_ot_feature_matrix()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_feature_matrix_proven() -> bool {
    crate::gui_ot_feature_matrix_proven()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_ot_paint_matrix() -> bool {
    crate::gui_prove_ot_paint_matrix()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_ot_default_paint() -> bool {
    crate::gui_prove_ot_default_paint()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_default_paint_proven() -> bool {
    crate::gui_ot_default_paint_proven()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_default_paint_fontmgr_used() -> bool {
    crate::gui_default_paint_fontmgr_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_paint_matrix_proven() -> bool {
    crate::gui_ot_paint_matrix_proven()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_vf_wght() -> bool {
    crate::gui_prove_vf_wght()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_wght_used() -> bool {
    crate::gui_vf_wght_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_vf_multi_axis() -> bool {
    crate::gui_prove_vf_multi_axis()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_multi_axis_used() -> bool {
    crate::gui_vf_multi_axis_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_vf_axes_ge3() -> bool {
    crate::gui_prove_vf_axes_ge3()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_axes_ge3_used() -> bool {
    crate::gui_vf_axes_ge3_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_prove_vf_paint_variations() -> bool {
    crate::gui_prove_vf_paint_variations()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vf_paint_variations_used() -> bool {
    crate::gui_vf_paint_variations_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ot_calt_used() -> bool {
    crate::gui_ot_calt_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vertical_rl_fingerprint() -> *const c_char {
    c"gui-vertical-unify-v1:hb-advance".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_harfbuzz_fingerprint() -> *const c_char {
    c"gui-harfbuzz-v1:shape-mvp".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_font_fallback_fingerprint() -> *const c_char {
    c"gui-font-fallback-v1:system-cjk".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_shaped() -> bool {
    crate::gui_shaped()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_paint_shaped() -> bool {
    crate::gui_paint_shaped()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_fallback_hits() -> u32 {
    crate::gui_fallback_hits()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_glyph_coverage_percent() -> u32 {
    crate::gui_glyph_coverage_percent()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_cjk_subset_char_count() -> u32 {
    crate::gui_cjk_subset_char_count()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ime_preedit_supported() -> bool {
    crate::gui_ime_preedit_supported()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ime_commit_supported() -> bool {
    crate::gui_ime_commit_supported()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_caret_supported() -> bool {
    crate::gui_caret_supported()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_caret_metric_px() -> u32 {
    crate::gui_caret_metric_px()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_wgpu_composite_fingerprint() -> *const c_char {
    c"gui-wgpu-composite-v1:hb-emoji-tex".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_wgpu_present_used() -> bool {
    crate::gui_wgpu_present_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_wgpu_composite_used() -> bool {
    crate::gui_wgpu_composite_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_emoji_engine() -> *const c_char {
    match crate::gui_emoji_engine() {
        "colr" => b"colr\0".as_ptr() as *const c_char,
        "colr-grad" => b"colr-grad\0".as_ptr() as *const c_char,
        "cbdt" => b"cbdt\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vertical_rl_layout_used() -> bool {
    crate::gui_vertical_rl_layout_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_doctor_present_mode() -> *const c_char {
    // Must return NUL-terminated C strings — Rust &str is not CStr-safe.
    match crate::gui_doctor_present_mode() {
        "softbuffer-dev-fallback" => c"softbuffer-dev-fallback".as_ptr(),
        "egui-wgpu-v1:hb-emoji-composite" => c"egui-wgpu-v1:hb-emoji-composite".as_ptr(),
        "egui-wgpu-v1" => c"egui-wgpu-v1".as_ptr(),
        _ => c"".as_ptr(),
    }
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_font_system_fingerprint() -> *const c_char {
    c"gui-font-system-v5:os-only-all-apis".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_doctor_engine_tag() -> *const c_char {
    match crate::gui_doctor_engine_tag() {
        "colr" => b"colr\0".as_ptr() as *const c_char,
        "colr-radial-and-sweep" => b"colr-radial-and-sweep\0".as_ptr() as *const c_char,
        "colr-radial" => b"colr-radial\0".as_ptr() as *const c_char,
        "colr-sweep" => b"colr-sweep\0".as_ptr() as *const c_char,
        "colr-grad" => b"colr-grad\0".as_ptr() as *const c_char,
        "cbdt" => b"cbdt\0".as_ptr() as *const c_char,
        _ => b"unknown\0".as_ptr() as *const c_char,
    }
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_paragraph_paint_used() -> bool {
    crate::gui_paragraph_paint_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_skia_layout_fingerprint() -> *const c_char {
    c"gui-skia-paragraph-v6:os-ot-aligned".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_skia_layout_used() -> bool {
    crate::gui_skia_layout_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_bidi_used() -> bool {
    crate::gui_bidi_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vertical_unify_used() -> bool {
    crate::gui_vertical_unify_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_skia_glyph_fingerprint() -> *const c_char {
    c"gui-skia-glyph-v1:raster-mvp".as_ptr()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_skia_glyph_used() -> bool {
    crate::gui_skia_glyph_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vertical_ink_used() -> bool {
    crate::gui_vertical_ink_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_egui_text_suppressed_used() -> bool {
    crate::gui_egui_text_suppressed_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_no_double_paint_probe() -> bool {
    crate::gui_no_double_paint_probe()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_font_mgr_os_used() -> bool {
    crate::gui_font_mgr_os_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_font_mgr_bundled_used() -> bool {
    crate::gui_font_mgr_bundled_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_emoji_atlas_hit() -> bool {
    crate::gui_emoji_atlas_hit()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_font_mgr_fallback_used() -> bool {
    crate::gui_font_mgr_fallback_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vertical_ascii_ink_used() -> bool {
    crate::gui_vertical_ascii_ink_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_hb_paint_path_used() -> bool {
    crate::gui_hb_paint_path_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_vertical_hb_vertical_used() -> bool {
    crate::gui_vertical_hb_vertical_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_colr_radial_used() -> bool {
    crate::emoji::colr_radial_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_colr_sweep_used() -> bool {
    crate::emoji::colr_sweep_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_colr_radial_or_sweep_used() -> bool {
    crate::emoji::colr_radial_or_sweep_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_colr_grad_used() -> bool {
    crate::emoji::colr_grad_used()
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_doctor_widgets(out: *mut *mut c_char) -> bool {
    push_c_string(out, &crate::gui_doctor_widgets())
}

#[no_mangle]
pub extern "C" fn syx_gui_dylib_ime_preedit_inject(win_id: *const c_char, preedit: *const c_char) -> bool {
    crate::apply_ime_preedit_inject(&read_c_str(win_id), &read_c_str(preedit))
}
