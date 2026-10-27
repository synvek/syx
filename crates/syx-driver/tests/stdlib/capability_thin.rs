//! Capability same-source thin-open evidence nails.

#[test]
fn capability_thin_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/a10-capability-source.md",
        "docs/lang/a10-capability-floor.md",
        "docs/lang/a10-capability-exit.md",
        "docs/capability-manifest-v0.md",
        "packages/internal/capabilities.toml",
        "packages/internal/capabilities.generated.json",
        "books/shared/capabilities-generated.md",
        "scripts/gen_capabilities.py",
        "crates/syx-driver/src/capabilities_report.rs",
        "crates/syx-lsp/src/lib.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "A-10 thin requires evidence file: {path}"
        );
    }
    let caps = include_str!("../../../../packages/internal/capabilities.toml");
    assert!(
        caps.contains("[meta]")
            && caps.contains("schema = \"syx-capability-v0\"")
            && caps.contains("[[capability]]"),
        "capabilities.toml must remain the authoritative v0 source"
    );
    let report = include_str!("../../src/capabilities_report.rs");
    assert!(
        report.contains("capabilities.generated.json"),
        "doctor report must embed generated JSON same-source"
    );
    let lsp = include_str!("../../../syx-lsp/src/lib.rs");
    assert!(
        lsp.contains("syx.capabilities")
            && lsp.contains("capabilities_for_lsp")
            && lsp.contains("run_capabilities_report"),
        "LSP must register syx.capabilities via capabilities_for_lsp → run_capabilities_report"
    );
}

#[test]
fn capability_thin_spec_signed() {
    let spec = include_str!("../../../../docs/lang/a10-capability-source.md");
    assert!(
        spec.contains("ADR-702")
            && (spec.contains("A-10 thin floor") || spec.contains("thin floor signed")),
        "spec must cite ADR-702 A-10 thin floor signed"
    );
    assert!(
        spec.contains("ADR-703")
            && spec.contains("syx.capabilities")
            && (spec.contains("partial met") || spec.contains("**partial met**")),
        "spec must cite ADR-703 LSP deepen (syx.capabilities; partial met)"
    );
    assert!(
        spec.contains("ADR-704")
            && (spec.contains("a10-capability-floor") || spec.contains("floor signed")),
        "spec must cross-index ADR-704 A-10 floor"
    );
    assert!(
        spec.contains("ADR-705")
            && (spec.contains("a10-capability-exit") || spec.contains("exit signed")),
        "spec must cross-index ADR-705 A-10 exit"
    );
    assert!(
        (spec.contains("**met** (narrow)") || spec.contains("ADR-722") || spec.contains("W378"))
            && (spec.contains("网站") || spec.contains("website")),
        "spec must mark website consumption as met (narrow) after W378"
    );
    assert!(
        !spec.contains("| **LSP 消费同一源** | **unmet** |"),
        "LSP cluster must no longer be unmet after W359"
    );
    assert!(
        !spec.contains("| **网站消费同一源** | **unmet** |")
            && !spec.contains("| **网站消费同一源** | **partial met** |"),
        "website cluster must be met (narrow) after W378"
    );
    for cluster in [
        "权威源 schema",
        "生成器 + CI freshness",
        "Books 同源矩阵",
        "Doctor 同源 JSON",
        "LSP 消费同一源",
        "网站消费同一源",
        "Residual native raise",
    ] {
        assert!(spec.contains(cluster), "spec must cover cluster {cluster}");
    }
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("website")
                || spec.contains("网站")
                || spec.contains("residual")
                || spec.contains("Residual")),
        "spec Frozen must keep website / residual fences"
    );
    assert!(
        spec.contains("W357") || spec.contains("ADR-701") || spec.contains("a08-qs4-exit"),
        "spec must cross-index W357 A-08 exit"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
    assert!(
        spec.contains("website") || spec.contains("hosted") || spec.contains("网站"),
        "spec must keep website / hosted product semantics"
    );
}

#[test]
fn capability_thin_roadmap_thin() {
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
            || a10.contains("exit"),
        "A-10 row must cite W361 exit: {a10}"
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
