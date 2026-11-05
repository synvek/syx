//! Phase 117 Track A — all-widgets ink + OS-only FontMgr + font COLR Sweep + OT calt + doctor v5.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{
    ensure_gui_loaded, gui_dlopen_loaded, gui_doctor_coverage_digest, gui_emoji_digest,
    reset_phase98_state,
};

#[test]
fn p117_gui_doctor_backend_report_engine() {
    reset_phase98_state();
    ensure_gui_loaded();
    if gui_dlopen_loaded() {
        assert_eq!(
            gui_doctor_coverage_digest(),
            "gui-doctor-engine-v5:phase117"
        );
        assert_eq!(
            gui_emoji_digest(),
            "gui-emoji-v9:font-radial-and-sweep"
        );
    }
}

#[ignore = "Phase 124: syx-gui string-id API retired; use SynUI"]
#[test]
fn p117_gui_all_widgets_ink_no_double_paint() {
    reset_phase98_state();
    ensure_gui_loaded();
    if !gui_dlopen_loaded() {
        eprintln!("skip p117 all-widgets-ink: libsyx_gui not built");
        return;
    }
    let source = r#"
func main() {
    let win = __syx_gui_winit_window_create("p117-all")
    __syx_gui_label_create(win, "XYZ")
    __syx_gui_button_create(win, "OK")
}
"#;
    let _ = compile_and_run_embedded(source, &DriverOptions::default())
        .expect("p117 all-widgets-ink compile-run");
}
