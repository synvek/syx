//! Phase 91: winit headless GUI smoke.

use syx_driver::{compile_and_run_embedded, DriverOptions};

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_winit_window_event_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    let w = __syx_gui_winit_window_create("P90")
    let e = __syx_gui_event_pump(w)
    print(e)
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains("pump")), "{out:?}");
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_winit_canvas_draw_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    let w = __syx_gui_winit_window_create("draw")
    print(__syx_gui_winit_canvas_draw(w, "blue"))
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains("true")), "{out:?}");
}

#[test]
fn p98_gui_winit_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gui-winit-real-v2"));
}

#[test]
fn p98_gui_winit_package_surface() {
    // Phase 124: packages/ui is SynUI-shaped; legacy asserts retired.
    let ui = include_str!("../../../../packages/ui/mod.sy");
    assert!(ui.contains("__syx_synui_button"));
    assert!(ui.contains("__syx_synui_app_create"));
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_winit_button_onclick_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let out = compile_and_run_embedded(
        r#"func main() {
    let w = __syx_gui_winit_window_create("btn")
    let b = __syx_gui_button_create(w, "ok")
    print(__syx_gui_on_click(w, b, "handler"))
}"#,
        &DriverOptions::default(),
    )
    .unwrap()
    .printed;
    assert!(out.iter().any(|l| l.contains("true")), "{out:?}");
}
