//! S2-I153 S2-04 lint CST-first policy contracts.
//! Nails: ADR + lint_source_auto + lint_file/document → auto; ≠ lint-on-CST / ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-lint-cst-first.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_lint_cst_first_policy() {
    let adr = repo_file("docs/adr/stage2-s2-04-lint-cst-first.md");
    assert!(adr.contains("S2-I153"), "lint-cst-first ADR must exist");
    assert!(
        adr.contains("lint_source_auto"),
        "ADR must name lint_source_auto"
    );
    assert!(
        adr.contains("parse_cst_subset") || adr.contains("CST-first"),
        "ADR must lock CST-first probe policy"
    );
    assert!(
        adr.contains("≠") && (adr.contains("lossless") || adr.contains("L001")),
        "ADR must Non-claim lint-on-CST / shared lossless tree"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-04") || adr.contains("[x]")),
        "ADR must Non-claim S2-04 [x]"
    );

    let lint = repo_file("crates/syx-lint/src/lib.rs");
    let auto_idx = lint
        .find("pub fn lint_source_auto(")
        .expect("lint_source_auto must exist");
    let auto_chunk = &lint[auto_idx..auto_idx + 700.min(lint.len() - auto_idx)];
    assert!(
        auto_chunk.contains("parse_cst_subset"),
        "lint_source_auto must probe parse_cst_subset"
    );
    assert!(
        auto_chunk.contains("CST001"),
        "lint_source_auto must fall back on CST001"
    );
    assert!(
        auto_chunk.contains("lint_source("),
        "lint_source_auto must still call lint_source (AST Module rules)"
    );
    assert!(
        auto_chunk.contains("SYX_FMT_AST"),
        "lint_source_auto must honor SYX_FMT_AST hatch"
    );

    let driver = repo_file("crates/syx-driver/src/format_lint.rs");
    let lint_file_idx = driver.find("fn lint_file(").expect("lint_file must exist");
    let lint_file_chunk =
        &driver[lint_file_idx..lint_file_idx + 900.min(driver.len() - lint_file_idx)];
    assert!(
        lint_file_chunk.contains("lint_source_auto(&source)"),
        "lint_file must call lint_source_auto"
    );
    assert!(
        !lint_file_chunk.contains("lint_source(&source)"),
        "lint_file must not call AST-only lint_source as product SSOT"
    );
    assert!(
        driver.contains("pub fn lint_document_for_lsp("),
        "lint_document_for_lsp must exist"
    );

    // Behavioral: CST subset + L001/L011 via buffer API.
    let src = "// TODO: review\nfunc BadName() {\n    print(1)\n}\n";
    let issues = syx_driver::lint_document_for_lsp("t.sy", src).expect("lint document");
    assert!(
        issues.iter().any(|i| i.code == "L001"),
        "expected L001; got {issues:?}"
    );
    assert!(
        issues.iter().any(|i| i.code == "L011"),
        "expected L011; got {issues:?}"
    );
}
