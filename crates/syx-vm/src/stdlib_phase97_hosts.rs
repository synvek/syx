//! Phase 97: L14 prost schema evolution, GUI dlopen default, GC churn re-export.

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use prost::Message;
use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

pub use crate::stdlib_phase96_hosts::{
    host_gc_churn_heap, host_orm_find_where_dict_prepare, orm_prepared_row_count, orm_prepared_rows,
};

use crate::stdlib_phase91_hosts::host_gui_pixel_read as hashmap_pixel_read;
use crate::stdlib_phase91_hosts::{
    host_gui_button_create as hashmap_button_create, host_gui_label_create as hashmap_label_create,
    host_gui_text_field_create as hashmap_text_field_create,
    host_gui_winit_canvas_draw as hashmap_canvas_draw,
};
// Phase 99: use Phase 93 create so VISIBLE_WINDOWS is updated (Phase 91 alone does not).
use crate::stdlib_phase93_hosts::host_gui_winit_window_create as hashmap_window_create;

static CODABLE_L14_SCHEMA: LazyLock<Mutex<HashMap<String, Vec<String>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

static CODABLE_L14_RENAMES: LazyLock<Mutex<HashMap<String, Vec<(String, String)>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, PartialEq, Message)]
struct SyxCodableL14Msg {
    #[prost(string, tag = "1")]
    payload: String,
    #[prost(string, repeated, tag = "2")]
    fields: Vec<String>,
    #[prost(string, repeated, tag = "3")]
    renames: Vec<String>,
}

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

pub fn reset_phase97_state() {
    crate::stdlib_phase96_hosts::reset_phase96_state();
    if let Ok(mut g) = CODABLE_L14_SCHEMA.lock() {
        g.clear();
    }
    if let Ok(mut g) = CODABLE_L14_RENAMES.lock() {
        g.clear();
    }
    #[cfg(feature = "gui-dlopen")]
    crate::gui_dlopen::reset_dlopen_state();
}

pub fn host_codable_l14_register_migration(stack: &mut Vec<Value>) -> SyxResult<()> {
    let field = pop_str(stack);
    let type_name = pop_str(stack);
    if let Ok(mut g) = CODABLE_L14_SCHEMA.lock() {
        g.entry(type_name).or_default().push(field);
    }
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_codable_l14_register_rename(stack: &mut Vec<Value>) -> SyxResult<()> {
    let new_name = pop_str(stack);
    let old_name = pop_str(stack);
    let type_name = pop_str(stack);
    if let Ok(mut g) = CODABLE_L14_RENAMES.lock() {
        g.entry(type_name).or_default().push((old_name, new_name));
    }
    stack.push(Value::Bool(true));
    Ok(())
}

pub fn host_codable_encode_l14(stack: &mut Vec<Value>) -> SyxResult<()> {
    // Materialize HeapSlot strings (offheap) — Debug-fallback must not encode slot meta.
    let value = Value::materialize_receiver(stack.pop().unwrap_or(Value::Nil));
    let type_name = pop_str(stack);
    let payload = match &value {
        Value::Str(s) => s.clone(),
        Value::I32(n) => n.to_string(),
        _ => format!("{value:?}"),
    };
    let fields = CODABLE_L14_SCHEMA
        .lock()
        .ok()
        .and_then(|g| g.get(&type_name).cloned())
        .unwrap_or_default();
    let renames = CODABLE_L14_RENAMES
        .lock()
        .ok()
        .and_then(|g| g.get(&type_name).cloned())
        .unwrap_or_default()
        .into_iter()
        .flat_map(|(o, n)| [format!("{o}->{n}")])
        .collect();
    let msg = SyxCodableL14Msg {
        payload,
        fields,
        renames,
    };
    let mut wire = msg.encode_to_vec();
    wire.insert(0, b'L');
    wire.insert(1, b'1');
    wire.insert(2, b'4');
    stack.push(Value::Array(wire.into_iter().map(Value::U8).collect()));
    Ok(())
}

pub fn host_codable_decode_l14(stack: &mut Vec<Value>) -> SyxResult<()> {
    let bytes_val = Value::materialize_receiver(stack.pop().unwrap_or(Value::Array(vec![])));
    let _type_name = pop_str(stack);
    let data: Vec<u8> = match bytes_val {
        Value::Array(items) => items
            .into_iter()
            .filter_map(|v| v.as_i64().try_into().ok())
            .collect(),
        Value::Str(s) => s.into_bytes(),
        _ => Vec::new(),
    };
    let start = if data.starts_with(b"L14") { 3 } else { 0 };
    let msg = SyxCodableL14Msg::decode(&data[start..]).unwrap_or(SyxCodableL14Msg {
        payload: String::new(),
        fields: vec![],
        renames: vec![],
    });
    stack.push(Value::Str(msg.payload));
    Ok(())
}

fn use_dlopen_path() -> bool {
    #[cfg(feature = "gui-dlopen")]
    {
        crate::gui_dlopen::require_dylib_if_visible();
        return crate::gui_dlopen::ensure_loaded();
    }
    #[cfg(not(feature = "gui-dlopen"))]
    {
        false
    }
}

pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        crate::gui_dlopen::host_window_create(stack)?;
        // Mirror into process-local visible set so doctor/gates work when dylib count lags.
        if let Some(Value::Str(id)) = stack.last() {
            if !id.contains("fail") {
                let mut show = vec![Value::Str(id.clone())];
                let _ = crate::stdlib_phase93_hosts::host_gui_window_show(&mut show);
            }
        }
        return Ok(());
    }
    hashmap_window_create(stack)
}

