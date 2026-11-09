//! S2-I241 S2-05 syx-semantic collect_decls extract contracts.
//! Nails: collect_decls left lib (I241); home may be analyzer or semantic_collect_decls (I250).
//! See docs/adr/stage2-s2-05-semantic-collect-decls-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn holds_fn(src: &str, name: &str) -> bool {
    src.contains(&format!("fn {name}(")) || src.contains(&format!("pub(crate) fn {name}("))
}

#[test]
fn s2_05_semantic_collect_decls_extract_baseline() {
    let collect = repo_file("crates/syx-semantic/src/semantic_collect_decls.rs");
    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        holds_fn(&collect, "collect_decls") || holds_fn(&sa, "collect_decls"),
        "collect_decls must live in semantic_collect_decls (I250) or semantic_analyzer"
    );
    assert!(
        holds_fn(&collect, "collect_nested_type_decl")
            || sa.contains("fn collect_nested_type_decl("),
        "collect_nested_type_decl must live with collect_decls face"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        !holds_fn(&lib, "collect_decls"),
        "collect_decls definition must leave lib.rs"
    );
    let decl = repo_file("crates/syx-semantic/src/semantic_analyze_decl.rs");
    assert!(
        holds_fn(&decl, "analyze_decl") || holds_fn(&sa, "analyze_decl"),
        "analyze_decl must live in semantic_analyze_decl (I253) or semantic_analyzer"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-collect-decls-extract.md");
    assert!(adr.contains("S2-I241"), "extract ADR must name S2-I241");
    assert!(
        adr.contains("S2-I242") && adr.contains("analyze_decl"),
        "ADR must name S2-I242 analyze_decl ladder"
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
