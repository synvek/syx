//! S2-I175 S2-05 syx-mir mir_lower_expr_member extract contracts.
//! Nails: expr_member module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-member-extract.md.

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
fn s2_05_mir_lower_expr_member_extract_baseline() {
    let member = repo_file("crates/syx-mir/src/mir_lower_expr_member.rs");
    assert!(
        member.contains("fn lower_expr_member")
            || member.contains("pub(crate) fn lower_expr_member"),
        "mir_lower_expr_member must hold lower_expr_member"
    );
    assert!(
        member.contains("ObjectGet") || member.contains("extension_call_op"),
        "expr_member module should hold Member-read lowering"
    );
    assert!(
        !member.contains("pub fn lower_module"),
        "mir_lower_expr_member must not pull lower_module"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_member;"),
        "lib.rs must declare mod mir_lower_expr_member"
    );
    assert!(
        (lib.contains("lower_expr_member(") || mir_lower_expr_src().contains("lower_expr_member(")),
        "lib.rs must delegate top-level Expr::Member to lower_expr_member"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain in lib.rs"
    );
    assert!(
        lib.contains("mod mir_lower_expr_member_assign;")
            || lib.contains("Expr::MemberAssign"),
        "MemberAssign remains reachable (lib arm or mir_lower_expr_member_assign after I176)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-member-extract.md");
    assert!(adr.contains("S2-I175"), "member ADR must name S2-I175");
    assert!(
        adr.contains("S2-I176") && adr.contains("MemberAssign"),
        "ADR must lock next knife = S2-I176 MemberAssign"
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