pub fn host_gui_winit_canvas_draw_phase97(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_canvas_draw(stack);
    }
    hashmap_canvas_draw(stack)
}

pub fn host_gui_winit_event_pump_phase97(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    let mut inner = vec![Value::Str(win_id.clone())];
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        crate::gui_dlopen::host_event_pump(&mut inner)?;
    } else {
        hashmap_event_pump(&mut inner)?;
    }
    let raw = inner.pop().unwrap_or(Value::Str("pump".into())).as_str();
    let event = crate::gui_invoke::finish_pump_event(&win_id, &raw)?;
    stack.push(Value::Str(event));
    Ok(())
}

pub fn host_gui_winit_run_loop_once_phase97(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    let mut inner = vec![Value::Str(win_id.clone())];
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        crate::gui_dlopen::host_run_loop_once(&mut inner)?;
    } else {
        hashmap_run_loop_once(&mut inner)?;
    }
    let raw = inner.pop().unwrap_or(Value::Str("tick".into())).as_str();
    let event = crate::gui_invoke::finish_pump_event(&win_id, &raw)?;
    stack.push(Value::Str(event));
    Ok(())
}

pub fn host_gui_pixel_read_phase97(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_pixel_read(stack);
    }
    hashmap_pixel_read(stack)
}

pub fn host_gui_label_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_label_create(stack);
    }
    hashmap_label_create(stack)
}

pub fn host_gui_button_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_button_create(stack);
    }
    hashmap_button_create(stack)
}

pub fn host_gui_text_field_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_text_field_create(stack);
    }
    hashmap_text_field_create(stack)
}

pub fn host_gui_on_click(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handler = pop_str(stack);
    let btn_id = pop_str(stack);
    let win_id = pop_str(stack);
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        let mut inner = vec![Value::Str(win_id), Value::Str(btn_id), Value::Str(handler)];
        crate::gui_dlopen::host_on_click_register(&mut inner)?;
        stack.push(inner.pop().unwrap_or(Value::Bool(false)));
        return Ok(());
    }
    let mut inner = vec![Value::Str(win_id), Value::Str(btn_id), Value::Str(handler)];
    crate::stdlib_phase91_hosts::host_gui_on_click(&mut inner)?;
    stack.push(inner.pop().unwrap_or(Value::Bool(false)));
    Ok(())
}

