//! Hosted product met exit charter
//! Policy + meta flip only — scope narrow; marketing full site remains unmet.

#[test]
fn hosted_site_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/hosted-site-exit.md",
        "docs/lang/hosted-site-vectors.md",
        "crates/syx-driver/src/hosted_capabilities.rs",
        "crates/syx-driver/tests/stdlib/hosted_site_thin.rs",
        "crates/syx-driver/tests/stdlib/hosted_site_deepen.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "hosted exit requires evidence file: {path}"
        );
    }
    let helper = include_str!("../../src/hosted_capabilities.rs");
    assert!(
        helper.contains("fn capabilities_for_hosted")
            && helper.contains("run_capabilities_report"),
        "exit evidence must retain W380 deepen helper"
    );
}

#[test]
fn hosted_site_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/hosted-site-exit.md");
    assert!(
        spec.contains("ADR-725")
            && (spec.contains("exit signed") || spec.contains("Exit signed")),
        "spec must cite ADR-725 exit signed"
    );
    assert!(
        spec.contains("scope narrow") || spec.contains("Hosted scope narrow"),
        "spec must cite hosted scope narrow"
    );
    assert!(
        spec.contains("W379")
            && spec.contains("W380")
            && (spec.contains("partial met") || spec.contains("sufficient")),
        "spec must index inventory + deepen partial-sufficient prerequisites"
    );
    assert!(
        (spec.contains("Hosted full site") || spec.contains("marketing") || spec.contains("deploy"))
            && (spec.contains("unmet") || spec.contains("defer")),
        "spec must keep marketing / Hosted full site deferred unmet"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-04") || spec.contains("hosted") || spec.contains("deploy")),
        "spec Frozen must keep hosted-site / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

