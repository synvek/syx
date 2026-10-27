//! Phase 96: dynamic load `libsyx_gui` — keeps winit/softbuffer out of `libsyx_rt`.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use libloading::Library;
use syx_diagnostics::SyxResult;
use syx_runtime_core::Value;

type BackendNameFn = unsafe extern "C" fn() -> *const c_char;
type ResetFn = unsafe extern "C" fn();
type FreeStringFn = unsafe extern "C" fn(*mut c_char);
type WindowCreateFn = unsafe extern "C" fn(*const c_char, *mut *mut c_char) -> bool;
type CanvasDrawFn = unsafe extern "C" fn(*const c_char, *const c_char) -> bool;
type PixelReadFn = unsafe extern "C" fn(*const c_char) -> i32;
type RunLoopFn = unsafe extern "C" fn(*const c_char, *mut *mut c_char) -> bool;
type EventPumpFn = unsafe extern "C" fn(*const c_char, *mut *mut c_char) -> bool;
type VisibleCountFn = unsafe extern "C" fn() -> usize;

type WidgetCreateFn = unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> bool;
type LayoutCreateFn = unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> bool;
type LayoutConfigureFn =
    unsafe extern "C" fn(*const c_char, i32, *const c_char, *mut *mut c_char) -> bool;
type WidgetSetTextFn = unsafe extern "C" fn(*const c_char, *const c_char, *const c_char) -> bool;
type WidgetGetTextFn = unsafe extern "C" fn(*const c_char, *const c_char, *mut *mut c_char) -> bool;
type OnClickRegisterFn = unsafe extern "C" fn(*const c_char, *const c_char, *const c_char) -> bool;
type ClickInjectFn = unsafe extern "C" fn(*const c_char, *const c_char) -> bool;
type KeyInjectFn = unsafe extern "C" fn(*const c_char, *const c_char) -> bool;
type DrainPendingFn = unsafe extern "C" fn(*const c_char, *mut *mut c_char) -> bool;
type ButtonClickedFn = unsafe extern "C" fn(*const c_char, *const c_char) -> bool;
type ButtonTakeClickedFn = unsafe extern "C" fn(*const c_char, *const c_char) -> bool;
type FingerprintFn = unsafe extern "C" fn() -> *const c_char;
type U32Fn = unsafe extern "C" fn() -> u32;
type BoolFn = unsafe extern "C" fn() -> bool;
type ImePreeditInjectFn = unsafe extern "C" fn(*const c_char, *const c_char) -> bool;
type DoctorWidgetsFn = unsafe extern "C" fn(*mut *mut c_char) -> bool;

struct GuiDylib {
    _lib: Library,
    backend_name: BackendNameFn,
    reset: ResetFn,
    free_string: FreeStringFn,
    window_create: WindowCreateFn,
    canvas_draw: CanvasDrawFn,
    pixel_read: PixelReadFn,
    run_loop_once: RunLoopFn,
    event_pump: EventPumpFn,
    visible_count: VisibleCountFn,
    label_create: WidgetCreateFn,
    button_create: WidgetCreateFn,
    text_field_create: WidgetCreateFn,
    layout_create: LayoutCreateFn,
    layout_configure: LayoutConfigureFn,
    widget_set_text: WidgetSetTextFn,
    widget_get_text: WidgetGetTextFn,
    on_click_register: OnClickRegisterFn,
    click_inject: ClickInjectFn,
    key_inject: KeyInjectFn,
    drain_pending: DrainPendingFn,
    button_clicked: ButtonClickedFn,
    button_take_clicked: ButtonTakeClickedFn,
    ttf_fingerprint: FingerprintFn,
    ime_preedit_fingerprint: FingerprintFn,
    doctor_coverage_fingerprint: FingerprintFn,
    harfbuzz_fingerprint: FingerprintFn,
    font_fallback_fingerprint: FingerprintFn,
    glyph_coverage_percent: U32Fn,
    cjk_subset_char_count: U32Fn,
    caret_metric_px: U32Fn,
    fallback_hits: U32Fn,
    ime_preedit_supported: BoolFn,
    ime_commit_supported: BoolFn,
    caret_supported: BoolFn,
    shaped: BoolFn,
    paint_shaped: BoolFn,
    ime_preedit_inject: ImePreeditInjectFn,
    wgpu_composite_fingerprint: FingerprintFn,
    wgpu_present_used: BoolFn,
    wgpu_composite_used: BoolFn,
    emoji_engine: FingerprintFn,
    doctor_engine_tag: FingerprintFn,
    vertical_rl_layout_used: BoolFn,
    doctor_present_mode: FingerprintFn,
    emoji_fingerprint: FingerprintFn,
    vertical_rl_fingerprint: FingerprintFn,
    skia_glyph_fingerprint: FingerprintFn,
    skia_glyph_used: BoolFn,
    paragraph_paint_used: BoolFn,
    vertical_ink_used: BoolFn,
    vertical_hb_vertical_used: BoolFn,
    egui_text_suppressed_used: BoolFn,
    no_double_paint_probe: BoolFn,
    emoji_atlas_hit: BoolFn,
    hb_paint_path_used: BoolFn,
    font_mgr_fallback_used: BoolFn,
    font_mgr_os_used: BoolFn,
    font_mgr_bundled_used: BoolFn,
    vertical_ascii_ink_used: BoolFn,
    colr_radial_or_sweep_used: BoolFn,
    colr_radial_used: BoolFn,
    colr_sweep_used: BoolFn,
    colr_grad_used: BoolFn,
    doctor_widgets: DoctorWidgetsFn,
}

