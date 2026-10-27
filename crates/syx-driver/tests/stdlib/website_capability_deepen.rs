//! Website same-source deepen
//! Locks deepen helper+nail; product met exit is (narrow); bans hosted full site / B `[x]`.

#[test]
fn website_capability_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/website-capability-vectors.md",
        "docs/lang/website-capability-exit.md",
        "crates/syx-driver/src/website_capabilities.rs",
        "packages/internal/capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "website deepen requires evidence file: {path}"
        );
    }
    let helper = include_str!("../../src/website_capabilities.rs");
    assert!(
        helper.contains("fn capabilities_for_website")
            && helper.contains("fn render_capabilities_html_snippet")
            && helper.contains("run_capabilities_report")
            && !helper.contains("capabilities.generated.json"),
        "helper must call run_capabilities_report without a second JSON embed"
    );
}

#[test]
fn website_capability_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/lang/website-capability-vectors.md");
    assert!(
        spec.contains("ADR-721")
            && (spec.contains("deepen") || spec.contains("partial")),
        "spec must cite ADR-721 website deepen"
    );
    assert!(
        spec.contains("独立 website 消费者")
            && (spec.contains("**met** (W378") || spec.contains("ADR-722"))
            && (spec.contains("capabilities_for_website")
                || spec.contains("website_capabilities")),
        "spec must index website consumer met (narrow) with helper evidence"
    );
    assert!(
        !spec.contains("| **独立 website 消费者** | **unmet** |")
            && !spec.contains("| **独立 website 消费者** | **partial met** (W377) |"),
        "website consumer must be product-met (narrow) after W378"
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
fn website_capability_deepen_nail_behavior() {
    use syx_driver::website_capabilities::{
        capabilities_for_website, render_capabilities_html_snippet,
    };
    use syx_driver::capabilities_report::run_capabilities_report;

    let summary = capabilities_for_website(None);
    let report = run_capabilities_report();
    assert_eq!(summary["schema"], "syx-capability-v0");
    assert_eq!(
        summary["count"].as_u64().unwrap() as usize,
        report.rows.len()
    );
    let row = capabilities_for_website(Some("datetime.calendar"));
    assert_eq!(row["id"], "datetime.calendar");
    assert_eq!(row["status"], "implemented");
    assert_eq!(row["native"], "real");
    let html = render_capabilities_html_snippet(Some("datetime.calendar"));
    assert!(
        html.contains("syx-capability-v0") && html.contains("datetime.calendar"),
        "HTML must include schema and id: {html}"
    );
}

#[test]
fn website_capability_deepen_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W377")
            && (roadmap.contains("website")
                || roadmap.contains("ADR-721")
                || roadmap.contains("deepen")),
        "roadmap must cite W377 website deepen"
    );
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} is delivered after Wave B; do not re-pin the not-[x] fence: {row}"
        );
    }
    let b08 = roadmap
        .lines()
        .find(|l| l.starts_with("| B-08 |"))
        .expect("B-08 row");
    assert!(b08.contains("| [x] |"), "B-08 must remain [x]: {b08}");
    assert!(
        roadmap.contains("Q-L5") && roadmap.contains("已签"),
        "Q-L5 must remain signed"
    );
}
