//! W1019 / ADR-1363: Profiler family ×9 wasm smoke (status remains partial by charter).

use std::fs;

use syx_driver::{run_wasm_source_printed, DriverOptions};

fn lines_joined_vec(lines: &[String]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut prev: Option<String> = None;
    for raw in lines {
        let l = raw.trim_end_matches('\r').trim_end_matches('\n');
        if l.is_empty() {
            continue;
        }
        if prev.as_deref() == Some(l) {
            continue;
        }
        prev = Some(l.to_string());
        out.push(l.to_string());
    }
    out
}

fn run_wasm(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let dir = std::env::temp_dir().join(format!(
        "syx_profiler_family_wasm_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm Profiler family smoke failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    lines_joined_vec(&lines)
}

/// Structural smoke: policy exact + cpu/snapshot (avoid wall/CPU exact).
const PROFILER_WASM_SMOKE: &str = r#"
func main() {
  print("P")
  print(__syx_profiler_policy())
  print("R")
  print(__syx_profiler_reset("work"))
  print("E0")
  print(__syx_profiler_cpu_elapsed("work"))
  print("S")
  print(__syx_profiler_cpu_start("work"))
  print("T")
  print(__syx_profiler_cpu_stop("work"))
  print("E")
  print(__syx_profiler_cpu_elapsed("work"))
  print("W")
  print(__syx_profiler_start("work"))
  print("X")
  print(__syx_profiler_stop("work"))
  print("N")
  print(__syx_profiler_snapshot())
}
"#;

fn assert_profiler_wasm_smoke(lines: &[String]) {
    assert_eq!(
        lines.first().map(String::as_str),
        Some("P"),
        "wasm must start with P: {lines:?}"
    );
    let after = |marker: &str| -> &str {
        let idx = lines
            .iter()
            .position(|l| l == marker)
            .unwrap_or_else(|| panic!("wasm missing {marker}: {lines:?}"));
        lines
            .get(idx + 1)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("wasm missing value after {marker}: {lines:?}"))
    };
    assert_eq!(after("P"), "profiler-v1-cpu", "policy: {lines:?}");
    assert_eq!(after("R"), "true", "reset: {lines:?}");
    assert_eq!(after("E0"), "0", "cpuElapsed after reset: {lines:?}");
    assert_eq!(after("S"), "true", "cpuStart: {lines:?}");
    let stop_ms: i64 = after("T")
        .parse()
        .unwrap_or_else(|_| panic!("cpuStop must be int: {lines:?}"));
    assert!(stop_ms >= 0, "cpuStop ms >= 0: {lines:?}");
    let elapsed_ms: i64 = after("E")
        .parse()
        .unwrap_or_else(|_| panic!("cpuElapsed must be int: {lines:?}"));
    assert!(elapsed_ms >= 0, "cpuElapsed ms >= 0: {lines:?}");
    assert!(
        elapsed_ms >= stop_ms,
        "cpuElapsed must accumulate stop: stop={stop_ms} elapsed={elapsed_ms}: {lines:?}"
    );
    assert_eq!(after("W"), "true", "wall start: {lines:?}");
    let wall_stop: i64 = after("X")
        .parse()
        .unwrap_or_else(|_| panic!("wall stop must be int: {lines:?}"));
    assert!(wall_stop >= 0, "wall stop ms >= 0: {lines:?}");
    let snap = after("N");
    assert!(
        snap.contains("c:work="),
        "snapshot must include c:work=: {snap}"
    );
    let has_wall = snap
        .split_whitespace()
        .any(|p| p.starts_with("work=") && !p.starts_with("c:"));
    assert!(has_wall, "snapshot must include wall work=: {snap}");
    let c_pos = snap.find("c:work=").expect("c:work");
    let wall_pos = snap
        .split_whitespace()
        .find(|p| p.starts_with("work=") && !p.starts_with("c:"))
        .and_then(|p| snap.find(p))
        .expect("wall part");
    assert!(
        c_pos < wall_pos,
        "snapshot keys must be sorted (c: before wall): {snap}"
    );
}

#[test]
fn profiler_family_smoke_wasm() {
    let lines = run_wasm("host_smoke", PROFILER_WASM_SMOKE);
    assert_profiler_wasm_smoke(&lines);
}
