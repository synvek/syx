//! Hosted same-source deepen
//! Locks helper+fixture nail; Hosted full site remains unmet; bans marketing deploy / B `[x]`.

#[test]
fn hosted_site_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/hosted-site-vectors.md",
        "docs/lang/hosted-site-exit.md",
        "docs/lang/website-capability-vectors.md",
        "crates/syx-driver/src/hosted_capabilities.rs",
        "packages/internal/capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "hosted deepen requires evidence file: {path}"
        );
    }
    let helper = include_str!("../../src/hosted_capabilities.rs");
    assert!(
        helper.contains("fn capabilities_for_hosted")
            && helper.contains("fn render_hosted_capabilities_page")
            && helper.contains("fn write_hosted_capabilities_fixture")
            && helper.contains("run_capabilities_report")
            && !helper.contains("capabilities.generated.json"),
        "helper must call run_capabilities_report without a second JSON embed"
    );
}

#[test]
fn hosted_site_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/lang/hosted-site-vectors.md");
    assert!(
        spec.contains("ADR-724")
            && (spec.contains("deepen") || spec.contains("partial")),
        "spec must cite ADR-724 hosted deepen"
    );
    assert!(
        spec.contains("Deployable same-source hosted consumer")
            && (spec.contains("**met** (W381") || spec.contains("ADR-725"))
            && (spec.contains("hosted_capabilities")
                || spec.contains("capabilities_for_hosted")
                || spec.contains("render_hosted_capabilities_page")),
        "spec must index Deployable consumer met (narrow) with helper evidence"
    );
    assert!(
        !spec.contains("| **Deployable same-source hosted consumer** | **unmet** |")
            && !spec.contains("| **Deployable same-source hosted consumer** | **partial met** (W380) |"),
        "Deployable consumer must be product-met (narrow) after W381"
    );
    assert!(
        spec.contains("Hosted full site") && spec.contains("**unmet**"),
        "hosted full site must remain unmet"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("hosted") || spec.contains("deploy") || spec.contains("B-04")),
        "spec Frozen must keep hosted-site / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
}

#[test]
fn hosted_site_deepen_nail_behavior() {
    use syx_driver::capabilities_report::run_capabilities_report;
    use syx_driver::hosted_capabilities::{
        capabilities_for_hosted, render_hosted_capabilities_page, write_hosted_capabilities_fixture,
    };

    let summary = capabilities_for_hosted(None);
    let report = run_capabilities_report();
    assert_eq!(summary["schema"], "syx-capability-v0");
    assert_eq!(
        summary["count"].as_u64().unwrap() as usize,
        report.rows.len()
    );
    let row = capabilities_for_hosted(Some("datetime.calendar"));
    assert_eq!(row["id"], "datetime.calendar");
    assert_eq!(row["status"], "implemented");
    assert_eq!(row["native"], "real");
    let html = render_hosted_capabilities_page(Some("datetime.calendar"));
    assert!(
        html.contains("<html>")
            && html.contains("syx-capability-v0")
            && html.contains("datetime.calendar"),
        "HTML page must include html/schema/id: {html}"
    );

    let path = std::env::temp_dir().join("syx-hosted-deepen-w380-nail.html");
    write_hosted_capabilities_fixture(&path, Some("datetime.calendar")).expect("write fixture");
    let got = std::fs::read_to_string(&path).expect("read fixture");
    assert!(
        got.contains("<html>") && got.contains("datetime.calendar") && got.contains("syx-capability-v0"),
        "fixture must contain same-source hosted page: {got}"
    );
    let _ = std::fs::remove_file(&path);
}

