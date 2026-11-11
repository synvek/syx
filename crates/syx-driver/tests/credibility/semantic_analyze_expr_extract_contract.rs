//! S2-I244 S2-05 syx-semantic analyze_expr extract contracts.
//! Nails: analyze_expr left lib (I244); home may be analyzer or semantic_analyze_expr (I251).
//! See docs/adr/stage2-s2-05-semantic-analyze-expr-extract.md.

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
fn s2_05_semantic_analyze_expr_extract_baseline() {
    let expr = repo_file("crates/syx-semantic/src/semantic_analyze_expr.rs");
    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        holds_fn(&expr, "analyze_expr") || holds_fn(&sa, "analyze_expr"),
        "analyze_expr must live in semantic_analyze_expr (I251) or semantic_analyzer"
    );
    assert!(
        holds_fn(&expr, "analyze_expr_inner") || holds_fn(&sa, "analyze_expr_inner"),
        "analyze_expr_inner must live with analyze_expr face"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        !holds_fn(&lib, "analyze_expr"),
        "analyze_expr definition must leave lib.rs"
    );
    assert!(
        !holds_fn(&lib, "analyze_expr_inner"),
        "analyze_expr_inner definition must leave lib.rs"
    );
    let call = repo_file("crates/syx-semantic/src/semantic_analyze_call.rs");
    assert!(
        holds_fn(&call, "analyze_call") || holds_fn(&sa, "analyze_call"),
        "analyze_call must live in semantic_analyze_call (I254) or semantic_analyzer"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyze-expr-extract.md");
    assert!(adr.contains("S2-I244"), "extract ADR must name S2-I244");
    assert!(
        adr.contains("S2-I245") && adr.contains("analyze_call"),
        "ADR must lock next knife = S2-I245 analyze_call"
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
