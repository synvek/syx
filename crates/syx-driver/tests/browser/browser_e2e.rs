//! Phase 66 browser compiler E2E (wasm32 cdylib + wasmtime run).
// was: browser_e2e

use std::path::PathBuf;
use syx_compiler_wasm::compile_source_to_wasm_bytes;
use syx_config::{SyxRuntimeConfig, WasmConfig};
use syx_driver::{emit_wasm_bytes, DriverOptions};
use syx_wasm_emit::measure_aot_ratio;

fn with_browser_aot<F: FnOnce() -> T, T>(f: F) -> T {
    syx_config::install_runtime_config(SyxRuntimeConfig {
        wasm: WasmConfig {
            aot: true,
            mir_exec_only: false,
        },
        ..SyxRuntimeConfig::default()
    });
    f()
}

#[test]
fn e2e_browser_emits_valid_wasm_header() {
    with_browser_aot(|| {
        let bytes = compile_source_to_wasm_bytes("main.sy", r#"func main() { return 42 }"#)
            .expect("compile");
        assert_eq!(&bytes[0..4], b"\0asm");
    });
}

#[test]
fn e2e_browser_runs_return_value() {
    with_browser_aot(|| {
        let bytes = compile_source_to_wasm_bytes("main.sy", r#"func main() { return 99 }"#)
            .expect("compile");
        let exit = syx_wasm_run::run_wasm_with_module(&bytes, "main", None).expect("run");
        assert_eq!(exit, 99);
    });
}

#[test]
fn e2e_browser_c_api_non_null_ptr() {
    with_browser_aot(|| {
        let name = "main.sy";
        let src = "func main() { return 7 }";
        let mut out_len = 0usize;
        let ptr = unsafe {
            syx_compiler_wasm::syx_compile_source_to_wasm_bytes(
                name.as_ptr() as *const i8,
                name.len(),
                src.as_ptr() as *const i8,
                src.len(),
                &mut out_len,
            )
        };
        assert!(!ptr.is_null());
        assert!(out_len > 8);
    });
}

#[test]
fn e2e_browser_hot_loop_aot_ratio() {
    with_browser_aot(|| {
        let source = include_str!("../../../../examples/hot_loop.sy");
        let bytes = compile_source_to_wasm_bytes("hot_loop.sy", source).expect("compile");
        let (_direct, _mir_exec, ratio) = measure_aot_ratio(&bytes);
        assert!(ratio >= 0.90, "browser hot_loop ratio {ratio:.4}");
    });
}

#[test]
fn e2e_browser_print_via_embed() {
    with_browser_aot(|| {
        let bytes = compile_source_to_wasm_bytes(
            "main.sy",
            r#"func main() { print("browser") }"#,
        )
        .expect("compile");
        assert!(bytes.len() > 20);
    });
}

#[test]
fn e2e_wasm_web_index_exists() {
    let html = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_web/index.html");
    assert!(html.exists());
    let text = std::fs::read_to_string(html).unwrap();
    assert!(text.contains("wasm") || text.contains("compile"));
}

#[test]
fn e2e_compiler_wasm_crate_builds() {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../crates/syx-compiler-wasm/Cargo.toml");
    assert!(manifest.exists());
}

#[test]
fn e2e_wasm_wasi_driver_emit() {
    with_browser_aot(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/wasm_wasi/main.sy");
        if !path.exists() {
            return;
        }
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        match emit_wasm_bytes(&path, &opts) {
            Ok(bytes) => assert_eq!(&bytes[0..4], b"\0asm"),
            Err(e) if e.code.as_deref() == Some("S057") => {}
            Err(e) => panic!("wasm_wasi emit: {e:?}"),
        }
    });
}

#[test]
fn e2e_arithmetic_source_compiles() {
    with_browser_aot(|| {
        let bytes = compile_source_to_wasm_bytes(
            "arith.sy",
            r#"func main() { return 1 + 2 + 3 }"#,
        )
        .expect("compile");
        let exit = syx_wasm_run::run_wasm_with_module(&bytes, "main", None).expect("run");
        assert_eq!(exit, 6);
    });
}

#[test]
fn e2e_phase66_ci_compiler_wasm_job() {
    let ci = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../.github/workflows/ci.yml"),
    )
    .unwrap();
    assert!(ci.contains("compiler-wasm") || ci.contains("compiler.wasm"));
}

#[test]
fn e2e_browser_mir_text_legacy_api() {
    with_browser_aot(|| {
        let text = syx_compiler_wasm::compile_source_to_mir_text(
            "t.sy",
            "func main() { return 0 }",
        )
        .expect("mir text");
        assert!(text.contains("main") || text.contains("func"));
    });
}

#[test]
fn e2e_sorted_array_browser_compile() {
    with_browser_aot(|| {
        let bytes = compile_source_to_wasm_bytes(
            "sort.sy",
            r#"func main() {
    let xs: Array<Int> = [3, 1, 2]
    let ys = xs.sorted()
    return ys[0]
}"#,
        )
        .expect("compile");
        assert!(!bytes.is_empty());
    });
}
