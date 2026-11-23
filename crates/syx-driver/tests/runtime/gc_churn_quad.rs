//! Phase 98 / C-1d: Host `__syx_gc_churn_heap` label-echo contract (four ends).
//! Nails `print` of the returned `gc:churn:{label}` string — **≠ GC health KPI**.
//! OS user-stdout telemetry stays gated (`SYX_GC_CHURN_STDOUT`; C-1a).

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

const SRC: &str = r#"func main() {
    print(__syx_gc_churn_heap("quad"))
    print("done")
}
"#;

fn opts() -> DriverOptions {
    DriverOptions::default()
}

/// C-1a: compact is opt-in for host compact_global loops.
fn enable_compact_for_churn_host() {
    std::env::set_var("SYX_HEAP_COMPACT", "1");
}

#[test]
fn p98_gc_churn_quad_vm_mir_wasm_native() {
    // C-1d: assert Host label echo via program print — not "GC stdout mandatory".
    enable_compact_for_churn_host();
    syx_host_plugin::register_echo_plugin_in_process();
    let vm = compile_and_run_embedded(SRC, &opts()).unwrap().printed;
    assert!(
        vm.iter().any(|l| l.contains("gc:churn:quad")),
        "C-1d host label echo (vm): {vm:?}"
    );
    assert!(vm.iter().any(|l| l.contains("done")));

    let dir = std::env::temp_dir().join(format!("syx_p98_gc_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), SRC).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let path = dir.join("main.sy");

    let (_, mir) = run_native_mir_file_printed(&path, &opts()).unwrap();
    assert!(
        mir.iter().any(|l| l.contains("gc:churn:quad")),
        "C-1d host label echo (mir): {mir:?}"
    );

    let (_, wasm) = run_wasm_source_printed(&path, "main", &opts()).unwrap();
    assert!(
        wasm.iter().any(|l| l.contains("gc:churn:quad")),
        "C-1d host label echo (wasm): {wasm:?}"
    );

    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let exe = dir.join("app");
    match build_native_executable(&dir, &opts(), &exe) {
        Ok(_) => {
            let output = Command::new(&exe).output().unwrap();
            let native = String::from_utf8_lossy(&output.stdout);
            assert!(
                native.contains("gc:churn:quad"),
                "C-1d host label echo (native): {native}"
            );
            assert!(native.contains("done"), "native: {native}");
        }
        Err(e) if e.code.as_deref() == Some("L006") => {
            // C-2 park: no full-native toolchain — VM/MIR/wasm legs already asserted.
            eprintln!("native host-echo soft-skip L006: {e}");
        }
        Err(e) => panic!("native build: {e}"),
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p98_gc_churn_host_stdout_line() {
    // C-1d: in-memory telemetry buffer only — ≠ user OS stdout (C-1a gated).
    enable_compact_for_churn_host();
    syx_vm::reset_phase98_state();
    syx_runtime_core::reset_gc_churn_stdout();
    let mut stack = vec![syx_runtime_core::Value::Str("compact".into())];
    syx_vm::host_gc_churn_heap(&mut stack).unwrap();
    let lines = syx_runtime_core::gc_churn_stdout_lines();
    assert!(
        lines.iter().any(|l| l.contains("gc:churn:")),
        "C-1d in-memory record buffer: {lines:?}"
    );
}
