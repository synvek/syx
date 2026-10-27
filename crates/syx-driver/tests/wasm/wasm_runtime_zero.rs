//! Phase 99–100: mir_exec/run_op hard zero + guest control thinning.
//! Probe pivot (Wave A W163 / ADR-507): examples/main.sy + guidemo.sy absent from tree;
//! use hello.sy (guidemo role) + hot_loop.sy (main role) — aligned with W159 driver_gate.

use std::path::PathBuf;

use syx_driver::{emit_wasm_bytes, DriverOptions};
use syx_mir_run::op;
use syx_wasm_emit::{measure_aot_mir_exec_histogram, measure_aot_run_op_count, measure_aot_tier_stats};

fn fixture(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn opts() -> DriverOptions {
    let mut o = DriverOptions::default();
    o.wasm_aot = true;
    o
}

fn module_has_import(wasm: &[u8], name: &str) -> bool {
    // Lightweight scan of UTF-8 name in custom/import section bytes.
    let needle = name.as_bytes();
    wasm.windows(needle.len()).any(|w| w == needle)
}

fn assert_wasm_hard_zero(path: &PathBuf) {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_WASM_AOT_STATS", "1");
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    struct ClearEnv;
    impl Drop for ClearEnv {
        fn drop(&mut self) {
            std::env::remove_var("SYX_WASM_AOT_STATS");
            std::env::remove_var("SYX_WASM_AOT_TIER3");
        }
    }
    let _clear = ClearEnv;
    let wasm = emit_wasm_bytes(path, &opts()).expect("emit wasm");
    let stats = measure_aot_tier_stats(&wasm);
    assert_eq!(stats.mir_exec, 0, "{} mir_exec must be zero: {stats:?}", path.display());
    assert_eq!(
        measure_aot_run_op_count(&wasm),
        0,
        "{} run_op must be zero",
        path.display()
    );
    assert!(
        !module_has_import(&wasm, "__syx_aot_run_op"),
        "{} must not import __syx_aot_run_op (rename banned)",
        path.display()
    );
    assert!(
        !module_has_import(&wasm, "__syx_aot_exec_op"),
        "{} must not import __syx_aot_exec_op",
        path.display()
    );
}

fn assert_guest_control_no_mir_exec_control_ops(path: &PathBuf) {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_WASM_AOT_TIER3", "1");
    struct ClearEnv;
    impl Drop for ClearEnv {
        fn drop(&mut self) {
            std::env::remove_var("SYX_WASM_AOT_TIER3");
        }
    }
    let _clear = ClearEnv;
    let wasm = emit_wasm_bytes(path, &opts()).expect("emit wasm");
    let hist = measure_aot_mir_exec_histogram(&wasm);
    for banned in [op::POP_BOOL, op::PEEK_IS_NIL, op::POP, op::CONST_STR, op::CONCAT_STR] {
        assert_eq!(
            hist.get(&banned).copied().unwrap_or(0),
            0,
            "{} mir_exec must not dispatch guest-control op {banned}: {hist:?}",
            path.display()
        );
    }
    // ConstStr guests use the dedicated AOT import, not mir_exec.
    assert!(
        module_has_import(&wasm, "__syx_aot_const_str")
            || measure_aot_tier_stats(&wasm).mir_exec == 0,
        "{} expected __syx_aot_const_str guest import (or zero mir_exec)",
        path.display()
    );
}

#[test]
fn p99_wasm_runtime_main_hard_zero() {
    // Probe pivot: main.sy absent → hot_loop.sy (aot_matrix fixture)
    assert_wasm_hard_zero(&fixture("../../examples/hot_loop.sy"));
}

#[test]
fn p99_wasm_runtime_guidemo_hard_zero() {
    // Probe pivot: guidemo.sy absent → hello.sy (aot_matrix fixture)
    assert_wasm_hard_zero(&fixture("../../examples/hello.sy"));
}

#[test]
fn p100_wasm_guest_control_main() {
    // Probe pivot: main.sy absent → hot_loop.sy (aot_matrix fixture)
    assert_guest_control_no_mir_exec_control_ops(&fixture("../../examples/hot_loop.sy"));
}

#[test]
fn p100_wasm_guest_control_guidemo() {
    // Probe pivot: guidemo.sy absent → hello.sy (aot_matrix fixture)
    assert_guest_control_no_mir_exec_control_ops(&fixture("../../examples/hello.sy"));
}
