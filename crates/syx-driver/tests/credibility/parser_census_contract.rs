//! S2-I143 S2-04 parser dual-track census contracts (open baseline).
//! Proof lives in tests/ + docs/ only (≠ unify trees / ≠ delete syx-cst).
//! See docs/adr/stage2-s2-04-open.md · docs/census/stage2-parser-census.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_parser_dual_track_baseline() {
    // Census + open ADR must exist (S2-I143 baseline artifacts).
    let census = repo_file("docs/census/stage2-parser-census.md");
    assert!(
        census.contains("S2-I143"),
        "parser census must name S2-I143 baseline"
    );
    assert!(
        census.contains("Write vs `--check`")
            || census.contains("Remaining honesty debt")
            || census.contains("lint still AST"),
        "census must document fmt honesty debt (historical split or post-I152 remaining)"
    );
    assert!(
        census.contains("≠") && census.contains("S2-04"),
        "census must keep Non-claim honesty for S2-04"
    );

    let adr = repo_file("docs/adr/stage2-s2-04-open.md");
    assert!(
        adr.contains("S2-I143"),
        "S2-04 open ADR must exist"
    );
    assert!(
        adr.contains("≠ S2-04 `[x]`") || adr.contains("≠ S2-04 [x]"),
        "open ADR must Non-claim S2-04 [x]"
    );

    // Honest dual-track: both grammar faces and fmt entries still present.
    let parser = repo_file("crates/syx-parser/src/lib.rs");
    let cst = repo_file("crates/syx-cst/src/lib.rs");
    let fmt = repo_file("crates/syx-fmt/src/lib.rs");
    assert!(
        parser.contains("impl Parser") || parser.contains("pub struct Parser"),
        "syx-parser production face must remain"
    );
    assert!(
        cst.contains("parse_cst_subset") || cst.contains("fn parse_cst"),
        "syx-cst face must remain at open baseline (≠ deleted)"
    );
    assert!(
        fmt.contains("pub fn format_source_auto"),
        "fmt CST-first auto path must remain"
    );
    assert!(
        fmt.contains("pub fn format_source("),
        "fmt AST pretty path must remain (write)"
    );
    assert!(
        fmt.contains("CST001"),
        "CST001 fallback charter must remain"
    );

    // Script present for re-runnable KPI.
    let script = repo_file("scripts/s2_04_parser_census.sh");
    assert!(
        script.contains("syx_parser_lib_loc"),
        "census script must emit parser LOC KPI"
    );
}
