//! S2-I164 S2-04 plan claim contracts.
//! Nails: claim ADR + plan/02 S2-04 [x]; scoped Non-claims; Frontend still Parser-only;
//! freeze charter retained. ≠ Stage-2 exit / ≠ Frontend on CST / ≠ delete syx-cst.
//! See docs/adr/stage2-s2-04-claim.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_plan_claim_locked() {
    let adr = repo_file("docs/adr/stage2-s2-04-claim.md");
    assert!(adr.contains("S2-I164"), "S2-04 claim ADR must exist");
    assert!(
        adr.contains("S2-04") && adr.contains("`[x]`"),
        "ADR Status must claim S2-04 [x]"
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
        adr.contains("≠")
            && (adr.contains("Frontend already") || adr.contains("Frontend already on CST")),
        "ADR must Non-claim Frontend already on CST"
    );
    assert!(
        adr.contains("≠") && adr.contains("syx-cst"),
        "ADR must Non-claim delete syx-cst"
    );
    assert!(
        adr.contains("acceptance checklist")
            || adr.contains("I159")
            || adr.contains("Decision 7"),
        "ADR Evidence must tie to I159 checklist"
    );

    let plan02 = repo_file("plan/02-improvement-roadmap.md");
    // S2-04 row must show [x] (not still [~] for the task).
    let s2_04_line = plan02
        .lines()
        .find(|l| l.contains("| S2-04 |"))
        .expect("plan/02 must have S2-04 row");
    assert!(
        s2_04_line.contains("| [x] |") || s2_04_line.contains("| `[x]` |"),
        "plan/02 S2-04 status must be [x]; got: {s2_04_line}"
    );
    assert!(
        plan02.contains("S2-I164") || plan02.contains("stage2-s2-04-claim"),
        "plan/02 must cite S2-I164 claim"
    );
    // Stage-2 overall still in progress.
    assert!(
        plan02.contains("| **2** |") && plan02.contains("`[~]`"),
        "plan/02 stage-2 summary must remain [~]"
    );

    let freeze = repo_file("docs/adr/stage2-s2-04-dual-grammar-freeze-charter.md");
    assert!(
        freeze.contains("S2-I163"),
        "freeze charter must remain as claim evidence"
    );

    let frontend_toml = repo_file("crates/syx-frontend/Cargo.toml");
    assert!(
        frontend_toml.contains("syx-parser"),
        "Frontend must still depend on syx-parser after claim"
    );
    assert!(
        !frontend_toml.contains("syx-cst"),
        "Frontend must not gain syx-cst in claim window"
    );

    let frontend = repo_file("crates/syx-frontend/src/lib.rs");
    assert!(
        frontend.contains("Parser::parse"),
        "FrontendPipeline must still call Parser::parse (interim honesty)"
    );
}
