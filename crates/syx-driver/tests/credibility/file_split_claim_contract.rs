//! S2-I259 S2-05 plan claim contracts.
//! Nails: claim ADR + plan/02 S2-05 [x]; scoped Non-claims; Stage-2 still [~].
//! See docs/adr/stage2-s2-05-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn repo_path(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn s2_05_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage2-s2-05-claim.md");
    assert!(adr.contains("S2-I259"), "S2-05 claim ADR must exist");
    assert!(
        adr.contains("S2-05") && adr.contains("`[x]`"),
        "ADR Status must claim S2-05 [x]"
    );
    assert!(
        adr.contains("Scoped") || adr.contains("scoped"),
        "ADR must define scoped [x] meaning"
    );
    assert!(
        adr.contains("≠") && adr.contains("Stage-2"),
        "ADR must Non-claim Stage-2 exit"
    );
    assert!(
        adr.contains("≠") && (adr.contains("tests") || adr.contains("cfg(test)")),
        "ADR must Non-claim migrate tests / #[cfg(test)]"
    );
    assert!(
        adr.contains("S2-06"),
        "ADR must lock next = S2-06 open"
    );
    assert!(
        adr.contains("31222")
            || adr.contains("census")
            || adr.contains("file-split-census"),
        "ADR Evidence must tie to file-split census"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    let s2_05_line = plan02
        .lines()
        .find(|l| l.contains("| S2-05 |"))
        .expect("plan/02 must have S2-05 row");
    assert!(
        s2_05_line.contains("| [x] |") || s2_05_line.contains("| `[x]` |"),
        "plan/02 S2-05 status must be [x]; got: {s2_05_line}"
    );
    assert!(
        plan02.contains("S2-I259") || plan02.contains("stage2-s2-05-claim"),
        "plan/02 must cite S2-I259 claim"
    );
    assert!(
        plan02.contains("| **2** |") && plan02.contains("`[~]`"),
        "plan/02 stage-2 summary must remain [~]"
    );

    let census = repo_file("docs/census/stage2-file-split-census.md");
    assert!(
        census.contains("S2-I259"),
        "census must name S2-I259"
    );

    let sa = repo_file("crates/syx-semantic/src/semantic_analyzer.rs");
    assert!(
        sa.contains("pub struct SemanticAnalyzer"),
        "analyzer hub must still exist after claim (entry/builtins OK)"
    );
    assert!(
        repo_path("crates/syx-semantic/src/semantic_concurrency.rs").is_file()
            && repo_path("crates/syx-semantic/src/semantic_ide.rs").is_file(),
        "claim evidence peels must remain on disk"
    );
}
