//! W1011 / ADR-1355: Trace family ×5 wasm smoke (status remains partial by charter).

use std::fs;

use syx_driver::{run_wasm_source_printed, DriverOptions};

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

fn run_wasm(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let dir = std::env::temp_dir().join(format!(
        "syx_trace_family_wasm_{}_{}",
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
        .unwrap_or_else(|e| panic!("wasm Trace family smoke failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    lines_joined_vec(&lines)
}

/// Markers + contextLine structure (span-id digits are global TRACE_SPAN_SEQ).
const TRACE_WASM_SMOKE: &str = r#"
func main() {
  print("P")
  print(__syx_trace_policy())
  print("S")
  let id = __syx_trace_start("demo")
  print(id)
  print("C")
  print(id == __syx_trace_current())
  print("L")
  print(__syx_trace_context_line())
  print("E")
  print(__syx_trace_end())
  print("X")
  print(__syx_str_utf8_count(__syx_trace_current()))
}
"#;

fn assert_trace_wasm_smoke(lines: &[String]) {
    let out = lines.join("\n");
    assert_eq!(
        lines.first().map(String::as_str),
        Some("P"),
        "wasm must start with P: {lines:?}"
    );
    assert!(
        lines.iter().any(|l| l == "trace-v1-logctx"),
        "wasm policy fingerprint: {out}"
    );
    let s_idx = lines
        .iter()
        .position(|l| l == "S")
        .unwrap_or_else(|| panic!("wasm missing S marker: {lines:?}"));
    let span_id = lines
        .get(s_idx + 1)
        .cloned()
        .unwrap_or_else(|| panic!("wasm missing span id after S: {lines:?}"));
    assert!(
        span_id.starts_with('s')
            && span_id.chars().nth(1).is_some_and(|c| c.is_ascii_digit()),
        "wasm span id shape: {span_id}"
    );
    let c_idx = lines
        .iter()
        .position(|l| l == "C")
        .unwrap_or_else(|| panic!("wasm missing C marker: {lines:?}"));
    assert_eq!(
        lines.get(c_idx + 1).map(String::as_str),
        Some("true"),
        "wasm current must match start: {lines:?}"
    );
    let l_idx = lines
        .iter()
        .position(|l| l == "L")
        .unwrap_or_else(|| panic!("wasm missing L marker: {lines:?}"));
    let ctx = lines
        .get(l_idx + 1)
        .cloned()
        .unwrap_or_else(|| panic!("wasm missing contextLine after L: {lines:?}"));
    assert!(
        ctx.contains("span=")
            && ctx.contains("task_id=")
            && ctx.contains("actor_id=")
            && ctx.contains(&format!("span={span_id}")),
        "wasm contextLine must include span/task/actor for {span_id}: {ctx}"
    );
    let e_idx = lines
        .iter()
        .position(|l| l == "E")
        .unwrap_or_else(|| panic!("wasm missing E marker: {lines:?}"));
    assert_eq!(
        lines.get(e_idx + 1).map(String::as_str),
        Some("true"),
        "wasm Trace.end must return true: {lines:?}"
    );
    let x_idx = lines
        .iter()
        .position(|l| l == "X")
        .unwrap_or_else(|| panic!("wasm missing X marker: {lines:?}"));
    assert_eq!(
        lines.get(x_idx + 1).map(String::as_str),
        Some("0"),
        "wasm current after end must be empty: {lines:?}"
    );
}

#[test]
fn trace_family_smoke_wasm() {
    let lines = run_wasm("host_smoke", TRACE_WASM_SMOKE);
    assert_trace_wasm_smoke(&lines);
}
