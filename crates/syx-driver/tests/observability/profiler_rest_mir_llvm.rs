//! W1018 / ADR-1362: Profiler rest ×5 mir+llvm smoke (status remains partial; ≠ wasm exact).

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, run_native_mir_file_printed, DriverOptions};

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

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "syx_profiler_rest_{}_{}_{}",
        label,
        std::process::id(),
        SEQ.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let path = dir.join("main.sy");
    fs::write(&path, src).expect("write main");
    path
}

fn run_native_mir(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &DriverOptions::default()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
    lines_joined_vec(&lines)
}

/// W1018 / ADR-1362 — PROFILER_REST_SMOKE via native llvm AOT.
#[cfg(unix)]
fn run_native_llvm(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_profiler_rest_llvm_{}_{}_{}",
        label,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    fs::write(dir.join("main.sy"), src).expect("write main");
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .expect("write toml");
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native llvm Profiler rest build failed: {e}"));
    let output = Command::new(&exe)
        .output()
        .expect("run native llvm Profiler rest");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        code, 0,
        "native llvm Profiler rest exit {code}: stdout={stdout} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    lines_joined_vec(
        &stdout
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect::<Vec<_>>(),
    )
}

/// Structural smoke: policy exact + cpu/snapshot (avoid wall/CPU exact).
/// Marker lines keep llvm consecutive-line fold from collapsing steps.
const PROFILER_REST_SMOKE: &str = r#"
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

fn assert_profiler_rest_smoke(lines: &[String]) {
    assert_eq!(
        lines.first().map(String::as_str),
        Some("P"),
        "must start with P: {lines:?}"
    );
    let after = |marker: &str| -> &str {
        let idx = lines
            .iter()
            .position(|l| l == marker)
            .unwrap_or_else(|| panic!("missing {marker}: {lines:?}"));
        lines
            .get(idx + 1)
            .map(String::as_str)
            .unwrap_or_else(|| panic!("missing value after {marker}: {lines:?}"))
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
fn profiler_rest_smoke_native_mir() {
    let lines = run_native_mir("host_smoke", PROFILER_REST_SMOKE);
    assert_profiler_rest_smoke(&lines);
}

#[cfg(unix)]
#[test]
fn profiler_rest_smoke_native_llvm() {
    let lines = run_native_llvm("host_smoke", PROFILER_REST_SMOKE);
    assert_profiler_rest_smoke(&lines);
}
