//! S2-I163 S2-04 dual-grammar freeze charter contracts.
//! Nails: ADR Decision locks (stop take_* expand, tools=CST+lower, compile=Parser,
//! KPI freeze baseline); ≠ S2-04 [x] / ≠ FrontendPipeline / ≠ new take_*.
//! See docs/adr/stage2-s2-04-dual-grammar-freeze-charter.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_dual_grammar_freeze_charter_locked() {
    let adr = repo_file("docs/adr/stage2-s2-04-dual-grammar-freeze-charter.md");
    assert!(adr.contains("S2-I163"), "freeze charter ADR must exist");
    assert!(
        adr.contains("stop expanding") || adr.contains("Default stop expanding"),
        "ADR must lock default stop expanding take_*"
    );
    assert!(
        adr.contains("KPI freeze") || adr.contains("combined_grammar_loc"),
        "ADR must lock KPI freeze baseline"
    );
    assert!(
        adr.contains("12381"),
        "ADR must nail freeze combined_grammar_loc baseline"
    );
    assert!(
        adr.contains("lower_cst_subset_probe") && adr.contains("lint_source_auto"),
        "ADR must document tools = CST + lower / lint_source_auto"
    );
    assert!(
        adr.contains("FrontendPipeline") && adr.contains("Parser"),
        "ADR must document compile = Parser / FrontendPipeline until migrate"
    );
    assert!(
        adr.contains("≠") && (adr.contains("S2-04") || adr.contains("[x]")),
        "ADR must Non-claim S2-04 [x]"
    );
    assert!(
        adr.contains("≠") && adr.contains("FrontendPipeline"),
        "ADR must Non-claim FrontendPipeline change this window"
    );
    assert!(
        (adr.contains("`[x]` claim") || adr.contains("[x] claim"))
            && (adr.contains("Next default") || adr.contains("next default")),
        "ADR must name S2-04 [x] claim as next default"
    );

    let census = repo_file("docs/census/stage2-parser-census.md");
    assert!(
        census.contains("S2-I163") || census.contains("Freeze baseline"),
        "census must record freeze / S2-I163"
    );
    assert!(
        census.contains("12381") || census.contains("combined_grammar_loc"),
        "census must carry freeze combined LOC"
    );

    let frontend_toml = repo_file("crates/syx-frontend/Cargo.toml");
    assert!(
        frontend_toml.contains("syx-parser"),
        "Frontend must still depend on syx-parser"
    );
    assert!(
        !frontend_toml.contains("syx-cst"),
        "Frontend must not gain syx-cst in freeze window"
    );

    let frontend = repo_file("crates/syx-frontend/src/lib.rs");
    assert!(
        frontend.contains("Parser::parse"),
        "FrontendPipeline must still call Parser::parse"
    );

    let lint = repo_file("crates/syx-lint/src/lib.rs");
    let auto_idx = lint
        .find("pub fn lint_source_auto(")
        .expect("lint_source_auto must exist");
    let auto_chunk = &lint[auto_idx..auto_idx + 900.min(lint.len() - auto_idx)];
    assert!(
        auto_chunk.contains("lower_cst_subset_probe"),
        "lint_source_auto must still call lower_cst_subset_probe"
    );
}
