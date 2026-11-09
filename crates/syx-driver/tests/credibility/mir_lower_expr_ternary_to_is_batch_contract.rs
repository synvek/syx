//! S2-I184–I188 S2-05 syx-mir lower_expr Ternary→Is batch extract contracts.
//! Nails: 5 modules + wiring + Cast helpers + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_ternary_to_is_batch_baseline() {
    let mods = [
        (
            "crates/syx-mir/src/mir_lower_expr_ternary.rs",
            "lower_expr_ternary",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_tuple.rs",
            "lower_expr_tuple",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_cast.rs",
            "lower_expr_cast",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_optional_cast.rs",
            "lower_expr_optional_cast",
        ),
        ("crates/syx-mir/src/mir_lower_expr_is.rs", "lower_expr_is"),
    ];
    for (path, fn_name) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(&format!("fn {fn_name}"))
                || src.contains(&format!("pub(crate) fn {fn_name}")),
            "{path} must hold {fn_name}"
        );
    }

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    for mod_name in [
        "mir_lower_expr_ternary",
        "mir_lower_expr_tuple",
        "mir_lower_expr_cast",
        "mir_lower_expr_optional_cast",
        "mir_lower_expr_is",
    ] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    let shell = repo_file("crates/syx-mir/src/mir_lower_expr.rs");
    for fn_name in [
        "lower_expr_ternary(",
        "lower_expr_tuple(",
        "lower_expr_cast(",
        "lower_expr_optional_cast(",
        "lower_expr_is(",
    ] {
        assert!(
            lib.contains(fn_name) || shell.contains(fn_name),
            "lower_expr shell must delegate via {fn_name}"
        );
    }
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );
    assert!(
        lib.contains("pub(crate) fn infer_expr_init_type")
            || lib.contains("infer_expr_init_type,"),
        "infer_expr_init_type must be pub(crate) for Cast sidecar (def or re-export)"
    );
    assert!(
        lib.contains("pub(crate) fn emit_integer_numeric_cast_if_needed")
            || lib.contains("emit_integer_numeric_cast_if_needed,"),
        "emit_integer_numeric_cast_if_needed must be pub(crate) (def or re-export)"
    );
    assert!(
        lib.contains("pub(crate) fn emit_float_to_int_numeric_cast_if_needed")
            || lib.contains("emit_float_to_int_numeric_cast_if_needed,"),
        "emit_float_to_int_numeric_cast_if_needed must be pub(crate) (def or re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-ternary-to-is-batch.md");
    assert!(
        adr.contains("S2-I184")
            && adr.contains("S2-I185")
            && adr.contains("S2-I186")
            && adr.contains("S2-I187")
            && adr.contains("S2-I188"),
        "batch ADR must name I184–I188"
    );
    assert!(
        adr.contains("S2-I189") && adr.contains("Call"),
        "ADR must lock next knife = S2-I189 Call shell"
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
