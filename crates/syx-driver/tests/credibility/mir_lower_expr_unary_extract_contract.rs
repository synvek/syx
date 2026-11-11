//! S2-I179 S2-05 syx-mir mir_lower_expr_unary extract contracts.
//! Nails: unary module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-unary-extract.md.

fn mir_lower_expr_src() -> String {
    repo_file("crates/syx-mir/src/mir_lower_expr.rs")
}

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_unary_extract_baseline() {
    let unary = repo_file("crates/syx-mir/src/mir_lower_expr_unary.rs");
    assert!(
        unary.contains("fn lower_expr_unary") || unary.contains("pub(crate) fn lower_expr_unary"),
        "mir_lower_expr_unary must hold lower_expr_unary"
    );
    assert!(
        unary.contains("UnaryOp::") || unary.contains("BeginTry"),
        "unary module should hold Unary lowering"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_unary;"),
        "lib.rs must declare mod mir_lower_expr_unary"
    );
    assert!(
        (lib.contains("lower_expr_unary(") || mir_lower_expr_src().contains("lower_expr_unary(")),
        "lib.rs must delegate Unary to lower_expr_unary"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );
    assert!(
        lib.contains("mod mir_lower_expr_closure;") || lib.contains("Expr::Closure"),
        "Closure remains reachable (lib arm or mir_lower_expr_closure after I180)"
    );
    assert!(
        lib.contains("pub(crate) fn unary_numeric_kind") || lib.contains("unary_numeric_kind,"),
        "unary_numeric_kind must be pub(crate) in lib.rs (def or re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-unary-extract.md");
    assert!(adr.contains("S2-I179"), "unary ADR must name S2-I179");
    assert!(
        adr.contains("S2-I180") && adr.contains("Closure"),
        "ADR must lock next knife = S2-I180 Closure"
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
