//! S2-I180 S2-05 syx-mir mir_lower_expr_closure extract contracts.
//! Nails: closure module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-closure-extract.md.

fn mir_lower_expr_src() -> String {
    repo_file("crates/syx-mir/src/mir_lower_expr.rs")
}

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_closure_extract_baseline() {
    let closure = repo_file("crates/syx-mir/src/mir_lower_expr_closure.rs");
    assert!(
        closure.contains("fn lower_expr_closure")
            || closure.contains("pub(crate) fn lower_expr_closure"),
        "mir_lower_expr_closure must hold lower_expr_closure"
    );
    assert!(
        closure.contains("MakeClosure") || closure.contains("collect_closure_free_vars"),
        "closure module should hold Closure lowering"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_closure;"),
        "lib.rs must declare mod mir_lower_expr_closure"
    );
    assert!(
        (lib.contains("lower_expr_closure(") || mir_lower_expr_src().contains("lower_expr_closure(")),
        "lib.rs must delegate Closure to lower_expr_closure"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );
    assert!(
        lib.contains("mod mir_lower_expr_index;")
            || lib.contains("Expr::Index {")
            || lib.contains("Expr::Index{"),
        "Index remains reachable (lib arm or mir_lower_expr_index after I181)"
    );
    let free_vars = repo_file("crates/syx-mir/src/mir_collect_closure_free_vars.rs");
    assert!(
        (lib.contains("pub(crate) fn collect_closure_free_vars")
            || free_vars.contains("fn collect_closure_free_vars")
            || lib.contains("mod mir_collect_closure_free_vars;"))
            && (lib.contains("pub(crate) fn lower_function_with_params")
                || lib.contains("lower_function_with_params,")),
        "collect_closure_free_vars remains reachable; lower_function_with_params stays reachable (def or re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-closure-extract.md");
    assert!(adr.contains("S2-I180"), "closure ADR must name S2-I180");
    assert!(
        adr.contains("S2-I181") && adr.contains("Index"),
        "ADR must lock next knife = S2-I181 Index"
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
