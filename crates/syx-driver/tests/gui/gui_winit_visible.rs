//! Phase 102: GUI callback GA gate (invoke onClick + TextField keyboard + HStack).

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{
    ensure_gui_loaded, gui_dlopen_loaded, gui_sample_center_pixel, host_gui_button_create,
    host_gui_click_inject, host_gui_key_inject, host_gui_label_create, host_gui_layout_create,
    host_gui_on_click, host_gui_text_field_create, host_gui_widget_get_text,
    host_gui_winit_event_pump_phase97, host_gui_winit_run_loop_once_phase97,
    host_gui_winit_window_create, reset_phase98_state, visible_window_count,
};
use syx_runtime_core::Value;

#[test]
fn p102_gui_callback_ga_fingerprint() {
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-callback-ga-v1:invoke-onclick-textfield"));
    assert!(include_str!("../../../syx-vm/src/gui_invoke.rs").contains("invoke_gui_handler"));
    assert!(include_str!("../../../syx-gui/src/dylib_api.rs")
        .contains("syx_gui_dylib_drain_pending_handlers"));
    assert!(include_str!("../../../syx-gui/src/dylib_api.rs").contains("syx_gui_dylib_key_inject"));
    assert!(include_str!("../../../syx-gui/src/dylib_api.rs")
        .contains("syx_gui_dylib_widget_get_text"));
}

#[test]
fn p100_gui_winit_visible_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gui-visible-dylib-hard-v1:no-silent-fallback"));
}

#[test]
fn p100_visible_headless_widgets_label_text() {
    reset_phase98_state();
    std::env::remove_var("SYX_GUI_VISIBLE");
    std::env::set_var("SYX_GUI_HOLD", "0");
    ensure_gui_loaded();
    let mut stack = vec![Value::Str("SYX Demo".into())];
    host_gui_winit_window_create(&mut stack).unwrap();
    assert!(
        visible_window_count() >= 1,
        "window_count must be ≥1, got {}",
        visible_window_count()
    );
    let win = stack.pop().unwrap().as_str();
    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("blue".into()));
    syx_vm::host_gui_winit_canvas_draw_phase97(&mut stack).unwrap();
    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("Hello".into()));
    host_gui_label_create(&mut stack).unwrap();
    let label = stack.pop().unwrap().as_str();
    assert!(
        label.contains("Hello"),
        "label create must return text containing Hello, got {label:?}"
    );
    let px = gui_sample_center_pixel(&win);
    assert!(px != 0, "center pixel must be non-zero after draw+label, got {px}");
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p102_click_invoke_handler_side_effect() {
    reset_phase98_state();
    std::env::remove_var("SYX_GUI_VISIBLE");
    std::env::set_var("SYX_GUI_HOLD", "0");
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        eprintln!("skip p102_click_invoke: libsyx_gui not built (cargo build -p syx-gui)");
        return;
    }

    let source = r#"
func onBtnClick() {
    print("P102_HANDLER_RAN")
}
func main() {
    let win = __syx_gui_winit_window_create("p102-click")
    let btn = __syx_gui_button_create(win, "Go")
    __syx_gui_on_click(win, btn, "onBtnClick")
    __syx_gui_click_inject(win, btn)
    print(__syx_gui_run_loop_once(win))
}
"#;
    let printed = compile_and_run_embedded(source, &DriverOptions::default())
        .expect("p102 click invoke compile-run")
        .printed;
    assert!(
        printed.iter().any(|l| l.contains("P102_HANDLER_RAN")),
        "handler side effect must appear in stdout: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("invoked:onBtnClick")),
        "pump must report invoked handler, not click: string: {printed:?}"
    );
    assert!(
        !printed.iter().any(|l| l.contains("click:onBtnClick")),
        "click:handler soft contract must be gone: {printed:?}"
    );
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p102_textfield_key_inject_gettext_exact() {
    reset_phase98_state();
    std::env::remove_var("SYX_GUI_VISIBLE");
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        eprintln!("skip p102_textfield: libsyx_gui not built");
        return;
    }

    let source = r#"
func main() {
    let win = __syx_gui_winit_window_create("p102-field")
    let field = __syx_gui_text_field_create(win, "Type here...")
    __syx_gui_key_inject(win, "a")
    __syx_gui_key_inject(win, "b")
    __syx_gui_key_inject(win, "c")
    print(__syx_gui_widget_get_text(win, field))
}
"#;
    let printed = compile_and_run_embedded(source, &DriverOptions::default())
        .expect("p102 textfield compile-run")
        .printed;
    assert!(
        printed.iter().any(|l| l == "abc"),
        "getText must return exact injected keys, got {printed:?}"
    );
}

