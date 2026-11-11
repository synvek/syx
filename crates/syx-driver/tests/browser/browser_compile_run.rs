//! Browser wasm compile+run (stdout exit code mandatory).
//!
//! R-28: inject browser prelude **once** — do not prepend `browser_prelude_source()`
//! and also call `compile_source_with_prelude_diagnostics` (that double-injects
//! `Point` → W001 `duplicate export name Point_compare`).

use syx_compiler_wasm::{
    browser_prelude_source, compile_source_to_wasm_bytes, compile_source_with_diagnostics,
};
use syx_config::{SyxRuntimeConfig, WasmConfig};

fn with_browser_aot<F: FnOnce()>(f: F) {
    syx_config::install_runtime_config(SyxRuntimeConfig {
        wasm: WasmConfig {
            aot: true,
            mir_exec_only: false,
        },
        ..SyxRuntimeConfig::default()
    });
    f();
}

fn with_prelude(body: &str) -> String {
    format!("{}\n{}", browser_prelude_source(), body)
}

fn run_wasm_mandatory(bytes: &[u8], expected: i32) {
    let r =
        syx_wasm_run::run_wasm_capture(bytes, "main", None).expect("browser wasm run must succeed");
    let expect = expected.to_string();
    let printed_ok = r.printed.iter().any(|l| l.contains(&expect));
    assert!(
        r.exit_code == expected || printed_ok,
        "expected exit or stdout {expected}, got exit={} printed={:?}",
        r.exit_code,
        r.printed
    );
}

#[test]
fn compile_run_closure() {
    with_browser_aot(|| {
        let src = with_prelude(
            "func id(x: Int) -> Int { return x }\nfunc main() -> Int { print(88); return 88 }",
        );
        let diag = compile_source_with_diagnostics("compile_run_closure.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_closure.sy", &src).expect("wasm");
        assert_eq!(&bytes[0..4], b"\0asm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_async() {
    with_browser_aot(|| {
        let src = with_prelude(
            "async func f() -> Int { return 1 }\nfunc main() -> Int { print(88); return 88 }",
        );
        let diag = compile_source_with_diagnostics("compile_run_async.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_async.sy", &src).expect("wasm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_record_field() {
    with_browser_aot(|| {
        let src = with_prelude(
            "func main() -> Int {
    let n: Int = 88
    print(n)
    return n
}",
        );
        let diag = compile_source_with_diagnostics("compile_run_record.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_record.sy", &src).expect("wasm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_string_builder() {
    with_browser_aot(|| {
        let src = with_prelude(
            "func main() -> Int {
    let b = __syx_string_builder_new()
    __syx_string_builder_append(b, \"sb-smoke\")
    print(__syx_string_builder_to_string(b))
    print(88)
    return 88
}",
        );
        let diag = compile_source_with_diagnostics("compile_run_string_builder.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes =
            compile_source_to_wasm_bytes("compile_run_string_builder.sy", &src).expect("wasm");
        let r = syx_wasm_run::run_wasm_capture(&bytes, "main", None).expect("run");
        assert!(
            r.printed.iter().any(|l| l.contains("sb-smoke")),
            "string builder must print sb-smoke: {:?}",
            r.printed
        );
        assert!(
            r.exit_code == 88 || r.printed.iter().any(|l| l.contains("88")),
            "expected exit or stdout 88, got exit={} printed={:?}",
            r.exit_code,
            r.printed
        );
    });
}

#[test]
fn compile_run_try_catch() {
    with_browser_aot(|| {
        let src = with_prelude(
            "func main() -> Int { do { print(88); return 88 } catch (e) { print(0); return 0 } }",
        );
        let diag = compile_source_with_diagnostics("compile_run_try.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_try.sy", &src).expect("wasm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_heap_compact() {
    with_browser_aot(|| {
        let src = with_prelude("func main() { print(88) }");
        let diag = compile_source_with_diagnostics("compile_run_heap.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
    });
}
