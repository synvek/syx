//! S2-I250–I252 S2-05 semantic_analyzer core-faces batch contracts.
//! Nails: collect_decls / analyze_expr / analyze_stmt sidecars; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-semantic-analyzer-core-faces-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_semantic_analyzer_core_faces_batch_baseline() {
    let collect = repo_file("crates/syx-semantic/src/semantic_collect_decls.rs");
    assert!(
        collect.contains("fn collect_decls(") || collect.contains("pub(crate) fn collect_decls("),
        "semantic_collect_decls must hold collect_decls"
    );

    let expr = repo_file("crates/syx-semantic/src/semantic_analyze_expr.rs");
    assert!(
        expr.contains("fn analyze_expr_inner(")
            || expr.contains("pub(crate) fn analyze_expr_inner("),
        "semantic_analyze_expr must hold analyze_expr_inner"
    );

    let stmt = repo_file("crates/syx-semantic/src/semantic_analyze_stmt.rs");
    assert!(
        stmt.contains("fn analyze_stmt(") || stmt.contains("pub(crate) fn analyze_stmt("),
        "semantic_analyze_stmt must hold analyze_stmt"
    );

    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        !sa.contains("pub(crate) fn collect_decls(") && !sa.contains("\n    fn collect_decls("),
        "collect_decls definition must leave semantic_analyzer.rs"
    );
    assert!(
        !sa.contains("fn analyze_expr_inner(") && !sa.contains("pub(crate) fn analyze_expr_inner("),
        "analyze_expr_inner definition must leave semantic_analyzer.rs"
    );
    assert!(
        !sa.contains("\n    fn analyze_stmt(") && !sa.contains("pub(crate) fn analyze_stmt("),
        "analyze_stmt definition must leave semantic_analyzer.rs"
    );
    assert!(
        sa.contains("pub struct SemanticAnalyzer"),
        "SemanticAnalyzer struct must remain in semantic_analyzer hub"
    );
    assert!(
        sa.lines().count() >= 800,
        "semantic_analyzer.rs must remain substantial while S2-05 remains [~] (analyzer floor tracks I256–I258 → 800)"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        lib.contains("mod semantic_collect_decls")
            && lib.contains("mod semantic_analyze_expr")
            && lib.contains("mod semantic_analyze_stmt"),
        "lib.rs must declare the three core-face mods"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyzer-core-faces-batch.md");
    assert!(
        adr.contains("S2-I250") && adr.contains("S2-I251") && adr.contains("S2-I252"),
        "batch ADR must name I250–I252"
    );
    assert!(
        adr.contains("S2-I253") && (adr.contains("call") || adr.contains("type_member")),
        "ADR must lock next = S2-I253+ call/decl/type_member peel"
    );
    assert!(
        adr.contains("≠") && adr.contains("S2-05"),
        "ADR must Non-claim S2-05 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
}
