//! S2-I189–I194 S2-05 syx-mir lower_expr Call→MacroInvoke batch extract contracts.
//! Nails: 6 modules + wiring + Call/Assign helpers + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-call-to-macro-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_call_to_macro_batch_baseline() {
    let mods = [
        (
            "crates/syx-mir/src/mir_lower_expr_call.rs",
            "lower_expr_call",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_array.rs",
            "lower_expr_array",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_await.rs",
            "lower_expr_await",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_assign.rs",
            "lower_expr_assign",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_type_self.rs",
            "lower_expr_type_self",
        ),
        (
            "crates/syx-mir/src/mir_lower_expr_macro_invoke.rs",
            "lower_expr_macro_invoke",
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

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    for mod_name in [
        "mir_lower_expr_call",
        "mir_lower_expr_array",
        "mir_lower_expr_await",
        "mir_lower_expr_assign",
        "mir_lower_expr_type_self",
        "mir_lower_expr_macro_invoke",
    ] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    let shell = repo_file("crates/syx-mir/src/mir_lower_expr.rs");
    for fn_name in [
        "lower_expr_call(",
        "lower_expr_array(",
        "lower_expr_await(",
        "lower_expr_assign(",
        "lower_expr_type_self(",
        "lower_expr_macro_invoke(",
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
        lib.contains("pub(crate) fn classify_member_call") || lib.contains("classify_member_call,"),
        "classify_member_call must be pub(crate) for Call sidecar (def or re-export)"
    );
    assert!(
        lib.contains("pub(crate) fn host_simd_lowers_own_args")
            || lib.contains("host_simd_lowers_own_args,"),
        "host_simd_lowers_own_args must be pub(crate) (def or re-export)"
    );
    assert!(
        lib.contains("pub(crate) fn keypath_from_callee") || lib.contains("keypath_from_callee,"),
        "keypath_from_callee must be pub(crate) (def or re-export)"
    );
    assert!(
        lib.contains("pub(crate) fn emit_integer_narrow_cast_if_needed")
            || lib.contains("emit_integer_narrow_cast_if_needed,"),
        "emit_integer_narrow_cast_if_needed must be pub(crate) for Assign sidecar (def or re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-call-to-macro-batch.md");
    assert!(
        adr.contains("S2-I189")
            && adr.contains("S2-I190")
            && adr.contains("S2-I191")
            && adr.contains("S2-I192")
            && adr.contains("S2-I193")
            && adr.contains("S2-I194"),
        "batch ADR must name I189–I194"
    );
    assert!(
        adr.contains("S2-I195") && adr.contains("Range"),
        "ADR must lock next knife = S2-I195 Range cluster"
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
