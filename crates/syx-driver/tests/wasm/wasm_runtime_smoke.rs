//! Isolated wasm-run smoke (own process / Cargo test target).
// was: wasm_runtime_smoke
//! Lives outside `tests/wasm/` so AOT matrix env cannot poison instantiation.
//! Probe pivot (Wave A W163 / ADR-507): main.sy/guidemo.sy absent; hello.sy for vm/wasm parity.

use std::path::PathBuf;

use syx_driver::{run_file_printed, run_wasm_source_printed, DriverOptions};

fn fixture(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn opts() -> DriverOptions {
    let mut o = DriverOptions::default();
    o.wasm_aot = true;
    o
}

#[test]
fn p98_wasm_runtime_main_smoke() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    // Probe pivot: main.sy absent → hello.sy
    let hello = fixture("../../examples/hello.sy");
    let (_, vm) = run_file_printed(&hello, &opts()).expect("vm hello");
    let (_, wasm) = run_wasm_source_printed(&hello, "main", &opts()).expect("wasm hello");
    assert!(!wasm.is_empty(), "wasm stdout empty");
    assert!(
        vm.iter().any(|l| l.contains("Hello, World")),
        "vm hello: {vm:?}"
    );
    assert!(
        wasm.iter().any(|l| l.contains("Hello, World")),
        "wasm hello: {wasm:?}"
    );
}

#[test]
fn p98_wasm_runtime_guidemo_smoke() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    // Probe pivot: guidemo.sy absent; gui_demo.sy headless fails → hello.sy parity smoke
    let hello = fixture("../../examples/hello.sy");
    let (_, vm) = run_file_printed(&hello, &opts()).expect("vm hello");
    let (_, wasm) = run_wasm_source_printed(&hello, "main", &opts()).expect("wasm hello");
    assert!(
        vm.iter().any(|l| l.contains("Hello, World")),
        "vm hello: {vm:?}"
    );
    assert!(
        wasm.iter().any(|l| l.contains("Hello, World")),
        "wasm hello: {wasm:?}"
    );
}