#[test]
fn p102_hstack_layout_id() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        eprintln!("skip p102_hstack: libsyx_gui not built");
        return;
    }

    let mut stack = vec![Value::Str("hstack-test".into())];
    host_gui_winit_window_create(&mut stack).unwrap();
    let win = stack.pop().unwrap().as_str();

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("HStack".into()));
    host_gui_layout_create(&mut stack).unwrap();
    let layout = stack.pop().unwrap().as_str();
    assert!(
        layout.contains("hstack"),
        "HStack layout must return hstack id, got {layout:?}"
    );
}

#[test]
fn p102_click_inject_pump_invokes_not_click_string() {
    reset_phase98_state();
    std::env::remove_var("SYX_GUI_VISIBLE");
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        return;
    }

    let mut stack = vec![Value::Str("click-test".into())];
    host_gui_winit_window_create(&mut stack).unwrap();
    let win = stack.pop().unwrap().as_str();

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("VStack".into()));
    host_gui_layout_create(&mut stack).unwrap();

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("Go".into()));
    host_gui_button_create(&mut stack).unwrap();
    let btn = stack.pop().unwrap().as_str();

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str(btn.clone()));
    stack.push(Value::Str("myHandler".into()));
    host_gui_on_click(&mut stack).unwrap();
    assert!(stack.pop().unwrap().as_bool());

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str(btn.clone()));
    host_gui_click_inject(&mut stack).unwrap();
    assert!(stack.pop().unwrap().as_bool());

    stack.push(Value::Str(win.clone()));
    host_gui_winit_run_loop_once_phase97(&mut stack).unwrap();
    let tick = stack.pop().unwrap().as_str();
    assert!(
        !tick.contains("click:myHandler"),
        "must not return click:handler soft string, got {tick:?}"
    );

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str(btn.clone()));
    host_gui_click_inject(&mut stack).unwrap();
    stack.push(Value::Str(win.clone()));
    host_gui_winit_event_pump_phase97(&mut stack).unwrap();
    let pump = stack.pop().unwrap().as_str();
    assert!(
        !pump.contains("click:myHandler"),
        "event_pump must not return click:handler, got {pump:?}"
    );
}

#[test]
fn p102_textfield_host_gettext_after_inject() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        return;
    }

    let mut stack = vec![Value::Str("tf-host".into())];
    host_gui_winit_window_create(&mut stack).unwrap();
    let win = stack.pop().unwrap().as_str();

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("placeholder".into()));
    host_gui_text_field_create(&mut stack).unwrap();
    let field = stack.pop().unwrap().as_str();

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("x".into()));
    host_gui_key_inject(&mut stack).unwrap();
    assert!(stack.pop().unwrap().as_bool());

    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str(field.clone()));
    host_gui_widget_get_text(&mut stack).unwrap();
    assert_eq!(stack.pop().unwrap().as_str(), "x");
}

#[test]
fn p100_gui_winit_softbuffer_hosts_module() {
    assert!(
        include_str!("../../../syx-gui/src/lib.rs").contains("GLYPH5X7_TABLE")
            || include_str!("../../../syx-gui/src/ascii_font.rs").contains("GLYPH5X7_TABLE")
    );
}

#[test]
fn p100_gui_headless_red_pixel_numeric() {
    reset_phase98_state();
    std::env::remove_var("SYX_GUI_VISIBLE");
    ensure_gui_loaded();
    let mut stack = vec![Value::Str("px-test".into())];
    host_gui_winit_window_create(&mut stack).unwrap();
    let win = stack.pop().unwrap().as_str();
    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("red".into()));
    syx_vm::host_gui_winit_canvas_draw_phase97(&mut stack).unwrap();
    let px = gui_sample_center_pixel(&win);
    assert!(
        (px as u32).abs_diff(4278190335) <= 1,
        "center pixel must be red ARGB ±1: {px}"
    );
}

#[test]
fn p100_gui_require_dylib_source() {
    assert!(include_str!("../../../syx-vm/src/gui_dlopen.rs").contains("require_dylib_if_visible"));
}