static GUI_DYLIB: LazyLock<Mutex<Option<GuiDylib>>> = LazyLock::new(|| Mutex::new(None));

fn dylib_search_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(p) = std::env::var("SYX_GUI_DYLIB") {
        paths.push(PathBuf::from(p));
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            #[cfg(target_os = "macos")]
            {
                paths.push(dir.join("libsyx_gui.dylib"));
                paths.push(dir.join("deps/libsyx_gui.dylib"));
            }
            #[cfg(target_os = "linux")]
            {
                paths.push(dir.join("libsyx_gui.so"));
                paths.push(dir.join("deps/libsyx_gui.so"));
            }
            #[cfg(target_os = "windows")]
            {
                paths.push(dir.join("syx_gui.dll"));
                paths.push(dir.join("deps/syx_gui.dll"));
            }
        }
    }
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let base = PathBuf::from(manifest);
        for rel in [
            "../../target/debug/libsyx_gui.dylib",
            "../../target/debug/libsyx_gui.so",
            "../../target/debug/syx_gui.dll",
            "../../target/release/libsyx_gui.dylib",
            "../../target/release/libsyx_gui.so",
            "../../../target/debug/libsyx_gui.dylib",
            "../../../target/debug/libsyx_gui.so",
            "../../../target/release/libsyx_gui.dylib",
        ] {
            paths.push(base.join(rel));
        }
    }
    #[cfg(target_os = "macos")]
    {
        paths.push(PathBuf::from("libsyx_gui.dylib"));
        paths.push(PathBuf::from("target/debug/libsyx_gui.dylib"));
        paths.push(PathBuf::from("target/release/libsyx_gui.dylib"));
    }
    #[cfg(target_os = "linux")]
    {
        paths.push(PathBuf::from("libsyx_gui.so"));
        paths.push(PathBuf::from("target/debug/libsyx_gui.so"));
    }
    #[cfg(target_os = "windows")]
    {
        paths.push(PathBuf::from("syx_gui.dll"));
    }
    paths
}

