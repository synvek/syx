//! S2-I174 S2-05 syx-mir mir_lower_expr_call_member extract contracts.
//! Nails: call_member module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-call-member-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_call_member_extract_baseline() {
    let member = repo_file("crates/syx-mir/src/mir_lower_expr_call_member.rs");
    assert!(
        member.contains("fn lower_expr_call_member")
            || member.contains("pub(crate) fn lower_expr_call_member"),
        "mir_lower_expr_call_member must hold lower_expr_call_member"
    );
    assert!(
        member.contains("resolve_host_call") || member.contains("MemberCallKind"),
        "call_member module should hold Member-callee dispatch"
    );
    assert!(
        !member.contains("pub fn lower_module"),
        "mir_lower_expr_call_member must not pull lower_module"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_call_member;"),
        "lib.rs must declare mod mir_lower_expr_call_member"
    );
    let call_shell = repo_file("crates/syx-mir/src/mir_lower_expr_call.rs");
    assert!(
        lib.contains("lower_expr_call_member(")
            || call_shell.contains("lower_expr_call_member(")
            || lib.contains("mod mir_lower_expr_call;"),
        "Call Member-callee remains reachable (lib, call shell, or mir_lower_expr_call after I189)"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain in lib.rs"
    );
    assert!(
        lib.contains("KeyPathGet")
            || call_shell.contains("KeyPathGet")
            || lib.contains("mod mir_lower_expr_call;"),
        "Call keypath fallback remains reachable (lib or call shell after I189)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-call-member-extract.md");
    assert!(adr.contains("S2-I174"), "call_member ADR must name S2-I174");
    assert!(
        adr.contains("S2-I175") && adr.contains("Member"),
        "ADR must lock next knife = S2-I175 Expr::Member arm"
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
