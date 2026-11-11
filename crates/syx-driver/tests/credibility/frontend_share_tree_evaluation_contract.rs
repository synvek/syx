//! S2-I159 S2-04 Frontend share-tree evaluation contracts.
//! Nails: ADR Decision locks (CST→AST lower, deepen stop-line, [x] checklist);
//! ≠ FrontendPipeline impl / ≠ delete syx-cst / ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-frontend-share-tree-evaluation.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_frontend_share_tree_evaluation_locked() {
    let adr = repo_file("docs/adr/stage2-s2-04-frontend-share-tree-evaluation.md");
    assert!(
        adr.contains("S2-I159"),
        "frontend share-tree evaluation ADR must exist"
    );
    assert!(
        adr.contains("CST→AST lower") || adr.contains("CST->AST lower"),
        "ADR must lock CST→AST lower as preferred path"
    );
    assert!(
        adr.contains("Deepen stop-line") || adr.contains("deepen stop"),
        "ADR must lock deepen stop-line"
    );
    assert!(
        adr.contains("acceptance checklist") || adr.contains("`[x]` acceptance"),
        "ADR must publish S2-04 [x] acceptance checklist"
    );
    assert!(
        adr.contains("≠") && adr.contains("FrontendPipeline"),
        "ADR must Non-claim FrontendPipeline implementation this window"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-04") || adr.contains("[x]")),
        "ADR must Non-claim S2-04 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("rowan"),
        "ADR must Non-claim rowan"
    );
    assert!(
        adr.contains("lower probe") || adr.contains("CST→AST lower probe"),
        "ADR Consequences must name lower probe as next default"
    );

    // Honesty: Frontend still pure parser (evaluation ≠ migrate).
    let frontend_toml = repo_file("crates/syx-frontend/Cargo.toml");
    assert!(
        frontend_toml.contains("syx-parser"),
        "Frontend must still depend on syx-parser"
    );
    assert!(
        !frontend_toml.contains("syx-cst"),
        "Frontend must not gain syx-cst dep in evaluation window"
    );
}
