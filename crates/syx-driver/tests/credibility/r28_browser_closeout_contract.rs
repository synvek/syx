//! R-28 Browser suite closeout contract.
//!
//! - No double-prelude inject in `browser_compile_run`
//! - AOT exit=0 product contract retained (W18 I32Const(0) path)
//! - Harness uses print/capture, not exit==N alone
//! - e2e paths are `../../` from `crates/syx-driver`

use std::path::PathBuf;

fn repo_path(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

#[test]
fn r28_compile_run_no_double_prelude() {
    let src = std::fs::read_to_string(repo_path(
        "crates/syx-driver/tests/browser/browser_compile_run.rs",
    ))
    .expect("browser_compile_run.rs");
    let calls_with_prelude = src.lines().any(|l| {
        let t = l.trim_start();
        !t.starts_with("//") && t.contains("compile_source_with_prelude_diagnostics(")
    });
    assert!(
        !calls_with_prelude,
        "R-28: do not call *_with_prelude_* after manual prelude (Point_compare W001)"
    );
    assert!(
        src.contains("compile_source_with_diagnostics"),
        "R-28: use compile_source_with_diagnostics with a single prelude inject"
    );
    assert!(
        src.contains("with_prelude(") || src.contains("browser_prelude_source()"),
        "prelude must still be injected once for Comparable/Point graph"
    );
}

#[test]
fn r28_aot_exit_zero_contract_retained() {
    let emit = std::fs::read_to_string(repo_path("crates/syx-wasm-emit/src/mir_emit.rs"))
        .expect("mir_emit.rs");
    // Wave A W18: aot + exit_returns_i32 still emits I32Const(0) for wasm exit.
    assert!(
        emit.contains("exit_returns_i32 && plan.aot")
            && emit.contains("I32Const(0)")
            && (emit.contains("Wave A W18") || emit.contains("wasm exit code is 0")),
        "R-28 must not rewrite AOT main exit=0 product contract"
    );
}

#[test]
fn r28_browser_compile_uses_print_capture() {
    let compile = std::fs::read_to_string(repo_path(
        "crates/syx-driver/tests/browser/browser_compile.rs",
    ))
    .expect("browser_compile.rs");
    assert!(
        compile.contains("run_wasm_capture") || compile.contains("run_aot_value"),
        "browser_compile must assert via capture/print, not exit==N alone"
    );
    assert!(
        compile.contains("print({}") || compile.contains("print(42)"),
        "smoke sources must print the expected value for AOT exit=0"
    );
    let e2e = std::fs::read_to_string(repo_path("crates/syx-driver/tests/browser/browser_e2e.rs"))
        .expect("browser_e2e.rs");
    assert!(
        e2e.contains("print(99)") && e2e.contains("run_wasm_capture"),
        "e2e return-value test must use print/capture"
    );
}

#[test]
fn r28_e2e_paths_from_syx_driver_manifest() {
    let e2e = std::fs::read_to_string(repo_path("crates/syx-driver/tests/browser/browser_e2e.rs"))
        .expect("browser_e2e.rs");
    assert!(
        e2e.contains("../../crates/syx-compiler-wasm/Cargo.toml"),
        "e2e_compiler_wasm path must be ../../crates/…"
    );
    assert!(
        e2e.contains("../../.github/workflows/ci.yml"),
        "e2e ci.yml path must be ../../.github/…"
    );
    assert!(
        !e2e.contains("../../../crates/syx-compiler-wasm")
            && !e2e.contains("../../../.github/workflows/ci.yml"),
        "stale ../../../ path pins must be gone"
    );
}

#[test]
fn r28_no_new_ignore_on_browser_core() {
    for rel in [
        "crates/syx-driver/tests/browser/browser_compile.rs",
        "crates/syx-driver/tests/browser/browser_compile_run.rs",
        "crates/syx-driver/tests/browser/browser_e2e.rs",
    ] {
        let src = std::fs::read_to_string(repo_path(rel)).unwrap_or_else(|e| panic!("{rel}: {e}"));
        assert!(
            !src.contains("#[ignore]"),
            "fail-closed: do not #[ignore] core browser path {rel}"
        );
    }
}
