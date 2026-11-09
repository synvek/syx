//! S2-I245 S2-05 syx-semantic analyze_call extract contracts.
//! Nails: analyze_call left lib (I245); home may be analyzer or semantic_analyze_call (I254).
//! See docs/adr/stage2-s2-05-semantic-analyze-call-extract.md.

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
fn s2_05_semantic_analyze_call_extract_baseline() {
    let call = repo_file("crates/syx-semantic/src/semantic_analyze_call.rs");
    let member = repo_file("crates/syx-semantic/src/semantic_analyze_type_member.rs");
    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        holds_fn(&call, "analyze_call") || holds_fn(&sa, "analyze_call"),
        "analyze_call must live in semantic_analyze_call (I254) or semantic_analyzer"
    );
    assert!(
        holds_fn(&call, "analyze_expr_with_expected")
            || holds_fn(&sa, "analyze_expr_with_expected"),
        "analyze_expr_with_expected must live with analyze_call face"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        !holds_fn(&lib, "analyze_call"),
        "analyze_call definition must leave lib.rs"
    );
    assert!(
        !holds_fn(&lib, "analyze_expr_with_expected"),
        "analyze_expr_with_expected definition must leave lib.rs"
    );
    assert!(
        holds_fn(&member, "analyze_type_member") || holds_fn(&sa, "analyze_type_member"),
        "analyze_type_member must live in semantic_analyze_type_member (I255) or analyzer"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyze-call-extract.md");
    assert!(adr.contains("S2-I245"), "extract ADR must name S2-I245");
    assert!(
        adr.contains("S2-I246") && adr.contains("analyze_type_member"),
        "ADR must lock next knife = S2-I246 analyze_type_member"
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
