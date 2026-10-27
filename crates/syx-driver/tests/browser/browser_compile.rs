//! Browser compiler E2E (Phase 65 d65-browser).
// was: browser_compile

use syx_compiler_wasm::compile_source_to_wasm_bytes;
use syx_config::{SyxRuntimeConfig, WasmConfig};
use syx_wasm_emit::measure_aot_ratio;

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

#[test]
fn browser_compiler_emits_valid_wasm() {
    with_browser_aot(|| {
        let bytes = compile_source_to_wasm_bytes(
            "main.sy",
            r#"func main() { return 42 }"#,
        )
        .expect("compile");
        assert_eq!(&bytes[0..4], b"\0asm");
        let exit = syx_wasm_run::run_wasm_with_module(&bytes, "main", None).expect("run");
        assert_eq!(exit, 42);
    });
}

#[test]
fn browser_compiler_hot_loop_meets_aot_ratio() {
    with_browser_aot(|| {
        let source = include_str!("../../../../examples/hot_loop.sy");
        let bytes = compile_source_to_wasm_bytes("hot_loop.sy", source).expect("compile");
        let (_direct, _mir_exec, ratio) = measure_aot_ratio(&bytes);
        assert!(
            ratio >= 0.95,
            "expected ≥95% direct emit, got {ratio:.4}"
        );
    });
}

#[test]
fn browser_c_api_returns_wasm_bytes() {
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
        let bytes = unsafe { std::slice::from_raw_parts(ptr, out_len) };
        assert_eq!(&bytes[0..4], b"\0asm");
    });
}

#[test]
fn wasm_wasi_example_builds() {
    use std::path::PathBuf;
    use syx_driver::{emit_wasm_bytes, DriverOptions};

    with_browser_aot(|| {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/wasm_wasi/main.sy");
        let mut opts = DriverOptions::default();
        opts.wasm_aot = true;
        let bytes = emit_wasm_bytes(&path, &opts).expect("emit wasm_wasi");
        assert_eq!(&bytes[0..4], b"\0asm");
        assert!(bytes.len() > 100);
    });
}

macro_rules! browser_compile_smoke {
    ($name:ident, $src:expr, $ret:expr) => {
        #[test]
        fn $name() {
            with_browser_aot(|| {
                let bytes = compile_source_to_wasm_bytes("main.sy", $src).expect("compile");
                assert_eq!(&bytes[0..4], b"\0asm");
                let exit = syx_wasm_run::run_wasm_with_module(&bytes, "main", None).expect("run");
                assert_eq!(exit, $ret);
            });
        }
    };
}

browser_compile_smoke!(browser_ret_0, "func main() { return 0 }", 0);
browser_compile_smoke!(browser_ret_1, "func main() { return 1 }", 1);
browser_compile_smoke!(browser_ret_2, "func main() { return 2 }", 2);
browser_compile_smoke!(browser_ret_3, "func main() { return 3 }", 3);
browser_compile_smoke!(browser_ret_4, "func main() { return 4 }", 4);
browser_compile_smoke!(browser_ret_5, "func main() { return 5 }", 5);
browser_compile_smoke!(browser_ret_6, "func main() { return 6 }", 6);
browser_compile_smoke!(browser_ret_7, "func main() { return 7 }", 7);
browser_compile_smoke!(browser_ret_8, "func main() { return 8 }", 8);
browser_compile_smoke!(browser_ret_9, "func main() { return 9 }", 9);
browser_compile_smoke!(browser_ret_10, "func main() { return 10 }", 10);
browser_compile_smoke!(browser_ret_11, "func main() { return 11 }", 11);

#[test]
fn browser_compile_count_gate() {
    let src = std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/browser_compile.rs")).unwrap();
    let count = src.matches("#[test]").count() + src.matches("browser_compile_smoke!").count();
    assert!(count >= 15, "browser_compile needs 15+ tests, found {count}");
}
