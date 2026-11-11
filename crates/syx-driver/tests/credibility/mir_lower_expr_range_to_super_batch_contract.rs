//! S2-I195–I200 S2-05 syx-mir lower_expr Range→Super batch extract contracts.
//! Nails: 6 modules + wiring + Range helpers + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-range-to-super-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_range_to_super_batch_baseline() {
    let mods = [
        (
            "crates/syx-mir/src/mir_lower_expr_range.rs",
            "lower_range_dict",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_ident.rs",
            "lower_expr_ident",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_group.rs",
            "lower_expr_group",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_borrow.rs",
            "lower_expr_borrow",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_keypath.rs",
            "lower_expr_keypath",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_super.rs",
            "lower_expr_super",
        ),
    ];
    for (path, fn_name) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(&format!("fn {fn_name}"))
                || src.contains(&format!("pub(crate) fn {fn_name}")),
            "{path} must hold {fn_name}"
        );
    }

    let range = repo_file("crates/syx-mir/src/mir_lower_expr_range.rs");
    assert!(
        range.contains("range_bounds_from_expr")
            && range.contains("lower_range_contains_mir")
            && range.contains("lower_range_dict_dyn"),
        "range module must hold full Range helper family"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    for mod_name in [
        "mir_lower_expr_range",
        "mir_lower_expr_ident",
        "mir_lower_expr_group",
        "mir_lower_expr_borrow",
        "mir_lower_expr_keypath",
        "mir_lower_expr_super",
    ] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    let shell = repo_file("crates/syx-mir/src/mir_lower_expr.rs");
    for fn_name in [
        "lower_expr_half_open_range(",
        "lower_expr_closed_range(",
        "lower_expr_range_dyn(",
        "lower_expr_ident(",
        "lower_expr_group(",
        "lower_expr_borrow(",
        "lower_expr_keypath(",
        "lower_expr_super(",
    ] {
        assert!(
            lib.contains(fn_name) || shell.contains(fn_name),
            "lower_expr shell must delegate via {fn_name}"
        );
    }
    assert!(
        !lib.contains("fn lower_range_dict(")
            && !lib.contains("fn lower_range_dict_dyn(")
            && !lib.contains("fn range_bounds_from_expr(")
            && !lib.contains("fn lower_range_contains_mir("),
        "Range helpers must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );

    let call_member = repo_file("crates/syx-mir/src/mir_lower_expr_call_member.rs");
    assert!(
        call_member.contains("mir_lower_expr_range")
            && (call_member.contains("range_bounds_from_expr")
                || call_member.contains("lower_range_contains_mir")),
        "call_member must import Range helpers from mir_lower_expr_range"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-range-to-super-batch.md");
    assert!(
        adr.contains("S2-I195")
            && adr.contains("S2-I196")
            && adr.contains("S2-I197")
            && adr.contains("S2-I198")
            && adr.contains("S2-I199")
            && adr.contains("S2-I200"),
        "batch ADR must name I195–I200"
    );
    assert!(
        adr.contains("S2-I201") && adr.contains("collect_closure_free_vars"),
        "ADR must lock next knife = S2-I201 collect_closure_free_vars"
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
