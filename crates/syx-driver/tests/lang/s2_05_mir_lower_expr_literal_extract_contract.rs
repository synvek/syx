//! S2-I172 S2-05 syx-mir mir_lower_expr_literal extract contracts.
//! Nails: literal module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-literal-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_literal_extract_baseline() {
    let lit = repo_file("crates/syx-mir/src/mir_lower_expr_literal.rs");
    assert!(
        lit.contains("fn lower_expr_literal") || lit.contains("pub(crate) fn lower_expr_literal"),
        "mir_lower_expr_literal must hold lower_expr_literal"
    );
    assert!(
        lit.contains("fn contextual_const_bits"),
        "mir_lower_expr_literal must hold contextual_const_bits"
    );
    assert!(
        !lit.contains("pub fn lower_module"),
        "mir_lower_expr_literal must not pull lower_module"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_literal;"),
        "lib.rs must declare mod mir_lower_expr_literal"
    );
    let shell = repo_file("crates/syx-mir/src/mir_lower_expr.rs");
    assert!(
        lib.contains("lower_expr_literal(expr, dest, ctx, block)")
            || shell.contains("lower_expr_literal(expr, dest, ctx, block)"),
        "lower_expr shell must delegate Expr::Literal to lower_expr_literal"
    );
    assert!(
        !lib.lines().any(|l| l.starts_with("fn contextual_const_bits")
            || l.starts_with("pub(crate) fn contextual_const_bits")),
        "contextual_const_bits must leave lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-literal-extract.md");
    assert!(adr.contains("S2-I172"), "literal ADR must name S2-I172");
    assert!(
        adr.contains("S2-I173") && (adr.contains("Call") || adr.contains("lower_expr")),
        "ADR must lock next knife = S2-I173 lower_expr large-arm continuation"
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