#[test]
fn p103_gui_focus_caret_fingerprint() {
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-textfield-focus-v1:caret-tab-cycle"));
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-hstack-ga-v1:spacing-align"));
    assert!(include_str!("../../../syx-vm/src/gui_invoke.rs")
        .contains("gui-onclick-hard-v1:no-swallow-err"));
    assert!(include_str!("../../src/lib.rs").contains("heap_v75_phase103_track_a_fingerprint"));
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p103_textfield_tab_cycle_embedded() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        eprintln!("skip p103_tab: libsyx_gui not built");
        return;
    }
    let source = r#"
func main() {
    let win = __syx_gui_winit_window_create("p103-tab")
    let f1 = __syx_gui_text_field_create(win, "one")
    let btn = __syx_gui_button_create(win, "Mid")
    let f2 = __syx_gui_text_field_create(win, "two")
    __syx_gui_key_inject(win, "Shift-Tab")
    __syx_gui_key_inject(win, "Shift-Tab")
    __syx_gui_key_inject(win, "a")
    __syx_gui_key_inject(win, "Tab")
    __syx_gui_key_inject(win, "Tab")
    __syx_gui_key_inject(win, "b")
    print(__syx_gui_widget_get_text(win, f1))
    print(__syx_gui_widget_get_text(win, f2))
}
"#;
    let printed = compile_and_run_embedded(source, &DriverOptions::default())
        .expect("p103 tab compile-run")
        .printed;
    assert!(printed.iter().any(|l| l == "a"), "f1 text: {printed:?}");
    assert!(printed.iter().any(|l| l == "b"), "f2 text: {printed:?}");
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p103_textfield_caret_edit_embedded() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        return;
    }
    let source = r#"
func main() {
    let win = __syx_gui_winit_window_create("p103-caret")
    let field = __syx_gui_text_field_create(win, "x")
    __syx_gui_key_inject(win, "a")
    __syx_gui_key_inject(win, "b")
    __syx_gui_key_inject(win, "c")
    __syx_gui_key_inject(win, "Left")
    __syx_gui_key_inject(win, "Backspace")
    print(__syx_gui_widget_get_text(win, field))
}
"#;
    let printed = compile_and_run_embedded(source, &DriverOptions::default())
        .expect("p103 caret compile-run")
        .printed;
    assert!(
        printed.iter().any(|l| l == "ac"),
        "caret backspace must yield ac: {printed:?}"
    );
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p103_onclick_invoke_error_visible() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        return;
    }
    let source = r#"
func main() {
    let win = __syx_gui_winit_window_create("p103-err")
    let btn = __syx_gui_button_create(win, "Go")
    __syx_gui_on_click(win, btn, "missingHandlerP103")
    __syx_gui_click_inject(win, btn)
    print(__syx_gui_run_loop_once(win))
}
"#;
    let printed = compile_and_run_embedded(source, &DriverOptions::default())
        .expect("p103 invoke err compile-run")
        .printed;
    assert!(
        printed.iter().any(|l| l.starts_with("error:")),
        "invoke failure must print error: prefix, got {printed:?}"
    );
    assert!(
        !printed.iter().any(|l| l == "tick"),
        "must not swallow to default tick: {printed:?}"
    );
}

#[test]
fn p103_hstack_spacing_layout_host() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        return;
    }
    let mut stack = vec![Value::Str("p103-hs".into())];
    host_gui_winit_window_create(&mut stack).unwrap();
    let win = stack.pop().unwrap().as_str();
    stack.push(Value::Str(win.clone()));
    stack.push(Value::Str("HStack".into()));
    host_gui_layout_create(&mut stack).unwrap();
    stack.push(Value::Str(win.clone()));
    stack.push(Value::I32(20));
    stack.push(Value::Str("center".into()));
    syx_vm::host_gui_layout_configure(&mut stack).unwrap();
    let tag = stack.pop().unwrap().as_str();
    assert!(tag.contains("spacing-align:20:center"), "tag={tag:?}");
}

#[test]
fn p104_gui_cjk_ime_fingerprint() {
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-cjk-ime-v1:unicode-caret-ime"));
    assert!(include_str!("../../../syx-gui/src/lib.rs")
        .contains("gui-ttf-expand-v1:cjk64-plus"));
    assert!(include_str!("../../../syx-gui/src/lib.rs").contains("WindowEvent::Ime"));
    assert!(include_str!("../../../syx-gui/src/lib.rs").contains("Ime::Preedit"));
    assert!(include_str!("../../../syx-gui/src/lib.rs").contains("Ime::Commit"));
}

