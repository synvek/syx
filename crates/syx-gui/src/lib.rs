//! Phase 118 Track A: egui + egui-wgpu present + all-widgets frame CPU ink + doctor v5.
//! Fingerprints:
//!   gui-egui-wgpu-v1:winit-present
//!   gui-skia-paragraph-v6:os-ot-aligned
//!   gui-skia-glyph-v1:raster-mvp
//!   gui-wgpu-composite-v1:hb-emoji-tex
//!   gui-hb-paint-v1:string-shape
//!   gui-emoji-v9:font-radial-and-sweep
//!   gui-font-system-v5:os-only-all-apis
//!   gui-ot-features-v7:os-fontmgr-default-paint
//!   gui-vertical-unify-v1:hb-advance
//!   gui-doctor-engine-v6:phase118
//! Carry: gui-pump-reentrancy-v1:dirty-defer / gui-harfbuzz-v1 / gui-font-fallback-v1
//! Softbuffer only when `SYX_GUI_SOFTBUFFER=1` (doctor reports fallback honestly).
//!
//! When `SYX_GUI_VISIBLE=1`, OS windows are kept alive on a thread-local event loop
//! and pumped by `run_loop_once` / `event_pump`. Without the env var, only an
//! in-memory pixel buffer is used (headless).

mod ascii_font;
mod cjk_font;
mod dylib_api;
mod egui_backend;
mod emoji;
mod shape;
#[cfg(feature = "skia-glyph")]
mod skia_glyph;
#[cfg(feature = "skia-glyph")]
mod skia_layout;
mod wgpu_present;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::time::Duration;

use parking_lot::Mutex;
use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, Ime, KeyEvent, MouseButton, WindowEvent};
use winit::keyboard::{Key, NamedKey};
use winit::event_loop::EventLoop;
use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::window::{Window, WindowBuilder};

static WINDOWS: LazyLock<Mutex<HashMap<String, GuiWindow>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy, PartialEq, Eq)]
enum LayoutAxis {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum VAlign {
    Top,
    Center,
}

struct StackLayout {
    id: String,
    axis: LayoutAxis,
    next_x: u32,
    next_y: u32,
    row_y: u32,
    row_height: u32,
    spacing: u32,
    v_align: VAlign,
}

struct GuiWindow {
    #[allow(dead_code)]
    title: String,
    width: u32,
    height: u32,
    pixels: Vec<u32>,
    last_event: String,
    widgets: Vec<Widget>,
    next_widget_y: u32,
    layout: Option<StackLayout>,
    click_handlers: HashMap<String, String>,
    pending_handlers: Vec<String>,
    focus_widget: Option<String>,
    cursor_x: f64,
    cursor_y: f64,
    /// IME composition string (not committed to TextField `text` until Commit).
    ime_preedit: String,
    /// Byte range highlight from winit (inverted preedit segment).
    ime_preedit_highlight: Option<(usize, usize)>,
    /// Phase 108: per-window egui context + frame counter + texture atlas.
    egui: egui_backend::EguiWindowState,
}

#[derive(Clone)]
pub(crate) struct Widget {
    id: String,
    kind: WidgetKind,
    text: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
    clicked: bool,
    edited: bool,
    /// Insertion caret as **character index** (not UTF-8 byte offset).
    caret: usize,
    writing_mode: WritingMode,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum WidgetKind {
    Label,
    Button,
    TextField,
}

/// Label writing-mode (Phase 109 vertical-rl MVP).
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum WritingMode {
    #[default]
    Horizontal,
    VerticalRl,
}

struct OsGuiWindow {
    window: Window,
    wgpu: Option<wgpu_present::WgpuWindowPresent>,
}

/// Live OS window state (macOS winit EventLoop is not Send — keep thread-local).
///
/// By default `reset_gui_state` drops the loop before thread teardown. Set
/// `SYX_GUI_LEAK_LOOP=1` to keep the intentional process-lifetime leak (HOLD DX).
struct LiveGui {
    event_loop: EventLoop<()>,
    os_windows: HashMap<String, OsGuiWindow>,
}

thread_local! {
    // Option<*mut T> Drop is a no-op — intentional process-lifetime leak.
    static LIVE: RefCell<Option<*mut LiveGui>> = const { RefCell::new(None) };
    /// >0 while inside `EventLoop::pump_events` callback chain — defer OS present.
    static PUMP_DEPTH: Cell<u32> = const { Cell::new(0) };
    static DIRTY_OS_WINDOWS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

fn with_live_mut<R>(f: impl FnOnce(&mut LiveGui) -> R) -> Option<R> {
    // Copy the raw pointer then drop the RefCell borrow before invoking `f`.
    // Keyboard/IME handlers refresh pixels from inside `pump_events`; holding
    // `borrow_mut` across the callback caused `RefCell already borrowed`.
    let ptr = LIVE.with(|cell| *cell.borrow())?;
    // SAFETY: pointer from Box::into_raw on this thread; never freed.
    Some(f(unsafe { &mut *ptr }))
}

fn pump_enter() {
    PUMP_DEPTH.set(PUMP_DEPTH.get().saturating_add(1));
}

fn pump_leave() {
    PUMP_DEPTH.set(PUMP_DEPTH.get().saturating_sub(1));
}

fn mark_os_window_dirty(win_id: &str) {
    DIRTY_OS_WINDOWS.with(|cell| {
        let mut v = cell.borrow_mut();
        if !v.iter().any(|id| id == win_id) {
            v.push(win_id.to_string());
        }
    });
}

fn flush_dirty_os_windows() {
    let ids = DIRTY_OS_WINDOWS.with(|cell| std::mem::take(&mut *cell.borrow_mut()));
    for id in ids {
        refresh_window_pixels_now(&id);
    }
}

pub fn gui_backend_name() -> &'static str {
    if wgpu_present::softbuffer_fallback_requested() {
        "syx-gui-egui-wgpu-v1:softbuffer-fallback"
    } else if gui_visible_requested() {
        "syx-gui-egui-wgpu-v1:visible"
    } else {
        "syx-gui-egui-wgpu-v1:winit-present"
    }
}

pub fn gui_font_cjk_subset_fingerprint() -> &'static str {
    cjk_font::ttf_subset_fingerprint()
}

pub fn gui_harfbuzz_fingerprint() -> &'static str {
    shape::harfbuzz_fingerprint()
}

pub fn gui_hb_paint_fingerprint() -> &'static str {
    shape::hb_paint_fingerprint()
}

pub fn gui_font_fallback_fingerprint() -> &'static str {
    shape::font_fallback_fingerprint()
}

pub fn gui_ime_preedit_fingerprint() -> &'static str {
    "gui-ime-highlight-v1:preedit-invert"
}

pub fn gui_egui_mvp_fingerprint() -> &'static str {
    egui_backend::EGUI_WGPU_FINGERPRINT
}

pub fn gui_egui_wgpu_fingerprint() -> &'static str {
    egui_backend::EGUI_WGPU_FINGERPRINT
}

pub fn gui_emoji_fingerprint() -> &'static str {
    emoji::emoji_fingerprint()
}

pub fn gui_skia_glyph_fingerprint() -> &'static str {
    #[cfg(feature = "skia-glyph")]
    {
        return skia_glyph::skia_glyph_fingerprint();
    }
    #[cfg(not(feature = "skia-glyph"))]
    {
        ""
    }
}

pub fn gui_skia_glyph_used() -> bool {
    #[cfg(feature = "skia-glyph")]
    {
        return skia_glyph::skia_glyph_used();
    }
    #[cfg(not(feature = "skia-glyph"))]
    {
        false
    }
}

pub fn gui_skia_layout_fingerprint() -> &'static str {
    #[cfg(feature = "skia-glyph")]
    {
        return skia_layout::skia_layout_fingerprint();
    }
    #[cfg(not(feature = "skia-glyph"))]
    {
        ""
    }
}

pub fn gui_skia_layout_used() -> bool {
    #[cfg(feature = "skia-glyph")]
    {
        return skia_layout::layout_used();
    }
    #[cfg(not(feature = "skia-glyph"))]
    {
        false
    }
}

pub fn gui_bidi_used() -> bool {
    #[cfg(feature = "skia-glyph")]
    {
        return skia_layout::bidi_used();
    }
    #[cfg(not(feature = "skia-glyph"))]
    {
        false
    }
}

pub fn gui_vertical_unify_used() -> bool {
    egui_backend::vertical_unify_used()
}

pub fn gui_vertical_ink_used() -> bool {
    egui_backend::vertical_ink_used()
}

pub fn gui_vertical_hb_vertical_used() -> bool {
    egui_backend::vertical_hb_vertical_used()
}

pub fn gui_egui_text_suppressed_used() -> bool {
    egui_backend::egui_text_suppressed_used()
}

pub fn gui_no_double_paint_probe() -> bool {
    egui_backend::no_double_paint_probe()
}

pub fn gui_vertical_ascii_ink_used() -> bool {
    egui_backend::vertical_ascii_ink_used()
}

#[cfg(feature = "skia-glyph")]
pub fn gui_font_mgr_os_used() -> bool {
    skia_layout::font_mgr_os_used()
}

#[cfg(not(feature = "skia-glyph"))]
pub fn gui_font_mgr_os_used() -> bool {
    false
}

#[cfg(feature = "skia-glyph")]
pub fn gui_font_mgr_bundled_used() -> bool {
    skia_layout::font_mgr_bundled_used()
}

#[cfg(not(feature = "skia-glyph"))]
pub fn gui_font_mgr_bundled_used() -> bool {
    false
}

#[cfg(feature = "skia-glyph")]
pub fn gui_font_mgr_fallback_used() -> bool {
    skia_layout::font_mgr_fallback_used()
}

#[cfg(not(feature = "skia-glyph"))]
pub fn gui_font_mgr_fallback_used() -> bool {
    false
}

pub fn gui_hb_paint_path_used() -> bool {
    egui_backend::hb_paint_path_used()
}

/// Doctor widgets string — dynamic coverage from probes that actually ran.
pub fn gui_doctor_widgets() -> String {
    let mut parts = Vec::new();
    if egui_backend::fonts_injected() {
        parts.push("egui");
    }
    if egui_backend::egui_text_suppressed_used() {
        parts.push("egui-text-suppressed");
    }
    if egui_backend::no_double_paint_probe() {
        parts.push("no-double-paint");
    }
    #[cfg(feature = "skia-glyph")]
    if skia_layout::paragraph_paint_used() {
        parts.push("paragraph-paint");
    }
    #[cfg(feature = "skia-glyph")]
    if skia_layout::vertical_paragraph_used() {
        parts.push("vertical-paragraph");
    }
    if egui_backend::hb_paint_path_used() {
        parts.push("hb-paint");
    }
    if egui_backend::vertical_hb_vertical_used() {
        parts.push("hb-vertical");
    }
    #[cfg(feature = "skia-glyph")]
    if skia_layout::font_mgr_os_used() {
        parts.push("font-mgr-os");
        parts.push("layout-os-only");
    }
    #[cfg(feature = "skia-glyph")]
    if skia_layout::font_mgr_bundled_used() {
        parts.push("font-mgr-bundled");
        parts.push("bundled-not-ga");
    }
    if shape::ot_subst_proven() {
        parts.push("ot-subst");
    }
    if shape::ot_script_subst_proven() {
        parts.push("ot-script");
    }
    if shape::ot_feature_matrix_proven() {
        parts.push("ot-feature-matrix");
    }
    if shape::ot_paint_matrix_proven() {
        parts.push("ot-paint-matrix");
    }
    if shape::ot_default_paint_proven() {
        parts.push("ot-default-paint");
        parts.push("ot-os-fontmgr-default-paint");
    }
    if shape::vf_wght_used() {
        parts.push("vf-wght");
    }
    if shape::vf_multi_axis_used() {
        parts.push("vf-multi-axis");
    }
    if shape::vf_axes_ge3_used() {
        parts.push("vf-axes-ge3");
    }
    if shape::vf_paint_variations_used() {
        parts.push("vf-paint-variations");
    }
    if emoji::atlas_hit() {
        parts.push(emoji::doctor_engine_tag());
    }
    if emoji::colr_radial_used() {
        parts.push("colr-radial");
    }
    if emoji::colr_sweep_used() {
        parts.push("colr-sweep");
    }
    if emoji::colr_grad_used() && !emoji::colr_radial_used() && !emoji::colr_sweep_used() {
        parts.push("colr-grad");
    }
    if egui_backend::vertical_unify_used() {
        parts.push("vertical-hb-advance");
    }
    if egui_backend::vertical_ascii_ink_used() {
        parts.push("vertical-ascii-ink");
    }
    if egui_backend::vertical_ink_used() {
        parts.push("vertical-ink");
    }
    if wgpu_present::wgpu_composite_used() {
        parts.push("wgpu-composite");
    } else if wgpu_present::wgpu_present_used() {
        parts.push("wgpu-present");
    }
    parts.join(":")
}

pub fn gui_vertical_rl_fingerprint() -> &'static str {
    egui_backend::VERTICAL_RL_FINGERPRINT
}

pub fn gui_pump_reentrancy_fingerprint() -> &'static str {
    egui_backend::PUMP_REENTRANCY_FINGERPRINT
}

pub fn gui_wgpu_composite_fingerprint() -> &'static str {
    wgpu_present::wgpu_composite_fingerprint()
}

pub fn gui_wgpu_composite_used() -> bool {
    wgpu_present::wgpu_composite_used()
}

pub fn gui_wgpu_present_used() -> bool {
    wgpu_present::wgpu_present_used()
}

pub fn gui_emoji_engine() -> &'static str {
    emoji::emoji_engine()
}

pub fn gui_doctor_engine_tag() -> &'static str {
    emoji::doctor_engine_tag()
}

#[cfg(feature = "skia-glyph")]
pub fn gui_font_system_fingerprint() -> &'static str {
    skia_layout::font_system_fingerprint()
}

#[cfg(not(feature = "skia-glyph"))]
pub fn gui_font_system_fingerprint() -> &'static str {
    ""
}

#[cfg(feature = "skia-glyph")]
pub fn gui_paragraph_paint_used() -> bool {
    skia_layout::paragraph_paint_used()
}

