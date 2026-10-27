//! Phase 118 Track B: simd128 guest-v11 zero-scratch + SIMD16 i8x16 + SIMD8 mul pilots.

use std::fs;
use std::sync::Mutex;

use syx_driver::{emit_wasm_bytes, DriverOptions};
use syx_wasm_emit::PHASE118_TRACK_B_WASM_FINGERPRINTS;

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
fn p118_wasm_track_b_fingerprints_in_driver_cache() {
    let lib = include_str!("../../src/lib.rs");
    for fp in PHASE118_TRACK_B_WASM_FINGERPRINTS {
        assert!(lib.contains(fp), "missing cache fingerprint {fp}");
    }
    let emit = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../syx-wasm-emit/src/exec.rs"
    ));
    for fp in PHASE118_TRACK_B_WASM_FINGERPRINTS {
        assert!(emit.contains(fp), "missing emit fingerprint {fp}");
    }
    let mir_emit = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../syx-wasm-emit/src/mir_emit.rs"
    ));
    for fp in PHASE118_TRACK_B_WASM_FINGERPRINTS {
        assert!(mir_emit.contains(fp), "missing mir_emit fingerprint {fp}");
    }
}

#[test]
fn p118_wasm_simd128_pilot_source_covers_guest_v11() {
    let mir_emit = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../syx-wasm-emit/src/mir_emit.rs"
    ));
    assert!(mir_emit.contains("emit_simd_binop_v128_nospill"));
    assert!(mir_emit.contains("emit_simd_guest_pop_v128"));
    assert!(mir_emit.contains("emit_simd_guest_push_v128"));
    assert!(mir_emit.contains("collect_guest_v11_cmp_imports"));
    assert!(mir_emit.contains("emit_simd_add_i16x8_pilot"));
    assert!(mir_emit.contains("emit_simd_mul_i16x8_pilot"));
    assert!(mir_emit.contains("emit_simd_add_i8x16_pilot"));
    assert!(mir_emit.contains("emit_simd_sub_i8x16_pilot"));
    assert!(mir_emit.contains("I16x8Add"));
    assert!(mir_emit.contains("I16x8Mul"));
    assert!(mir_emit.contains("I8x16Add"));
    assert!(mir_emit.contains("I8x16Sub"));
    assert!(mir_emit.contains("wasm-simd128-guest-v11:zero-scratch-simd16"));
    assert!(
        !mir_emit.contains("fn emit_simd_wasm_spill("),
        "guest-v11 must not revive emit_simd_wasm_spill helper"
    );
    assert!(
        !mir_emit.contains("fn emit_simd_wasm_fill("),
        "guest-v11 must not revive emit_simd_wasm_fill helper"
    );

    let cmp_collect = mir_emit
        .split("fn collect_guest_v11_cmp_imports")
        .nth(1)
        .and_then(|s| s.split("fn ").next())
        .unwrap_or("");
    assert!(
        cmp_collect.contains("SimdGuestPushV128") && cmp_collect.contains("SimdGuestPopV128"),
        "cmp collect must use stack-native guest push/pop"
    );
    let simd128_branch = cmp_collect
        .split("} else {")
        .nth(1)
        .unwrap_or("");
    assert!(
        !simd128_branch.contains("SimdWasmSpill") && !simd128_branch.contains("SimdWasmFill"),
        "simd128 cmp path must not insert unused spill/fill imports: {simd128_branch}"
    );

    let load_pilot = mir_emit
        .split("fn emit_simd_load_v128_pilot")
        .nth(1)
        .and_then(|s| s.split("fn emit_simd_store_v128_pilot").next())
        .unwrap_or("");
    assert!(
        !load_pilot.contains("emit_v128_load_from_scratch")
            && !load_pilot.contains("V128Load"),
        "guest-v11 load pilot must not use linear-memory v128 scratch"
    );

    let stdlib = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../packages/core/simd.sy"));
    assert!(stdlib.contains("public class SIMD8"));
    assert!(stdlib.contains("__syx_simd_i16_mul"));
    assert!(stdlib.contains("public class SIMD16"));
    assert!(stdlib.contains("__syx_simd_i8_add"));
    assert!(stdlib.contains("__syx_simd_i8_sub"));
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

#[test]
fn p118_wasm_simd128_stack_native_no_scratch_only_abi() {
    let mir_emit = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../syx-wasm-emit/src/mir_emit.rs"
    ));
    let binop = mir_emit
        .split("fn emit_simd_binop_v128_nospill")
        .nth(1)
        .and_then(|s| s.split("fn emit_simd_unary_v128_nospill").next())
        .unwrap_or("");
    assert!(
        binop.contains("emit_simd_guest_pop_v128") && binop.contains("emit_simd_guest_push_v128"),
        "binop must use stack-native pop/push"
    );
    assert!(
        !binop.contains("emit_v128_load_from_scratch")
            && !binop.contains("emit_v128_store_to_scratch"),
        "guest-v11 binop success path must not round-trip via linear-memory scratch alone"
    );
}
