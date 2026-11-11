//! Phase 139 Track C: simd16 guest-v22 all_true / any_true MVP (+ inherited guest-v21 bitmask).

use std::fs;

use syx_driver::{emit_wasm_bytes, DriverOptions};

use crate::runtime_config_guard::with_simd128_tier3;

fn opts() -> DriverOptions {
    let mut o = DriverOptions::default();
    o.wasm_aot = true;
    o
}

fn assert_bool_stdout(lines: &[String], expected: &str, label: &str) {
    assert!(
        lines.iter().any(|l| l.trim() == expected),
        "{label} expected {expected}, got {lines:?}"
    );
}

#[test]
fn p139_wasm_simd128_simd16_all_any_true_stdout_hard() {
    syx_host_plugin::register_echo_plugin_in_process();
    // R-25-02: SIMD16 behind SIMD4 facade import.
    const SRC: &str = r#"import SIMD4
func main() {
    let all = SIMD16.fromArray([1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    let not_all = SIMD16.fromArray([1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    let any = SIMD16.fromArray([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
    let none = SIMD16.fromArray([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
    print(SIMD16.allTrue(all))
    print(SIMD16.allTrue(not_all))
    print(SIMD16.anyTrue(any))
    print(SIMD16.anyTrue(none))
}"#;
    with_simd128_tier3(|| {
        let dir =
            std::env::temp_dir().join(format!("syx_p139_simd16_all_any_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("main.sy"), SRC).unwrap();
        fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
        let path = dir.join("main.sy");
        let (_, vm_lines) =
            syx_driver::run_file_printed(&path, &opts()).expect("vm simd16 all/any");
        assert_bool_stdout(&vm_lines, "true", "vm allTrue all");
        assert_bool_stdout(&vm_lines, "false", "vm allTrue not_all");
        assert_bool_stdout(&vm_lines, "true", "vm anyTrue any");
        assert_bool_stdout(&vm_lines, "false", "vm anyTrue none");
        let (_, native_lines) =
            syx_driver::run_native_mir_file_printed(&path, &opts()).expect("native simd16 all/any");
        assert_bool_stdout(&native_lines, "true", "native allTrue all");
        assert_bool_stdout(&native_lines, "false", "native allTrue not_all");
        assert_bool_stdout(&native_lines, "true", "native anyTrue any");
        assert_bool_stdout(&native_lines, "false", "native anyTrue none");
        let (_, wasm_lines) = syx_driver::run_wasm_source_printed(&path, "main", &opts())
            .expect("wasm simd16 all/any");
        let wasm = emit_wasm_bytes(&path, &opts()).expect("emit simd16 all/any wasm");
        let wasm_text = String::from_utf8_lossy(&wasm);
        assert!(
            !wasm_text.contains("__syx_aot_simd_wasm_spill")
                && !wasm_text.contains("__syx_aot_simd_wasm_fill"),
            "guest-v22 simd16 all/any true must not link spill/fill imports"
        );
        assert_bool_stdout(&wasm_lines, "true", "wasm allTrue all");
        assert_bool_stdout(&wasm_lines, "false", "wasm allTrue not_all");
        assert_bool_stdout(&wasm_lines, "true", "wasm anyTrue any");
        assert_bool_stdout(&wasm_lines, "false", "wasm anyTrue none");
        let _ = fs::remove_dir_all(&dir);
    });
}