#[cfg(not(feature = "skia-glyph"))]
pub fn gui_paragraph_paint_used() -> bool {
    false
}

pub fn gui_vertical_rl_layout_used() -> bool {
    egui_backend::vertical_rl_layout_used()
}

pub fn probe_wgpu_composite_readback_chroma(pixels: &[u32], width: u32, height: u32) -> bool {
    wgpu_present::probe_composite_readback_chroma(pixels, width, height)
}

pub fn gui_doctor_coverage_fingerprint() -> &'static str {
    egui_backend::DOCTOR_WGPU_FINGERPRINT
}

pub fn gui_doctor_present_mode() -> &'static str {
    wgpu_present::doctor_present_mode()
}

pub fn gui_emoji_atlas_hit() -> bool {
    emoji::atlas_hit()
}

pub fn gui_paint_shaped() -> bool {
    shape::paint_shaped()
}

pub fn gui_ot_features_fingerprint() -> &'static str {
    shape::ot_features_fingerprint()
}

pub fn gui_ot_calt_used() -> bool {
    shape::ot_calt_used()
}

pub fn gui_ot_subst_proven() -> bool {
    shape::ot_subst_proven()
}

pub fn gui_prove_ot_substitution() -> bool {
    shape::prove_ot_substitution()
}

pub fn gui_prove_ot_script_subset() -> bool {
    shape::prove_ot_script_subset()
}

pub fn gui_ot_script_subst_proven() -> bool {
    shape::ot_script_subst_proven()
}

pub fn gui_prove_ot_feature_matrix() -> bool {
    shape::prove_ot_feature_matrix()
}

pub fn gui_ot_feature_matrix_proven() -> bool {
    shape::ot_feature_matrix_proven()
}

pub fn gui_prove_ot_paint_matrix() -> bool {
    shape::prove_ot_paint_matrix()
}

pub fn gui_ot_paint_matrix_proven() -> bool {
    shape::ot_paint_matrix_proven()
}

pub fn gui_prove_ot_default_paint() -> bool {
    shape::prove_ot_default_paint()
}

pub fn gui_ot_default_paint_proven() -> bool {
    shape::ot_default_paint_proven()
}

pub fn gui_set_default_paint_ot_hook(enabled: bool) {
    shape::set_default_paint_ot_hook(enabled);
}

pub fn gui_set_default_paint_ot_fontmgr(enabled: bool) {
    shape::set_default_paint_ot_fontmgr(enabled);
}

pub fn gui_default_paint_fontmgr_used() -> bool {
    shape::default_paint_fontmgr_used()
}

pub fn gui_dylib_ot_vf_parity_fingerprint() -> &'static str {
    shape::dylib_ot_vf_parity_fingerprint()
}

pub fn gui_vf_wght_fingerprint() -> &'static str {
    shape::vf_wght_fingerprint()
}

pub fn gui_vf_multi_axis_fingerprint() -> &'static str {
    shape::vf_multi_axis_fingerprint()
}

pub fn gui_prove_vf_multi_axis() -> bool {
    shape::prove_vf_multi_axis()
}

pub fn gui_vf_multi_axis_used() -> bool {
    shape::vf_multi_axis_used()
}

pub fn gui_vf_axes_ge3_fingerprint() -> &'static str {
    shape::vf_axes_ge3_fingerprint()
}

pub fn gui_prove_vf_axes_ge3() -> bool {
    shape::prove_vf_axes_ge3()
}

pub fn gui_vf_axes_ge3_used() -> bool {
    shape::vf_axes_ge3_used()
}

pub fn gui_vf_paint_variations_fingerprint() -> &'static str {
    shape::vf_paint_variations_fingerprint()
}

pub fn gui_prove_vf_paint_variations() -> bool {
    shape::prove_vf_paint_variations()
}

pub fn gui_vf_paint_variations_used() -> bool {
    shape::vf_paint_variations_used()
}

pub fn gui_set_paint_vf_wght(wght: f32) {
    shape::set_paint_vf_wght(wght);
}

pub fn gui_vf_wght_used() -> bool {
    shape::vf_wght_used()
}

pub fn gui_prove_vf_wght() -> bool {
    shape::prove_vf_wght()
}

pub fn gui_last_paint_glyph_count() -> u32 {
    shape::last_paint_glyph_count()
}

/// Cumulative egui frames run for a window (0 if missing).
pub fn gui_egui_frames(win_id: &str) -> u32 {
    WINDOWS
        .lock()
        .get(win_id)
        .map(|w| w.egui.frames)
        .unwrap_or(0)
}

pub fn gui_shaped() -> bool {
    shape::shaped()
}

pub fn gui_fallback_hits() -> u32 {
    shape::fallback_hits()
}

pub fn gui_glyph_coverage_percent() -> u32 {
    cjk_font::glyph_coverage_percent()
}

pub fn gui_cjk_subset_char_count() -> u32 {
    cjk_font::ci_subset_unique_char_count()
}

/// Real capability probe: preedit draw path produces highlight pixels (not fingerprint equality).
pub fn gui_ime_preedit_supported() -> bool {
    probe_ime_preedit_highlight_capability() && shape::probe_shape_apis_ok()
}

pub fn gui_ime_commit_supported() -> bool {
    true
}

pub fn gui_caret_supported() -> bool {
    gui_caret_metric_px() > 0
}

/// Caret height derived from font/layout metrics (not a hardcoded `10`).
pub fn gui_caret_metric_px() -> u32 {
    shape::caret_metric_from_font()
}

pub fn ensure_loaded() -> bool {
    true
}