fn load_dylib(path: &Path) -> Option<GuiDylib> {
    let lib = unsafe { Library::new(path).ok()? };
    unsafe {
        Some(GuiDylib {
            backend_name: *lib.get(b"syx_gui_dylib_backend_name").ok()?,
            reset: *lib.get(b"syx_gui_dylib_reset").ok()?,
            free_string: *lib.get(b"syx_gui_dylib_free_string").ok()?,
            window_create: *lib.get(b"syx_gui_dylib_window_create").ok()?,
            canvas_draw: *lib.get(b"syx_gui_dylib_canvas_draw").ok()?,
            pixel_read: *lib.get(b"syx_gui_dylib_pixel_read").ok()?,
            run_loop_once: *lib.get(b"syx_gui_dylib_run_loop_once").ok()?,
            event_pump: *lib.get(b"syx_gui_dylib_event_pump").ok()?,
            visible_count: *lib.get(b"syx_gui_dylib_visible_window_count").ok()?,
            label_create: *lib.get(b"syx_gui_dylib_label_create").ok()?,
            button_create: *lib.get(b"syx_gui_dylib_button_create").ok()?,
            text_field_create: *lib.get(b"syx_gui_dylib_text_field_create").ok()?,
            layout_create: *lib.get(b"syx_gui_dylib_layout_create").ok()?,
            layout_configure: *lib.get(b"syx_gui_dylib_layout_configure").ok()?,
            widget_set_text: *lib.get(b"syx_gui_dylib_widget_set_text").ok()?,
            widget_get_text: *lib.get(b"syx_gui_dylib_widget_get_text").ok()?,
            on_click_register: *lib.get(b"syx_gui_dylib_on_click_register").ok()?,
            click_inject: *lib.get(b"syx_gui_dylib_click_inject").ok()?,
            key_inject: *lib.get(b"syx_gui_dylib_key_inject").ok()?,
            drain_pending: *lib.get(b"syx_gui_dylib_drain_pending_handlers").ok()?,
            button_clicked: *lib.get(b"syx_gui_dylib_button_clicked").ok()?,
            button_take_clicked: *lib.get(b"syx_gui_dylib_button_take_clicked").ok()?,
            ttf_fingerprint: *lib.get(b"syx_gui_dylib_ttf_fingerprint").ok()?,
            ime_preedit_fingerprint: *lib.get(b"syx_gui_dylib_ime_preedit_fingerprint").ok()?,
            doctor_coverage_fingerprint: *lib
                .get(b"syx_gui_dylib_doctor_coverage_fingerprint")
                .ok()?,
            harfbuzz_fingerprint: *lib.get(b"syx_gui_dylib_harfbuzz_fingerprint").ok()?,
            font_fallback_fingerprint: *lib.get(b"syx_gui_dylib_font_fallback_fingerprint").ok()?,
            glyph_coverage_percent: *lib.get(b"syx_gui_dylib_glyph_coverage_percent").ok()?,
            cjk_subset_char_count: *lib.get(b"syx_gui_dylib_cjk_subset_char_count").ok()?,
            caret_metric_px: *lib.get(b"syx_gui_dylib_caret_metric_px").ok()?,
            fallback_hits: *lib.get(b"syx_gui_dylib_fallback_hits").ok()?,
            ime_preedit_supported: *lib.get(b"syx_gui_dylib_ime_preedit_supported").ok()?,
            ime_commit_supported: *lib.get(b"syx_gui_dylib_ime_commit_supported").ok()?,
            caret_supported: *lib.get(b"syx_gui_dylib_caret_supported").ok()?,
            shaped: *lib.get(b"syx_gui_dylib_shaped").ok()?,
            paint_shaped: *lib.get(b"syx_gui_dylib_paint_shaped").ok()?,
            ime_preedit_inject: *lib.get(b"syx_gui_dylib_ime_preedit_inject").ok()?,
            wgpu_composite_fingerprint: *lib
                .get(b"syx_gui_dylib_wgpu_composite_fingerprint")
                .ok()?,
            wgpu_present_used: *lib.get(b"syx_gui_dylib_wgpu_present_used").ok()?,
            wgpu_composite_used: *lib.get(b"syx_gui_dylib_wgpu_composite_used").ok()?,
            emoji_engine: *lib.get(b"syx_gui_dylib_emoji_engine").ok()?,
            doctor_engine_tag: *lib.get(b"syx_gui_dylib_doctor_engine_tag").ok()?,
            vertical_rl_layout_used: *lib.get(b"syx_gui_dylib_vertical_rl_layout_used").ok()?,
            doctor_present_mode: *lib.get(b"syx_gui_dylib_doctor_present_mode").ok()?,
            emoji_fingerprint: *lib.get(b"syx_gui_dylib_emoji_fingerprint").ok()?,
            vertical_rl_fingerprint: *lib.get(b"syx_gui_dylib_vertical_rl_fingerprint").ok()?,
            skia_glyph_fingerprint: *lib.get(b"syx_gui_dylib_skia_glyph_fingerprint").ok()?,
            skia_glyph_used: *lib.get(b"syx_gui_dylib_skia_glyph_used").ok()?,
            paragraph_paint_used: *lib.get(b"syx_gui_dylib_paragraph_paint_used").ok()?,
            vertical_ink_used: *lib.get(b"syx_gui_dylib_vertical_ink_used").ok()?,
            vertical_hb_vertical_used: *lib.get(b"syx_gui_dylib_vertical_hb_vertical_used").ok()?,
            egui_text_suppressed_used: *lib.get(b"syx_gui_dylib_egui_text_suppressed_used").ok()?,
            no_double_paint_probe: *lib.get(b"syx_gui_dylib_no_double_paint_probe").ok()?,
            emoji_atlas_hit: *lib.get(b"syx_gui_dylib_emoji_atlas_hit").ok()?,
            hb_paint_path_used: *lib.get(b"syx_gui_dylib_hb_paint_path_used").ok()?,
            font_mgr_fallback_used: *lib.get(b"syx_gui_dylib_font_mgr_fallback_used").ok()?,
            font_mgr_os_used: *lib.get(b"syx_gui_dylib_font_mgr_os_used").ok()?,
            font_mgr_bundled_used: *lib.get(b"syx_gui_dylib_font_mgr_bundled_used").ok()?,
            vertical_ascii_ink_used: *lib.get(b"syx_gui_dylib_vertical_ascii_ink_used").ok()?,
            colr_radial_or_sweep_used: *lib.get(b"syx_gui_dylib_colr_radial_or_sweep_used").ok()?,
            colr_radial_used: *lib.get(b"syx_gui_dylib_colr_radial_used").ok()?,
            colr_sweep_used: *lib.get(b"syx_gui_dylib_colr_sweep_used").ok()?,
            colr_grad_used: *lib.get(b"syx_gui_dylib_colr_grad_used").ok()?,
            doctor_widgets: *lib.get(b"syx_gui_dylib_doctor_widgets").ok()?,
            _lib: lib,
        })
    }
}

