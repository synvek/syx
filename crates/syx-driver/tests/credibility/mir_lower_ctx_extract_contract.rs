//! S2-I171 S2-05 syx-mir mir_lower_ctx extract contracts.
//! Nails: lower_ctx module + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-lower-ctx-extract.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_ctx_extract_baseline() {
    let ctx = repo_file("crates/syx-mir/src/mir_lower_ctx.rs");
    assert!(
        ctx.contains("struct LowerCtx") && ctx.contains("impl LowerCtx"),
        "mir_lower_ctx must hold LowerCtx struct + impl"
    );
    assert!(
        !ctx.contains("fn lower_expr(") && !ctx.contains("pub(crate) fn lower_expr("),
        "mir_lower_ctx must not pull lower_expr body"
    );
    assert!(
        !ctx.contains("pub fn lower_module"),
        "mir_lower_ctx must not pull lower_module"
    );

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    assert!(
        lib.contains("mod mir_lower_ctx;"),
        "lib.rs must declare mod mir_lower_ctx"
    );
    assert!(
        lib.contains("use mir_lower_ctx::LowerCtx;") || lib.contains("mod mir_lower_ctx;"),
        "lib.rs must declare mir_lower_ctx (LowerCtx use may live in sidecars)"
    );
    assert!(
        !lib.lines().any(
            |l| l.starts_with("struct LowerCtx") || l.starts_with("pub(crate) struct LowerCtx")
        ),
        "struct LowerCtx definition must leave lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr must remain in lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn class_name_from_mangled(")
            || lib
                .lines()
                .any(|l| l.starts_with("fn class_name_from_mangled("))
            || lib.contains("class_name_from_mangled,"),
        "class_name_from_mangled must remain reachable (pub(crate) def or re-export for LowerCtx)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-ctx-extract.md");
    assert!(adr.contains("S2-I171"), "lower_ctx ADR must name S2-I171");
    assert!(
        adr.contains("S2-I172") && adr.contains("lower_expr"),
        "ADR must lock next knife = S2-I172 lower_expr first extract"
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
