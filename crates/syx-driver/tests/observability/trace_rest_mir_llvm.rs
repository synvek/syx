//! W1010 / ADR-1354: Trace rest ×1 (ContextLine) mir+llvm smoke
//! (status remains partial; ≠ wasm exact).

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, run_native_mir_file_printed, DriverOptions};

fn lines_joined_vec(lines: &[String]) -> Vec<String> {
    let mut out = Vec::new();
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
        "syx_trace_rest_{}_{}_{}",
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

/// W1010 / ADR-1354 — TRACE_REST_SMOKE via native llvm AOT.
#[cfg(unix)]
fn run_native_llvm(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_trace_rest_llvm_{}_{}_{}",
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
        .unwrap_or_else(|e| panic!("native llvm Trace rest build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native llvm Trace rest");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        code, 0,
        "native llvm Trace rest exit {code}: stdout={stdout} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    lines_joined_vec(
        &stdout
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect::<Vec<_>>(),
    )
}

/// Markers + contextLine structure (span-id digits are global TRACE_SPAN_SEQ).
const TRACE_REST_SMOKE: &str = r#"
func main() {
  print("P")
  print(__syx_trace_policy())
  print("S")
  let id = __syx_trace_start("demo")
  print(id)
  print("L")
  print(__syx_trace_context_line())
  print("E")
  print(__syx_trace_end())
  print("X")
  print(__syx_str_utf8_count(__syx_trace_current()))
}
"#;

fn assert_trace_rest_smoke(lines: &[String], backend: &str) {
    let out = lines.join("\n");
    assert_eq!(
        lines.first().map(String::as_str),
        Some("P"),
        "{backend} must start with P: {lines:?}"
    );
    assert!(
        lines.iter().any(|l| l == "trace-v1-logctx"),
        "{backend} policy fingerprint: {out}"
    );
    let s_idx = lines
        .iter()
        .position(|l| l == "S")
        .unwrap_or_else(|| panic!("{backend} missing S marker: {lines:?}"));
    let span_id = lines
        .get(s_idx + 1)
        .cloned()
        .unwrap_or_else(|| panic!("{backend} missing span id after S: {lines:?}"));
    assert!(
        span_id.starts_with('s')
            && span_id.chars().nth(1).is_some_and(|c| c.is_ascii_digit()),
        "{backend} span id shape: {span_id}"
    );
    let l_idx = lines
        .iter()
        .position(|l| l == "L")
        .unwrap_or_else(|| panic!("{backend} missing L marker: {lines:?}"));
    let ctx = lines
        .get(l_idx + 1)
        .cloned()
        .unwrap_or_else(|| panic!("{backend} missing contextLine after L: {lines:?}"));
    assert!(
        ctx.contains("span=")
            && ctx.contains("task_id=")
            && ctx.contains("actor_id=")
            && ctx.contains(&format!("span={span_id}")),
        "{backend} contextLine must include span/task/actor for {span_id}: {ctx}"
    );
    let e_idx = lines
        .iter()
        .position(|l| l == "E")
        .unwrap_or_else(|| panic!("{backend} missing E marker: {lines:?}"));
    assert_eq!(
        lines.get(e_idx + 1).map(String::as_str),
        Some("true"),
        "{backend} Trace.end must return true: {lines:?}"
    );
    let x_idx = lines
        .iter()
        .position(|l| l == "X")
        .unwrap_or_else(|| panic!("{backend} missing X marker: {lines:?}"));
    assert_eq!(
        lines.get(x_idx + 1).map(String::as_str),
        Some("0"),
        "{backend} current after end must be empty: {lines:?}"
    );
}

#[test]
fn trace_rest_smoke_native_mir() {
    let lines = run_native_mir("host_smoke", TRACE_REST_SMOKE);
    assert_trace_rest_smoke(&lines, "native mir");
}

#[cfg(unix)]
#[test]
fn trace_rest_smoke_native_llvm() {
    let lines = run_native_llvm("host_smoke", TRACE_REST_SMOKE);
    assert_trace_rest_smoke(&lines, "native llvm");
}