/// Attempt dlopen of `libsyx_gui`; returns whether a dylib was loaded.
pub fn ensure_loaded() -> bool {
    let mut guard = GUI_DYLIB.lock().unwrap();
    if guard.is_some() {
        return true;
    }
    for path in dylib_search_paths() {
        if path.exists() {
            if let Some(dylib) = load_dylib(&path) {
                eprintln!("syx-gui: loaded {}", path.display());
                *guard = Some(dylib);
                return true;
            }
            eprintln!(
                "syx-gui: found {} but failed to load symbols",
                path.display()
            );
        }
    }
    false
}

/// Phase 100: VISIBLE without dylib is a hard process failure (no silent headless).
pub fn require_dylib_if_visible() {
    if !std::env::var("SYX_GUI_VISIBLE")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        return;
    }
    if ensure_loaded() {
        return;
    }
    eprintln!(
        "syx-gui: SYX_GUI_VISIBLE=1 but libsyx_gui not found — hard fail (Phase 100).\n\
         Hint: cargo build -p syx-gui -p syx-cli\n\
         Or: export SYX_GUI_DYLIB=target/debug/libsyx_gui.dylib"
    );
    std::process::exit(2);
}

pub fn dlopen_loaded() -> bool {
    GUI_DYLIB.lock().unwrap().is_some()
}

pub fn dlopen_backend_name() -> &'static str {
    if let Some(dylib) = GUI_DYLIB.lock().unwrap().as_ref() {
        unsafe {
            let ptr = (dylib.backend_name)();
            if !ptr.is_null() {
                if let Ok(s) = CStr::from_ptr(ptr).to_str() {
                    return leak_str(s);
                }
            }
        }
    }
    "hashmap-dlopen-fallback"
}

