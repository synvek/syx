//! W1007 / ADR-1351: Log family ×12 wasm smoke (status remains partial by charter).

use std::fs;

use syx_driver::{run_wasm_source_printed, DriverOptions};

fn lines_joined(lines: &[String]) -> String {
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
    out.join("\n")
}

fn run_wasm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let dir = std::env::temp_dir().join(format!(
        "syx_log_family_wasm_{}_{}",
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
        .unwrap_or_else(|e| panic!("wasm Log family smoke failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    lines_joined(&lines)
}

/// Narrow family smoke: policy + levels + fields + filter + core min-level/takeLast.
const LOG_WASM_SMOKE: &str = r#"
func main() {
  print(__syx_log_policy())
  print(__syx_log_set_min_level("DEBUG"))
  print(__syx_log_min_level())
  __syx_log_debug("d")
  print(__syx_log_take_last())
  __syx_log_warn("w")
  print(__syx_log_take_last())
  let line = __syx_log_info_fields("hi", { "b": 2, "a": 1 })
  print(line)
  print(__syx_log_set_min_level("ERROR"))
  __syx_log_warn("hidden")
  print(__syx_str_utf8_count(__syx_log_take_last()))
  __syx_log_error("e")
  print(__syx_log_take_last())
  __syx_log_info("hidden2")
  print(__syx_str_utf8_count(__syx_log_take_last()))
  // Restore default min level so in-process wasm does not poison sibling Log smokes.
  let _ = __syx_log_set_min_level("DEBUG")
}
"#;

const LOG_WASM_SMOKE_EXPECTED: &str = "log-v1-fields\ntrue\nDEBUG\n[DEBUG] d\n[WARN] w\n[INFO] hi a=1 actor_id=- b=2 span=- task_id=-\ntrue\n0\n[ERROR] e\n0";

#[test]
fn log_family_smoke_wasm() {
    let out = run_wasm("host_smoke", LOG_WASM_SMOKE);
    assert_eq!(out, LOG_WASM_SMOKE_EXPECTED, "wasm Log family smoke: {out}");
}
