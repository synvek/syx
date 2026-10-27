//! Phase 98: GC churn four-end stdout mandatory gate (SYX program calls host).

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

#[test]
fn p98_gc_churn_quad_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gc-churn-stress-v1:multi-churn-stdout-quad"));
}

#[test]
fn p98_gc_churn_quad_vm_mir_wasm_native() {
    syx_host_plugin::register_echo_plugin_in_process();
    let vm = compile_and_run_embedded(SRC, &opts()).unwrap().printed;
    assert!(vm.iter().any(|l| l.contains("gc:churn:")), "vm: {vm:?}");
    assert!(vm.iter().any(|l| l.contains("done")));

    let dir = std::env::temp_dir().join(format!("syx_p98_gc_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), SRC).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let path = dir.join("main.sy");

    let (_, mir) = run_native_mir_file_printed(&path, &opts()).unwrap();
    assert!(mir.iter().any(|l| l.contains("gc:churn:")), "mir: {mir:?}");

    let (_, wasm) = run_wasm_source_printed(&path, "main", &opts()).unwrap();
    assert!(wasm.iter().any(|l| l.contains("gc:churn:")), "wasm: {wasm:?}");

    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    let native = String::from_utf8_lossy(&output.stdout);
    assert!(native.contains("gc:churn:"), "native: {native}");
    assert!(native.contains("done"), "native: {native}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p98_gc_churn_host_stdout_line() {
    syx_vm::reset_phase98_state();
    syx_runtime_core::reset_gc_churn_stdout();
    let mut stack = vec![syx_runtime_core::Value::Str("compact".into())];
    syx_vm::host_gc_churn_heap(&mut stack).unwrap();
    let lines = syx_runtime_core::gc_churn_stdout_lines();
    assert!(lines.iter().any(|l| l.contains("gc:churn:")), "{lines:?}");
}
