//! S2-I253–I255 S2-05 semantic_analyzer decl/call/member batch contracts.
//! Nails: three sidecars; definitions leave analyzer hub; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-semantic-analyzer-decl-call-member-batch.md.

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
fn s2_05_semantic_analyzer_decl_call_member_batch_baseline() {
    let decl = repo_file("crates/syx-semantic/src/semantic_analyze_decl.rs");
    assert!(
        holds_fn(&decl, "analyze_decl"),
        "semantic_analyze_decl must hold analyze_decl"
    );

    let call = repo_file("crates/syx-semantic/src/semantic_analyze_call.rs");
    assert!(
        holds_fn(&call, "analyze_call"),
        "semantic_analyze_call must hold analyze_call"
    );

    let member = repo_file("crates/syx-semantic/src/semantic_analyze_type_member.rs");
    assert!(
        holds_fn(&member, "analyze_type_member"),
        "semantic_analyze_type_member must hold analyze_type_member"
    );

    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        !holds_fn(&sa, "analyze_decl"),
        "analyze_decl definition must leave semantic_analyzer.rs"
    );
    assert!(
        !holds_fn(&sa, "analyze_call"),
        "analyze_call definition must leave semantic_analyzer.rs"
    );
    assert!(
        !holds_fn(&sa, "analyze_type_member"),
        "analyze_type_member definition must leave semantic_analyzer.rs"
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
        lib.contains("mod semantic_analyze_decl")
            && lib.contains("mod semantic_analyze_call")
            && lib.contains("mod semantic_analyze_type_member"),
        "lib.rs must declare the three decl/call/member mods"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyzer-decl-call-member-batch.md");
    assert!(
        adr.contains("S2-I253") && adr.contains("S2-I254") && adr.contains("S2-I255"),
        "batch ADR must name I253–I255"
    );
    assert!(
        adr.contains("S2-I256") && (adr.contains("concurrency") || adr.contains("IDE")),
        "ADR must lock next = S2-I256+ concurrency/IDE peel"
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
