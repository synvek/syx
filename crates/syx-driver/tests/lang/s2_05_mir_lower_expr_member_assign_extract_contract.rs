//! S2-I176 S2-05 syx-mir mir_lower_expr_member_assign extract contracts.
//! Nails: member_assign module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-member-assign-extract.md.

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
fn s2_05_mir_lower_expr_member_assign_extract_baseline() {
    let assign = repo_file("crates/syx-mir/src/mir_lower_expr_member_assign.rs");
    assert!(
        assign.contains("fn lower_expr_member_assign")
            || assign.contains("pub(crate) fn lower_expr_member_assign"),
        "mir_lower_expr_member_assign must hold lower_expr_member_assign"
    );
    assert!(
        assign.contains("ObjectSet") || assign.contains("property_setters"),
        "member_assign module should hold assign lowering"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_member_assign;"),
        "lib.rs must declare mod mir_lower_expr_member_assign"
    );
    assert!(
        (lib.contains("lower_expr_member_assign(") || mir_lower_expr_src().contains("lower_expr_member_assign(")),
        "lib.rs must delegate MemberAssign to lower_expr_member_assign"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain in lib.rs"
    );
    assert!(
        lib.contains("mod mir_lower_expr_index_assign;")
            || lib.contains("Expr::IndexAssign"),
        "IndexAssign remains reachable (lib arm or mir_lower_expr_index_assign after I177)"
    );
    assert!(
        lib.contains("mod mir_types;") && lib.contains("mod mir_lower_ctx;"),
        "post-I175 repair: types + lower_ctx wiring must remain"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-member-assign-extract.md");
    assert!(adr.contains("S2-I176"), "assign ADR must name S2-I176");
    assert!(
        adr.contains("S2-I177") && adr.contains("IndexAssign"),
        "ADR must lock next knife = S2-I177 IndexAssign"
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