pub fn gui_visible_requested() -> bool {
    std::env::var("SYX_GUI_VISIBLE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn gui_leak_loop_requested() -> bool {
    std::env::var("SYX_GUI_LEAK_LOOP")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

pub fn reset_gui_state() {
    WINDOWS.lock().clear();
    egui_backend::reset_doctor_probes();
    shape::reset_shape_stats();
    emoji::reset_emoji_stats();
    #[cfg(feature = "skia-glyph")]
    {
        skia_glyph::reset_skia_glyph_stats();
        skia_layout::reset_skia_layout_stats();
    }
    wgpu_present::reset_used_probes();
    LIVE.with(|cell| {
        let mut slot = cell.borrow_mut();
        if gui_leak_loop_requested() {
            if let Some(ptr) = *slot {
                // SAFETY: pointer from Box::into_raw on this thread.
                unsafe {
                    (*ptr).os_windows.clear();
                }
            }
            return;
        }
        if let Some(ptr) = slot.take() {
            // SAFETY: reclaim Box while still in normal execution (not TLS teardown).
            unsafe {
                drop(Box::from_raw(ptr));
            }
        }
    });
}

/// Read-clear: returns whether the button was clicked since last take.
pub fn button_take_clicked(win_id: &str, btn_id: &str) -> bool {
    WINDOWS
        .lock()
        .get_mut(win_id)
        .and_then(|w| {
            w.widgets
                .iter_mut()
                .find(|x| {
                    x.kind == WidgetKind::Button && (x.id == btn_id || x.text == btn_id)
                })
                .map(|btn| {
                    let was = btn.clicked;
                    btn.clicked = false;
                    was
                })
        })
        .unwrap_or(false)
}

pub fn visible_window_count() -> usize {
    let mapped = WINDOWS.lock().len();
    let os = with_live_mut(|gui| gui.os_windows.len()).unwrap_or(0);
    mapped.max(os)
}

fn pop_str(stack: &mut Vec<Value>) -> String {
    stack.pop().unwrap_or(Value::Str(String::new())).as_str()
}

fn widget_matches(widget: &Widget, widget_id: &str) -> bool {
    widget.id == widget_id
        || widget.text == widget_id
        || widget.id == format!("tf-{widget_id}")
        || widget.id == format!("btn-{widget_id}")
        || widget.id == format!("lbl-{widget_id}")
}

fn focusable_widget_ids(w: &GuiWindow) -> Vec<String> {
    w.widgets
        .iter()
        .filter(|x| x.kind == WidgetKind::TextField || x.kind == WidgetKind::Button)
        .map(|x| x.id.clone())
        .collect()
}

fn cycle_focus(win_id: &str, reverse: bool) -> bool {
    let mut guard = WINDOWS.lock();
    let Some(w) = guard.get_mut(win_id) else {
        return false;
    };
    let ids = focusable_widget_ids(w);
    if ids.is_empty() {
        return false;
    }
    let next = match w.focus_widget.as_ref() {
        None => {
            if reverse {
                ids.len() - 1
            } else {
                0
            }
        }
        Some(cur) => match ids.iter().position(|id| id == cur) {
            None => 0,
            Some(i) => {
                if reverse {
                    if i == 0 {
                        ids.len() - 1
                    } else {
                        i - 1
                    }
                } else if i + 1 >= ids.len() {
                    0
                } else {
                    i + 1
                }
            }
        },
    };
    w.focus_widget = Some(ids[next].clone());
    true
}


/// Test hook: widget origin x for layout gates.
pub fn widget_origin_x(win_id: &str, widget_id: &str) -> u32 {
    WINDOWS
        .lock()
        .get(win_id)
        .and_then(|w| {
            w.widgets
                .iter()
                .find(|x| widget_matches(x, widget_id))
                .map(|x| x.x)
        })
        .unwrap_or(0)
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

/// Ink color for bitmap glyphs (non-background so visible gates can sample).
const GLYPH_INK: u32 = 0xFFEEEEEE;
const PREEDIT_HL_BG: u32 = 0xFFDDEEFF;
const PREEDIT_HL_INK: u32 = 0xFF111122;
const PREEDIT_PLAIN_INK: u32 = 0xFFEEEECC;
const PREEDIT_UNDERLINE: u32 = 0xFFEEEECC;
#[allow(dead_code)]
const CARET_INK: u32 = 0xFFFFFFFF;
#[allow(dead_code)]
const BUTTON_BG: u32 = 0xFF3366AA;
const FIELD_BG: u32 = 0xFF222222;
const ASCII_ADVANCE: u32 = 6;

fn text_char_len(text: &str) -> usize {
    text.chars().count()
}

#[allow(dead_code)]
fn char_byte_index(text: &str, char_idx: usize) -> usize {
    text.char_indices()
        .nth(char_idx)
        .map(|(i, _)| i)
        .unwrap_or_else(|| text.len())
}

fn char_advance(ch: char) -> u32 {
    shape::shape_rasterize_char(ch).advance
}

fn text_advance_width(text: &str, char_count: usize) -> u32 {
    shape::measure_width(text, char_count)
}

#[allow(dead_code)]
fn insert_char_at_caret(text: &mut String, caret_chars: usize, ch: char) {
    let at = char_byte_index(text, caret_chars);
    text.insert(at, ch);
}

#[allow(dead_code)]
fn insert_str_at_caret(text: &mut String, caret_chars: usize, s: &str) {
    let at = char_byte_index(text, caret_chars);
    text.insert_str(at, s);
}

#[allow(dead_code)]
fn remove_char_before(text: &mut String, caret_chars: usize) {
    if caret_chars == 0 {
        return;
    }
    let end = char_byte_index(text, caret_chars);
    let start = char_byte_index(text, caret_chars - 1);
    text.replace_range(start..end, "");
}

#[allow(dead_code)]
fn remove_char_at(text: &mut String, caret_chars: usize) {
    let start = char_byte_index(text, caret_chars);
    let end = char_byte_index(text, caret_chars + 1);
    if start < end {
        text.replace_range(start..end, "");
    }
}

/// Tiny 5×7 ASCII font (printable 0x20..=0x7E); bit rows MSB-left.
fn glyph5x7(c: u8) -> [u8; 7] {
    if (0x20..=0x7E).contains(&c) {
        ascii_font::GLYPH5X7_TABLE[(c - 0x20) as usize]
    } else {
        [0x1F, 0x11, 0x11, 0x11, 0x11, 0x11, 0x1F]
    }
}

fn blit_glyph5x7(buf: &mut [u32], stride: u32, x0: u32, y0: u32, g: [u8; 7], ink: u32) {
    for (row, bits) in g.iter().enumerate() {
        for col in 0..5u32 {
            if bits & (1 << (4 - col)) != 0 {
                let x = x0 + col;
                let y = y0 + row as u32;
                if x < stride && ((y as usize) * (stride as usize) + (x as usize)) < buf.len() {
                    buf[(y as usize) * stride as usize + x as usize] = ink;
                }
            }
        }
    }
}

fn blit_fontdue(
    buf: &mut [u32],
    stride: u32,
    x0: u32,
    y_baseline: u32,
    metrics: fontdue::Metrics,
    bitmap: &[u8],
    ink: u32,
) {
    let ox = x0 as i32 + metrics.xmin;
    let oy = y_baseline as i32 + metrics.ymin;
    for y in 0..metrics.height {
        for x in 0..metrics.width {
            let alpha = bitmap[y * metrics.width + x];
            if alpha > 48 {
                let px = ox + x as i32;
                let py = oy + y as i32;
                if px >= 0 && py >= 0 {
                    let xu = px as u32;
                    let yu = py as u32;
                    if xu < stride && (yu as usize * stride as usize + xu as usize) < buf.len() {
                        buf[yu as usize * stride as usize + xu as usize] = ink;
                    }
                }
            }
        }
    }
}

fn blit_hline(buf: &mut [u32], _stride: u32, width: u32, height: u32, x0: u32, x1: u32, y: u32, ink: u32) {
    if y >= height {
        return;
    }
    for x in x0..=x1.min(width.saturating_sub(1)) {
        let idx = (y * width + x) as usize;
        if idx < buf.len() {
            buf[idx] = ink;
        }
    }
}

fn fill_rect(buf: &mut [u32], width: u32, x0: u32, y0: u32, w: u32, h: u32, fill: u32) {
    for dy in 0..h {
        let y = y0 + dy;
        for dx in 0..w {
            let x = x0 + dx;
            if x < width {
                let idx = (y * width + x) as usize;
                if idx < buf.len() {
                    buf[idx] = fill;
                }
            }
        }
    }
}

fn ime_preedit_highlight_range(preedit: &str, highlight: Option<(usize, usize)>) -> (usize, usize) {
    match highlight {
        Some((start, end)) if start < end && end <= preedit.len() => (start, end),
        _ => (0, preedit.len()),
    }
}

fn blit_preedit_char(buf: &mut [u32], stride: u32, x0: u32, y0: u32, ch: char, ink: u32) -> u32 {
    if emoji::is_color_emoji(ch) {
        return emoji::blit_color_emoji(buf, stride, x0, y0, ch);
    }
    let g = shape::shape_rasterize_char(ch);
    let y_base = y0 + 11;
    #[cfg(feature = "skia-glyph")]
    if let Some(ref bmp) = g.skia {
        if !g.is_box {
            skia_glyph::blit_alpha8(buf, stride, x0, y_base, bmp, ink);
            return g.advance;
        }
    }
    #[cfg(not(feature = "skia-glyph"))]
    if !g.is_box {
        blit_fontdue(buf, stride, x0, y_base, g.metrics, &g.bitmap, ink);
        return g.advance;
    }
    // Missing glyph: skip 5×7 — shaped box metrics only.
    g.advance.max(ASCII_ADVANCE)
}

#[allow(dead_code)]
fn blit_text(buf: &mut [u32], stride: u32, x0: u32, y0: u32, text: &str, ink: u32) {
    // Phase 117: hard-ban ASCII 5x7 bitmap path — shaped/fontdue/skia only (IME uses blit_preedit_char).
    let glyphs = shape::shape_rasterize_text(text, 48);
    let mut cx = x0;
    for g in glyphs {
        let y_base = y0 + 11;
        if g.is_box {
            cx += g.advance.max(ASCII_ADVANCE);
            continue;
        }
        #[cfg(feature = "skia-glyph")]
        if let Some(ref bmp) = g.skia {
            skia_glyph::blit_alpha8(buf, stride, cx, y_base, bmp, ink);
            cx += g.advance;
            continue;
        }
        #[cfg(not(feature = "skia-glyph"))]
        {
            blit_fontdue(buf, stride, cx, y_base, g.metrics, &g.bitmap, ink);
            cx += g.advance;
            continue;
        }
        // Missing bitmap: advance only — no legacy ASCII table fallback.
        cx += g.advance.max(ASCII_ADVANCE);
    }
}

/// Scratch-buffer probe: preedit highlight draw path paints non-background pixels.
pub fn probe_ime_preedit_highlight_capability() -> bool {
    const W: u32 = 64;
    const H: u32 = 24;
    let mut buf = vec![FIELD_BG; (W * H) as usize];
    let preedit = "你";
    let (hl_start, hl_end) = (0usize, preedit.len());
    let mut cx = 2u32;
    let pre_y = 2u32;
    let mut byte_pos = 0usize;
    for ch in preedit.chars() {
        let ch_len = ch.len_utf8();
        let in_hl = byte_pos < hl_end && byte_pos + ch_len > hl_start;
        let adv = char_advance(ch);
        if in_hl {
            fill_rect(&mut buf, W, cx, pre_y, adv, gui_caret_metric_px().min(12), PREEDIT_HL_BG);
        }
        let ink = if in_hl {
            PREEDIT_HL_INK
        } else {
            PREEDIT_PLAIN_INK
        };
        let _ = blit_preedit_char(&mut buf, W, cx, pre_y, ch, ink);
        cx = cx.saturating_add(adv);
        byte_pos += ch_len;
    }
    buf.iter().any(|&px| px == PREEDIT_HL_BG || px == PREEDIT_HL_INK)
}

#[allow(dead_code)]
fn blit_caret(buf: &mut [u32], _stride: u32, width: u32, height: u32, x: u32, y0: u32, y1: u32) {
    if x >= width {
        return;
    }
    for y in y0..=y1 {
        if y >= height {
            break;
        }
        let idx = (y * width + x) as usize;
        if idx < buf.len() {
            buf[idx] = CARET_INK;
        }
    }
}

fn composite_window(w: &mut GuiWindow) {
    // Phase 109: Label/Button/TextField via egui; HB glyphs + emoji feed paint path.
    let bg = w.pixels.first().copied().unwrap_or(0xFF808080);
    egui_backend::run_egui_frame(
        &mut w.egui,
        &mut w.widgets,
        &mut w.focus_widget,
        &mut w.pixels,
        w.width,
        w.height,
        bg,
    );
    // IME preedit overlay (invert highlight) on focused TextField — P106/P107 carry.
    let focus_id = w.focus_widget.clone();
    if !w.ime_preedit.is_empty() {
        if let Some(widget) = w.widgets.iter().find(|x| {
            x.kind == WidgetKind::TextField && focus_id.as_ref().is_some_and(|id| id == &x.id)
        }) {
            let caret_chars = widget.caret.min(text_char_len(&widget.text));
            let mut cx = widget.x + 1 + text_advance_width(&widget.text, caret_chars);
            let pre_y = widget.y + 2;
            let caret_h = gui_caret_metric_px().min(12);
            let ul_y = widget.y + caret_h.saturating_add(1).min(11);
            let pre_start = cx;
            let (hl_start, hl_end) =
                ime_preedit_highlight_range(&w.ime_preedit, w.ime_preedit_highlight);
            let mut byte_pos = 0usize;
            for ch in w.ime_preedit.chars().take(24) {
                let ch_len = ch.len_utf8();
                let in_hl = byte_pos < hl_end && byte_pos + ch_len > hl_start;
                let adv = char_advance(ch);
                if in_hl {
                    fill_rect(
                        &mut w.pixels,
                        w.width,
                        cx,
                        pre_y,
                        adv,
                        caret_h,
                        PREEDIT_HL_BG,
                    );
                }
                let ink = if in_hl {
                    PREEDIT_HL_INK
                } else {
                    PREEDIT_PLAIN_INK
                };
                let _ = blit_preedit_char(&mut w.pixels, w.width, cx, pre_y, ch, ink);
                cx = cx.saturating_add(adv);
                byte_pos += ch_len;
            }
            blit_hline(
                &mut w.pixels,
                w.width,
                w.width,
                w.height,
                pre_start,
                cx.saturating_sub(1),
                ul_y,
                PREEDIT_UNDERLINE,
            );
        }
    }
}

fn present_gui_window(win_id: &str) {
    let (pixels, width, height, primitives, textures_delta, ppp) = {
        let guard = WINDOWS.lock();
        let Some(w) = guard.get(win_id) else {
            return;
        };
        (
            w.pixels.clone(),
            w.width,
            w.height,
            w.egui.last_primitives.clone(),
            w.egui.last_textures_delta.clone(),
            w.egui.last_pixels_per_point,
        )
    };
    let _ = with_live_mut(|gui| {
        if let Some(os) = gui.os_windows.get_mut(win_id) {
            present_os_window(
                os,
                &pixels,
                width,
                height,
                &primitives,
                &textures_delta,
                ppp,
            );
        }
    });
}

fn present_os_window(
    os: &mut OsGuiWindow,
    pixels: &[u32],
    width: u32,
    height: u32,
    primitives: &[egui::epaint::ClippedPrimitive],
    textures_delta: &egui::TexturesDelta,
    pixels_per_point: f32,
) {
    if wgpu_present::softbuffer_fallback_requested() {
        wgpu_present::present_softbuffer(&os.window, pixels, width, height);
        return;
    }
    let size = os.window.inner_size();
    if os.wgpu.is_none() {
        os.wgpu = wgpu_present::WgpuWindowPresent::new(&os.window);
    }
    if let Some(ref mut wgpu) = os.wgpu {
        wgpu.resize(size.width.max(1), size.height.max(1));
        if wgpu.present_egui(
            textures_delta,
            primitives,
            pixels_per_point,
            Some((pixels, width, height)),
        ) {
            os.window.request_redraw();
            return;
        }
    }
    // Default is wgpu; without SYX_GUI_SOFTBUFFER=1 we do not silently softbuffer-present.
    os.window.request_redraw();
}

pub fn softbuffer_probe_ok() -> bool {
    EventLoop::new().is_ok()
}

fn ensure_live_gui() -> bool {
    LIVE.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_some() {
            return true;
        }
        match EventLoop::new() {
            Ok(event_loop) => {
                let gui = Box::new(LiveGui {
                    event_loop,
                    os_windows: HashMap::new(),
                });
                *slot = Some(Box::into_raw(gui));
                true
            }
            Err(_) => false,
        }
    })
}

fn gui_hold_requested() -> bool {
    std::env::var("SYX_GUI_HOLD")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
}

fn logical_point_from_cursor(win: &GuiWindow, x: f64, y: f64, os_w: u32, os_h: u32) -> (u32, u32) {
    let lx = (x / os_w.max(1) as f64 * win.width as f64) as u32;
    let ly = (y / os_h.max(1) as f64 * win.height as f64) as u32;
    (lx.min(win.width.saturating_sub(1)), ly.min(win.height.saturating_sub(1)))
}

fn hit_test_widgets(win_id: &str, lx: u32, ly: u32) {
    if let Some(w) = WINDOWS.lock().get_mut(win_id) {
        let pos = egui::pos2(lx as f32, ly as f32);
        w.egui.queue_pointer(pos, Some(true));
        w.egui.queue_pointer(pos, Some(false));
        composite_window(w);
    }
}

fn queue_click_handlers(win_id: &str) {
    let mut guard = WINDOWS.lock();
    let Some(w) = guard.get_mut(win_id) else {
        return;
    };
    for widget in &mut w.widgets {
        if widget.kind != WidgetKind::Button || !widget.clicked {
            continue;
        }
        if let Some(handler) = w.click_handlers.get(&widget.id).cloned() {
            w.pending_handlers.push(handler);
        }
        widget.clicked = false;
    }
}

/// Drain handler names queued during pump (invoke happens in syx-vm after pump).
pub fn drain_pending_handlers(win_id: &str) -> Vec<String> {
    WINDOWS
        .lock()
        .get_mut(win_id)
        .map(|w| std::mem::take(&mut w.pending_handlers))
        .unwrap_or_default()
}

fn refresh_window_pixels(win_id: &str) {
    if PUMP_DEPTH.get() > 0 {
        mark_os_window_dirty(win_id);
        return;
    }
    refresh_window_pixels_now(win_id);
}

fn refresh_window_pixels_now(win_id: &str) {
    present_gui_window(win_id);
}

fn apply_text_to_focused_field(win_id: &str, text: &str) -> bool {
    if text.is_empty() {
        return false;
    }
    let mut guard = WINDOWS.lock();
    let Some(w) = guard.get_mut(win_id) else {
        return false;
    };
    let focus = match w.focus_widget.clone() {
        Some(id) => id,
        None => focusable_widget_ids(w).into_iter().next().unwrap_or_default(),
    };
    if focus.is_empty() {
        return false;
    }
    let Some(widget) = w.widgets.iter_mut().find(|x| x.id == focus) else {
        return false;
    };
    if widget.kind != WidgetKind::TextField {
        return false;
    }
    if !widget.edited {
        widget.text.clear();
        widget.edited = true;
        widget.caret = 0;
    }
    w.focus_widget = Some(focus);
    // egui applies request_focus on the following frame — focus then inject.
    composite_window(w);
    egui_backend::enqueue_key_inject(&mut w.egui, text);
    composite_window(w);
    drop(guard);
    refresh_window_pixels(win_id);
    true
}

fn apply_key_to_focused_field(win_id: &str, key: &str) -> bool {
    if key == "Tab" {
        let ok = cycle_focus(win_id, false);
        if ok {
            if let Some(w) = WINDOWS.lock().get_mut(win_id) {
                composite_window(w);
            }
            refresh_window_pixels(win_id);
        }
        return ok;
    }
    if key == "Shift-Tab" || key == "Shift+Tab" {
        let ok = cycle_focus(win_id, true);
        if ok {
            if let Some(w) = WINDOWS.lock().get_mut(win_id) {
                composite_window(w);
            }
            refresh_window_pixels(win_id);
        }
        return ok;
    }

    let mut guard = WINDOWS.lock();
    let Some(w) = guard.get_mut(win_id) else {
        return false;
    };
    let focus = match w.focus_widget.clone() {
        Some(id) => id,
        None => {
            if let Some(id) = focusable_widget_ids(w).into_iter().next() {
                id
            } else {
                return false;
            }
        }
    };
    let Some(widget) = w.widgets.iter_mut().find(|x| x.id == focus) else {
        return false;
    };
    if widget.kind != WidgetKind::TextField {
        return false;
    }
    let is_edit_key = !matches!(
        key,
        "Left" | "ArrowLeft" | "Right" | "ArrowRight" | "Backspace" | "\u{8}" | "Delete"
    );
    if is_edit_key && !widget.edited {
        widget.text.clear();
        widget.edited = true;
        widget.caret = 0;
    }
    w.focus_widget = Some(focus);
    // Focus frame then event frame so TextEdit receives egui events.
    composite_window(w);
    egui_backend::enqueue_key_inject(&mut w.egui, key);
    composite_window(w);
    drop(guard);
    refresh_window_pixels(win_id);
    true
}

fn apply_ime_commit_to_focused_field(win_id: &str, text: &str) -> bool {
    {
        let mut guard = WINDOWS.lock();
        if let Some(w) = guard.get_mut(win_id) {
            w.ime_preedit.clear();
            w.ime_preedit_highlight = None;
        }
    }
    apply_text_to_focused_field(win_id, text)
}

fn apply_ime_preedit_to_window(win_id: &str, preedit: &str, highlight: Option<(usize, usize)>) {
    {
        let mut guard = WINDOWS.lock();
        let Some(w) = guard.get_mut(win_id) else {
            return;
        };
        w.ime_preedit = preedit.to_string();
        w.ime_preedit_highlight = highlight;
        composite_window(w);
    }
    if gui_visible_requested() {
        refresh_window_pixels(win_id);
    }
}

/// Test hook: inject IME preedit without OS IME (headless gate).
pub fn apply_ime_preedit_inject(win_id: &str, preedit: &str) -> bool {
    let has_window = WINDOWS.lock().contains_key(win_id);
    if !has_window {
        return false;
    }
    apply_ime_preedit_to_window(win_id, preedit, Some((0, preedit.len())));
    true
}

fn handle_keyboard_event(win_id: &str, event: &KeyEvent) {
    if event.state != ElementState::Pressed {
        return;
    }
    let key = match &event.logical_key {
        Key::Character(s) => s.to_string(),
        Key::Named(NamedKey::Backspace) => "Backspace".into(),
        Key::Named(NamedKey::Enter) => "Enter".into(),
        Key::Named(NamedKey::Tab) => "Tab".into(),
        Key::Named(NamedKey::ArrowLeft) => "Left".into(),
        Key::Named(NamedKey::ArrowRight) => "Right".into(),
        Key::Named(NamedKey::Delete) => "Delete".into(),
        _ => return,
    };
    let _ = apply_key_to_focused_field(win_id, &key);
}

fn handle_window_pump_event(win_id: &str, event: &WindowEvent, os_w: u32, os_h: u32) {
    // Real egui-winit event bridge (not a stub import).
    let _ = with_live_mut(|gui| {
        if let Some(os) = gui.os_windows.get(win_id) {
            if let Some(w) = WINDOWS.lock().get_mut(win_id) {
                let _resp = w.egui.on_window_event(&os.window, event);
            }
        }
    });
    match event {
        WindowEvent::CursorMoved { position, .. } => {
            if let Some(w) = WINDOWS.lock().get_mut(win_id) {
                w.cursor_x = position.x;
                w.cursor_y = position.y;
            }
        }
        WindowEvent::MouseInput {
            state: ElementState::Pressed,
            button: MouseButton::Left,
            ..
        } => {
            let (cx, cy) = WINDOWS
                .lock()
                .get(win_id)
                .map(|w| (w.cursor_x, w.cursor_y))
                .unwrap_or((0.0, 0.0));
            let (lx, ly) = {
                let guard = WINDOWS.lock();
                let w = guard.get(win_id).unwrap();
                logical_point_from_cursor(w, cx, cy, os_w, os_h)
            };
            hit_test_widgets(win_id, lx, ly);
        }
        WindowEvent::KeyboardInput { event, .. } => {
            handle_keyboard_event(win_id, event);
        }
        WindowEvent::Ime(ime) => match ime {
            Ime::Preedit(text, highlight) => {
                apply_ime_preedit_to_window(win_id, text, *highlight);
            }
            Ime::Commit(text) => {
                if !text.is_empty() {
                    let _ = apply_ime_commit_to_focused_field(win_id, &text);
                }
            }
            _ => {}
        },
        _ => {}
    }
}

struct PumpGuard;

impl Drop for PumpGuard {
    fn drop(&mut self) {
        pump_leave();
    }
}

fn pump_live(timeout_ms: u64) {
    // Hit-test only — leave `clicked` for run_loop_once / event_pump to drain
    // into `click:{handler}` (do not discard here).
    let _ = with_live_mut(|gui| {
        pump_enter();
        let _guard = PumpGuard;
        let _ = gui.event_loop.pump_events(
            Some(Duration::from_millis(timeout_ms)),
            |event, _| {
                if let Event::WindowEvent { window_id, event } = &event {
                    if let WindowEvent::CloseRequested = event {
                        gui.os_windows.retain(|_, w| w.window.id() != *window_id);
                        return;
                    }
                    let mapped = gui
                        .os_windows
                        .iter()
                        .find(|(_, w)| w.window.id() == *window_id)
                        .map(|(id, w)| {
                            let s = w.window.inner_size();
                            (id.clone(), s.width, s.height)
                        });
                    if let Some((win_id, os_w, os_h)) = mapped {
                        handle_window_pump_event(&win_id, event, os_w, os_h);
                    }
                }
            },
        );
    });
    flush_dirty_os_windows();
}

fn pump_until_close_or_timeout(timeout: Duration) {
    use std::time::Instant;
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        let empty = with_live_mut(|gui| {
            pump_enter();
            let _guard = PumpGuard;
            let _ = gui.event_loop.pump_events(
                Some(Duration::from_millis(50)),
                |event, _| {
                    if let Event::WindowEvent { window_id, event } = &event {
                        if let WindowEvent::CloseRequested = event {
                            gui.os_windows.retain(|_, w| w.window.id() != *window_id);
                            return;
                        }
                        let mapped = gui
                            .os_windows
                            .iter()
                            .find(|(_, w)| w.window.id() == *window_id)
                            .map(|(id, w)| {
                                let s = w.window.inner_size();
                                (id.clone(), s.width, s.height)
                            });
                        if let Some((win_id, os_w, os_h)) = mapped {
                            handle_window_pump_event(&win_id, event, os_w, os_h);
                        }
                    }
                },
            );
            gui.os_windows.is_empty()
        })
        .unwrap_or(true);
        flush_dirty_os_windows();
        if empty {
            break;
        }
    }
}

