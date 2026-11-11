//! S2-I239–I240 S2-05 syx-semantic SemanticAnalyzer entry→builtins batch contracts.
//! Nails: semantic_analyzer module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-semantic-analyzer-entry-to-builtins-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_semantic_analyzer_entry_to_builtins_batch_baseline() {
    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        sa.contains("pub struct SemanticAnalyzer")
            && sa.contains("analyze_collecting_with_value_builtins")
            && sa.contains("fn register_builtins"),
        "semantic_analyzer must hold struct + entry + register_builtins"
    );
    assert!(
        sa.contains("infer_member_type") || sa.contains("fn infer_member_type"),
        "semantic_analyzer must hold builtins/support member helpers"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        lib.contains("mod semantic_analyzer;"),
        "lib.rs must declare mod semantic_analyzer"
    );
    assert!(
        lib.contains("pub use semantic_analyzer::SemanticAnalyzer"),
        "lib.rs must pub-use SemanticAnalyzer"
    );
    assert!(
        !lib.contains("pub struct SemanticAnalyzer"),
        "SemanticAnalyzer definition must leave lib.rs"
    );
    let collect = repo_file("crates/syx-semantic/src/semantic_collect_decls.rs");
    let sa_now = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        collect.contains("fn collect_decls(")
            || collect.contains("pub(crate) fn collect_decls(")
            || sa_now.contains("fn collect_decls(")
            || sa_now.contains("pub(crate) fn collect_decls("),
        "collect_decls must live in semantic_collect_decls (I250) or semantic_analyzer (I241)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyzer-entry-to-builtins-batch.md");
    assert!(
        adr.contains("S2-I239") && adr.contains("S2-I240"),
        "batch ADR must name I239–I240"
    );
    assert!(
        adr.contains("S2-I241") && adr.contains("collect_decls"),
        "ADR must name S2-I241 collect_decls ladder"
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
