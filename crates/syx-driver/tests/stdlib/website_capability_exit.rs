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

#[test]
fn website_capability_exit_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W378")
            && (roadmap.contains("website")
                || roadmap.contains("ADR-722")
                || roadmap.contains("met")),
        "roadmap must cite W378 website product met exit"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(
        a10.contains("| [x] |"),
        "A-10 must remain [x] after W378: {a10}"
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
    assert!(
        b08.contains("| [x] |"),
        "B-08 must remain [x]: {b08}"
    );
    assert!(
        roadmap.contains("Q-L5") && roadmap.contains("已签"),
        "Q-L5 must remain signed"
    );
}
