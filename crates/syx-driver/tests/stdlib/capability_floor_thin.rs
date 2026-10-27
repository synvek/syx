//! Capability floor evidence nails (schema/gen/books/doctor + LSP).

#[test]
fn capability_floor_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/a10-capability-floor.md",
        "docs/lang/a10-capability-source.md",
        "docs/lang/a10-capability-exit.md",
        "packages/internal/capabilities.toml",
        "packages/internal/capabilities.generated.json",
        "books/shared/capabilities-generated.md",
        "scripts/gen_capabilities.py",
        "crates/syx-driver/src/capabilities_report.rs",
        "crates/syx-driver/tests/stdlib/capability_thin.rs",
        "crates/syx-lsp/src/lib.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "A-10 floor requires evidence file: {path}"
        );
    }
    let lsp = include_str!("../../../syx-lsp/src/lib.rs");
    assert!(
        lsp.contains("syx.capabilities")
            && lsp.contains("lsp_a10_capabilities_same_source_count_and_lookup"),
        "floor evidence must retain W359 LSP deepen nail"
    );
}

#[test]
fn capability_floor_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/a10-capability-floor.md");
    assert!(
        spec.contains("ADR-704")
            && (spec.contains("A-10 floor signed") || spec.contains("floor signed")),
        "spec must cite ADR-704 A-10 floor signed"
    );
    assert!(
        spec.contains("ADR-705")
            || spec.contains("a10-capability-exit")
            || spec.contains("exit signed"),
        "spec must cross-index W361 exit"
    );
    assert!(
        spec.contains("Floor ≠ exit")
            || spec.contains("floor ≠ exit")
            || (spec.contains("Exit") && spec.contains("separately")),
        "spec must keep floor ≠ exit distinction"
    );
    for cluster in [
        "权威源 schema + gen + CI",
        "Books + Doctor 同源",
        "LSP 同源 deepen",
        "网站同源消费",
        "Residual native raise",
    ] {
        assert!(spec.contains(cluster), "spec must cover cluster {cluster}");
    }
    assert!(
        spec.contains("**met**")
            && spec.contains("**partial met**")
            && spec.contains("**unmet**")
            && spec.contains("**Frozen**"),
        "spec must index met / partial met / unmet / Frozen statuses"
    );
    assert!(
        spec.contains("ADR-703") || spec.contains("syx.capabilities"),
        "spec must cross-index W359 LSP deepen"
    );
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("website") || spec.contains("网站") || spec.contains("residual")),
        "spec Frozen must keep website / residual fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
    assert!(
        spec.contains("website") || spec.contains("hosted") || spec.contains("网站"),
        "spec must keep website / hosted product semantics"
    );
}

#[test]
fn capability_floor_thin_roadmap_thin() {
    let roadmap = include_str!("../../../../plan/02-improvement-roadmap.md");
    let a10 = roadmap
        .lines()
        .find(|l| l.starts_with("| A-10 |"))
        .expect("A-10 row");
    assert!(
        a10.contains("| [x] |"),
        "A-10 must be [x] after W361 exit: {a10}"
    );
    assert!(
        a10.contains("W361")
            || a10.contains("ADR-705")
            || a10.contains("exit")
            || a10.contains("W360")
            || a10.contains("ADR-704")
            || a10.contains("floor"),
        "A-10 row must cite W360 floor and/or W361 exit: {a10}"
    );
    assert!(
        roadmap.contains("W361")
            && (roadmap.contains("ADR-705") || roadmap.contains("exit")),
        "roadmap must cite W361 A-10 exit"
    );
    for id in ["B-04", "B-05", "B-06"] {
        let row = roadmap
            .lines()
            .find(|l| l.starts_with(&format!("| {id} |")))
            .unwrap_or_else(|| panic!("{id} row"));
        assert!(
            row.contains("| [x] |"),
            "{id} is delivered after Wave B; do not re-pin the A-10-era not-[x] fence: {row}"
        );
    }
}
