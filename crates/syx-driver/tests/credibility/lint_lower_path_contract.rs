//! S2-I161 S2-04 lint CST-success → lower path contracts.
//! Nails: ADR + lint_source_auto → lower_cst_subset_probe; LWR001 → Parser;
//! probe-face issue parity; ≠ expand probe / ≠ Frontend / ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-lint-lower-path.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_lint_lower_path_adr_and_wire() {
    let adr = repo_file("docs/adr/stage2-s2-04-lint-lower-path.md");
    assert!(adr.contains("S2-I161"), "lint-lower-path ADR must exist");
    assert!(
        adr.contains("lower_cst_subset_probe"),
        "ADR must name lower_cst_subset_probe"
    );
    assert!(
        adr.contains("LWR001"),
        "ADR must name LWR001 fallback"
    );
    assert!(
        adr.contains("≠") && adr.contains("FrontendPipeline"),
        "ADR must Non-claim FrontendPipeline"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-04") || adr.contains("[x]")),
        "ADR must Non-claim S2-04 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("expand"),
        "ADR must Non-claim expand lower_probe face"
    );

    let lint = repo_file("crates/syx-lint/src/lib.rs");
    let auto_idx = lint
        .find("pub fn lint_source_auto(")
        .expect("lint_source_auto must exist");
    let auto_chunk = &lint[auto_idx..auto_idx + 900.min(lint.len() - auto_idx)];
    assert!(
        auto_chunk.contains("lower_cst_subset_probe"),
        "lint_source_auto must call lower_cst_subset_probe"
    );
    assert!(
        auto_chunk.contains("lint_module"),
        "lint_source_auto lower-Ok path must call lint_module"
    );
    assert!(
        auto_chunk.contains("lint_source("),
        "lint_source_auto must retain lint_source fallback"
    );

    let frontend_toml = repo_file("crates/syx-frontend/Cargo.toml");
    assert!(
        !frontend_toml.contains("syx-cst"),
        "Frontend must not gain syx-cst dep in lint-lower window"
    );
}

#[test]
fn s2_04_lint_lower_path_probe_parity() {
    let src = "// TODO: review\nfunc main() {\n    print(1)\n}\n";
    let file = syx_diagnostics::SourceFile::new("probe.sy", src);
    let via_auto = syx_lint::lint_source_auto(&file).expect("auto");
    let via_parser = syx_lint::lint_source(&file).expect("parser");
    assert_eq!(
        via_auto, via_parser,
        "probe-face lint_source_auto must match lint_source"
    );
    assert!(
        via_auto.iter().any(|i| i.code == "L011"),
        "expected L011; got {via_auto:?}"
    );

    // Empty body on probe face → L003 via lower (no Parser).
    let empty = syx_diagnostics::SourceFile::new("empty.sy", "func main() {\n}\n");
    let empty_issues = syx_lint::lint_source_auto(&empty).expect("empty auto");
    assert!(
        empty_issues.iter().any(|i| i.code == "L003"),
        "expected L003; got {empty_issues:?}"
    );

    // S2-I162: let+print is on probe face → lower path (not LWR001 fallback).
    let let_src = "func main() {\n    let x = 1\n    print(x)\n}\n";
    let let_file = syx_diagnostics::SourceFile::new("let.sy", let_src);
    assert!(
        syx_cst::lower_cst_subset_probe_from_source(&let_file).is_ok(),
        "let+print must lower Ok after I162"
    );
    let let_auto = syx_lint::lint_source_auto(&let_file).expect("let auto");
    let let_parser = syx_lint::lint_source(&let_file).expect("let parser");
    assert_eq!(
        let_auto, let_parser,
        "I162 let+print lint_source_auto must match lint_source via lower"
    );
}

#[test]
fn s2_04_lint_lower_path_lwr001_fallback() {
    // CST Ok (while) but out of I162 probe → LWR001 → Parser; issues match lint_source.
    let src = "func main() {\n    while x {\n        print(1)\n    }\n}\n";
    let file = syx_diagnostics::SourceFile::new("while.sy", src);
    assert!(
        syx_cst::parse_cst_subset(&file).is_ok(),
        "fixture must remain CST-Ok"
    );
    let err = syx_cst::lower_cst_subset_probe_from_source(&file).expect_err("must LWR001");
    assert_eq!(err.code.as_deref(), Some("LWR001"));

    let via_auto = syx_lint::lint_source_auto(&file).expect("auto fallback");
    let via_parser = syx_lint::lint_source(&file).expect("parser");
    assert_eq!(
        via_auto, via_parser,
        "LWR001 fallback must match lint_source"
    );
}
