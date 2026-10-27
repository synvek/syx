//! Phase 123 Track B — OS FontMgr default-paint OT (historical).
//! Phase 124: ot-v7 is **retired** as product GA; SynUI fingerprints are `gui-synui-v1:*`.
//! These tests only assert the syx-gui source still contains the former OT proof code.

#[test]
fn p123_gui_ot_v7_source_present_but_retired_as_ga() {
    let gui_lib = include_str!("../../../syx-gui/src/lib.rs");
    let shape = include_str!("../../../syx-gui/src/shape.rs");
    let dylib = include_str!("../../../syx-gui/src/dylib_api.rs");
    assert!(gui_lib.contains("gui-ot-features-v7:os-fontmgr-default-paint"));
    assert!(shape.contains("prove_ot_default_paint"));
    assert!(dylib.contains("gui-ot-features-v7:os-fontmgr-default-paint"));
    // Honesty: ot-v7 must not be claimed as Phase 124 SynUI GA.
    assert_ne!(
        "gui-ot-features-v7:os-fontmgr-default-paint",
        "gui-synui-v1:host-bridge"
    );
}

#[test]
fn p123_gui_os_fontmgr_default_paint_historical() {
    let shape = include_str!("../../../syx-gui/src/shape.rs");
    assert!(shape.contains("prove_ot_default_paint"));
    assert!(shape.contains("OrderedFontMgr"));
    assert!(shape.contains("FontMgr::default"));
}

#[test]
fn p123_gui_hook_only_not_synui_ga() {
    let shape = include_str!("../../../syx-gui/src/shape.rs");
    assert!(shape.contains("DEFAULT_PAINT_OT_HOOK"));
    assert!(shape.contains("resolve_default_paint_ot_ttf_via_fontmgr"));
}

#[test]
fn p123_gui_dylib_ot_export_historical() {
    let dylib = include_str!("../../../syx-gui/src/dylib_api.rs");
    assert!(dylib.contains("syx_gui_dylib_prove_ot_default_paint"));
    assert!(dylib.contains("syx_gui_dylib_default_paint_fontmgr_used"));
}
