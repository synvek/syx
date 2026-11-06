//! S2-I182 S2-05 syx-mir mir_lower_expr_task_block extract contracts.
//! Nails: task_block module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-expr-task-block-extract.md.

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
fn s2_05_mir_lower_expr_task_block_extract_baseline() {
    let task = repo_file("crates/syx-mir/src/mir_lower_expr_task_block.rs");
    assert!(
        task.contains("fn lower_expr_task_block")
            || task.contains("pub(crate) fn lower_expr_task_block"),
        "mir_lower_expr_task_block must hold lower_expr_task_block"
    );
    assert!(
        task.contains("TaskSpawn") || task.contains("TaskDetach"),
        "task_block module should hold TaskBlock lowering"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_expr_task_block;"),
        "lib.rs must declare mod mir_lower_expr_task_block"
    );
    assert!(
        (lib.contains("lower_expr_task_block(") || mir_lower_expr_src().contains("lower_expr_task_block(")),
        "lib.rs must delegate TaskBlock to lower_expr_task_block"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );
    assert!(
        lib.contains("mod mir_lower_expr_compound_assign;")
            || lib.contains("Expr::CompoundAssign"),
        "CompoundAssign remains reachable (lib arm or mir_lower_expr_compound_assign after I183)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-task-block-extract.md");
    assert!(adr.contains("S2-I182"), "task_block ADR must name S2-I182");
    assert!(
        adr.contains("S2-I183") && adr.contains("CompoundAssign"),
        "ADR must lock next knife = S2-I183 CompoundAssign"
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
