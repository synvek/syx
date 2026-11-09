//! S2-I247–I248 S2-05 syx-semantic production close-out batch contracts.
//! Nails: concurrency/IDE left lib (I247–I248); may live in sidecars after I256–I257.
//! See docs/adr/stage2-s2-05-semantic-production-closeout-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn holds_fn(src: &str, name: &str) -> bool {
    src.contains(&format!("fn {name}("))
        || src.contains(&format!("pub(crate) fn {name}("))
        || src.contains(&format!("pub fn {name}("))
}

#[test]
fn s2_05_semantic_production_closeout_batch_baseline() {
    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    let conc = repo_file("crates/syx-semantic/src/semantic_concurrency.rs");
    let ide = repo_file("crates/syx-semantic/src/semantic_ide.rs");
    assert!(
        holds_fn(&conc, "check_concurrency_rules") || holds_fn(&sa, "check_concurrency_rules"),
        "check_concurrency_rules must live in semantic_concurrency (I256) or analyzer"
    );
    assert!(
        holds_fn(&ide, "members_for_type") || holds_fn(&sa, "members_for_type"),
        "members_for_type must live in semantic_ide (I257) or analyzer"
    );

    let lib = repo_file("crates/syx-semantic/src/lib.rs");
    assert!(
        !lib.contains("impl SemanticAnalyzer"),
        "lib.rs must no longer host SemanticAnalyzer production impl"
    );
    assert!(
        lib.contains("#[cfg(test)]"),
        "#[cfg(test)] must remain in lib (not migrated this window)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-semantic-production-closeout-batch.md");
    assert!(
        adr.contains("S2-I247") && adr.contains("S2-I248"),
        "batch ADR must name I247–I248"
    );
    assert!(
        adr.contains("S2-I249"),
        "ADR must lock next = S2-I249 helpers/claim-prep"
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
