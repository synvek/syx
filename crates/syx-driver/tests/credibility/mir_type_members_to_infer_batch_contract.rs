//! S2-I205–I208 S2-05 syx-mir type_members→infer batch extract contracts.
//! Nails: 4 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-type-members-to-infer-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_type_members_to_infer_batch_baseline() {
    let mods = [
        (
            "crates/syx-mir/src/mir_lower_type_members.rs",
            "lower_type_members",
        ),
        (
            "crates/syx-mir/src/mir_lower_class_init.rs",
            "lower_class_init",
        ),
        (
            "crates/syx-mir/src/mir_host_cast_helpers.rs",
            "resolve_host_call",
        ),
        (
            "crates/syx-mir/src/mir_type_infer_helpers.rs",
            "infer_expr_init_type",
        ),
    ];
    for (path, fn_name) in mods {
        let src = repo_file(path);
        assert!(
            src.contains(&format!("fn {fn_name}"))
                || src.contains(&format!("pub(crate) fn {fn_name}"))
                || src.contains(&format!("pub fn {fn_name}")),
            "{path} must hold {fn_name}"
        );
    }

    let lib = repo_file("crates/syx-mir/src/lib.rs");
    for mod_name in [
        "mir_lower_type_members",
        "mir_lower_class_init",
        "mir_host_cast_helpers",
        "mir_type_infer_helpers",
    ] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("use mir_lower_type_members::lower_type_members")
            || lib.contains("lower_type_members("),
        "lib.rs must wire lower_type_members"
    );
    assert!(
        lib.contains("pub(crate) use mir_host_cast_helpers::")
            && lib.contains("pub(crate) use mir_type_infer_helpers::"),
        "lib.rs must re-export host/infer helpers"
    );
    assert!(
        !lib.contains("\npub(crate) fn lower_type_members(")
            && !lib.contains("\npub(crate) fn lower_class_init(")
            && !lib.contains("\npub(crate) fn resolve_host_call(")
            && !lib.contains("\npub(crate) fn infer_expr_init_type("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-type-members-to-infer-batch.md");
    assert!(
        adr.contains("S2-I205")
            && adr.contains("S2-I206")
            && adr.contains("S2-I207")
            && adr.contains("S2-I208"),
        "batch ADR must name I205–I208"
    );
    assert!(
        adr.contains("S2-I209") && adr.contains("SIMD"),
        "ADR must lock next knife = S2-I209 SIMD helpers"
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
