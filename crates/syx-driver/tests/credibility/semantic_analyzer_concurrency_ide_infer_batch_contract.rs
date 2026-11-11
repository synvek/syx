//! S2-I256–I258 S2-05 semantic_analyzer concurrency/IDE/infer batch contracts.
//! Nails: three sidecars; definitions leave hub; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn holds_fn(src: &str, name: &str) -> bool {
    src.contains(&format!("fn {name}("))
        || src.contains(&format!("pub(crate) fn {name}("))
        || src.contains(&format!("pub fn {name}("))
}

#[test]
fn s2_05_semantic_analyzer_concurrency_ide_infer_batch_baseline() {
    let conc = repo_file("crates/syx-semantic/src/semantic_concurrency.rs");
    assert!(
        holds_fn(&conc, "check_concurrency_rules"),
        "semantic_concurrency must hold check_concurrency_rules"
    );

    let ide = repo_file("crates/syx-semantic/src/semantic_ide.rs");
    assert!(
        holds_fn(&ide, "members_for_type"),
        "semantic_ide must hold members_for_type"
    );

    let infer = repo_file("crates/syx-semantic/src/semantic_infer_host.rs");
    assert!(
        holds_fn(&infer, "infer_call_type_args"),
        "semantic_infer_host must hold infer_call_type_args"
    );

    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        !holds_fn(&sa, "check_concurrency_rules"),
        "check_concurrency_rules definition must leave semantic_analyzer.rs"
    );
    assert!(
        !holds_fn(&sa, "members_for_type"),
        "members_for_type definition must leave semantic_analyzer.rs"
    );
    assert!(
        !holds_fn(&sa, "infer_call_type_args"),
        "infer_call_type_args definition must leave semantic_analyzer.rs"
    );
    assert!(
        sa.contains("pub struct SemanticAnalyzer") && holds_fn(&sa, "register_builtins"),
        "hub must keep struct + register_builtins"
    );
    assert!(
        sa.lines().count() >= 800,
        "semantic_analyzer.rs must remain substantial while S2-05 remains [~] (analyzer floor 800)"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        lib.contains("mod semantic_concurrency")
            && lib.contains("mod semantic_ide")
            && lib.contains("mod semantic_infer_host"),
        "lib.rs must declare the three concurrency/IDE/infer mods"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-analyzer-concurrency-ide-infer-batch.md");
    assert!(
        adr.contains("S2-I256") && adr.contains("S2-I257") && adr.contains("S2-I258"),
        "batch ADR must name I256–I258"
    );
    assert!(
        adr.contains("S2-I259") && adr.contains("claim"),
        "ADR must lock next = S2-I259 S2-05 claim"
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