pub fn host_gui_winit_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let title = pop_str(stack);
    let id = window_id(&title);
    let mut pixels = vec![0u32; 64 * 64];
    pixels.fill(0xFF808080);

    if gui_visible_requested() && ensure_live_gui() {
        let _ = with_live_mut(|gui| {
            let built = WindowBuilder::new()
                .with_title(title.clone())
                .with_inner_size(LogicalSize::new(480.0, 320.0))
                .with_visible(true)
                .build(&gui.event_loop);
            if let Ok(win) = built {
                win.focus_window();
                let wgpu = if wgpu_present::softbuffer_fallback_requested() {
                    None
                } else {
                    wgpu_present::WgpuWindowPresent::new(&win)
                };
                if wgpu_present::softbuffer_fallback_requested() {
                    wgpu_present::present_softbuffer(&win, &pixels, 64, 64);
                }
                gui.os_windows.insert(
                    id.clone(),
                    OsGuiWindow {
                        window: win,
                        wgpu,
                    },
                );
            }
        });
        // Brief pump so the window maps on macOS AppKit.
        pump_live(50);
    }

    WINDOWS.lock().insert(
        id.clone(),
        GuiWindow {
            title,
            width: 64,
            height: 64,
            pixels,
            last_event: "init".into(),
            widgets: Vec::new(),
            next_widget_y: 4,
            layout: None,
            click_handlers: HashMap::new(),
            pending_handlers: Vec::new(),
            focus_widget: None,
            cursor_x: 0.0,
            cursor_y: 0.0,
            ime_preedit: String::new(),
            ime_preedit_highlight: None,
            egui: egui_backend::EguiWindowState::default(),
        },
    );
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_gui_winit_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    let color = pop_str(stack);
    let win_id = pop_str(stack);
    let px = color_to_pixel(&color);
    let ok = {
        let mut guard = WINDOWS.lock();
        if let Some(w) = guard.get_mut(&win_id) {
            w.pixels.fill(px);
            w.last_event = format!("draw:{color}");
            composite_window(w);
            true
        } else {
            false
        }
    };
    if ok {
        present_gui_window(&win_id);
        if gui_visible_requested() {
            pump_live(16);
        }
        stack.push(Value::Bool(true));
        return Ok(());
    }
    stack.push(Value::Bool(false));
    Ok(())
}

fn push_widget(
    stack: &mut Vec<Value>,
    kind: WidgetKind,
    id_prefix: &str,
) -> SyxResult<()> {
    let text = pop_str(stack);
    let win_id = pop_str(stack);
    let id = format!("{id_prefix}-{text}");
    let mut ok = false;
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        let (x, y) = if let Some(ref mut layout) = w.layout {
            match layout.axis {
                LayoutAxis::Vertical => {
                    let y = layout.next_y;
                    layout.next_y = y
                        .saturating_add(12)
                        .saturating_add(layout.spacing);
                    (2, y)
                }
                LayoutAxis::Horizontal => {
                    let x = layout.next_x;
                    let y = match layout.v_align {
                        VAlign::Top => layout.row_y,
                        VAlign::Center => {
                            layout.row_y
                                + (layout.row_height.saturating_sub(12)) / 2
                        }
                    };
                    layout.row_height = layout.row_height.max(12);
                    layout.next_x = x.saturating_add(60).saturating_add(layout.spacing);
                    (x, y)
                }
            }
        } else {
            let y = w.next_widget_y;
            w.next_widget_y = y.saturating_add(14);
            (2, y)
        };
        w.widgets.push(Widget {
            id: id.clone(),
            kind,
            text: text.clone(),
            x,
            y,
            w: 60,
            h: 12,
            clicked: false,
            edited: false,
            caret: 0,
            writing_mode: WritingMode::Horizontal,
        });
        if kind == WidgetKind::TextField || kind == WidgetKind::Button {
            w.focus_widget = Some(id.clone());
            if kind == WidgetKind::TextField {
                if let Some(tf) = w.widgets.iter_mut().find(|x| x.id == id) {
                    tf.caret = 0;
                }
            }
        }
        composite_window(w);
        ok = true;
    }
    if ok {
        present_gui_window(&win_id);
        if gui_visible_requested() {
            pump_live(16);
        }
        // Return human text so guidemo stdout shows "Hello SYX" not empty.
        stack.push(Value::Str(text));
        return Ok(());
    }
    stack.push(Value::Str(String::new()));
    Ok(())
}

pub fn host_gui_label_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    push_widget(stack, WidgetKind::Label, "lbl")
}

/// Set Label writing-mode: `horizontal` (default) or `vertical-rl`.
pub fn host_gui_label_set_writing_mode(stack: &mut Vec<Value>) -> SyxResult<()> {
    let mode = pop_str(stack);
    let widget_id = pop_str(stack);
    let win_id = pop_str(stack);
    let writing = if mode.eq_ignore_ascii_case("vertical-rl")
        || mode.eq_ignore_ascii_case("vertical_rl")
        || mode.eq_ignore_ascii_case("verticalRl")
    {
        WritingMode::VerticalRl
    } else {
        WritingMode::Horizontal
    };
    let ok = {
        let mut guard = WINDOWS.lock();
        if let Some(w) = guard.get_mut(&win_id) {
            if let Some(widget) = w
                .widgets
                .iter_mut()
                .find(|x| widget_matches(x, &widget_id) && x.kind == WidgetKind::Label)
            {
                widget.writing_mode = writing;
                if writing == WritingMode::VerticalRl {
                    widget.h = widget.h.max(48);
                    widget.w = widget.w.max(16);
                }
                composite_window(w);
                true
            } else {
                false
            }
        } else {
            false
        }
    };
    if ok {
        present_gui_window(&win_id);
        stack.push(Value::Str(egui_backend::VERTICAL_RL_FINGERPRINT.into()));
        return Ok(());
    }
    stack.push(Value::Str(String::new()));
    Ok(())
}

/// Test hook: set writing-mode without host stack.
pub fn label_set_writing_mode(win_id: &str, widget_id: &str, mode: &str) -> bool {
    let mut stack = vec![
        Value::Str(win_id.into()),
        Value::Str(widget_id.into()),
        Value::Str(mode.into()),
    ];
    host_gui_label_set_writing_mode(&mut stack).is_ok()
        && !stack.pop().map(|v| v.as_str()).unwrap_or_default().is_empty()
}

pub fn host_gui_button_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    push_widget(stack, WidgetKind::Button, "btn")
}

pub fn host_gui_text_field_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    push_widget(stack, WidgetKind::TextField, "tf")
}

pub fn host_gui_widget_set_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = pop_str(stack);
    let widget_id = pop_str(stack);
    let win_id = pop_str(stack);
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        if let Some(widget) = w
            .widgets
            .iter_mut()
            .find(|x| widget_matches(x, &widget_id))
        {
            widget.text = text;
            widget.edited = true;
            widget.caret = text_char_len(&widget.text);
            composite_window(w);
            present_gui_window(&win_id);
            stack.push(Value::Bool(true));
            return Ok(());
        }
    }
    stack.push(Value::Bool(false));
    Ok(())
}

pub fn host_gui_layout_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let axis = pop_str(stack);
    let win_id = pop_str(stack);
    let layout_axis = if axis.eq_ignore_ascii_case("HStack") {
        LayoutAxis::Horizontal
    } else {
        LayoutAxis::Vertical
    };
    let prefix = if layout_axis == LayoutAxis::Horizontal {
        "hstack"
    } else {
        "vstack"
    };
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        let id = format!("{prefix}-{win_id}");
        w.layout = Some(StackLayout {
            id: id.clone(),
            axis: layout_axis,
            next_x: 2,
            next_y: 4,
            row_y: 4,
            row_height: 12,
            spacing: 2,
            v_align: VAlign::Top,
        });
        w.next_widget_y = 4;
        stack.push(Value::Str(id));
        return Ok(());
    }
    stack.push(Value::Str(String::new()));
    Ok(())
}

pub fn host_gui_layout_configure(stack: &mut Vec<Value>) -> SyxResult<()> {
    let align = pop_str(stack);
    let spacing = stack
        .pop()
        .unwrap_or(Value::I32(2))
        .as_i64()
        .max(0) as u32;
    let win_id = pop_str(stack);
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        if let Some(ref mut layout) = w.layout {
            layout.spacing = spacing;
            layout.v_align = if align.eq_ignore_ascii_case("center") {
                VAlign::Center
            } else {
                VAlign::Top
            };
            let tag = format!("gui-hstack-ga-v1:spacing-align:{spacing}:{align}");
            stack.push(Value::Str(tag));
            return Ok(());
        }
    }
    stack.push(Value::Str(String::new()));
    Ok(())
}

pub fn host_gui_widget_get_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    let widget_id = pop_str(stack);
    let win_id = pop_str(stack);
    let text = WINDOWS
        .lock()
        .get(&win_id)
        .and_then(|w| {
            w.widgets
                .iter()
                .find(|x| widget_matches(x, &widget_id))
                .map(|x| x.text.clone())
        })
        .unwrap_or_default();
    stack.push(Value::Str(text));
    Ok(())
}

pub fn host_gui_key_inject(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = pop_str(stack);
    let win_id = pop_str(stack);
    stack.push(Value::Bool(apply_key_to_focused_field(&win_id, &key)));
    Ok(())
}

