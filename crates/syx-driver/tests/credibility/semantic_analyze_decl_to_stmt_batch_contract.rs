//! S2-I242–I243 S2-05 syx-semantic analyze_decl→stmt batch contracts.
//! Nails: decl/stmt left lib; stmt may live in semantic_analyze_stmt (I252).
//! See docs/adr/stage2-s2-05-semantic-analyze-decl-to-stmt-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn holds_fn(src: &str, name: &str) -> bool {
    src.contains(&format!("fn {name}(")) || src.contains(&format!("pub(crate) fn {name}("))
}

#[test]
fn s2_05_semantic_analyze_decl_to_stmt_batch_baseline() {
    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    let decl = repo_file("crates/syx-semantic/src/semantic_analyze_decl.rs");
    let stmt = repo_file("crates/syx-semantic/src/semantic_analyze_stmt.rs");
    let expr = repo_file("crates/syx-semantic/src/semantic_analyze_expr.rs");
    assert!(
        holds_fn(&decl, "analyze_decls") || holds_fn(&sa, "analyze_decls"),
        "analyze_decls must live in semantic_analyze_decl (I253) or semantic_analyzer"
    );
    assert!(
        holds_fn(&decl, "analyze_decl") || holds_fn(&sa, "analyze_decl"),
        "analyze_decl must live in semantic_analyze_decl (I253) or semantic_analyzer"
    );
    assert!(
        holds_fn(&stmt, "analyze_stmt") || holds_fn(&sa, "analyze_stmt"),
        "analyze_stmt must live in semantic_analyze_stmt (I252) or semantic_analyzer"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        !holds_fn(&lib, "analyze_decls"),
        "analyze_decls definition must leave lib.rs"
    );
    assert!(
        !holds_fn(&lib, "analyze_decl"),
        "analyze_decl definition must leave lib.rs"
    );
    assert!(
        !holds_fn(&lib, "analyze_stmt"),
        "analyze_stmt definition must leave lib.rs"
    );
    assert!(
        holds_fn(&expr, "analyze_expr") || holds_fn(&sa, "analyze_expr"),
        "analyze_expr must live in semantic_analyze_expr (I251) or semantic_analyzer"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyze-decl-to-stmt-batch.md");
    assert!(
        adr.contains("S2-I242") && adr.contains("S2-I243"),
        "batch ADR must name I242–I243"
    );
    assert!(
        adr.contains("S2-I244") && adr.contains("analyze_expr"),
        "ADR must lock next knife = S2-I244 analyze_expr"
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
