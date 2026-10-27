//! Phase 98: GC churn stress — multi-round stdout on VM/MIR/wasm/native.

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

const SRC: &str = r#"func main() {
    print(__syx_gc_churn_heap("stress-a"))
    print(__syx_gc_churn_heap("stress-b"))
    print("done")
}
"#;

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn churn_lines(lines: &[String]) -> usize {
    lines.iter().filter(|l| l.contains("gc:churn:")).count()
}

#[test]
fn p98_gc_churn_stress_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("gc-churn-stress-v1:multi-churn-stdout-quad"));
}

#[test]
fn p98_gc_churn_stress_quad_two_plus_lines() {
    syx_host_plugin::register_echo_plugin_in_process();
    let vm = compile_and_run_embedded(SRC, &opts()).unwrap().printed;
    assert!(churn_lines(&vm) >= 2, "vm: {vm:?}");

    let dir = std::env::temp_dir().join(format!("syx_p98_gc_stress_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), SRC).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let path = dir.join("main.sy");

    let (_, mir) = run_native_mir_file_printed(&path, &opts()).unwrap();
    assert!(churn_lines(&mir) >= 2, "mir: {mir:?}");

    let (_, wasm) = run_wasm_source_printed(&path, "main", &opts()).unwrap();
    assert!(churn_lines(&wasm) >= 2, "wasm: {wasm:?}");

    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    let native = String::from_utf8_lossy(&output.stdout);
    let native_lines: Vec<String> = native.lines().map(|s| s.to_string()).collect();
    assert!(churn_lines(&native_lines) >= 2, "native: {native}");
    assert!(native.contains("done"));
    let _ = fs::remove_dir_all(&dir);
}
