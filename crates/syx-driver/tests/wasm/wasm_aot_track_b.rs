//! Phase 118 Track B: simd128 guest-v11 zero-scratch + SIMD16 i8x16 + SIMD8 mul pilots.

use std::fs;
use std::sync::Mutex;

use syx_driver::{emit_wasm_bytes, DriverOptions};

fn opts() -> DriverOptions {
    let mut o = DriverOptions::default();
    o.wasm_aot = true;
    o
}

fn with_simd128_tier3<F: FnOnce()>(f: F) {
    static SIMD128_ENV_LOCK: Mutex<()> = Mutex::new(());
    let _guard = SIMD128_ENV_LOCK
        .lock()
        .unwrap_or_else(|e| e.into_inner());
    std::env::set_var("SYX_WASM_SIMD128", "1");
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    std::env::remove_var("SYX_WASM_MIR_EXEC_ONLY");
    let mut cfg = syx_config::runtime_config();
    cfg.enable_wasm_simd128 = true;
    syx_config::install_runtime_config(cfg);
    struct ClearEnv;
    impl Drop for ClearEnv {
        fn drop(&mut self) {
            std::env::remove_var("SYX_WASM_SIMD128");
            std::env::remove_var("SYX_WASM_AOT_TIER3");
            let mut cfg = syx_config::runtime_config();
            cfg.enable_wasm_simd128 = false;
            syx_config::install_runtime_config(cfg);
        }
    }
    let _clear = ClearEnv;
    f();
}

fn assert_i32_stdout(lines: &[String], expected: &str, label: &str) {
    assert!(
        lines.iter().any(|l| l.trim() == expected),
        "{label} expected {expected}, got {lines:?}"
    );
    assert!(
        !lines.iter().any(|l| l.trim() == "0"),
        "{label} must not soft-accept 0, got {lines:?}"
    );
}

#[test]
fn p118_wasm_simd128_simd8_mul_stdout_hard() {
    syx_host_plugin::register_echo_plugin_in_process();
    const SRC: &str = r#"func main() {
    let a = SIMD8.fromArray([1, 2, 3, 4, 5, 6, 7, 8])
    let b = SIMD8.fromArray([2, 2, 2, 2, 2, 2, 2, 2])
    print(SIMD8.reduceAdd(SIMD8.mul(a, b)))
}"#;
    with_simd128_tier3(|| {
        let dir = std::env::temp_dir().join(format!("syx_p118_simd8_mul_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("main.sy"), SRC).unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        let path = dir.join("main.sy");
        let (_, native_lines) =
            syx_driver::run_native_mir_file_printed(&path, &opts()).expect("native simd8 mul");
        assert_i32_stdout(&native_lines, "72", "native simd8 mul");
        let (_, wasm_lines) =
            syx_driver::run_wasm_source_printed(&path, "main", &opts()).expect("wasm simd8 mul");
        let wasm = emit_wasm_bytes(&path, &opts()).expect("emit simd8 mul wasm");
        let wasm_text = String::from_utf8_lossy(&wasm);
        assert!(
            !wasm_text.contains("__syx_aot_simd_wasm_spill")
                && !wasm_text.contains("__syx_aot_simd_wasm_fill"),
            "guest-v11 simd8 mul must not link spill/fill imports"
        );
        assert_i32_stdout(&wasm_lines, "72", "wasm simd8 mul");
        let _ = fs::remove_dir_all(&dir);
    });
}

#[test]
fn p118_wasm_simd128_simd16_add_sub_stdout_hard() {
    syx_host_plugin::register_echo_plugin_in_process();
    const SRC: &str = r#"func main() {
    let a = SIMD16.fromArray([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16])
    print(SIMD16.reduceAdd(a))
    let b = SIMD16.fromArray([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    print(SIMD16.reduceAdd(SIMD16.add(a, b)))
    print(SIMD16.reduceAdd(SIMD16.sub(a, b)))
}"#;
    with_simd128_tier3(|| {
        let dir = std::env::temp_dir().join(format!("syx_p118_simd16_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("main.sy"), SRC).unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        let path = dir.join("main.sy");
        let (_, native_lines) =
            syx_driver::run_native_mir_file_printed(&path, &opts()).expect("native simd16");
        assert_i32_stdout(&native_lines, "136", "native simd16 reduce");
        assert_i32_stdout(&native_lines, "152", "native simd16 add");
        assert_i32_stdout(&native_lines, "120", "native simd16 sub");
        let (_, wasm_lines) =
            syx_driver::run_wasm_source_printed(&path, "main", &opts()).expect("wasm simd16");
        let wasm = emit_wasm_bytes(&path, &opts()).expect("emit simd16 wasm");
        let wasm_text = String::from_utf8_lossy(&wasm);
        assert!(
            !wasm_text.contains("__syx_aot_simd_wasm_spill")
                && !wasm_text.contains("__syx_aot_simd_wasm_fill"),
            "guest-v11 simd16 must not link spill/fill imports"
        );
        assert_i32_stdout(&wasm_lines, "136", "wasm simd16 reduce");
        assert_i32_stdout(&wasm_lines, "152", "wasm simd16 add");
        assert_i32_stdout(&wasm_lines, "120", "wasm simd16 sub");
        let _ = fs::remove_dir_all(&dir);
    });
}

