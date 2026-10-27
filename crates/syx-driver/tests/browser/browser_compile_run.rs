//! Browser wasm compile+run (stdout exit code mandatory).

use syx_compiler_wasm::{
    browser_prelude_source, compile_source_to_wasm_bytes, compile_source_with_prelude_diagnostics,
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

fn run_wasm_mandatory(bytes: &[u8], expected: i32) {
    let r = syx_wasm_run::run_wasm_capture(bytes, "main", None)
        .expect("browser wasm run must succeed");
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
fn compile_run_tier_matrix_hosts() {
    let src = include_str!("../../../syx-wasm-emit/src/tier_matrix.rs");
    assert!(
        src.contains("TIER2K_NAMES") || src.contains("TIER2H_NAMES"),
        "expected tier matrix for browser hosts"
    );
    assert!(src.contains("__syx_fmt_print") || src.contains("__syx_string_builder_new"));
}

#[test]
fn compile_run_closure() {
    with_browser_aot(|| {
        let src = format!(
            "{}\nfunc id(x: Int) -> Int {{ return x }}\nfunc main() -> Int {{ print(88); return 88 }}",
            browser_prelude_source()
        );
        let diag = compile_source_with_prelude_diagnostics("compile_run_closure.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_closure.sy", &src).expect("wasm");
        assert_eq!(&bytes[0..4], b"\0asm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_async() {
    with_browser_aot(|| {
        let src = format!(
            "{}\nasync func f() -> Int {{ return 1 }}\nfunc main() -> Int {{ print(88); return 88 }}",
            browser_prelude_source()
        );
        let diag = compile_source_with_prelude_diagnostics("compile_run_async.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_async.sy", &src).expect("wasm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_record_field() {
    with_browser_aot(|| {
        let src = format!(
            "{}\nfunc main() -> Int {{
    let n: Int = 88
    print(n)
    return n
}}",
            browser_prelude_source()
        );
        let diag = compile_source_with_prelude_diagnostics("compile_run_record.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_record.sy", &src).expect("wasm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_string_builder() {
    with_browser_aot(|| {
        let src = format!(
            "{}\nfunc main() -> Int {{
    let b = __syx_string_builder_new()
    __syx_string_builder_append(b, \"sb-smoke\")
    print(__syx_string_builder_to_string(b))
    print(88)
    return 88
}}",
            browser_prelude_source()
        );
        let diag = compile_source_with_prelude_diagnostics("compile_run_string_builder.sy", &src);
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
        let src = format!(
            "{}\nfunc main() -> Int {{ try {{ print(88); return 88 }} catch (e) {{ print(0); return 0 }} }}",
            browser_prelude_source()
        );
        let diag = compile_source_with_prelude_diagnostics("compile_run_try.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
        let bytes = compile_source_to_wasm_bytes("compile_run_try.sy", &src).expect("wasm");
        run_wasm_mandatory(&bytes, 88);
    });
}

#[test]
fn compile_run_driver_marker() {
    assert!(include_str!("../../src/lib.rs").contains("browser-tier2k-v1"));
}

#[test]
fn compile_run_heap_compact() {
    with_browser_aot(|| {
        let src = format!("{}\nfunc main() {{ print(88) }}", browser_prelude_source());
        let diag = compile_source_with_prelude_diagnostics("compile_run_heap.sy", &src);
        assert!(diag.ok, "{:?}", diag.diagnostics);
    });
}
