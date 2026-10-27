//! Phase 91: grep gate banning soft parity assertions in P89 files.

use std::path::PathBuf;

fn p98_parity_sources() -> Vec<PathBuf> {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests");
    [
        "runtime/hof_closure.rs",
        "runtime/runtime_cast.rs",
        "runtime/hof_generic_witness.rs",
        "browser/browser_tier2k.rs",
        "runtime/net_http2.rs",
        "data/codable_l4.rs",
        "runtime/select_async.rs",
        "runtime/reflect_witness_mandatory.rs",
        "platform/borrow_prover_mandatory.rs",
        "platform/plugin_marketplace.rs",
    ]
    .into_iter()
    .map(|name| base.join(name))
    .collect()
}

#[test]
fn p98_parity_hard_no_let_wasm() {
    for path in p98_parity_sources() {
        let src = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(
            !src.contains("let _wasm"),
            "{} must not use let _wasm soft assert",
            path.display()
        );
    }
}

#[test]
fn p98_parity_hard_no_if_let_ok_wasm_run() {
    for path in p98_parity_sources() {
        let src = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(
            !src.contains("if let Ok"),
            "{} must not use if let Ok soft wasm run",
            path.display()
        );
    }
}

#[test]
fn p98_parity_hard_no_compile_is_ok_fallback() {
    for path in p98_parity_sources() {
        let src = std::fs::read_to_string(&path).unwrap_or_default();
        assert!(
            !src.contains("compile.is_ok()"),
            "{} must not use compile.is_ok() fallback",
            path.display()
        );
    }
}

#[test]
fn p98_parity_hard_hof_not_placeholder_zero() {
    let src = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/runtime/hof_closure.rs"),
    )
    .unwrap();
    assert!(
        !src.contains("__syx_array_sort_by([3, 1, 2], 0)"),
        "hof_closure_phase98 must not use , 0 placeholder comparator"
    );
}

#[test]
fn p98_parity_hard_net_no_scaffold_name() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/runtime/net_http2.rs");
    assert!(path.exists());
    let src = std::fs::read_to_string(&path).unwrap();
    assert!(
        !src.contains("scaffold"),
        "net_http2_phase98 test fn names must not contain scaffold"
    );
}

#[test]
fn p98_parity_hard_browser_mandatory_stdout() {
    let src = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/browser/browser_tier2k.rs"),
    )
    .unwrap();
    assert!(src.contains(".expect(") || src.contains(".unwrap()"));
}

#[test]
fn p98_parity_hard_borrow_no_coq_or_grep_fallback() {
    let src = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/platform/borrow_prover_mandatory.rs"),
    )
    .unwrap();
    assert!(
        !src.contains("coq || lean || src.contains"),
        "borrow_prover_mandatory_phase98 must not use coq||lean||grep fallback"
    );
}

#[test]
fn p98_parity_hard_hof_closure_native_strict() {
    let src = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/runtime/hof_closure.rs"),
    )
    .unwrap();
    assert!(src.contains("SYX_NATIVE_STRICT"));
}

#[test]
fn p98_parity_hard_runtime_cast_four_end() {
    let src = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/runtime/runtime_cast.rs"),
    )
    .unwrap();
    assert!(src.contains("run_wasm_source_printed"));
    assert!(src.contains("compile_and_run_embedded"));
}

#[test]
fn p98_parity_hard_net_http2_stdout_not_grep_only() {
    let src = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/runtime/net_http2.rs"),
    )
    .unwrap();
    assert!(src.contains("compile_and_run_embedded"));
    assert!(src.contains("assert_eq!") || src.contains("assert!("));
}
