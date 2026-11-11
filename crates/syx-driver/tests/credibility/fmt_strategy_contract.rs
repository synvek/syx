//! S2-I144 / S2-I151 S2-04 fmt convergence-strategy contracts.
//! Nails: write = format_dry_run → format_source_auto; --check = same SSOT.
//! ≠ S2-04 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-04-convergence-strategy.md · stage2-s2-04-fmt-write-migration.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_fmt_write_check_strategy() {
    let strategy = repo_file("docs/adr/stage2-s2-04-convergence-strategy.md");
    assert!(
        strategy.contains("S2-I144"),
        "convergence-strategy ADR must exist"
    );
    assert!(
        strategy.contains("CST reindent") || strategy.contains("reindent GA"),
        "strategy must lock CST reindent as write-migration prerequisite"
    );

    let migrate = repo_file("docs/adr/stage2-s2-04-fmt-write-migration.md");
    assert!(
        migrate.contains("S2-I151"),
        "fmt-write-migration ADR must exist"
    );
    assert!(
        migrate.contains("format_dry_run") || migrate.contains("format_source_auto"),
        "write-migration ADR must lock dry-run/auto SSOT"
    );
    assert!(
        migrate.contains("≠") && (migrate.contains("S2-04") || migrate.contains("[x]")),
        "write-migration ADR must Non-claim S2-04 [x]"
    );

    let driver = repo_file("crates/syx-driver/src/lib.rs");
    // Write path: format_dry_run (→ format_source_auto), aligned with --check.
    let write_idx = driver
        .find("fn format_file(")
        .expect("format_file must exist");
    let write_chunk = &driver[write_idx..write_idx + 1200.min(driver.len() - write_idx)];
    assert!(
        write_chunk.contains("format_dry_run("),
        "format_file must call format_dry_run (write/`--check` SSOT)"
    );
    assert!(
        !write_chunk.contains("format_source(&source)"),
        "format_file must not call AST format_source as write SSOT after I151"
    );
    assert!(
        write_chunk.contains("S2-I151") || write_chunk.contains("write-migration"),
        "format_file must document S2-I151 write migration"
    );

    // --check default: format_dry_run → format_source_auto.
    assert!(
        driver.contains("format_check_path_with(path, syx_fmt::format_dry_run)"),
        "format_check_path must use format_dry_run"
    );
    let fmt = repo_file("crates/syx-fmt/src/lib.rs");
    let dry = fmt
        .find("pub fn format_dry_run(")
        .expect("format_dry_run must exist");
    let dry_chunk = &fmt[dry..dry + 800.min(fmt.len() - dry)];
    assert!(
        dry_chunk.contains("format_source_auto"),
        "format_dry_run must call format_source_auto (CST-first check path)"
    );

    let census = repo_file("docs/census/stage2-parser-census.md");
    assert!(
        census.contains("S2-I144") || census.contains("convergence-strategy"),
        "census must note I144 strategy lock"
    );
}
