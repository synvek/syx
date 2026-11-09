//! S2-I173 S2-05 syx-mir mir_lower_expr_call_ident extract contracts.
//! Nails: call_ident module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-call-ident-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_call_ident_extract_baseline() {
    let ident = repo_file("crates/syx-mir/src/mir_lower_expr_call_ident.rs");
    assert!(
        ident.contains("fn lower_expr_call_ident")
            || ident.contains("pub(crate) fn lower_expr_call_ident"),
        "mir_lower_expr_call_ident must hold lower_expr_call_ident"
    );
    assert!(
        ident.contains("HostFnId") || ident.contains("host_fn_id_by_name"),
        "call_ident module should hold Ident-callee host dispatch"
    );
    assert!(
        !ident.contains("pub fn lower_module"),
        "mir_lower_expr_call_ident must not pull lower_module"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_call_ident;"),
        "lib.rs must declare mod mir_lower_expr_call_ident"
    );
    let call_shell = repo_file("crates/syx-mir/src/mir_lower_expr_call.rs");
    assert!(
        lib.contains("lower_expr_call_ident(")
            || call_shell.contains("lower_expr_call_ident(")
            || lib.contains("mod mir_lower_expr_call;"),
        "Call Ident-callee remains reachable (lib, call shell, or mir_lower_expr_call after I189)"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain in lib.rs"
    );
    assert!(
        lib.contains("Expr::Member { object, name, .. } =>")
            || lib.contains("mod mir_lower_expr_call_member;")
            || lib.contains("mod mir_lower_expr_member;"),
        "Member faces remain reachable (lib arm or call_member/member modules)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-call-ident-extract.md");
    assert!(adr.contains("S2-I173"), "call_ident ADR must name S2-I173");
    assert!(
        adr.contains("S2-I174") && adr.contains("Member"),
        "ADR must lock next knife = S2-I174 Call Member sub-face"
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