pub fn host_gui_on_click_register(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handler = pop_str(stack);
    let btn_id = pop_str(stack);
    let win_id = pop_str(stack);
    if win_id.is_empty() || btn_id.is_empty() || handler.is_empty() {
        stack.push(Value::Bool(false));
        return Ok(());
    }
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        let has_btn = w.widgets.iter().any(|x| {
            (x.id == btn_id || x.text == btn_id) && x.kind == WidgetKind::Button
        });
        if has_btn {
            let key = w
                .widgets
                .iter()
                .find(|x| (x.id == btn_id || x.text == btn_id) && x.kind == WidgetKind::Button)
                .map(|x| x.id.clone())
                .unwrap_or(btn_id);
            w.click_handlers.insert(key, handler);
            stack.push(Value::Bool(true));
            return Ok(());
        }
    }
    stack.push(Value::Bool(false));
    Ok(())
}

/// Test hook: inject a button click without OS pointer (headless gate).
pub fn host_gui_click_inject(stack: &mut Vec<Value>) -> SyxResult<()> {
    let btn_id = pop_str(stack);
    let win_id = pop_str(stack);
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        if let Some(widget) = w.widgets.iter_mut().find(|x| {
            (x.id == btn_id || x.text == btn_id) && x.kind == WidgetKind::Button
        }) {
            widget.clicked = true;
            w.focus_widget = Some(widget.id.clone());
            stack.push(Value::Bool(true));
            return Ok(());
        }
    }
    stack.push(Value::Bool(false));
    Ok(())
}

pub fn host_gui_button_take_clicked(stack: &mut Vec<Value>) -> SyxResult<()> {
    let btn_id = pop_str(stack);
    let win_id = pop_str(stack);
    stack.push(Value::Bool(button_take_clicked(&win_id, &btn_id)));
    Ok(())
}

pub fn host_gui_button_clicked(stack: &mut Vec<Value>) -> SyxResult<()> {
    let widget_id = pop_str(stack);
    let win_id = pop_str(stack);
    let clicked = WINDOWS
        .lock()
        .get(&win_id)
        .and_then(|w| {
            w.widgets
                .iter()
                .find(|x| x.id == widget_id || x.text == widget_id)
                .map(|x| x.clicked)
        })
        .unwrap_or(false);
    stack.push(Value::Bool(clicked));
    Ok(())
}

pub fn host_gui_pixel_read(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    let px = WINDOWS
        .lock()
        .get(&win_id)
        .and_then(|w| w.pixels.get((w.width * w.height / 2) as usize).copied())
        .unwrap_or(0);
    stack.push(Value::I32(px as i32));
    Ok(())
}

pub fn host_gui_run_loop_once(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    if gui_visible_requested() {
        pump_live(50);
    }
    queue_click_handlers(&win_id);
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        w.last_event = "tick".into();
        stack.push(Value::Str("tick".into()));
        return Ok(());
    }
    stack.push(Value::Str("no-window".into()));
    Ok(())
}

