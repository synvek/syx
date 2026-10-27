//! Phase 98: mobile touch GUI MVP smoke.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_mobile_touch_event_smoke() {
    let out = run(
        r#"func main() {
    let win = __syx_gui_mobile_window_create("mobile")
    let evt = __syx_gui_touch_event(win, 10, 20)
    print(evt)
}"#,
    );
    assert!(out.iter().any(|l| l.contains("touch")), "{out:?}");
}

#[test]
fn p98_gui_mobile_mvp_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gui-mobile-mvp-v1"));
}

#[test]
fn p98_gui_mobile_host_registry() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("GuiTouchEvent") && reg.contains("GuiMobileWindowCreate"));
}

#[test]
fn p98_gui_mobile_package_surface() {
    assert!(include_str!("../../../../packages/ui/mobile.sy").contains("touch"));
}
