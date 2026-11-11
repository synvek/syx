//! R-25 wasm suite closeout contract: config isolation + fixtures + Const*/Phase-100.

use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn r25_runtime_config_guard_restores() {
    let guard = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/wasm/runtime_config_guard.rs"),
    )
    .expect("runtime_config_guard.rs");
    assert!(
        guard.contains("struct RuntimeConfigGuard")
            && guard.contains("LOCK_HELD")
            && guard.contains("nested_restore"),
        "guard must be re-entrant (nested with_aot) and restore RUNTIME_CONFIG on drop"
    );
    assert!(
        guard.contains("fn with_aot") && guard.contains("fn with_simd128_tier3"),
        "shared with_aot / with_simd128_tier3 helpers must exist"
    );
    let main = std::fs::read_to_string(repo_root().join("crates/syx-driver/tests/wasm/main.rs"))
        .expect("main.rs");
    assert!(
        main.contains("mod runtime_config_guard"),
        "wasm harness must include the guard module"
    );
}

#[test]
fn r25_track_sources_import_simd4() {
    let b = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/wasm/wasm_aot_track_b.rs"),
    )
    .expect("track_b");
    let c = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/wasm/wasm_aot_track_c.rs"),
    )
    .expect("track_c");
    assert!(
        b.contains("import SIMD4") && c.contains("import SIMD4"),
        "track_b/c inline SRC must import SIMD4 facade"
    );
    assert!(
        !b.contains("fn with_simd128_tier3") && !c.contains("fn with_simd128_tier3"),
        "local with_simd128_tier3 must be replaced by shared guard"
    );
}

#[test]
fn r25_generics_uses_required_label() {
    let src = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/fixtures/parity/generics.sy"),
    )
    .expect("generics.sy");
    assert!(
        src.contains("id(x: 42)") || src.contains("id(x:42)"),
        "generics fixture must use required label x: (S042)"
    );
    assert!(
        !src.contains("print(id(42))"),
        "unlabeled id(42) must not remain"
    );
}

#[test]
fn r25_const_i32_non_aot_arm_and_p98_aot() {
    let emit = std::fs::read_to_string(repo_root().join("crates/syx-wasm-emit/src/mir_emit.rs"))
        .expect("mir_emit.rs");
    assert!(
        emit.contains("R-25-03: scalar constants on the non-AOT guest path"),
        "non-AOT Const* arms must cite R-25-03"
    );
    assert!(
        emit.contains("MirOp::ConstI32(v)")
            && emit.contains("MirOp::ConstBool(b)")
            && emit.contains("MirOp::ConstNil")
            && emit.contains("MirOp::ConstNumeric { kind, bits }"),
        "non-AOT match must handle ConstI32/Bool/Nil/Numeric"
    );
    let p98 = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/wasm/wasm_aot_exec_op_zero.rs"),
    )
    .expect("exec_op_zero");
    assert!(
        p98.contains("aot: true") && p98.contains("with_runtime_config"),
        "p98 WAT test must install AOT via with_runtime_config"
    );
}

#[test]
fn r25_mir_exec_only_phase100_contract() {
    let full = std::fs::read_to_string(
        repo_root().join("crates/syx-driver/tests/wasm/wasm_full_program_aot.rs"),
    )
    .expect("full_program_aot");
    assert!(
        full.contains("aot_mir_exec_only_incompatible_with_guest_control"),
        "mir_exec_only test must be the Phase-100 incompatibility contract"
    );
    assert!(
        !full.contains("aot_mir_exec_only_differs_from_hybrid"),
        "stale differs-from-hybrid premise must be gone"
    );
    assert!(
        !full.contains("#[ignore]"),
        "fail-closed: do not ignore the mir_exec_only contract"
    );
    assert!(
        full.contains("expect_err") && full.contains("Phase 100"),
        "contract must expect Phase 100 W001"
    );
}
