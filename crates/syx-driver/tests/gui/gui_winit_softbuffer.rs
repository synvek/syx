//! Phase 98: winit + softbuffer pixel readback headless gate.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_winit_softbuffer_pixel_readback() {
    syx_vm::ensure_gui_loaded();
    let out = run(
        r#"func main() {
    let win = __syx_gui_winit_window_create("p97")
    __syx_gui_winit_canvas_draw(win, "red")
    print(__syx_gui_pixel_read(win))
}"#,
    );
    let px: i64 = out.last().and_then(|l| l.parse().ok()).unwrap_or(0);
    assert_eq!(px as u32, 4278190335, "red pixel ARGB: {out:?}");
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_winit_softbuffer_blue_pixel_dlopen() {
    syx_vm::ensure_gui_loaded();
    let out = run(
        r#"func main() {
    let win = __syx_gui_winit_window_create("p97-blue")
    __syx_gui_winit_canvas_draw(win, "blue")
    print(__syx_gui_pixel_read(win))
}"#,
    );
    let px: i64 = out.last().and_then(|l| l.parse().ok()).unwrap_or(0);
    assert_eq!(px as u32, 4294901760, "blue pixel ARGB: {out:?}");
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p98_gui_winit_softbuffer_window_create() {
    let out = run(
        r#"func main() {
    let win = __syx_gui_winit_window_create("headless")
    print(win)
}"#,
    );
    assert!(out.iter().any(|l| l.contains("winit")), "{out:?}");
}

#[test]
fn p98_gui_winit_softbuffer_hosts_module() {
    assert!(include_str!("../../../syx-vm/src/lib.rs").contains("stdlib_phase98_hosts"));
}
