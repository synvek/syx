//! Website product met exit charter
//! Policy + meta flip only — scope narrow; hosted full site remains unmet.

#[test]
fn website_capability_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/website-capability-exit.md",
        "docs/lang/website-capability-vectors.md",
        "crates/syx-driver/src/website_capabilities.rs",
        "crates/syx-driver/tests/stdlib/website_capability_thin.rs",
        "crates/syx-driver/tests/stdlib/website_capability_deepen.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "website exit requires evidence file: {path}"
        );
    }
    let helper = include_str!("../../src/website_capabilities.rs");
    assert!(
        helper.contains("fn capabilities_for_website")
            && helper.contains("run_capabilities_report"),
        "exit evidence must retain W377 deepen helper"
    );
}

#[test]
fn website_capability_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/website-capability-exit.md");
    assert!(
        spec.contains("ADR-722")
            && (spec.contains("exit signed") || spec.contains("Exit signed")),
        "spec must cite ADR-722 exit signed"
    );
    assert!(
        spec.contains("scope narrow") || spec.contains("Website scope narrow"),
        "spec must cite website scope narrow"
    );
    assert!(
        spec.contains("W376")
            && spec.contains("W377")
            && (spec.contains("partial met") || spec.contains("sufficient")),
        "spec must index inventory + deepen partial-sufficient prerequisites"
    );
    assert!(
        (spec.contains("Hosted full site") || spec.contains("hosted") || spec.contains("deploy"))
            && (spec.contains("unmet") || spec.contains("defer")),
        "spec must keep hosted full site deferred unmet"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-04") || spec.contains("hosted") || spec.contains("deploy")),
        "spec Frozen must keep hosted-site / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

