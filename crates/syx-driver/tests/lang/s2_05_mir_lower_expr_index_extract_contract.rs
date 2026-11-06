//! S2-I181 S2-05 syx-mir mir_lower_expr_index extract contracts.
//! Nails: index module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-index-extract.md.

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
fn s2_05_mir_lower_expr_index_extract_baseline() {
    let index = repo_file("crates/syx-mir/src/mir_lower_expr_index.rs");
    assert!(
        index.contains("fn lower_expr_index")
            || index.contains("pub(crate) fn lower_expr_index"),
        "mir_lower_expr_index must hold lower_expr_index"
    );
    assert!(
        index.contains("ArrayGet") || index.contains("__subscript_get"),
        "index module should hold Index lowering"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_index;"),
        "lib.rs must declare mod mir_lower_expr_index"
    );
    assert!(
        (lib.contains("lower_expr_index(") || mir_lower_expr_src().contains("lower_expr_index(")),
        "lib.rs must delegate Index to lower_expr_index"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );
    assert!(
        lib.contains("mod mir_lower_expr_task_block;") || lib.contains("Expr::TaskBlock"),
        "TaskBlock remains reachable (lib arm or mir_lower_expr_task_block after I182)"
    );
    assert!(
        lib.contains("mod mir_lower_expr_index_assign;"),
        "IndexAssign module wiring must remain (separate from Index)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-index-extract.md");
    assert!(adr.contains("S2-I181"), "index ADR must name S2-I181");
    assert!(
        adr.contains("S2-I182") && adr.contains("TaskBlock"),
        "ADR must lock next knife = S2-I182 TaskBlock"
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
