//! R-29 Lang honesty + MIR `super.method` → CallSuper contract.

use std::path::PathBuf;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn r29_manifest_phase_is_139() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    assert_eq!(m["phase"], 139);
    assert_eq!(m["version"], "0.2.0");
}

#[test]
fn r29_int_compare_spells_syxhost() {
    let src = syx_driver::embedded_std::lookup_core_builtin("Int").expect("Int");
    assert!(
        src.contains("SyxHost.int_compare"),
        "Int.compare must spell SyxHost.int_compare"
    );
}

#[test]
fn r29_mir_super_method_not_init_only() {
    let src = std::fs::read_to_string(repo_path(
        "crates/syx-mir/src/mir_lower_expr_call_member.rs",
    ))
    .expect("call_member");
    // Must match Super without requiring name == "init" as the sole gate.
    assert!(
        src.contains("matches!(object.as_ref(), Expr::Super { .. })"),
        "super receiver match required"
    );
    let init_only = src.lines().any(|l| {
        let t = l.trim_start();
        !t.starts_with("//")
            && t.contains("Expr::Super")
            && t.contains("name == \"init\"")
    });
    assert!(
        !init_only,
        "R-29: Super member call must not be gated on name == \"init\" only"
    );
    assert!(
        src.contains("CallSuper") && src.contains("if name == \"init\""),
        "init self-storeback must remain CallSuper-adjacent"
    );
}

#[test]
fn r29_no_ignore_on_inheritance_comparable_super_paths() {
    for rel in [
        "crates/syx-driver/tests/lang/inheritance.rs",
        "crates/syx-driver/tests/lang/inheritance_polymorphic.rs",
        "crates/syx-driver/tests/lang/comparable.rs",
        "crates/syx-driver/tests/lang/array_element_check.rs",
        "crates/syx-driver/tests/lang/set_element_check.rs",
    ] {
        let src = std::fs::read_to_string(repo_path(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert!(
            !src.contains("#[ignore]"),
            "fail-closed: do not #[ignore] {rel}"
        );
    }
}
