//! Phase 91: GUI window/event loop loopback MVP.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::reset_phase98_state;

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_gui_mvp_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gui-winit-real-v2"));
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_mvp_window_event_stdout() {
    reset_phase98_state();
    let out = run(
        r#"func main() {
    let win = __syx_gui_window_create("SYX")
    print(win)
    let ev = __syx_gui_event_pump(win)
    print(ev)
    let ok = __syx_gui_canvas_draw(win, "black")
    print(ok)
}"#,
    );
    assert!(
        (out.contains("win:") || out.contains("winit-"))
            && (out.contains("event:") || out.contains("pump")),
        "{out}"
    );
}

#[test]
fn p98_gui_mvp_package_surface() {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../packages/ui/mod.sy");
    assert!(p.is_file(), "missing packages/ui/mod.sy");
}