pub fn host_gui_winit_event_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win_id = pop_str(stack);
    if gui_visible_requested() {
        if gui_hold_requested() {
            pump_until_close_or_timeout(Duration::from_secs(600));
        } else {
            pump_live(100);
        }
    }
    queue_click_handlers(&win_id);
    if let Some(w) = WINDOWS.lock().get_mut(&win_id) {
        w.last_event = "pump".into();
        stack.push(Value::Str("pump".into()));
        return Ok(());
    }
    stack.push(Value::Str("no-window".into()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use parking_lot::Mutex;
    use syx_runtime_core::Value;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn reset_test_gui() {
        std::env::remove_var("SYX_GUI_VISIBLE");
        std::env::remove_var("SYX_GUI_HOLD");
        std::env::remove_var("SYX_GUI_LEAK_LOOP");
        std::env::remove_var("SYX_GUI_SOFTBUFFER");
        reset_gui_state();
        shape::reset_shape_stats();
        emoji::reset_emoji_stats();
        #[cfg(feature = "skia-glyph")]
        skia_glyph::reset_skia_glyph_stats();
    }

    #[test]
    fn syx_gui_backend_name() {
        assert!(super::gui_backend_name().contains("egui-wgpu"));
    }

    #[test]
    fn syx_gui_ascii_font_full_printable() {
        for c in 0x20u8..=0x7E {
            let g = super::glyph5x7(c);
            assert_eq!(g, super::ascii_font::GLYPH5X7_TABLE[(c - 0x20) as usize]);
        }
    }

    #[test]
    fn syx_gui_click_inject_drains_handler() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("unit".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("VStack".into()));
        host_gui_layout_create(&mut stack).unwrap();
        let layout = stack.pop().unwrap().as_str();
        assert!(layout.contains("vstack"), "{layout}");

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Go".into()));
        host_gui_button_create(&mut stack).unwrap();
        let btn = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(btn.clone()));
        stack.push(Value::Str("h1".into()));
        host_gui_on_click_register(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(btn.clone()));
        host_gui_click_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        host_gui_run_loop_once(&mut stack).unwrap();
        let _tick = stack.pop().unwrap().as_str();
        let pending = drain_pending_handlers(&win);
        assert!(pending.iter().any(|h| h == "h1"), "expected h1 queued, got {pending:?}");
        reset_test_gui();
    }

    #[test]
    fn syx_gui_button_take_clicked_read_clear() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("take".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("B".into()));
        host_gui_button_create(&mut stack).unwrap();
        let btn = stack.pop().unwrap().as_str();
        assert_eq!(btn, "B");
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(btn.clone()));
        host_gui_click_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool(), "click inject should find button");
        // widget id is btn-B; public take accepts that id
        assert!(button_take_clicked(&win, "btn-B"));
        assert!(!button_take_clicked(&win, "btn-B"));
        reset_test_gui();
    }

    #[test]
    fn syx_gui_tab_cycle_focus_two_fields() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("tab".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("f1".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let f1 = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Go".into()));
        host_gui_button_create(&mut stack).unwrap();
        let _btn = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("f2".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let f2 = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Shift-Tab".into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Shift-Tab".into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("a".into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Tab".into()));
        host_gui_key_inject(&mut stack).unwrap();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Tab".into()));
        host_gui_key_inject(&mut stack).unwrap();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("b".into()));
        host_gui_key_inject(&mut stack).unwrap();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(f1.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "a");

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(f2.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "b");
        reset_test_gui();
    }

    #[test]
    fn syx_gui_caret_left_backspace_mid_text() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("caret".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("x".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        for ch in ["a", "b", "c"] {
            stack.push(Value::Str(win.clone()));
            stack.push(Value::Str(ch.into()));
            host_gui_key_inject(&mut stack).unwrap();
            assert!(stack.pop().unwrap().as_bool());
        }
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Left".into()));
        host_gui_key_inject(&mut stack).unwrap();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Backspace".into()));
        host_gui_key_inject(&mut stack).unwrap();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "ac");
        reset_test_gui();
    }

    #[test]
    fn syx_gui_hstack_spacing_second_widget_x() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("hstack".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("HStack".into()));
        host_gui_layout_create(&mut stack).unwrap();
        let _layout = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::I32(16));
        stack.push(Value::Str("top".into()));
        host_gui_layout_configure(&mut stack).unwrap();
        let tag = stack.pop().unwrap().as_str();
        assert!(tag.contains("gui-hstack-ga-v1:spacing-align"), "{tag}");

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("A".into()));
        host_gui_label_create(&mut stack).unwrap();
        let a = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("B".into()));
        host_gui_label_create(&mut stack).unwrap();
        let b = stack.pop().unwrap().as_str();

        assert_eq!(super::widget_origin_x(&win, &a), 2);
        assert_eq!(super::widget_origin_x(&win, &b), 2 + 60 + 16);
        reset_test_gui();
    }

    fn caret_column_pixel(win_id: &str, field_id: &str) -> (u32, u32, u32) {
        let guard = WINDOWS.lock();
        let w = guard.get(win_id).expect("window");
        let widget = w
            .widgets
            .iter()
            .find(|x| widget_matches(x, field_id))
            .expect("field");
        let caret_chars = widget.caret.min(text_char_len(&widget.text));
        let cx = widget.x + 1 + text_advance_width(&widget.text, caret_chars);
        let cy = widget.y + 5;
        let px = w.pixels[(cy * w.width + cx) as usize];
        (cx, cy, px)
    }

    #[test]
    fn p105_gui_ttf_fingerprints() {
        assert!(super::gui_backend_name().contains("egui-wgpu"));
        assert_eq!(
            super::gui_font_cjk_subset_fingerprint(),
            "gui-ttf-expand-v1:cjk64-plus"
        );
        assert_eq!(
            super::gui_ime_preedit_fingerprint(),
            "gui-ime-highlight-v1:preedit-invert"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let src = include_str!("lib.rs");
        assert!(src.contains("gui-ttf-expand-v1:cjk64-plus"));
        assert!(src.contains("Ime::Preedit"));
        assert!(src.contains("Ime::Commit"));
        assert!(super::gui_glyph_coverage_percent() >= 100);
    }

    #[test]
    fn p105_gui_multi_char_cjk_get_text_exact() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p105-multi".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("field".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你我他".into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "你我他");
        reset_test_gui();
    }

    #[test]
    fn p105_gui_ime_preedit_pixels_non_background() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p105-preedit".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("ime".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        assert!(apply_ime_preedit_inject(&win, "你"));

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(
            stack.pop().unwrap().as_str(),
            "ime",
            "preedit must not commit text"
        );

        let mut saw_preedit = false;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            let widget = w
                .widgets
                .iter()
                .find(|x| widget_matches(x, &field))
                .unwrap();
            for dy in 0..12u32 {
                for dx in 0..40u32 {
                    let x = widget.x + 1 + dx;
                    let y = widget.y + dy;
                    let idx = (y * w.width + x) as usize;
                    if idx < w.pixels.len() {
                        let px = w.pixels[idx];
                        if px != FIELD_BG && px != 0xFF808080 && px != GLYPH_INK {
                            saw_preedit = true;
                        }
                        if px == PREEDIT_HL_BG || px == PREEDIT_HL_INK || px == PREEDIT_UNDERLINE {
                            saw_preedit = true;
                        }
                    }
                }
            }
        }
        assert!(saw_preedit, "preedit must draw non-background pixels");
        reset_test_gui();
    }

    #[test]
    fn p105_gui_doctor_glyph_coverage_ci_table() {
        assert!(cjk_font::CI_SUBSET.chars().count() >= 16);
        assert_eq!(super::gui_glyph_coverage_percent(), 100);
    }

    #[test]
    fn p104_gui_cjk_fingerprints() {
        assert_eq!(
            super::gui_font_cjk_subset_fingerprint(),
            "gui-ttf-expand-v1:cjk64-plus"
        );
    }

    #[test]
    fn p104_gui_unicode_key_inject_ni() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p104-ni".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("field".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你".into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "你");

        let mut saw_ink = false;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            let widget = w
                .widgets
                .iter()
                .find(|x| widget_matches(x, &field))
                .unwrap();
            let bg = 0xFF808080u32;
            for dy in 0..12u32 {
                for dx in 0..40u32 {
                    let x = widget.x + 1 + dx;
                    let y = widget.y + 2 + dy;
                    let idx = (y * w.width + x) as usize;
                    if idx < w.pixels.len() {
                        let px = w.pixels[idx];
                        if px != bg && px != FIELD_BG {
                            saw_ink = true;
                        }
                    }
                }
            }
        }
        assert!(saw_ink, "CJK TTF/egui path must produce non-background pixels for 你");
        reset_test_gui();
    }

    #[test]
    fn p104_gui_ime_commit_ni() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p104-ime".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("ime".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        assert!(apply_ime_commit_to_focused_field(&win, "你"));

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "你");
        reset_test_gui();
    }

    #[test]
    fn p104_gui_caret_pixel_non_background() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p104-caret".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("c".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("a".into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        let (_cx, _cy, px) = caret_column_pixel(&win, &field);
        assert_ne!(px, FIELD_BG, "caret column must not be field background");
        assert_ne!(px, 0xFF808080, "caret column must not be window background");
        // Phase 108: caret may be egui-drawn (not necessarily CARET_INK constant).
        reset_test_gui();
    }

    #[test]
    fn p106_gui_cjk_subset_at_least_64() {
        assert!(cjk_font::ci_subset_unique_char_count() >= 64);
        assert!(super::gui_cjk_subset_char_count() >= 64);
        assert_eq!(super::gui_font_cjk_subset_fingerprint(), "gui-ttf-expand-v1:cjk64-plus");
    }

    #[test]
    fn p106_gui_multi_char_cjk_get_text_exact() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let sample = "你我他的是不在有";
        let mut stack = vec![Value::Str("p106-multi".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("field".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(sample.into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), sample);
        reset_test_gui();
    }

    #[test]
    fn p106_gui_preedit_highlight_pixels_not_background() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p106-preedit".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("ime".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        assert!(apply_ime_preedit_inject(&win, "你"));

        let mut saw_highlight_bg = false;
        let mut only_underline = true;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            let widget = w
                .widgets
                .iter()
                .find(|x| widget_matches(x, &field))
                .unwrap();
            for dy in 0..12u32 {
                for dx in 0..40u32 {
                    let x = widget.x + 1 + dx;
                    let y = widget.y + dy;
                    let idx = (y * w.width + x) as usize;
                    if idx >= w.pixels.len() {
                        continue;
                    }
                    let px = w.pixels[idx];
                    if px == PREEDIT_HL_BG {
                        saw_highlight_bg = true;
                    }
                    if px != FIELD_BG && px != PREEDIT_UNDERLINE && px != GLYPH_INK && px != 0xFF808080
                    {
                        only_underline = false;
                    }
                }
            }
        }
        assert!(saw_highlight_bg, "preedit must draw inverted highlight background");
        assert!(!only_underline, "preedit must not be underline-only");
        reset_test_gui();
    }

    #[test]
    fn p106_gui_ime_commit_clears_preedit_highlight() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p106-commit".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("ime".into()));
        host_gui_text_field_create(&mut stack).unwrap();

        assert!(apply_ime_preedit_inject(&win, "你"));
        assert!(WINDOWS.lock().get(&win).unwrap().ime_preedit_highlight.is_some());

        assert!(apply_ime_commit_to_focused_field(&win, "你"));
        let guard = WINDOWS.lock();
        let w = guard.get(&win).unwrap();
        assert!(w.ime_preedit.is_empty());
        assert!(w.ime_preedit_highlight.is_none());
        drop(guard);
        reset_test_gui();
    }

    #[test]
    fn p106_gui_doctor_capability_fingerprints() {
        assert!(super::gui_ime_preedit_supported());
        assert!(super::gui_ime_commit_supported());
        assert!(super::gui_caret_supported());
        assert_eq!(super::gui_ime_preedit_fingerprint(), "gui-ime-highlight-v1:preedit-invert");
        assert_eq!(super::gui_doctor_coverage_fingerprint(), "gui-doctor-engine-v6:phase118");
    }

    #[test]
    fn p107_gui_fingerprints() {
        assert_eq!(super::gui_harfbuzz_fingerprint(), "gui-harfbuzz-v1:shape-mvp");
        assert_eq!(
            super::gui_font_fallback_fingerprint(),
            "gui-font-fallback-v1:system-cjk"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let src = include_str!("lib.rs");
        assert!(src.contains("gui-harfbuzz-v1:shape-mvp"));
        assert!(src.contains("gui-font-fallback-v1:system-cjk"));
        assert!(src.contains("gui-doctor-engine-v6:phase118"));
        assert!(include_str!("shape.rs").contains("rustybuzz"));
    }

    #[test]
    fn p107_gui_shaped_inject_16_chars_exact_get_text() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        shape::reset_shape_stats();
        // ≥16 CJK chars from CI subset — shaped inject, exact get_text.
        let sample: String = cjk_font::CI_SUBSET.chars().take(16).collect();
        assert!(sample.chars().count() >= 16);
        let mut stack = vec![Value::Str("p107-shape16".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("field".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(sample.clone()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), sample);
        assert!(super::gui_shaped(), "text drawing must mark shaped");
        reset_test_gui();
    }

    #[test]
    fn p107_gui_pump_key_refresh_defers_without_refcell_panic() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p107-pump-key".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("tf".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let _ = stack.pop();

        // Simulate pump callback: keyboard apply refreshes while PUMP_DEPTH > 0.
        pump_enter();
        assert!(apply_key_to_focused_field(&win, "a"));
        assert!(
            DIRTY_OS_WINDOWS.with(|c| c.borrow().iter().any(|id| id == &win)),
            "refresh during pump must mark dirty, not re-enter with_live_mut present"
        );
        pump_leave();
        flush_dirty_os_windows();
        DIRTY_OS_WINDOWS.with(|c| assert!(c.borrow().is_empty()));

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("tf".into()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "a");
        reset_test_gui();
    }

    #[test]
    fn p107_gui_doctor_real_ime_not_fingerprint_only() {
        let src = include_str!("lib.rs");
        assert!(
            !src.contains(
                "gui_ime_preedit_fingerprint() == \"gui-ime-highlight-v1:preedit-invert\""
            ),
            "ime_preedit_supported must not be fingerprint string equality alone"
        );
        assert!(src.contains("probe_ime_preedit_highlight_capability"));
        assert!(super::gui_ime_preedit_supported());
        // Capability is behavioral (scratch highlight pixels + shape APIs), not FP-only.
        assert!(super::probe_ime_preedit_highlight_capability());
    }

    #[test]
    fn p107_gui_doctor_real_caret_from_metrics() {
        let src = include_str!("lib.rs");
        assert!(
            !src.contains("pub fn gui_caret_metric_px() -> u32 {\n    10\n}"),
            "caret metric must not be hardcoded 10"
        );
        assert!(src.contains("caret_metric_from_font"));
        let px = super::gui_caret_metric_px();
        assert!(px > 0);
        // Derived from 14px raster line metrics — must differ from the old literal constant path.
        assert_ne!(px, 10, "caret should come from font metrics, not hardcoded 10");
        assert_eq!(px, shape::caret_metric_from_font());
    }

    #[test]
    fn p107_gui_oob_fallback_non_box_when_glyph_available() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        shape::reset_shape_stats();
        let oob = shape::oob_probe_char();
        assert!(
            !cjk_font::CI_SUBSET.contains(oob),
            "probe char must be outside CI_SUBSET"
        );
        assert!(
            shape::fallback_has_glyph(oob),
            "embedded OOB fallback TTF must cover {oob}"
        );

        let mut stack = vec![Value::Str("p107-oob".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("field".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(oob.to_string()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), oob.to_string());

        assert!(super::gui_fallback_hits() >= 1);

        let g = shape::shape_rasterize_char(oob);
        assert!(!g.is_box, "OOB glyph must not be a box when fallback available");
        #[cfg(feature = "skia-glyph")]
        assert!(g.skia.is_some(), "OOB glyph must raster via skia");
        #[cfg(not(feature = "skia-glyph"))]
        assert!(!g.bitmap.is_empty() || g.metrics.width > 0);

        // Pixel ink present (egui tessellation or shape overlay — not empty field-only).
        let mut saw_ink = false;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            let widget = w
                .widgets
                .iter()
                .find(|x| widget_matches(x, &field))
                .unwrap();
            let bg = 0xFF808080u32;
            for dy in 0..12u32 {
                for dx in 0..40u32 {
                    let x = widget.x + 1 + dx;
                    let y = widget.y + 2 + dy;
                    let idx = (y * w.width + x) as usize;
                    if idx < w.pixels.len() {
                        let px = w.pixels[idx];
                        if px != bg && px != FIELD_BG {
                            saw_ink = true;
                        }
                    }
                }
            }
        }
        assert!(saw_ink, "OOB fallback path must produce non-background pixels");
        reset_test_gui();
    }

    #[test]
    fn p108_gui_fingerprints() {
        assert_eq!(
            super::gui_egui_mvp_fingerprint(),
            "gui-egui-wgpu-v1:winit-present"
        );
        assert_eq!(
            super::gui_pump_reentrancy_fingerprint(),
            "gui-pump-reentrancy-v1:dirty-defer"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let src = include_str!("lib.rs");
        assert!(src.contains("gui-egui-wgpu-v1:winit-present"));
        assert!(src.contains("gui-pump-reentrancy-v1:dirty-defer"));
        assert!(src.contains("gui-doctor-engine-v6:phase118"));
        assert!(src.contains("PUMP_DEPTH"));
        assert!(src.contains("DIRTY_OS_WINDOWS"));
        assert!(include_str!("egui_backend.rs").contains("run_egui_frame"));
        assert!(!include_str!("egui_backend.rs").contains("use egui_winit::EventResponse as _"));
    }

    #[test]
    fn p108_gui_egui_frame_at_least_one() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p108-frames".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Hello".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(
            super::gui_egui_frames(&win) >= 1,
            "creating a label must run ≥1 egui frame"
        );
        reset_test_gui();
    }

    #[test]
    fn p108_gui_inject_8_chars_get_text_exact() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let sample = "abcdefgh";
        assert_eq!(sample.chars().count(), 8);
        let mut stack = vec![Value::Str("p108-inject8".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("field".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(sample.into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), sample);
        assert!(super::gui_egui_frames(&win) >= 1);
        reset_test_gui();
    }

    #[test]
    fn p108_gui_pump_key_refresh_defers_without_refcell_panic() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p108-pump-key".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("tf".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let _ = stack.pop();

        // Simulate pump callback: keyboard apply refreshes while PUMP_DEPTH > 0.
        pump_enter();
        assert!(apply_key_to_focused_field(&win, "a"));
        assert!(
            DIRTY_OS_WINDOWS.with(|c| c.borrow().iter().any(|id| id == &win)),
            "refresh during pump must mark dirty, not re-enter with_live_mut present"
        );
        pump_leave();
        flush_dirty_os_windows();
        DIRTY_OS_WINDOWS.with(|c| assert!(c.borrow().is_empty()));

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("tf".into()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "a");
        assert_eq!(
            super::gui_pump_reentrancy_fingerprint(),
            "gui-pump-reentrancy-v1:dirty-defer"
        );
        reset_test_gui();
    }

    #[test]
    fn p108_gui_label_button_textfield_via_egui() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p108-widgets".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("L".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Go".into()));
        host_gui_button_create(&mut stack).unwrap();
        let _ = stack.pop();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("tf".into()));
        host_gui_text_field_create(&mut stack).unwrap();
        let field = stack.pop().unwrap().as_str();

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("xyz".into()));
        host_gui_key_inject(&mut stack).unwrap();
        assert!(stack.pop().unwrap().as_bool());

        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(field.clone()));
        host_gui_widget_get_text(&mut stack).unwrap();
        assert_eq!(stack.pop().unwrap().as_str(), "xyz");
        assert!(super::gui_egui_frames(&win) >= 1);
        assert_eq!(
            super::gui_egui_mvp_fingerprint(),
            "gui-egui-wgpu-v1:winit-present"
        );
        reset_test_gui();
    }

    #[test]
    fn p109_gui_fingerprints() {
        assert_eq!(
            super::gui_egui_wgpu_fingerprint(),
            "gui-egui-wgpu-v1:winit-present"
        );
        assert_eq!(
            super::gui_hb_paint_fingerprint(),
            "gui-hb-paint-v1:string-shape"
        );
        assert_eq!(super::gui_emoji_fingerprint(), "gui-emoji-v9:font-radial-and-sweep");
        assert_eq!(
            super::gui_vertical_rl_fingerprint(),
            "gui-vertical-unify-v1:hb-advance"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let egui = include_str!("egui_backend.rs");
        assert!(egui.contains("EguiWinitState"));
        assert!(egui.contains("on_window_event"));
        assert!(egui.contains("FontDefinitions"));
        assert!(!egui.contains("use egui_winit::EventResponse as _"));
        assert!(include_str!("shape.rs").contains("gui-hb-paint-v1:string-shape"));
        assert!(include_str!("shape.rs").contains("paint_features"));
        assert!(include_str!("wgpu_present.rs").contains("SYX_GUI_SOFTBUFFER"));
    }

    #[test]
    fn p109_gui_hb_string_shape_feeds_paint() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let sample: String = cjk_font::CI_SUBSET.chars().take(8).collect();
        let mut stack = vec![Value::Str("p109-hb".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(sample.clone()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_paint_shaped(), "paint must mark HB string shape");
        assert!(
            super::gui_last_paint_glyph_count() >= 1,
            "shaped glyphs must feed paint path"
        );
        assert!(include_str!("shape.rs").contains("Feature::new"));
        assert!(!include_str!("shape.rs").contains("shape(&face, &[], buffer)"));
        reset_test_gui();
    }

    #[test]
    fn p109_gui_emoji_not_tofu() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p109-emoji".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(format!("{}", emoji::EMOJI_GRIN)));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_emoji_atlas_hit(), "color emoji atlas must hit");
        let mut saw_chroma = false;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            for &px in &w.pixels {
                if emoji::pixel_is_emoji_chroma(px) {
                    saw_chroma = true;
                    break;
                }
            }
        }
        assert!(saw_chroma, "😀 must paint yellow chroma, not tofu box");
        reset_test_gui();
    }

    #[test]
    fn p109_gui_vertical_rl_label() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p109-vrl".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        // Use CI-subset CJK so paint path stays offline (no system TTC load).
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你我".into()));
        host_gui_label_create(&mut stack).unwrap();
        let label = stack.pop().unwrap().as_str();
        assert!(super::label_set_writing_mode(&win, &label, "vertical-rl"));
        assert!(egui_backend::vertical_rl_used());
        assert_eq!(
            super::gui_vertical_rl_fingerprint(),
            "gui-vertical-unify-v1:hb-advance"
        );
        reset_test_gui();
    }

    #[test]
    fn p110_gui_fingerprints() {
        assert_eq!(
            super::gui_wgpu_composite_fingerprint(),
            "gui-wgpu-composite-v1:hb-emoji-tex"
        );
        assert_eq!(super::gui_emoji_fingerprint(), "gui-emoji-v9:font-radial-and-sweep");
        assert_eq!(
            super::gui_vertical_rl_fingerprint(),
            "gui-vertical-unify-v1:hb-advance"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        assert_eq!(super::gui_emoji_engine(), "cbdt");
        let src = include_str!("lib.rs");
        assert!(src.contains("gui-wgpu-composite-v1:hb-emoji-tex"));
        assert!(src.contains("gui-doctor-engine-v6:phase118"));
        assert!(include_str!("emoji.rs").contains("paint_color_glyph"));
        assert!(include_str!("wgpu_present.rs").contains("composite_overlay"));
    }

    #[test]
    fn p110_gui_colr_emoji_chroma_pixels() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p110-emoji".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(format!("{}", emoji::EMOJI_GRIN)));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_emoji_atlas_hit(), "COLR atlas must hit");
        assert_eq!(super::gui_emoji_engine(), "cbdt");
        let mut saw_chroma = false;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            for &px in &w.pixels {
                if emoji::pixel_is_emoji_chroma(px) {
                    saw_chroma = true;
                    break;
                }
            }
        }
        assert!(saw_chroma, "😀 must paint COLR yellow chroma");
        reset_test_gui();
    }

    #[test]
    fn p110_gui_wgpu_composite_readback_chroma() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p110-composite".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(format!("{}", emoji::EMOJI_GRIN)));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        let (pixels, width, height) = {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            (w.pixels.clone(), w.width, w.height)
        };
        assert!(
            pixels.iter().any(|&px| emoji::pixel_is_emoji_chroma(px)),
            "CPU buffer must contain emoji chroma before wgpu probe"
        );
        assert!(
            super::probe_wgpu_composite_readback_chroma(&pixels, width, height),
            "wgpu composite readback must see emoji/CJK chroma"
        );
        assert!(super::gui_wgpu_composite_used());
        reset_test_gui();
    }

    #[test]
    fn p110_gui_vertical_rl_egui_layout() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p110-vrl".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你我".into()));
        host_gui_label_create(&mut stack).unwrap();
        let label = stack.pop().unwrap().as_str();
        assert!(super::label_set_writing_mode(&win, &label, "vertical-rl"));
        assert!(egui_backend::vertical_rl_layout_used());
        assert!(egui_backend::vertical_rl_used());
        assert!(egui_backend::vertical_ink_used());
        assert!(egui_backend::vertical_unify_used());
        let egui_src = include_str!("egui_backend.rs");
        assert!(!egui_src.contains("layout_no_wrap"));
        assert!(egui_src.contains("VERTICAL_UNIFY"));
        assert!(!egui_src.contains("skip CPU vertical stack"));
        reset_test_gui();
    }

    #[test]
    fn p110_gui_doctor_live_fields() {
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        shape::reset_shape_stats();
        wgpu_present::reset_used_probes();
        let mode = super::gui_doctor_present_mode();
        assert!(
            mode.is_empty() || mode == "softbuffer-dev-fallback",
            "present_mode must be used-only (no wgpu_available): {mode:?}"
        );
        assert!(!super::wgpu_present::softbuffer_fallback_requested());
    }

    #[test]
    fn p109_gui_softbuffer_only_when_env() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert!(!wgpu_present::softbuffer_fallback_requested());
        assert_ne!(
            super::gui_doctor_present_mode(),
            "softbuffer-dev-fallback"
        );
        std::env::set_var("SYX_GUI_SOFTBUFFER", "1");
        assert!(wgpu_present::softbuffer_fallback_requested());
        assert_eq!(
            super::gui_doctor_present_mode(),
            "softbuffer-dev-fallback"
        );
        std::env::remove_var("SYX_GUI_SOFTBUFFER");
        reset_test_gui();
    }

    #[test]
    fn p109_gui_cjk_font_definitions_injected() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p109-fonts".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(egui_backend::fonts_injected());
        assert!(include_str!("egui_backend.rs").contains("syx_cjk"));
        assert!(include_str!("egui_backend.rs").contains("set_fonts"));
        reset_test_gui();
    }

    #[test]
    fn p111_gui_fingerprints() {
        assert_eq!(
            super::gui_skia_glyph_fingerprint(),
            "gui-skia-glyph-v1:raster-mvp"
        );
        assert_eq!(super::gui_emoji_fingerprint(), "gui-emoji-v9:font-radial-and-sweep");
        assert_eq!(
            super::gui_vertical_rl_fingerprint(),
            "gui-vertical-unify-v1:hb-advance"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let egui = include_str!("egui_backend.rs");
        assert!(!egui.contains("skip CPU vertical stack"));
        assert!(include_str!("emoji.rs").contains("fill_path"));
        assert!(!include_str!("emoji.rs").contains("fill_bbox"));
        assert!(include_str!("skia_glyph.rs").contains("skia_glyph_fingerprint"));
    }

    #[test]
    fn p111_gui_skia_glyph_raster_used() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p111-skia".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你好".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_skia_glyph_used(), "HB paint must raster via skia");
        assert!(super::gui_paint_shaped());
        reset_test_gui();
    }

    #[test]
    fn p111_gui_colr_outline_chroma() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p111-emoji".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(format!("{}", emoji::EMOJI_GRIN)));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_emoji_atlas_hit());
        assert_eq!(super::gui_emoji_engine(), "cbdt");
        let mut saw_chroma = false;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            for &px in &w.pixels {
                if emoji::pixel_is_emoji_chroma(px) {
                    saw_chroma = true;
                    break;
                }
            }
        }
        assert!(saw_chroma, "COLR outline must paint yellow chroma");
        reset_test_gui();
    }

    #[test]
    fn p111_gui_vertical_hb_ink_on() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p111-vrl".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你我".into()));
        host_gui_label_create(&mut stack).unwrap();
        let label = stack.pop().unwrap().as_str();
        assert!(super::label_set_writing_mode(&win, &label, "vertical-rl"));
        assert!(egui_backend::vertical_ink_used());
        assert!(super::gui_vertical_ink_used());
        reset_test_gui();
    }

    #[test]
    fn p111_gui_doctor_widgets_used_only() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(super::gui_doctor_widgets(), "");
        let mut stack = vec![Value::Str("p111-doc".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        let widgets = super::gui_doctor_widgets();
        assert!(widgets.contains("paragraph-paint"), "widgets: {widgets}");
        assert!(
            !widgets.contains("hb-paint"),
            "single-path CJK label must not HB-paint: {widgets}"
        );
        assert!(!widgets.contains("hb-emoji-composite") || super::gui_wgpu_composite_used());
        reset_test_gui();
        assert_eq!(super::gui_doctor_widgets(), "");
    }

    #[test]
    fn p112_gui_fingerprints() {
        assert_eq!(
            super::gui_skia_layout_fingerprint(),
            "gui-skia-paragraph-v6:os-ot-aligned"
        );
        assert_eq!(super::gui_emoji_fingerprint(), "gui-emoji-v9:font-radial-and-sweep");
        assert_eq!(
            super::gui_vertical_rl_fingerprint(),
            "gui-vertical-unify-v1:hb-advance"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let src = include_str!("lib.rs");
        assert!(src.contains("gui-skia-paragraph-v6:os-ot-aligned"));
        assert!(include_str!("emoji.rs").contains("glyph_raster_image"));
        assert!(include_str!("shape.rs").contains("probe_shape_apis_ok"));
        assert!(!include_str!("shape.rs").contains("SHAPED_USED.store(true, Ordering::Relaxed);\n    PAINT_SHAPE_USED.store(true, Ordering::Relaxed);\n    LAST_PAINT_GLYPHS"));
        assert!(include_str!("wgpu_present.rs").contains("wgpu_present_used()"));
        assert!(
            !include_str!("wgpu_present.rs").contains("wgpu_present_used() || wgpu_available()")
        );
    }

    #[test]
    fn p112_gui_skia_paragraph_layout_used() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p112-layout".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你好世界".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_skia_layout_used(), "label paint must run skia paragraph layout");
        reset_test_gui();
    }

    #[test]
    fn p112_gui_cbdt_or_colr_emoji() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p112-emoji".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(format!("{}", emoji::EMOJI_GRIN)));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_emoji_atlas_hit());
        assert_eq!(super::gui_emoji_engine(), "cbdt");
        assert!(include_str!("emoji.rs").contains("rasterize_cbdt_glyph"));
        reset_test_gui();
    }

    #[test]
    fn p112_gui_vertical_hb_advance_ascii_ink() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p112-vrl".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("ab".into()));
        host_gui_label_create(&mut stack).unwrap();
        let label = stack.pop().unwrap().as_str();
        assert!(super::label_set_writing_mode(&win, &label, "vertical-rl"));
        assert!(super::gui_vertical_unify_used());
        let mut saw_ink = false;
        {
            let guard = WINDOWS.lock();
            let w = guard.get(&win).unwrap();
            for &px in &w.pixels {
                if px != 0xFF808080 && px != FIELD_BG {
                    saw_ink = true;
                    break;
                }
            }
        }
        assert!(saw_ink, "vertical ASCII must paint via HB advance path, not skip");
        assert!(egui_backend::vertical_ascii_ink_used());
        assert!(egui_backend::vertical_ink_used());
        let egui_src = include_str!("egui_backend.rs");
        assert!(!egui_src.contains("blit_glyph5x7"));
        reset_test_gui();
    }

    #[test]
    fn p112_gui_doctor_strict_used_only() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        shape::reset_shape_stats();
        wgpu_present::reset_used_probes();
        assert!(!super::gui_shaped(), "probe_shape_apis_ok must not set shaped");
        assert!(shape::probe_shape_apis_ok());
        assert!(!super::gui_shaped());
        assert_eq!(super::gui_doctor_present_mode(), "");
        let mut stack = vec![Value::Str("p112-doc".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_shaped());
        let widgets = super::gui_doctor_widgets();
        assert!(widgets.contains("paragraph-paint"), "widgets: {widgets}");
        assert!(!widgets.contains("hb-paint") || super::gui_hb_paint_path_used());
        reset_test_gui();
    }

    #[test]
    fn p112_gui_bidi_layout_mvp() {
        let _guard = TEST_LOCK.lock();
        #[cfg(feature = "skia-glyph")]
        {
            skia_layout::reset_skia_layout_stats();
            let sample = "\u{05D0}\u{05D1}\u{05D2}";
            let _ = skia_layout::layout_paragraph(sample, 120.0);
            assert!(skia_layout::layout_used());
            assert!(skia_layout::bidi_used(), "RTL run must set BIDI_USED");
            assert!(
                skia_layout::textlayout_api_used(),
                "must use skia textlayout ParagraphBuilder"
            );
        }
    }

    #[test]
    fn p113_gui_fingerprints() {
        assert_eq!(
            super::gui_skia_layout_fingerprint(),
            "gui-skia-paragraph-v6:os-ot-aligned"
        );
        assert_eq!(super::gui_emoji_fingerprint(), "gui-emoji-v9:font-radial-and-sweep");
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let layout = include_str!("skia_layout.rs");
        assert!(layout.contains("ParagraphBuilder"));
        assert!(layout.contains("textlayout"));
        assert!(!layout.contains("unicode_bidi"));
        assert!(!layout.contains("BidiInfo"));
        let emoji_src = include_str!("emoji.rs");
        assert!(emoji_src.contains("emoji_cbdt.ttf"));
        assert!(emoji_src.contains("LinearGradient"));
        assert!(!emoji_src.contains("\"cbdt-or-colr\""));
        assert!(!include_str!("egui_backend.rs").contains("cbdt-or-colr"));
        assert!(!include_str!("dylib_api.rs").contains("cbdt-or-colr"));
    }

    #[test]
    fn p113_gui_skia_textlayout_paragraph() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        #[cfg(feature = "skia-glyph")]
        {
            skia_layout::reset_skia_layout_stats();
        }
        let mut stack = vec![Value::Str("p113-layout".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你好世界段落换行测试".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_skia_layout_used(), "label paint must run skia paragraph");
        #[cfg(feature = "skia-glyph")]
        {
            assert!(
                skia_layout::textlayout_api_used(),
                "must touch ParagraphBuilder textlayout API"
            );
            assert!(
                skia_layout::paragraph_paint_used(),
                "label paint must use Paragraph::paint"
            );
            assert!(
                !skia_layout::font_mgr_fallback_used(),
                "subset text must not set proven fallback"
            );
            assert!(
                skia_layout::probe_font_mgr_fallback_paint(),
                "OOB glyph must prove FontMgr fallback"
            );
        }
        reset_test_gui();
    }

    #[test]
    fn p113_gui_cbdt_ci_engine() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(super::gui_emoji_engine(), "cbdt");
        let mut stack = vec![Value::Str("p113-cbdt".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(format!("{}", emoji::EMOJI_GRIN)));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(super::gui_emoji_atlas_hit());
        assert_eq!(super::gui_emoji_engine(), "cbdt");
        let widgets = super::gui_doctor_widgets();
        assert!(widgets.contains("cbdt"), "widgets: {widgets}");
        assert!(!widgets.contains("cbdt-or-colr"), "widgets: {widgets}");
        reset_test_gui();
    }

    #[test]
    fn p113_gui_colr_linear_gradient() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert!(
            emoji::probe_colr_gradient_atlas(),
            "COLR path must paint LinearGradient (not Solid-only)"
        );
        assert!(emoji::colr_grad_used());
        assert_eq!(emoji::emoji_colr_engine(), "colr-grad");
        let widgets = super::gui_doctor_widgets();
        assert!(widgets.contains("colr-grad"), "widgets: {widgets}");
        assert!(!widgets.contains("cbdt-or-colr"), "widgets: {widgets}");
        let src = include_str!("emoji.rs");
        assert!(src.contains("ColrPaint::LinearGradient"));
        assert!(src.contains("fill_linear_gradient"));
        reset_test_gui();
    }

    #[test]
    fn p113_gui_doctor_engine_accurate() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(super::gui_doctor_widgets(), "");
        let mut stack = vec![Value::Str("p113-doc".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str(format!("{}", emoji::EMOJI_GRIN)));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        let widgets = super::gui_doctor_widgets();
        assert!(widgets.contains("cbdt"), "widgets: {widgets}");
        assert!(!widgets.contains("cbdt-or-colr"), "ban sticky OR tag: {widgets}");
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        reset_test_gui();
    }

    #[test]
    fn p114_gui_paragraph_paint_and_font_mgr() {
        assert_eq!(
            super::gui_skia_layout_fingerprint(),
            "gui-skia-paragraph-v6:os-ot-aligned"
        );
        assert_eq!(
            super::gui_font_system_fingerprint(),
            "gui-font-system-v5:os-only-all-apis"
        );
        let layout = include_str!("skia_layout.rs");
        assert!(layout.contains("PARAGRAPH_PAINT_USED"));
        assert!(layout.contains("paint_paragraph"));
        assert!(layout.contains("probe_font_mgr_os_paint"));
        assert!(layout.contains("font_mgr_os_used"));
        assert!(layout.contains("font_mgr_bundled_used"));
        let emoji_src = include_str!("emoji.rs");
        assert!(!emoji_src.contains("Path::rect(Rect::from_xywh(0.0, 0.0, EMOJI_W"));
        assert!(!emoji_src.contains("Phase 113: Solid-only"));
        let egui_src = include_str!("egui_backend.rs");
        assert!(!egui_src.contains("blit_glyph5x7"));
        assert!(!egui_src.contains("touch_label_layout"));
    }

    #[test]
    fn p115_gui_frame_single_ink_and_doctor_v4() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        assert_eq!(
            super::gui_emoji_fingerprint(),
            "gui-emoji-v9:font-radial-and-sweep"
        );
        let mut stack = vec![Value::Str("p115-single".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("ABC".into()));
        host_gui_label_create(&mut stack).unwrap();
        let _ = stack.pop();
        #[cfg(feature = "skia-glyph")]
        {
            assert!(skia_layout::paragraph_paint_used());
            assert!(!super::gui_hb_paint_path_used(), "horizontal label must not double-paint");
        }
        assert!(egui_backend::egui_text_suppressed_used());
        assert!(egui_backend::no_double_paint_probe());
        reset_test_gui();
    }

    #[test]
    fn p116_gui_ime_preedit_no_glyph5x7() {
        let src = include_str!("lib.rs");
        let probe = src
            .split("fn probe_ime_preedit_highlight_capability")
            .nth(1)
            .and_then(|s| s.split("\nfn ").next())
            .unwrap_or("");
        assert!(!probe.contains("blit_glyph5x7"));
        let composite = src
            .split("// IME preedit overlay")
            .nth(1)
            .and_then(|s| s.split("\nfn present_gui_window").next())
            .unwrap_or("");
        assert!(!composite.contains("blit_glyph5x7"));
    }

    #[test]
    fn p115_gui_colr_radial_and_sweep_probe() {
        assert!(
            emoji::probe_colr_radial_or_sweep_atlas(),
            "COLR radial and sweep gradient paths must paint from font file"
        );
        assert!(emoji::colr_radial_used());
        assert!(emoji::colr_sweep_used());
        assert!(emoji::colr_radial_doctor_used());
        assert!(emoji::colr_sweep_doctor_used());
    }

    #[test]
    fn p117_gui_all_widgets_ink_and_ot_calt() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(
            super::gui_skia_layout_fingerprint(),
            "gui-skia-paragraph-v6:os-ot-aligned"
        );
        assert_eq!(
            super::gui_font_system_fingerprint(),
            "gui-font-system-v5:os-only-all-apis"
        );
        assert_eq!(
            super::gui_emoji_fingerprint(),
            "gui-emoji-v9:font-radial-and-sweep"
        );
        assert_eq!(
            super::gui_ot_features_fingerprint(),
            "gui-ot-features-v7:os-fontmgr-default-paint"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        let mut stack = vec![Value::Str("p117-btn".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("Go".into()));
        host_gui_button_create(&mut stack).unwrap();
        let _ = stack.pop();
        assert!(egui_backend::egui_text_suppressed_used());
        assert!(egui_backend::no_double_paint_probe());
        // OT calt is applied on the HB shape path (paragraph uses skia HB separately).
        let _ = shape::shape_string_for_paint("affi", 8);
        assert!(super::gui_ot_calt_used());
        assert!(include_str!("shape.rs").contains("gui-ot-features-v7:os-fontmgr-default-paint"));
        assert!(include_str!("shape.rs").contains("paint_features_matrix"));
        let egui_src = include_str!("egui_backend.rs");
        assert!(egui_src.contains("put_cpu_ink_button"));
        assert!(!egui_src.contains("Button::new(texts[i].as_str())"));
        let blit = include_str!("lib.rs")
            .split("fn blit_text")
            .nth(1)
            .and_then(|s| s.split("\nfn ").next())
            .unwrap_or("");
        assert!(!blit.contains("blit_glyph5x7"));
        assert!(!blit.contains("glyph5x7("));
        reset_test_gui();
    }

    #[test]
    fn p117_gui_vertical_paragraph_path() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        let mut stack = vec![Value::Str("p117-vpara".into())];
        host_gui_winit_window_create(&mut stack).unwrap();
        let win = stack.pop().unwrap().as_str();
        stack.push(Value::Str(win.clone()));
        stack.push(Value::Str("你好".into()));
        host_gui_label_create(&mut stack).unwrap();
        let label = stack.pop().unwrap().as_str();
        assert!(super::label_set_writing_mode(&win, &label, "vertical-rl"));
        #[cfg(feature = "skia-glyph")]
        {
            assert!(
                skia_layout::vertical_paragraph_used() || skia_layout::paragraph_paint_used(),
                "vertical-rl must attempt paragraph path"
            );
            assert!(egui_backend::vertical_ink_used());
        }
        let widgets = super::gui_doctor_widgets();
        assert!(
            widgets.contains("vertical-paragraph")
                || widgets.contains("paragraph-paint")
                || widgets.contains("vertical-ink"),
            "widgets: {widgets}"
        );
        reset_test_gui();
    }

    #[test]
    fn p117_gui_font_colr_sweep_not_doctor_alone() {
        emoji::reset_emoji_stats();
        assert!(
            emoji::probe_colr_radial_or_sweep_atlas(),
            "font-file COLR must paint radial and sweep"
        );
        assert!(emoji::colr_radial_used());
        assert!(emoji::colr_sweep_used());
        // Doctor may probe, but GA flags come from rasterize_colr_glyph.
        assert!(include_str!("emoji.rs").contains("COLR_SWEEP_DOCTOR"));
        assert!(!include_str!("emoji.rs").contains("prove_colr_sweep_font_paint_tree"));
    }

    #[test]
    fn p118_gui_os_ot_vf_doctor_v6() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(
            super::gui_skia_layout_fingerprint(),
            "gui-skia-paragraph-v6:os-ot-aligned"
        );
        assert_eq!(
            super::gui_font_system_fingerprint(),
            "gui-font-system-v5:os-only-all-apis"
        );
        assert_eq!(
            super::gui_ot_features_fingerprint(),
            "gui-ot-features-v7:os-fontmgr-default-paint"
        );
        assert_eq!(
            super::gui_vf_wght_fingerprint(),
            "gui-font-vf-v1:wght-mvp"
        );
        assert_eq!(
            super::gui_doctor_coverage_fingerprint(),
            "gui-doctor-engine-v6:phase118"
        );
        assert!(
            super::gui_prove_ot_substitution(),
            "OT liga must change glyph-id sequence vs baseline"
        );
        assert!(super::gui_ot_subst_proven());
        assert!(
            super::gui_prove_vf_wght(),
            "VF wght=100 vs wght=900 advances must differ"
        );
        assert!(super::gui_vf_wght_used());
        #[cfg(feature = "skia-glyph")]
        {
            let _ = skia_layout::layout_paragraph("hello", 120.0);
            assert!(
                include_str!("skia_layout.rs").contains("layout_paragraph_for_test"),
                "bundled layout must be explicit non-GA helper"
            );
            assert!(
                !include_str!("skia_layout.rs")
                    .split("pub fn layout_paragraph")
                    .nth(1)
                    .and_then(|s| s.split("pub fn ").next())
                    .unwrap_or("")
                    .contains("include_test: true")
                    && include_str!("skia_layout.rs").contains("build_paragraph(text, max_width, false)"),
                "layout_paragraph must be OS-only"
            );
        }
        let widgets = super::gui_doctor_widgets();
        assert!(
            widgets.contains("ot-subst") && widgets.contains("vf-wght"),
            "widgets: {widgets}"
        );
        reset_test_gui();
    }

    #[test]
    fn p119_gui_ot_script_vf_multi_axis() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(
            super::gui_ot_features_fingerprint(),
            "gui-ot-features-v7:os-fontmgr-default-paint"
        );
        assert_eq!(
            super::gui_vf_multi_axis_fingerprint(),
            "gui-font-vf-v2:multi-axis-mvp"
        );
        assert!(
            super::gui_prove_ot_script_subset(),
            "OT latn script-subset liga must change glyph-ids"
        );
        assert!(super::gui_ot_script_subst_proven());
        assert!(
            super::gui_prove_vf_multi_axis(),
            "VF wdth/wght multi-axis advances must differ"
        );
        assert!(super::gui_vf_multi_axis_used());
        let widgets = super::gui_doctor_widgets();
        assert!(
            widgets.contains("ot-script") && widgets.contains("vf-multi-axis"),
            "widgets: {widgets}"
        );
        reset_test_gui();
    }

    #[test]
    fn p122_gui_default_paint_ot_dylib_parity() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(
            super::gui_ot_features_fingerprint(),
            "gui-ot-features-v7:os-fontmgr-default-paint"
        );
        assert_eq!(
            super::gui_dylib_ot_vf_parity_fingerprint(),
            "gui-dylib-parity-v1:ot-vf-export"
        );
        assert!(
            super::gui_prove_ot_default_paint(),
            "default paint OT must differ smcp combos via FontMgr path without prefix routing"
        );
        assert!(super::gui_ot_default_paint_proven());
        assert!(super::gui_default_paint_fontmgr_used());
        let widgets = super::gui_doctor_widgets();
        assert!(
            widgets.contains("ot-default-paint") && widgets.contains("ot-os-fontmgr-default-paint"),
            "widgets: {widgets}"
        );
        assert!(
            !shape::DEFAULT_PAINT_OT_TEXT.contains('\x01'),
            "v7 GA must not rely on \\x01syx-* prefix strings"
        );
        // Prefix-only paint-matrix still works but is not the v7 default-paint gate.
        assert!(
            super::gui_prove_ot_paint_matrix(),
            "inherited prefix paint-matrix probe"
        );
        shape::reset_shape_stats();
        // Hook-only path must not count as v7 GA.
        shape::set_default_paint_ot_hook(true);
        shape::set_default_paint_ot_fontmgr(false);
        shape::set_paint_smcp_enabled(false);
        let hook_off = shape::shape_string_for_paint(shape::DEFAULT_PAINT_OT_TEXT, 8);
        shape::set_paint_smcp_enabled(true);
        let hook_on = shape::shape_string_for_paint(shape::DEFAULT_PAINT_OT_TEXT, 8);
        shape::set_default_paint_ot_hook(false);
        assert_ne!(
            hook_off.iter().map(|g| g.glyph_id).collect::<Vec<_>>(),
            hook_on.iter().map(|g| g.glyph_id).collect::<Vec<_>>(),
            "hook path may still inject proof font for doctor"
        );
        assert!(
            !super::gui_default_paint_fontmgr_used(),
            "hook-only must not set FontMgr-used flag"
        );
        shape::set_default_paint_ot_fontmgr(false);
        shape::set_paint_smcp_enabled(false);
        let off = shape::shape_string_for_paint(shape::DEFAULT_PAINT_OT_TEXT, 8);
        shape::set_paint_smcp_enabled(true);
        let on = shape::shape_string_for_paint(shape::DEFAULT_PAINT_OT_TEXT, 8);
        let prefix_off = shape::shape_string_for_paint(
            &format!("{}a", shape::PAINT_OT_SMCP_PREFIX),
            8,
        );
        assert_eq!(
            off.iter().map(|g| g.glyph_id).collect::<Vec<_>>(),
            on.iter().map(|g| g.glyph_id).collect::<Vec<_>>(),
            "without FontMgr or hook, default path must not count as v7 GA"
        );
        assert!(
            !prefix_off.is_empty(),
            "prefix path remains available for inherited paint-matrix"
        );
        let dylib = include_str!("dylib_api.rs");
        assert!(dylib.contains("gui-dylib-parity-v1:ot-vf-export"));
        assert!(dylib.contains("syx_gui_dylib_prove_ot_default_paint"));
        assert!(dylib.contains("syx_gui_dylib_default_paint_fontmgr_used"));
        assert!(dylib.contains("syx_gui_dylib_prove_ot_substitution"));
        assert!(dylib.contains("syx_gui_dylib_prove_vf_wght"));
        assert!(dylib.contains("syx_gui_dylib_prove_vf_multi_axis"));
        assert!(dylib.contains("syx_gui_dylib_prove_vf_axes_ge3"));
        assert!(dylib.contains("syx_gui_dylib_prove_vf_paint_variations"));
        reset_test_gui();
    }

    #[test]
    fn p123_gui_os_fontmgr_default_paint_ot() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(
            super::gui_ot_features_fingerprint(),
            "gui-ot-features-v7:os-fontmgr-default-paint"
        );
        assert!(include_str!("shape.rs").contains("DEFAULT_PAINT_OT_FAMILY"));
        assert!(include_str!("shape.rs").contains("resolve_default_paint_ot_ttf_via_fontmgr"));
        assert!(include_str!("shape.rs").contains("OrderedFontMgr"));
        assert!(include_str!("shape.rs").contains("FontMgr::default"));
        shape::set_default_paint_ot_hook(false);
        shape::set_default_paint_ot_fontmgr(true);
        shape::set_paint_smcp_enabled(false);
        let smcp_off = shape::shape_string_for_paint(shape::DEFAULT_PAINT_OT_TEXT, 8);
        shape::set_paint_smcp_enabled(true);
        let smcp_on = shape::shape_string_for_paint(shape::DEFAULT_PAINT_OT_TEXT, 8);
        shape::set_default_paint_ot_fontmgr(false);
        assert_ne!(
            smcp_off.iter().map(|g| g.glyph_id).collect::<Vec<_>>(),
            smcp_on.iter().map(|g| g.glyph_id).collect::<Vec<_>>(),
            "FontMgr default-paint path must apply smcp feature matrix"
        );
        assert!(shape::default_paint_fontmgr_used());
        reset_test_gui();
    }

    #[test]
    fn p121_gui_ot_paint_matrix_vf_paint_variations() {
        let _guard = TEST_LOCK.lock();
        reset_test_gui();
        assert_eq!(
            super::gui_ot_features_fingerprint(),
            "gui-ot-features-v7:os-fontmgr-default-paint"
        );
        assert_eq!(
            super::gui_vf_paint_variations_fingerprint(),
            "gui-font-vf-v4:paint-variations-mvp"
        );
        assert!(
            super::gui_prove_ot_paint_matrix(),
            "OT paint-matrix must differ smcp and liga combos on paint path"
        );
        assert!(super::gui_ot_paint_matrix_proven());
        assert!(
            super::gui_prove_vf_paint_variations(),
            "VF wght axis must move paint-path advances"
        );
        assert!(super::gui_vf_paint_variations_used());
        let smcp_off = shape::shape_string_for_paint(
            &format!("{}a", shape::PAINT_OT_SMCP_PREFIX),
            8,
        );
        shape::set_paint_vf_wght(100.0);
        let vf_light = shape::shape_string_for_paint(
            &format!("{}A", shape::PAINT_VF_VAR_PREFIX),
            8,
        );
        assert!(
            !smcp_off.is_empty() && !vf_light.is_empty(),
            "paint path must shape proof-font probes, not proof-only APIs"
        );
        let widgets = super::gui_doctor_widgets();
        assert!(
            widgets.contains("ot-paint-matrix") && widgets.contains("vf-paint-variations"),
            "widgets: {widgets}"
        );
        reset_test_gui();
    }
}