fn leak_str(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

pub fn reset_dlopen_state() {
    if let Some(dylib) = GUI_DYLIB.lock().unwrap().as_ref() {
        unsafe {
            (dylib.reset)();
        }
    }
}

fn cstr_arg(s: &str) -> CString {
    CString::new(s).unwrap_or_default()
}

fn read_out_string(ptr: *mut c_char, free: FreeStringFn) -> String {
    if ptr.is_null() {
        return String::new();
    }
    unsafe {
        let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        (free)(ptr);
        s
    }
}

pub fn host_window_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let title = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        return Err(syx_diagnostics::SyxDiagnostic::error(
            "gui dlopen not loaded",
        ));
    };
    let c_title = cstr_arg(&title);
    let mut out: *mut c_char = std::ptr::null_mut();
    let ok = unsafe { (dylib.window_create)(c_title.as_ptr(), &mut out) };
    if !ok {
        stack.push(Value::Str("dlopen-window-fail".into()));
        return Ok(());
    }
    let id = read_out_string(out, dylib.free_string);
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_canvas_draw(stack: &mut Vec<Value>) -> SyxResult<()> {
    let color = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_color = cstr_arg(&color);
    let ok = unsafe { (dylib.canvas_draw)(c_win.as_ptr(), c_color.as_ptr()) };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_pixel_read(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::I32(0));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let px = unsafe { (dylib.pixel_read)(c_win.as_ptr()) };
    stack.push(Value::I32(px));
    Ok(())
}

pub fn host_run_loop_once(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Str("no-dlopen".into()));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let mut out: *mut c_char = std::ptr::null_mut();
    let _ = unsafe { (dylib.run_loop_once)(c_win.as_ptr(), &mut out) };
    let ev = read_out_string(out, dylib.free_string);
    stack.push(Value::Str(ev));
    Ok(())
}

pub fn host_event_pump(stack: &mut Vec<Value>) -> SyxResult<()> {
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Str("no-dlopen".into()));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let mut out: *mut c_char = std::ptr::null_mut();
    let _ = unsafe { (dylib.event_pump)(c_win.as_ptr(), &mut out) };
    let ev = read_out_string(out, dylib.free_string);
    stack.push(Value::Str(ev));
    Ok(())
}

pub fn visible_window_count_dlopen() -> usize {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.visible_count)() })
        .unwrap_or(0)
}

fn host_widget_create(stack: &mut Vec<Value>, f: WidgetCreateFn) -> SyxResult<()> {
    let text = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Str(String::new()));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_text = cstr_arg(&text);
    let mut out: *mut c_char = std::ptr::null_mut();
    let ok = unsafe { f(c_win.as_ptr(), c_text.as_ptr(), &mut out) };
    if !ok {
        stack.push(Value::Str(String::new()));
        return Ok(());
    }
    let id = read_out_string(out, dylib.free_string);
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_label_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let f = {
        let dylib = GUI_DYLIB.lock().unwrap();
        let Some(dylib) = dylib.as_ref() else {
            stack.push(Value::Str(String::new()));
            return Ok(());
        };
        dylib.label_create
    };
    host_widget_create(stack, f)
}

pub fn host_button_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let f = {
        let dylib = GUI_DYLIB.lock().unwrap();
        let Some(dylib) = dylib.as_ref() else {
            stack.push(Value::Str(String::new()));
            return Ok(());
        };
        dylib.button_create
    };
    host_widget_create(stack, f)
}

pub fn host_text_field_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let f = {
        let dylib = GUI_DYLIB.lock().unwrap();
        let Some(dylib) = dylib.as_ref() else {
            stack.push(Value::Str(String::new()));
            return Ok(());
        };
        dylib.text_field_create
    };
    host_widget_create(stack, f)
}

pub fn host_layout_create(stack: &mut Vec<Value>) -> SyxResult<()> {
    let axis = stack.pop().unwrap_or(Value::Str("VStack".into())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Str(String::new()));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_axis = cstr_arg(&axis);
    let mut out: *mut c_char = std::ptr::null_mut();
    let ok = unsafe { (dylib.layout_create)(c_win.as_ptr(), c_axis.as_ptr(), &mut out) };
    if !ok {
        stack.push(Value::Str(String::new()));
        return Ok(());
    }
    let id = read_out_string(out, dylib.free_string);
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_layout_configure(stack: &mut Vec<Value>) -> SyxResult<()> {
    let align = stack.pop().unwrap_or(Value::Str("top".into())).as_str();
    let spacing = stack.pop().unwrap_or(Value::I32(2)).as_i64().max(0) as i32;
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Str(String::new()));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_align = cstr_arg(&align);
    let mut out: *mut c_char = std::ptr::null_mut();
    let ok =
        unsafe { (dylib.layout_configure)(c_win.as_ptr(), spacing, c_align.as_ptr(), &mut out) };
    if !ok {
        stack.push(Value::Str(String::new()));
        return Ok(());
    }
    let id = read_out_string(out, dylib.free_string);
    stack.push(Value::Str(id));
    Ok(())
}

