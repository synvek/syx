//! Phase 117 Track A — all-widgets ink + OS-only FontMgr + font COLR Sweep + OT calt + doctor v5.

use syx_driver::{compile_and_run_embedded, DriverOptions};
use syx_vm::{ensure_gui_loaded, gui_dlopen_loaded, reset_phase98_state};

#[test]
fn p117_gui_fingerprints_in_tree() {
    let gui_lib = include_str!("../../../syx-gui/src/lib.rs");
    let egui = include_str!("../../../syx-gui/src/egui_backend.rs");
    let emoji = include_str!("../../../syx-gui/src/emoji.rs");
    let skia_layout = include_str!("../../../syx-gui/src/skia_layout.rs");
    let shape = include_str!("../../../syx-gui/src/shape.rs");
    let dylib = include_str!("../../../syx-gui/src/dylib_api.rs");
    let cargo = include_str!("../../../syx-gui/Cargo.toml");
    assert!(gui_lib.contains("gui-skia-paragraph-v5:all-widgets-ink"));
    assert!(gui_lib.contains("gui-emoji-v9:font-radial-and-sweep"));
    assert!(gui_lib.contains("gui-font-system-v4:os-only-ga"));
    assert!(gui_lib.contains("gui-ot-features-v1:kern-liga-calt"));
    assert!(gui_lib.contains("gui-doctor-engine-v5:phase117"));
    assert!(egui.contains("put_cpu_ink_button"));
    assert!(egui.contains("put_cpu_ink_label"));
    assert!(egui.contains("egui_text_suppressed_used"));
    assert!(egui.contains("no_double_paint_probe"));
    assert!(egui.contains("mark_vertical_paragraph_used"));
    assert!(!egui.contains("Button::new(texts[i].as_str())"));
    assert!(!egui.contains("blit_glyph5x7"));
    assert!(emoji.contains("COLR_SWEEP_DOCTOR"));
    assert!(emoji.contains("COLR_RADIAL_DOCTOR"));
    assert!(emoji.contains("fill_sweep_gradient"));
    assert!(!emoji.contains("prove_colr_sweep_font_paint_tree"));
    assert!(skia_layout.contains("include_test=false") || skia_layout.contains(", false)?"));
    assert!(skia_layout.contains("probe_font_mgr_os_paint"));
    assert!(skia_layout.contains("font-mgr-bundled") || skia_layout.contains("font_mgr_bundled"));
    assert!(skia_layout.contains("vertical_paragraph_used"));
    assert!(shape.contains("gui-ot-features-v1:kern-liga-calt"));
    assert!(shape.contains(r#"b"calt""#));
    assert!(dylib.contains("gui-doctor-engine-v5:phase117"));
    assert!(dylib.contains("gui-ot-features-v1:kern-liga-calt"));
    assert!(dylib.contains("syx_gui_dylib_ot_calt_used"));
    assert!(cargo.contains("textlayout"));
    let blit = gui_lib
        .split("fn blit_text")
        .nth(1)
        .and_then(|s| s.split("\nfn ").next())
        .unwrap_or("");
    assert!(!blit.contains("blit_glyph5x7"));
    assert!(!blit.contains("glyph5x7("));
}

#[test]
fn p117_gui_doctor_backend_report_engine() {
    use syx_driver::gui_backend_report::run_gui_backend_report;
    reset_phase98_state();
    let report_fresh = run_gui_backend_report();
    assert_eq!(report_fresh.phase, 117);

    ensure_gui_loaded();
    let report = run_gui_backend_report();
    if gui_dlopen_loaded() {
        assert_eq!(
            report.doctor_coverage_fingerprint,
            "gui-doctor-engine-v5:phase117"
        );
        assert_eq!(
            report.emoji_fingerprint,
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
