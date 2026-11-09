//! S2-I213–I214 S2-05 syx-mir lower_expr→orchestrator close-out contracts.
//! Nails: 2 modules + wiring + ADR; mir production peel exhausted; ≠ S2-05 [x].
//! See docs/adr/stage2-s2-05-mir-lower-expr-to-module-batch.md.

fn repo_file(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

#[test]
fn s2_05_mir_lower_expr_to_module_batch_baseline() {
    let mods = [
        ("crates/syx-mir/src/mir_lower_expr.rs", "lower_expr"),
        ("crates/syx-mir/src/mir_lower_module.rs", "lower_module"),
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
    assert!(
        lib.contains("mod mir_lower_expr;") && lib.contains("mod mir_lower_module;"),
        "lib.rs must declare both batch mods"
    );
    assert!(
        lib.contains("pub(crate) use mir_lower_expr::lower_expr")
            && lib.contains("pub use mir_lower_module::{"),
        "lib.rs must re-export lower_expr and public orchestrator APIs"
    );
    assert!(
        !lib.contains("\npub(crate) fn lower_expr(")
            && !lib.contains("\npub fn lower_module(")
            && !lib.contains("\npub fn lower_module_with_extra_extensions(")
            && !lib.contains("\npub fn lower_module_with_extra_extensions_and_kinds(")
            && !lib.contains("\npub fn extension_methods_for_module("),
        "extracted faces must not remain defined in lib.rs"
    );
    assert!(
        !lib.contains("Emit `NumericCast` when `from_ty`")
            && !lib.contains("ADR-324: generic call mangling"),
        "orphan parked docs must be removed from lib.rs"
    );
    assert!(
        lib.contains("#[cfg(test)]"),
        "lib.rs may retain #[cfg(test)] (not migrated this window)"
    );

    let adr = repo_file("docs/adr/stage2-s2-05-mir-lower-expr-to-module-batch.md");
    assert!(
        adr.contains("S2-I213") && adr.contains("S2-I214"),
        "batch ADR must name I213–I214"
    );
    assert!(
        adr.contains("exhausted") || adr.contains("耗尽"),
        "ADR must record mir production peel exhausted"
    );
    assert!(
        adr.contains("S2-I215") && adr.contains("driver"),
        "ADR must lock next knife = S2-I215 syx-driver"
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
