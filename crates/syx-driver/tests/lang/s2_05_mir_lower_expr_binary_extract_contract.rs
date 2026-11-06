//! S2-I178 S2-05 syx-mir mir_lower_expr_binary extract contracts.
//! Nails: binary module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-binary-extract.md.

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
fn s2_05_mir_lower_expr_binary_extract_baseline() {
    let binary = repo_file("crates/syx-mir/src/mir_lower_expr_binary.rs");
    assert!(
        binary.contains("fn lower_expr_binary")
            || binary.contains("pub(crate) fn lower_expr_binary"),
        "mir_lower_expr_binary must hold lower_expr_binary"
    );
    assert!(
        binary.contains("BinOp::") || binary.contains("NilCoalesce"),
        "binary module should hold Binary lowering"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_binary;"),
        "lib.rs must declare mod mir_lower_expr_binary"
    );
    assert!(
        (lib.contains("lower_expr_binary(") || mir_lower_expr_src().contains("lower_expr_binary(")),
        "lib.rs must delegate Binary to lower_expr_binary"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );
    assert!(
        lib.contains("mod mir_lower_expr_unary;") || lib.contains("Expr::Unary"),
        "Unary remains reachable (lib arm or mir_lower_expr_unary after I179)"
    );
    assert!(
        (lib.contains("pub(crate) fn binary_numeric_kind")
            || lib.contains("binary_numeric_kind,"))
            && (lib.contains("pub(crate) fn binop_operator_symbol")
                || lib.contains("binop_operator_symbol,")),
        "Binary helpers must be pub(crate) in lib.rs (def or re-export)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-binary-extract.md");
    assert!(adr.contains("S2-I178"), "binary ADR must name S2-I178");
    assert!(
        adr.contains("S2-I179") && adr.contains("Unary"),
        "ADR must lock next knife = S2-I179 Unary"
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
