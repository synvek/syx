//! S2-I183 S2-05 syx-mir mir_lower_expr_compound_assign extract contracts.
//! Nails: compound_assign module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-compound-assign-extract.md.

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
fn s2_05_mir_lower_expr_compound_assign_extract_baseline() {
    let compound = repo_file("crates/syx-mir/src/mir_lower_expr_compound_assign.rs");
    assert!(
        compound.contains("fn lower_expr_compound_assign")
            || compound.contains("pub(crate) fn lower_expr_compound_assign"),
        "mir_lower_expr_compound_assign must hold lower_expr_compound_assign"
    );
    assert!(
        compound.contains("CompoundAssignOp") || compound.contains("__syx_int_add_assign"),
        "compound_assign module should hold CompoundAssign lowering"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_compound_assign;"),
        "lib.rs must declare mod mir_lower_expr_compound_assign"
    );
    assert!(
        (lib.contains("lower_expr_compound_assign(") || mir_lower_expr_src().contains("lower_expr_compound_assign(")),
        "lib.rs must delegate CompoundAssign to lower_expr_compound_assign"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );
    assert!(
        lib.contains("Expr::Ternary") || lib.contains("mod mir_lower_expr_ternary;"),
        "Ternary remains reachable (lib arm or mir_lower_expr_ternary after I184)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-compound-assign-extract.md");
    assert!(adr.contains("S2-I183"), "compound_assign ADR must name S2-I183");
    assert!(
        adr.contains("S2-I184") && adr.contains("Ternary"),
        "ADR must lock next knife = S2-I184 Ternary"
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
