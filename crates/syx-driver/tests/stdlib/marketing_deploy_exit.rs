//! Marketing product met exit charter
//! Policy + meta flip only — scope narrow; marketing full site remains unmet.

#[test]
fn marketing_deploy_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/marketing-deploy-exit.md",
        "docs/lang/marketing-deploy-vectors.md",
        "crates/syx-driver/src/marketing_capabilities.rs",
        "crates/syx-driver/tests/stdlib/marketing_deploy_thin.rs",
        "crates/syx-driver/tests/stdlib/marketing_deploy_deepen.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "marketing exit requires evidence file: {path}"
        );
    }
    let helper = include_str!("../../src/marketing_capabilities.rs");
    assert!(
        helper.contains("fn capabilities_for_marketing")
            && helper.contains("run_capabilities_report"),
        "exit evidence must retain W390 deepen helper"
    );
}

#[test]
fn marketing_deploy_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/marketing-deploy-exit.md");
    assert!(
        spec.contains("ADR-735")
            && (spec.contains("exit signed") || spec.contains("Exit signed")),
        "spec must cite ADR-735 exit signed"
    );
    assert!(
        spec.contains("scope narrow") || spec.contains("Marketing scope narrow"),
        "spec must cite marketing scope narrow"
    );
    assert!(
        spec.contains("W389")
            && spec.contains("W390")
            && (spec.contains("partial met") || spec.contains("sufficient")),
        "spec must index inventory + deepen partial-sufficient prerequisites"
    );
    assert!(
        (spec.contains("full site") || spec.contains("Static") || spec.contains("deploy"))
            && (spec.contains("unmet") || spec.contains("defer") || spec.contains("partial")),
        "spec must keep marketing full site / Static full met deferred"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("B-04") || spec.contains("marketing") || spec.contains("deploy")),
        "spec Frozen must keep marketing / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);

    let vectors = include_str!("../../../../docs/lang/marketing-deploy-vectors.md");
    assert!(
        vectors.contains("Marketing product met (narrow)")
            && (vectors.contains("**met** (W391)") || vectors.contains("ADR-735")),
        "vectors must index marketing product met (narrow)"
    );
    assert!(
        vectors.contains("Static / marketing deploy tree")
            && vectors.contains("**partial met**"),
        "vectors must keep Static tree partial met"
    );
    assert!(
        (vectors.contains("Hosted full site") || vectors.contains("marketing full site"))
            && vectors.contains("**unmet**"),
        "vectors must keep full site unmet"
    );
}

#[test]
fn marketing_deploy_exit_roadmap_honesty() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    assert!(
        roadmap.contains("W391")
            && (roadmap.contains("marketing")
                || roadmap.contains("ADR-735")
                || roadmap.contains("exit")),
        "roadmap must cite W391 marketing product met exit"
    );
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(
        a10.contains("| [x] |"),
        "A-10 must remain [x] after W391: {a10}"
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
