//! S2-I201–I204 S2-05 syx-mir free_vars→shells batch extract contracts.
//! Nails: 4 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-free-vars-to-shells-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_free_vars_to_shells_batch_baseline() {
    let mods = [
        (
            "crates/syx-mir/src/mir_collect_closure_free_vars.rs",
            "collect_closure_free_vars",
        ),
        (
            "crates/syx-mir/src/mir_lower_switch_pattern.rs",
            "lower_switch_pattern_match",
        ),
        ("crates/syx-mir/src/mir_lower_stmt.rs", "lower_stmt"),
        ("crates/syx-mir/src/mir_lower_shells.rs", "mir_shell_fn"),
    ];
    for (path, fn_name) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(&format!("fn {fn_name}"))
                || src.contains(&format!("pub(crate) fn {fn_name}"))
                || src.contains(&format!("pub fn {fn_name}")),
            "{path} must hold {fn_name}"
        );
    }

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    for mod_name in [
        "mir_collect_closure_free_vars",
        "mir_lower_switch_pattern",
        "mir_lower_stmt",
        "mir_lower_shells",
    ] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("lower_stmt(") || lib.contains("use mir_lower_stmt::lower_stmt"),
        "lib.rs must wire lower_stmt"
    );
    assert!(
        lib.contains("lower_dataclass_shells")
            && lib.contains("merge_mir_functions")
            && lib.contains("prefix_user_dep_mir_batch"),
        "lib.rs must pub-use public shell APIs"
    );
    assert!(
        !lib.contains("pub(crate) fn collect_closure_free_vars(")
            && !lib.contains("\nfn lower_switch_pattern_match(")
            && !lib.contains("\npub(crate) fn lower_stmt(")
            && !lib.contains("\nfn mir_shell_fn("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-free-vars-to-shells-batch.md");
    assert!(
        adr.contains("S2-I201")
            && adr.contains("S2-I202")
            && adr.contains("S2-I203")
            && adr.contains("S2-I204"),
        "batch ADR must name I201–I204"
    );
    assert!(
        adr.contains("S2-I205") && adr.contains("lower_type_members"),
        "ADR must lock next knife = S2-I205 lower_type_members"
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