pub fn host_gui_layout_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let axis = pop_str(stack);
    let win_id = pop_str(stack);
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        let mut inner = vec![Value::Str(win_id), Value::Str(axis)];
        crate::gui_dlopen::host_layout_create(&mut inner)?;
        stack.push(inner.pop().unwrap_or(Value::Str(String::new())));
        return Ok(());
    }
    let mut inner = vec![Value::Str(win_id), Value::Str(axis)];
    crate::stdlib_phase91_hosts::host_gui_layout_create(&mut inner)?;
    stack.push(inner.pop().unwrap_or(Value::Str(String::new())));
    Ok(())
}

pub fn host_gui_layout_configure(stack: &mut Vec<Value>) -> SyxResult<()> {
    let align = pop_str(stack);
    let spacing = stack.pop().unwrap_or(Value::I32(2));
    let win_id = pop_str(stack);
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        let mut inner = vec![Value::Str(win_id), spacing, Value::Str(align)];
        crate::gui_dlopen::host_layout_configure(&mut inner)?;
        stack.push(inner.pop().unwrap_or(Value::Str(String::new())));
        return Ok(());
    }
    let _ = (win_id, spacing, align);
    stack.push(Value::Str(String::new()));
    Ok(())
}

pub fn host_gui_widget_get_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_widget_get_text(stack);
    }
    let widget = pop_str(stack);
    let win_id = pop_str(stack);
    stack.push(Value::Str(String::new()));
    let _ = (win_id, widget);
    Ok(())
}

pub fn host_gui_widget_set_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_widget_set_text(stack);
    }
    let _text = pop_str(stack);
    let _widget = pop_str(stack);
    let _win_id = pop_str(stack);
    stack.push(Value::Bool(false));
    Ok(())
}

pub fn host_gui_key_inject(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_key_inject(stack);
    }
    let _key = pop_str(stack);
    let _win_id = pop_str(stack);
    stack.push(Value::Bool(false));
    Ok(())
}

pub fn host_gui_click_inject(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_click_inject(stack);
    }
    let btn_id = pop_str(stack);
    let win_id = pop_str(stack);
    let mut inner = vec![
        Value::Str(win_id),
        Value::Str(btn_id),
        Value::Str("inject".into()),
    ];
    crate::stdlib_phase91_hosts::host_gui_on_click(&mut inner)?;
    stack.push(inner.pop().unwrap_or(Value::Bool(false)));
    Ok(())
}

pub fn host_gui_button_take_clicked(stack: &mut Vec<Value>) -> SyxResult<()> {
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return crate::gui_dlopen::host_button_take_clicked(stack);
    }
    let btn_id = pop_str(stack);
    let win_id = pop_str(stack);
    stack.push(Value::Bool(false));
    let _ = (win_id, btn_id);
    Ok(())
}

pub fn visible_window_count() -> usize {
    let hashmap = crate::stdlib_phase93_hosts::visible_window_count();
    #[cfg(feature = "gui-dlopen")]
    if use_dlopen_path() {
        return hashmap.max(crate::gui_dlopen::visible_window_count_dlopen());
    }
    hashmap
}

pub fn gui_backend_name() -> &'static str {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_backend_name();
    }
    if use_dlopen_path() {
        return "syx-gui-interactive-v1:dlopen-pending";
    }
    "syx-gui-interactive-v1:hashmap-headless"
}

pub fn gui_visible_requested() -> bool {
    std::env::var("SYX_GUI_VISIBLE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
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

pub fn gui_glyph_coverage_percent() -> u32 {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_glyph_coverage_percent();
    }
    0
}

pub fn gui_ttf_subset_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_ttf_fingerprint();
    }
    String::new()
}

pub fn gui_ime_preedit_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_ime_preedit_fingerprint();
    }
    String::new()
}

pub fn gui_doctor_coverage_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_doctor_coverage_fingerprint();
    }
    String::new()
}

pub fn gui_wgpu_composite_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_wgpu_composite_fingerprint();
    }
    String::new()
}