pub fn host_on_click_register(stack: &mut Vec<Value>) -> SyxResult<()> {
    let handler = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let btn = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_btn = cstr_arg(&btn);
    let c_handler = cstr_arg(&handler);
    let ok =
        unsafe { (dylib.on_click_register)(c_win.as_ptr(), c_btn.as_ptr(), c_handler.as_ptr()) };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_click_inject(stack: &mut Vec<Value>) -> SyxResult<()> {
    let btn = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_btn = cstr_arg(&btn);
    let ok = unsafe { (dylib.click_inject)(c_win.as_ptr(), c_btn.as_ptr()) };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_button_take_clicked(stack: &mut Vec<Value>) -> SyxResult<()> {
    let btn = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_btn = cstr_arg(&btn);
    let clicked = unsafe { (dylib.button_take_clicked)(c_win.as_ptr(), c_btn.as_ptr()) };
    stack.push(Value::Bool(clicked));
    Ok(())
}

pub fn host_widget_get_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    let widget = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Str(String::new()));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_widget = cstr_arg(&widget);
    let mut out: *mut c_char = std::ptr::null_mut();
    let ok = unsafe { (dylib.widget_get_text)(c_win.as_ptr(), c_widget.as_ptr(), &mut out) };
    if !ok {
        stack.push(Value::Str(String::new()));
        return Ok(());
    }
    let text = read_out_string(out, dylib.free_string);
    stack.push(Value::Str(text));
    Ok(())
}

pub fn host_widget_set_text(stack: &mut Vec<Value>) -> SyxResult<()> {
    let text = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let widget = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_widget = cstr_arg(&widget);
    let c_text = cstr_arg(&text);
    let ok = unsafe { (dylib.widget_set_text)(c_win.as_ptr(), c_widget.as_ptr(), c_text.as_ptr()) };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn host_key_inject(stack: &mut Vec<Value>) -> SyxResult<()> {
    let key = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let win = stack.pop().unwrap_or(Value::Str(String::new())).as_str();
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        stack.push(Value::Bool(false));
        return Ok(());
    };
    let c_win = cstr_arg(&win);
    let c_key = cstr_arg(&key);
    let ok = unsafe { (dylib.key_inject)(c_win.as_ptr(), c_key.as_ptr()) };
    stack.push(Value::Bool(ok));
    Ok(())
}

pub fn drain_pending_handlers(win_id: &str) -> Vec<String> {
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        return Vec::new();
    };
    let c_win = cstr_arg(win_id);
    let mut out: *mut c_char = std::ptr::null_mut();
    let ok = unsafe { (dylib.drain_pending)(c_win.as_ptr(), &mut out) };
    if !ok {
        return Vec::new();
    }
    let joined = read_out_string(out, dylib.free_string);
    if joined.is_empty() {
        Vec::new()
    } else {
        joined.split(',').map(str::to_string).collect()
    }
}

fn read_fingerprint(f: FingerprintFn) -> String {
    unsafe {
        let ptr = f();
        if ptr.is_null() {
            return String::new();
        }
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

pub fn dlopen_ttf_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.ttf_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_ime_preedit_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.ime_preedit_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_doctor_coverage_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.doctor_coverage_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_harfbuzz_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.harfbuzz_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_font_fallback_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.font_fallback_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_shaped() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.shaped)() })
        .unwrap_or(false)
}

pub fn dlopen_paint_shaped() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.paint_shaped)() })
        .unwrap_or(false)
}

pub fn dlopen_fallback_hits() -> u32 {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.fallback_hits)() })
        .unwrap_or(0)
}

