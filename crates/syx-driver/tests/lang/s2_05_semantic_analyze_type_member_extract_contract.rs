//! S2-I246 S2-05 syx-semantic analyze_type_member extract contracts.
//! Nails: type_member left lib (I246); home may be analyzer or semantic_analyze_type_member (I255).
//! See docs/adr/stage2-s2-05-semantic-analyze-type-member-extract.md.

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
fn s2_05_semantic_analyze_type_member_extract_baseline() {
    let member = repo_file("crates/syx-semantic/src/semantic_analyze_type_member.rs");
    let call = repo_file("crates/syx-semantic/src/semantic_analyze_call.rs");
    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        holds_fn(&member, "analyze_type_member") || holds_fn(&sa, "analyze_type_member"),
        "analyze_type_member must live in semantic_analyze_type_member (I255) or analyzer"
    );
    assert!(
        holds_fn(&call, "is_compatible") || holds_fn(&sa, "is_compatible"),
        "is_compatible method must live with call face (I254) or analyzer"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        !holds_fn(&lib, "analyze_type_member"),
        "analyze_type_member definition must leave lib.rs"
    );
    assert!(
        !holds_fn(&lib, "is_compatible"),
        "is_compatible method definition must leave lib.rs"
    );
    let conc = repo_file("crates/syx-semantic/src/semantic_concurrency.rs");
    assert!(
        holds_fn(&conc, "check_concurrency_rules") || holds_fn(&sa, "check_concurrency_rules"),
        "concurrency face must live in semantic_concurrency (I256) or analyzer"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyze-type-member-extract.md");
    assert!(adr.contains("S2-I246"), "extract ADR must name S2-I246");
    assert!(
        adr.contains("S2-I247") && (adr.contains("concurrency") || adr.contains("infer")),
        "ADR must lock next knife = S2-I247 infer/host→concurrency"
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