pub fn gui_emoji_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_emoji_fingerprint();
    }
    String::new()
}

pub fn gui_vertical_rl_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_vertical_rl_fingerprint();
    }
    String::new()
}

pub fn gui_wgpu_present_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_wgpu_present_used();
    }
    false
}

pub fn gui_wgpu_composite_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_wgpu_composite_used();
    }
    false
}

pub fn gui_emoji_engine() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_emoji_engine();
    }
    String::new()
}

pub fn gui_doctor_engine_tag() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_doctor_engine_tag();
    }
    String::new()
}

pub fn gui_vertical_rl_layout_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_vertical_rl_layout_used();
    }
    false
}

pub fn gui_doctor_present_mode() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_doctor_present_mode();
    }
    String::new()
}

pub fn gui_skia_glyph_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_skia_glyph_fingerprint();
    }
    String::new()
}

pub fn gui_skia_glyph_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_skia_glyph_used();
    }
    false
}

pub fn gui_vertical_ink_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_vertical_ink_used();
    }
    false
}

pub fn gui_vertical_hb_vertical_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_vertical_hb_vertical_used();
    }
    false
}

pub fn gui_egui_text_suppressed_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_egui_text_suppressed_used();
    }
    false
}

pub fn gui_no_double_paint_probe() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_no_double_paint_probe();
    }
    false
}

pub fn gui_doctor_widgets() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_doctor_widgets();
    }
    String::new()
}

pub fn gui_emoji_atlas_hit() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_emoji_atlas_hit();
    }
    false
}

pub fn gui_paragraph_paint_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_paragraph_paint_used();
    }
    false
}

pub fn gui_hb_paint_path_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_hb_paint_path_used();
    }
    false
}

pub fn gui_font_mgr_os_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_font_mgr_os_used();
    }
    false
}

pub fn gui_font_mgr_bundled_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_font_mgr_bundled_used();
    }
    false
}

pub fn gui_font_mgr_fallback_used() -> bool {
    gui_font_mgr_os_used() || gui_font_mgr_bundled_used()
}

pub fn gui_vertical_ascii_ink_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_vertical_ascii_ink_used();
    }
    false
}

pub fn gui_colr_radial_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_colr_radial_used();
    }
    false
}

pub fn gui_colr_sweep_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_colr_sweep_used();
    }
    false
}

pub fn gui_colr_radial_or_sweep_used() -> bool {
    gui_colr_radial_used() && gui_colr_sweep_used()
}

pub fn gui_colr_grad_used() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_colr_grad_used();
    }
    false
}

pub fn gui_harfbuzz_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_harfbuzz_fingerprint();
    }
    String::new()
}

pub fn gui_font_fallback_fingerprint() -> String {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_font_fallback_fingerprint();
    }
    String::new()
}

pub fn gui_shaped() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_shaped();
    }
    false
}

pub fn gui_paint_shaped() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_paint_shaped();
    }
    false
}

pub fn gui_fallback_hits() -> u32 {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_fallback_hits();
    }
    0
}

pub fn gui_caret_metric_px() -> u32 {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_caret_metric_px();
    }
    0
}

pub fn gui_cjk_subset_char_count() -> u32 {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_cjk_subset_char_count();
    }
    0
}

pub fn gui_ime_preedit_supported() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_ime_preedit_supported();
    }
    false
}

pub fn gui_ime_commit_supported() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_ime_commit_supported();
    }
    false
}

