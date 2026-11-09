//! S2-I145…I151 S2-04 CST reindent + write-migration contracts.
//! Nails: format_cst_reindent + GA default format_source_cst; format_cst / dry_run_cst concat;
//! format_file → format_dry_run (I151). ≠ S2-04 [x].
//! See docs/adr/stage2-s2-04-cst-reindent-*.md · stage2-s2-04-fmt-write-migration.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_04_cst_reindent_func_body() {
    let adr = repo_file("docs/adr/stage2-s2-04-cst-reindent-func-body.md");
    assert!(
        adr.contains("S2-I145"),
        "cst-reindent-func-body ADR must exist"
    );
    assert!(
        adr.contains("format_cst_reindent"),
        "ADR must name opt-in format_cst_reindent"
    );

    let nested = repo_file("docs/adr/stage2-s2-04-cst-reindent-func-nested.md");
    assert!(nested.contains("S2-I146"), "nested ADR must exist");

    let two_to_four = repo_file("docs/adr/stage2-s2-04-cst-reindent-func-2to4.md");
    assert!(two_to_four.contains("S2-I147"), "2to4 ADR must exist");

    let class_stored = repo_file("docs/adr/stage2-s2-04-cst-reindent-class-stored.md");
    assert!(class_stored.contains("S2-I148"), "class-stored ADR must exist");

    let gate = repo_file("docs/adr/stage2-s2-04-cst-reindent-env-gate.md");
    assert!(gate.contains("S2-I149"), "env-gate ADR must exist");

    let ga = repo_file("docs/adr/stage2-s2-04-cst-reindent-ga.md");
    assert!(ga.contains("S2-I150"), "cst-reindent-ga ADR must exist");
    assert!(
        ga.contains("format_source_cst") && ga.contains("format_cst_reindent"),
        "GA ADR must lock format_source_cst → format_cst_reindent"
    );
    assert!(
        ga.contains("≠") && ga.contains("format_file"),
        "GA ADR must Non-claim format_file write migration"
    );
    assert!(
        ga.contains("≠") && (ga.contains("S2-04") || ga.contains("[x]")),
        "GA ADR must Non-claim S2-04 [x]"
    );

    let cst = repo_file("crates/syx-cst/src/lib.rs");
    assert!(
        cst.contains("pub fn format_cst_reindent("),
        "syx-cst must export format_cst_reindent"
    );
    assert!(
        cst.contains("func_body_depths"),
        "format_cst_reindent must stack FunctionDecl body depths"
    );
    assert!(
        cst.contains("class_body_depths"),
        "format_cst_reindent must stack class body depths"
    );
    let fmt_idx = cst
        .find("pub fn format_cst(")
        .expect("format_cst must exist");
    let fmt_chunk = &cst[fmt_idx..fmt_idx + 400.min(cst.len() - fmt_idx)];
    assert!(
        fmt_chunk.contains("push_str(&tok.lexeme)"),
        "format_cst must stay lexeme concat"
    );
    assert!(
        !fmt_chunk.contains("format_cst_reindent"),
        "format_cst must not call format_cst_reindent"
    );

    let fmt = repo_file("crates/syx-fmt/src/lib.rs");
    let cst_fn = fmt
        .find("pub fn format_source_cst(")
        .expect("format_source_cst must exist");
    let cst_chunk = &fmt[cst_fn..cst_fn + 500.min(fmt.len() - cst_fn)];
    assert!(
        cst_chunk.contains("format_cst_reindent"),
        "format_source_cst must call format_cst_reindent (GA)"
    );
    assert!(
        !cst_chunk.contains("env_cst_reindent_enabled"),
        "format_source_cst must not use env gate after GA"
    );
    let dry_idx = fmt
        .find("pub fn format_dry_run_cst(")
        .expect("format_dry_run_cst must exist");
    let dry_chunk = &fmt[dry_idx..dry_idx + 900.min(fmt.len() - dry_idx)];
    assert!(
        dry_chunk.contains("format_cst(&"),
        "format_dry_run_cst must call format_cst"
    );
    assert!(
        !dry_chunk.contains("format_source_cst("),
        "format_dry_run_cst must not call format_source_cst"
    );

    let migrate = repo_file("docs/adr/stage2-s2-04-fmt-write-migration.md");
    assert!(
        migrate.contains("S2-I151"),
        "fmt-write-migration ADR must exist"
    );

    let driver = repo_file("crates/syx-driver/src/lib.rs");
    let write_idx = driver
        .find("fn format_file(")
        .expect("format_file must exist");
    let write_chunk = &driver[write_idx..write_idx + 1200.min(driver.len() - write_idx)];
    assert!(
        write_chunk.contains("format_dry_run("),
        "format_file must call format_dry_run after I151 write migration"
    );
    assert!(
        !write_chunk.contains("format_source(&source)"),
        "format_file must not call AST format_source as write SSOT"
    );
    assert!(
        write_chunk.contains("S2-I151") || write_chunk.contains("write-migration"),
        "format_file must document S2-I151 write migration"
    );
}
