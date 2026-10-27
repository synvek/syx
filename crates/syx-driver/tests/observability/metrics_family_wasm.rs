//! W1015 / ADR-1359: Metrics family ×9 wasm smoke (status remains partial by charter).

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
        "syx_metrics_family_wasm_{}_{}",
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
        .unwrap_or_else(|e| panic!("wasm Metrics family smoke failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    lines_joined_vec(&lines)
}

/// Structural smoke: policy + counter + gauge + labeled (avoid flaky global snapshot exact).
const METRICS_WASM_SMOKE: &str = r#"
func main() {
  print("P")
  print(__syx_metrics_policy())
  print("R")
  print(__syx_metrics_counter_reset("w1015c"))
  print("G0")
  print(__syx_metrics_counter_get("w1015c"))
  print("I")
  print(__syx_metrics_counter_inc("w1015c", 3))
  print("G3")
  print(__syx_metrics_counter_get("w1015c"))
  print("GS")
  print(__syx_metrics_gauge_set("w1015g", 7))
  print("GG")
  print(__syx_metrics_gauge_get("w1015g"))
  print("LI")
  print(__syx_metrics_counter_inc_labels("w1015l", 1, { "m": "GET", "c": "200" }))
  print("LG")
  print(__syx_metrics_counter_get_labels("w1015l", { "m": "GET", "c": "200" }))
  print("S")
  print(__syx_metrics_snapshot())
}
"#;

fn assert_metrics_wasm_smoke(lines: &[String]) {
    let out = lines.join("\n");
    assert_eq!(
        lines.first().map(String::as_str),
        Some("P"),
        "wasm must start with P: {lines:?}"
    );
    assert!(
        lines.iter().any(|l| l == "metrics-v1-gauge-labels"),
        "wasm policy fingerprint: {out}"
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
    assert_eq!(after("R"), "true", "reset: {lines:?}");
    assert_eq!(after("G0"), "0", "get after reset: {lines:?}");
    assert_eq!(after("I"), "3", "inc: {lines:?}");
    assert_eq!(after("G3"), "3", "get after inc: {lines:?}");
    assert_eq!(after("GS"), "7", "gauge set: {lines:?}");
    assert_eq!(after("GG"), "7", "gauge get: {lines:?}");
    assert_eq!(after("LI"), "1", "inc labels: {lines:?}");
    assert_eq!(after("LG"), "1", "get labels: {lines:?}");
    let snap = after("S");
    assert!(
        snap.contains("w1015c=3")
            && snap.contains("g:w1015g=7")
            && snap.contains("w1015l{c=200,m=GET}=1"),
        "wasm snapshot must include counter/gauge/labels: {snap}"
    );
}

#[test]
fn metrics_family_smoke_wasm() {
    let lines = run_wasm("host_smoke", METRICS_WASM_SMOKE);
    assert_metrics_wasm_smoke(&lines);
}