pub fn gui_caret_supported() -> bool {
    #[cfg(feature = "gui-dlopen")]
    if crate::gui_dlopen::dlopen_loaded() {
        return crate::gui_dlopen::dlopen_caret_supported();
    }
    false
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

pub fn require_gui_dylib_if_visible() {
    #[cfg(feature = "gui-dlopen")]
    {
        crate::gui_dlopen::require_dylib_if_visible();
    }
}

pub fn gui_sample_center_pixel(win_id: &str) -> i32 {
    let mut stack = vec![Value::Str(win_id.to_string())];
    let _ = host_gui_pixel_read_phase97(&mut stack);
    stack.pop().map(|v| v.as_i32()).unwrap_or(0)
}

pub fn gui_dylib_path_report() -> String {
    std::env::var("SYX_GUI_DYLIB").unwrap_or_else(|_| "auto".into())
}

fn flatten_rb_chunk(v: Value, out: &mut Vec<Value>) {
    let v = Value::resolve_heap_slot(&v);
    match v {
        Value::Array(items) => {
            for item in items {
                flatten_rb_chunk(item, out);
            }
        }
        other => out.push(other),
    }
}

/// Phase 102: concat two result-builder chunks into one flat Array.
pub fn host_result_builder_build_block(stack: &mut Vec<Value>) -> SyxResult<()> {
    let right = stack.pop().unwrap_or(Value::Nil);
    let left = stack.pop().unwrap_or(Value::Nil);
    let mut out = Vec::new();
    flatten_rb_chunk(left, &mut out);
    flatten_rb_chunk(right, &mut out);
    stack.push(Value::Array(out));
    Ok(())
}

/// Phase 102: pick then/else chunk by Bool condition.
pub fn host_result_builder_build_either(stack: &mut Vec<Value>) -> SyxResult<()> {
    let b = stack.pop().unwrap_or(Value::Nil);
    let a = stack.pop().unwrap_or(Value::Nil);
    let cond = stack.pop().unwrap_or(Value::Bool(false)).as_bool();
    let mut out = Vec::new();
    flatten_rb_chunk(if cond { a } else { b }, &mut out);
    stack.push(Value::Array(out));
    Ok(())
}

/// Phase 102: identity / normalize to flat Array.
pub fn host_result_builder_build_array(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    let mut out = Vec::new();
    flatten_rb_chunk(v, &mut out);
    stack.push(Value::Array(out));
    Ok(())
}

/// Phase 103: optional branch — include `then` chunk when binding is not `Nil`.
pub fn host_result_builder_build_optional(stack: &mut Vec<Value>) -> SyxResult<()> {
    let else_chunk = stack.pop().unwrap_or(Value::Nil);
    let then_chunk = stack.pop().unwrap_or(Value::Nil);
    let binding = stack.pop().unwrap_or(Value::Nil);
    let pick = if matches!(binding, Value::Nil) {
        else_chunk
    } else {
        then_chunk
    };
    let mut out = Vec::new();
    flatten_rb_chunk(pick, &mut out);
    stack.push(Value::Array(out));
    Ok(())
}

/// Phase 103: weak ref from class instance without retaining.
pub fn host_weak_from(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    // Class locals are MutRefCell wrappers; follow __syx_ref to ObjectRef.
    let resolved = Value::dispatch_receiver_for_method(Value::resolve_heap_slot(&v));
    stack.push(Value::weak_from_object_ref(&resolved));
    Ok(())
}

/// Phase 103: upgrade weak handle to strong ref or `Nil`.
pub fn host_weak_load(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    stack.push(Value::weak_load(&v));
    Ok(())
}

/// Phase 103: whether weak still points at a live object.
pub fn host_weak_alive(stack: &mut Vec<Value>) -> SyxResult<()> {
    let v = stack.pop().unwrap_or(Value::Nil);
    stack.push(Value::Bool(Value::weak_is_alive(&v)));
    Ok(())
}

#[cfg(test)]
mod result_builder_host_tests {
    use super::*;

    #[test]
    fn build_block_flattens_nested_arrays() {
        let mut stack = vec![
            Value::Array(vec![Value::Str("one".into())]),
            Value::Array(vec![Value::Str("two".into())]),
        ];
        host_result_builder_build_block(&mut stack).unwrap();
        let v = stack.pop().unwrap();
        match v {
            Value::Array(a) => {
                assert_eq!(a.len(), 2, "{a:?}");
                assert_eq!(a[0].as_str(), "one");
                assert_eq!(a[1].as_str(), "two");
            }
            other => panic!("expected array, got {other:?}"),
        }
    }
}
