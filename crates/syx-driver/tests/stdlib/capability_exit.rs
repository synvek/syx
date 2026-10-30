//! Capability charter exit evidence nails (A-10 scope-narrow).

#[test]
fn capability_exit_evidence_files_exist() {
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");
    for rel in [
        "docs/lang/a10-capability-exit.md",
        "docs/lang/a10-capability-floor.md",
        "docs/lang/a10-capability-source.md",
        "crates/syx-driver/tests/stdlib/capability_thin.rs",
        "crates/syx-driver/tests/stdlib/capability_floor_thin.rs",
        "crates/syx-lsp/src/lib.rs",
    ] {
        let path = format!("{root}{rel}");
        assert!(
            std::path::Path::new(&path).is_file(),
            "A-10 exit requires evidence file: {path}"
        );
    }
    let lsp = include_str!("../../../syx-lsp/src/lib.rs");
    assert!(
        lsp.contains("syx.capabilities")
            && lsp.contains("lsp_a10_capabilities_same_source_count_and_lookup"),
        "exit evidence must retain W359 LSP deepen nail"
    );
}

#[test]
fn capability_exit_spec_signed() {
    let spec = include_str!("../../../../docs/lang/a10-capability-exit.md");
    assert!(
        spec.contains("ADR-705")
            && (spec.contains("exit signed") || spec.contains("Exit signed")),
        "spec must cite ADR-705 exit signed"
    );
    assert!(
        spec.contains("scope narrow") || spec.contains("M0-C05 scope"),
        "spec must cite M0-C05 scope narrow"
    );
    assert!(
        (spec.contains("unmet") || spec.contains("deferred") || spec.contains("defer"))
            && (spec.contains("hosted") || spec.contains("website") || spec.contains("网站")),
        "spec must keep hosted full site deferred unmet"
    );
    assert!(
        (spec.contains("met (narrow)") || spec.contains("ADR-722") || spec.contains("W378"))
            && !spec.contains("website consumer remains **met**"),
        "spec may cite product met (narrow) while forbidding unrestricted met claims"
    );
    for marker in ["W358", "W359", "W360", "ADR-704"] {
        assert!(
            spec.contains(marker),
            "exit acceptance must cite prerequisite {marker}"
        );
    }
    assert!(
        spec.contains("## Frozen")
            && (spec.contains("hosted")
                || spec.contains("website")
                || spec.contains("网站")
                || spec.contains("residual")
                || spec.contains("Residual")
                || spec.contains("B-04")),
        "spec Frozen must keep hosted-site / residual / B fences"
    );
    super::construction_progress::assert_spec_progress_without_window_plus_or(spec);
    assert!(
        spec.contains("website") || spec.contains("hosted") || spec.contains("网站"),
        "spec must keep website / hosted product semantics"
    );
}