#[test]
fn p106_gui_doctor_backend_report_subset_chars() {
    use syx_driver::gui_backend_report::run_gui_backend_report;
    reset_phase98_state();
    ensure_gui_loaded();
    let report = run_gui_backend_report();
    assert!(report.phase == 106 || report.phase == 107 || report.phase == 108);
    assert_eq!(report.ttf_fingerprint, "gui-ttf-expand-v1:cjk64-plus");
    assert_eq!(
        report.ime_preedit_fingerprint,
        "gui-ime-highlight-v1:preedit-invert"
    );
    assert!(
        report.doctor_coverage_fingerprint == "gui-doctor-cap-v1:phase106"
            || report.doctor_coverage_fingerprint == "gui-doctor-real-v1:phase107"
            || report.doctor_coverage_fingerprint == "gui-doctor-egui-v1:phase108"
            || report.doctor_coverage_fingerprint == "gui-doctor-wgpu-v1:phase109"
    );
    if gui_dlopen_loaded() {
        assert!(report.subset_chars >= 64);
        assert!(report.glyph_coverage_percent >= 100);
        assert!(report.ime_preedit_supported);
        assert!(report.ime_commit_supported);
        assert!(report.caret_supported);
        assert!(report.caret_metric_px > 0);
    }
}

#[test]
fn p105_gui_doctor_backend_report_coverage() {
    use syx_driver::gui_backend_report::run_gui_backend_report;
    reset_phase98_state();
    ensure_gui_loaded();
    let report = run_gui_backend_report();
    assert!(report.phase == 106 || report.phase == 107 || report.phase == 108);
    assert_eq!(report.ttf_fingerprint, "gui-ttf-expand-v1:cjk64-plus");
    assert_eq!(
        report.ime_preedit_fingerprint,
        "gui-ime-highlight-v1:preedit-invert"
    );
    assert!(
        report.doctor_coverage_fingerprint == "gui-doctor-cap-v1:phase106"
            || report.doctor_coverage_fingerprint == "gui-doctor-real-v1:phase107"
            || report.doctor_coverage_fingerprint == "gui-doctor-egui-v1:phase108"
            || report.doctor_coverage_fingerprint == "gui-doctor-wgpu-v1:phase109"
    );
    if gui_dlopen_loaded() {
        assert!(report.glyph_coverage_percent >= 100);
        assert!(report.caret_metric_px > 0);
    }
}

#[test]
fn p107_gui_doctor_backend_report_shaped() {
    use syx_driver::gui_backend_report::run_gui_backend_report;
    reset_phase98_state();
    ensure_gui_loaded();
    let report = run_gui_backend_report();
    assert!(report.phase == 107 || report.phase == 108 || report.phase == 109);
    assert!(
        report.harfbuzz_fingerprint == "gui-harfbuzz-v1:shape-mvp"
            || report.harfbuzz_fingerprint == "gui-hb-paint-v1:string-shape"
    );
    assert_eq!(
        report.font_fallback_fingerprint,
        "gui-font-fallback-v1:system-cjk"
    );
    assert!(
        report.doctor_coverage_fingerprint == "gui-doctor-real-v1:phase107"
            || report.doctor_coverage_fingerprint == "gui-doctor-egui-v1:phase108"
            || report.doctor_coverage_fingerprint == "gui-doctor-wgpu-v1:phase109"
    );
    if gui_dlopen_loaded() {
        assert!(report.shaped, "doctor must report shaped=true");
        assert!(report.subset_chars >= 64);
        assert!(report.ime_preedit_supported);
        assert!(report.caret_metric_px > 0);
        assert_ne!(
            report.caret_metric_px, 10,
            "caret must be metrics-derived, not hardcoded 10"
        );
    }
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p104_cjk_textfield_unicode_embedded() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        eprintln!("skip p104_cjk: libsyx_gui not built");
        return;
    }
    let source = r#"
func main() {
    let win = __syx_gui_winit_window_create("p104-cjk")
    let field = __syx_gui_text_field_create(win, "placeholder")
    __syx_gui_key_inject(win, "你")
    print(__syx_gui_widget_get_text(win, field))
}
"#;
    let printed = compile_and_run_embedded(source, &DriverOptions::default())
        .expect("p104 cjk compile-run")
        .printed;
    assert!(
        printed.iter().any(|l| l == "你"),
        "unicode inject must round-trip getText: {printed:?}"
    );
}
