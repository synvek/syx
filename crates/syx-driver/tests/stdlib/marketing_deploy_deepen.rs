//! Marketing same-source deepen
//! Locks helper+fixture nail; Hosted full site / marketing met remain unmet; bans B `[x]`.

#[test]
fn marketing_deploy_deepen_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/marketing-deploy-vectors.md",
        "docs/lang/hosted-site-vectors.md",
        "docs/lang/website-capability-vectors.md",
        "crates/syx-driver/src/marketing_capabilities.rs",
        "crates/syx-driver/src/hosted_capabilities.rs",
        "packages/internal/capabilities.toml",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "marketing deepen requires evidence file: {path}"
        );
    }
    let helper = include_str!("../../src/marketing_capabilities.rs");
    assert!(
        helper.contains("fn capabilities_for_marketing")
            && helper.contains("fn render_marketing_capabilities_page")
            && helper.contains("fn write_marketing_capabilities_fixture")
            && helper.contains("run_capabilities_report")
            && helper.contains("docs/")
            && helper.contains("books/")
            && !helper.contains("capabilities.generated.json"),
        "helper must call run_capabilities_report, guard docs/books, without a second JSON embed"
    );
}

#[test]
fn marketing_deploy_deepen_spec_signed() {
    let spec = include_str!("../../../../docs/lang/marketing-deploy-vectors.md");
    assert!(
        spec.contains("ADR-734")
            && (spec.contains("deepen") || spec.contains("partial")),
        "spec must cite ADR-734 marketing same-source deepen"
    );
    assert!(
        (spec.contains("Marketing same-source deepen") || spec.contains("marketing_capabilities"))
            && (spec.contains("**partial met** (W390)") || spec.contains("ADR-734")),
        "spec must index marketing deepen partial met"
    );
    assert!(
        spec.contains("Static / marketing deploy tree")
            && (spec.contains("**partial met** (W390)") || spec.contains("**partial met**")),
        "spec must index Static/marketing deploy tree as partial met"
    );
    assert!(
        !spec.contains("| **Static / marketing deploy tree** | **unmet** |")
            && !spec.contains("| **Static / marketing deploy tree** | **met** |"),
        "Static tree must be partial met — not unmet and not full met"
    );
    assert!(
        (spec.contains("Hosted full site") || spec.contains("marketing met"))
            && spec.contains("**unmet**"),
        "spec must keep Hosted full site / marketing met unmet"
    );
    assert!(
        (spec.contains("Marketing deploy inventory thin") || spec.contains("inventory thin"))
            && (spec.contains("**met** (W389)") || spec.contains("ADR-733")),
        "inventory thin must remain met"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("marketing") || spec.contains("deploy") || spec.contains("B-04")),
        "spec Frozen must keep marketing / deploy / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let hosted = include_str!("../../../../docs/lang/hosted-site-vectors.md");
    assert!(
        hosted.contains("Hosted full site") && hosted.contains("**unmet**"),
        "hosted vectors must keep Hosted full site unmet after marketing deepen"
    );
    assert!(
        hosted.contains("Static / marketing")
            && (hosted.contains("**partial met**") || hosted.contains("partial")),
        "hosted vectors must cross-index Static/marketing deploy as partial"
    );
}

#[test]
fn marketing_deploy_deepen_nail_behavior() {
    use syx_driver::capabilities_report::run_capabilities_report;
    use syx_driver::marketing_capabilities::{
        capabilities_for_marketing, render_marketing_capabilities_page,
        write_marketing_capabilities_fixture,
    };

    let summary = capabilities_for_marketing(None);
    let report = run_capabilities_report();
    assert_eq!(summary["schema"], "syx-capability-v0");
    assert_eq!(
        summary["count"].as_u64().unwrap() as usize,
        report.rows.len()
    );
    let row = capabilities_for_marketing(Some("datetime.calendar"));
    assert_eq!(row["id"], "datetime.calendar");
    assert_eq!(row["status"], "implemented");
    assert_eq!(row["native"], "real");
    let html = render_marketing_capabilities_page(Some("datetime.calendar"));
    assert!(
        html.contains("<html>")
            && html.contains("syx-capability-v0")
            && html.contains("datetime.calendar")
            && html.contains("marketing fixture"),
        "HTML page must include html/schema/id/fixture label: {html}"
    );

    let path = std::env::temp_dir().join("syx-marketing-deepen-w390-nail.html");
    write_marketing_capabilities_fixture(&path, Some("datetime.calendar")).expect("write fixture");
    let got = std::fs::read_to_string(&path).expect("read fixture");
    assert!(
        got.contains("<html>")
            && got.contains("datetime.calendar")
            && got.contains("syx-capability-v0")
            && got.contains("marketing fixture"),
        "fixture must contain same-source marketing page: {got}"
    );
    let _ = std::fs::remove_file(&path);

    let docs = std::path::Path::new("/tmp/docs/marketing/forbidden.html");
    assert!(
        write_marketing_capabilities_fixture(docs, None).is_err(),
        "path guard must reject docs/ targets"
    );
}

#[test]
fn marketing_deploy_deepen_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W390")
            && (roadmap.contains("marketing")
                || roadmap.contains("ADR-734")
                || roadmap.contains("deepen")),
        "roadmap must cite W390 marketing deepen"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(a10.contains("| [x] |"), "A-10 must remain [x]: {a10}");
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(row.contains("| [x] |"), "{id} is delivered after Wave B; do not re-pin the not-[x] fence: {row}");
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
