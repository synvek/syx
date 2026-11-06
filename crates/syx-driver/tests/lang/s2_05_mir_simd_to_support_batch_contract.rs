//! S2-I209–I212 S2-05 syx-mir SIMD→support batch extract contracts.
//! Nails: 4 modules + wiring + ADR; ≠ S2-05 [x] / ≠ Stage-2 exit.
//! See docs/adr/stage2-s2-05-mir-simd-to-support-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_simd_to_support_batch_baseline() {
    let mods = [
        (
            "crates/syx-mir/src/mir_simd_helpers.rs",
            "host_simd_lowers_own_args",
        ),
        (
            "crates/syx-mir/src/mir_lower_function.rs",
            "lower_function_with_params",
        ),
        (
            "crates/syx-mir/src/mir_member_call_dispatch.rs",
            "classify_member_call",
        ),
        ("crates/syx-mir/src/mir_lower_support.rs", "flatten_blocks"),
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
        "mir_simd_helpers",
        "mir_lower_function",
        "mir_member_call_dispatch",
        "mir_lower_support",
    ] {
        assert!(
            lib.contains(&format!("mod {mod_name};")),
            "lib.rs must declare mod {mod_name}"
        );
    }
    assert!(
        lib.contains("pub(crate) use mir_simd_helpers::")
            && lib.contains("pub(crate) use mir_lower_function::")
            && lib.contains("pub(crate) use mir_member_call_dispatch::")
            && lib.contains("pub(crate) use mir_lower_support::"),
        "lib.rs must re-export batch helpers"
    );
    assert!(
        !lib.contains("\npub(crate) fn host_simd_lowers_own_args(")
            && !lib.contains("\npub(crate) fn lower_function_with_params(")
            && !lib.contains("\npub(crate) fn classify_member_call(")
            && !lib.contains("\npub(crate) fn flatten_blocks(")
            && !lib.contains("\npub(crate) enum MemberCallKind"),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        lib.contains("pub(crate) fn lower_expr(")
            || lib.lines().any(|l| l.starts_with("fn lower_expr("))
            || lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            || lib.contains("mod mir_lower_expr;"),
        "lower_expr shell must remain reachable (lib def or mir_lower_expr)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-simd-to-support-batch.md");
    assert!(
        adr.contains("S2-I209")
            && adr.contains("S2-I210")
            && adr.contains("S2-I211")
            && adr.contains("S2-I212"),
        "batch ADR must name I209–I212"
    );
    assert!(
        adr.contains("S2-I213") && adr.contains("lower_expr"),
        "ADR must lock next knife = S2-I213 lower_expr shell"
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