pub fn dlopen_glyph_coverage_percent() -> u32 {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.glyph_coverage_percent)() })
        .unwrap_or(0)
}

pub fn dlopen_caret_metric_px() -> u32 {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.caret_metric_px)() })
        .unwrap_or(0)
}

pub fn dlopen_cjk_subset_char_count() -> u32 {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.cjk_subset_char_count)() })
        .unwrap_or(0)
}

pub fn dlopen_ime_preedit_supported() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.ime_preedit_supported)() })
        .unwrap_or(false)
}

pub fn dlopen_ime_commit_supported() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.ime_commit_supported)() })
        .unwrap_or(false)
}

pub fn dlopen_caret_supported() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.caret_supported)() })
        .unwrap_or(false)
}

pub fn dlopen_wgpu_composite_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.wgpu_composite_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_emoji_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.emoji_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_vertical_rl_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.vertical_rl_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_wgpu_present_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.wgpu_present_used)() })
        .unwrap_or(false)
}

pub fn dlopen_wgpu_composite_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.wgpu_composite_used)() })
        .unwrap_or(false)
}

pub fn dlopen_emoji_engine() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.emoji_engine))
        .unwrap_or_default()
}

pub fn dlopen_doctor_engine_tag() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.doctor_engine_tag))
        .unwrap_or_default()
}

pub fn dlopen_vertical_rl_layout_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.vertical_rl_layout_used)() })
        .unwrap_or(false)
}

pub fn dlopen_doctor_present_mode() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.doctor_present_mode))
        .unwrap_or_default()
}

pub fn dlopen_skia_glyph_fingerprint() -> String {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| read_fingerprint(d.skia_glyph_fingerprint))
        .unwrap_or_default()
}

pub fn dlopen_skia_glyph_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.skia_glyph_used)() })
        .unwrap_or(false)
}

pub fn dlopen_vertical_ink_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.vertical_ink_used)() })
        .unwrap_or(false)
}

pub fn dlopen_vertical_hb_vertical_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.vertical_hb_vertical_used)() })
        .unwrap_or(false)
}

pub fn dlopen_egui_text_suppressed_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.egui_text_suppressed_used)() })
        .unwrap_or(false)
}

pub fn dlopen_no_double_paint_probe() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.no_double_paint_probe)() })
        .unwrap_or(false)
}

pub fn dlopen_emoji_atlas_hit() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.emoji_atlas_hit)() })
        .unwrap_or(false)
}

pub fn dlopen_paragraph_paint_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.paragraph_paint_used)() })
        .unwrap_or(false)
}

pub fn dlopen_hb_paint_path_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.hb_paint_path_used)() })
        .unwrap_or(false)
}

pub fn dlopen_font_mgr_os_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.font_mgr_os_used)() })
        .unwrap_or(false)
}

pub fn dlopen_font_mgr_bundled_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.font_mgr_bundled_used)() })
        .unwrap_or(false)
}

pub fn dlopen_vertical_ascii_ink_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.vertical_ascii_ink_used)() })
        .unwrap_or(false)
}

pub fn dlopen_font_mgr_fallback_used() -> bool {
    dlopen_font_mgr_os_used() || dlopen_font_mgr_bundled_used()
}

pub fn dlopen_colr_radial_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.colr_radial_used)() })
        .unwrap_or(false)
}

pub fn dlopen_colr_sweep_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.colr_sweep_used)() })
        .unwrap_or(false)
}

pub fn dlopen_doctor_widgets() -> String {
    let dylib = GUI_DYLIB.lock().unwrap();
    let Some(dylib) = dylib.as_ref() else {
        return String::new();
    };
    let mut out: *mut c_char = std::ptr::null_mut();
    let ok = unsafe { (dylib.doctor_widgets)(&mut out) };
    if !ok {
        return String::new();
    }
    read_out_string(out, dylib.free_string)
}
pub fn dlopen_colr_radial_or_sweep_used() -> bool {
    dlopen_colr_radial_used() && dlopen_colr_sweep_used()
}

pub fn dlopen_colr_grad_used() -> bool {
    GUI_DYLIB
        .lock()
        .unwrap()
        .as_ref()
        .map(|d| unsafe { (d.colr_grad_used)() })
        .unwrap_or(false)
}
