//! Phase 98 / C-1d: Host label-echo stress (multi-round print) on VM/MIR/wasm/native.
//! Asserts returned `gc:churn:{label}` echo lines — **≠ GC health KPI**.

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

fn host_echo_lines(lines: &[String]) -> usize {
    lines.iter().filter(|l| l.contains("gc:churn:")).count()
}

/// C-1a: compact opt-in for host `__syx_gc_churn_heap` compact_global loops.
fn enable_compact_for_churn_host() {
    std::env::set_var("SYX_HEAP_COMPACT", "1");
}

#[test]
fn p98_gc_churn_stress_quad_two_plus_lines() {
    // C-1d: ≥2 Host label-echo prints — not "GC stdout success".
    enable_compact_for_churn_host();
    syx_host_plugin::register_echo_plugin_in_process();
    let vm = compile_and_run_embedded(SRC, &opts()).unwrap().printed;
    assert!(host_echo_lines(&vm) >= 2, "C-1d host echo (vm): {vm:?}");

    let dir = std::env::temp_dir().join(format!("syx_p98_gc_stress_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), SRC).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let path = dir.join("main.sy");

    let (_, mir) = run_native_mir_file_printed(&path, &opts()).unwrap();
    assert!(host_echo_lines(&mir) >= 2, "C-1d host echo (mir): {mir:?}");

    let (_, wasm) = run_wasm_source_printed(&path, "main", &opts()).unwrap();
    assert!(host_echo_lines(&wasm) >= 2, "C-1d host echo (wasm): {wasm:?}");

    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let exe = dir.join("app");
    match build_native_executable(&dir, &opts(), &exe) {
        Ok(_) => {
            let output = Command::new(&exe).output().unwrap();
            let native = String::from_utf8_lossy(&output.stdout);
            let native_lines: Vec<String> = native.lines().map(|s| s.to_string()).collect();
            assert!(
                host_echo_lines(&native_lines) >= 2,
                "C-1d host echo (native): {native}"
            );
            assert!(native.contains("done"));
        }
        Err(e) if e.code.as_deref() == Some("L006") => {
            eprintln!("native host-echo soft-skip L006: {e}");
        }
        Err(e) => panic!("native build: {e}"),
    }
    let _ = fs::remove_dir_all(&dir);
}
