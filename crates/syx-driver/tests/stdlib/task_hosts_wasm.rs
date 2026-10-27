//! TaskLocalSet/Get + TaskSetDeadlineMs + TaskLiveCount HostFn wasm=real.

use std::fs;

use syx_driver::{run_wasm_source_printed, DriverOptions};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn run_wasm(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::remove_var("SYX_WASM_ZERO_MIR_EXEC");
    std::env::remove_var("SYX_WASM_AOT_STATS");
    let dir = std::env::temp_dir().join(format!(
        "syx_w275_task_hosts_wasm_{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("tmpdir");
    let main = dir.join("main.sy");
    fs::write(&main, src).expect("write main");
    let mut opts = DriverOptions::default();
    opts.wasm_aot = true;
    let (_, lines) = run_wasm_source_printed(&main, "main", &opts)
        .unwrap_or_else(|e| panic!("wasm task host failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    lines
        .into_iter()
        .filter(|l| !l.is_empty())
        .collect()
}

const TASK_LOCAL_SET_GET: &str = r#"
func main() {
  __syx_task_local_set("k", "V")
  print(__syx_task_local_get("k"))
  return 0
}
"#;

const TASK_LOCAL_MISSING: &str = r#"
func main() {
  print(__syx_task_local_get("missing"))
  return 0
}
"#;

const TASK_SET_DEADLINE: &str = r#"
func main() {
  __syx_task_set_deadline_ms(5000)
  print("DEADLINE_SET")
  return 0
}
"#;

const TASK_LIVE_COUNT_ORPHAN_JOIN: &str = r#"
func main() {
  print("BASE")
  print(__syx_task_live_count())
  let t = Task.detached {
    // silent body — avoid stdout interleaving with labeled counts
  }
  print("AFTER")
  print(__syx_task_live_count())
  try {
    __syx_task_join(t)
  } catch (err) {
    print("JOIN_TEARDOWN")
  }
  print("CLEAN")
  print(__syx_task_live_count())
  return 0
}
"#;

#[test]
fn task_local_set_get_wasm() {
    let lines = run_wasm(TASK_LOCAL_SET_GET);
    assert_eq!(
        lines,
        vec!["V".to_string()],
        "same-thread set→get must exact V: {lines:?}"
    );
}

#[test]
fn task_local_get_missing_wasm() {
    let lines = run_wasm(TASK_LOCAL_MISSING);
    assert!(
        lines.iter().any(|l| l.trim() == "nil" || l.trim() == "Nil"),
        "missing key must print as nil: {lines:?}"
    );
}

#[test]
fn task_set_deadline_ms_wasm() {
    let lines = run_wasm(TASK_SET_DEADLINE);
    assert_eq!(
        lines,
        vec!["DEADLINE_SET".to_string()],
        "set deadline must exact DEADLINE_SET (no sleep/C004): {lines:?}"
    );
}

#[test]
fn task_live_count_orphan_join_wasm() {
    let lines = run_wasm(TASK_LIVE_COUNT_ORPHAN_JOIN);
    let stdout = lines.join("\n");
    let mut baseline = None;
    let mut after = None;
    let mut cleaned = None;
    let mut iter = lines.iter().map(|l| l.as_str());
    while let Some(line) = iter.next() {
        match line.trim() {
            "BASE" => {
                baseline = iter.next().and_then(|s| s.trim().parse::<i64>().ok());
            }
            "AFTER" => {
                after = iter.next().and_then(|s| s.trim().parse::<i64>().ok());
            }
            "CLEAN" => {
                cleaned = iter.next().and_then(|s| s.trim().parse::<i64>().ok());
            }
            _ => {}
        }
    }
    let baseline = baseline.unwrap_or_else(|| panic!("missing BASE count: {stdout}"));
    let after = after.unwrap_or_else(|| panic!("missing AFTER count: {stdout}"));
    let cleaned = cleaned.unwrap_or_else(|| panic!("missing CLEAN count: {stdout}"));
    assert_eq!(
        after,
        baseline + 1,
        "unreaped detach must elevate live_count (baseline={baseline} after={after}): {stdout}"
    );
    assert_eq!(
        cleaned, baseline,
        "join mark_reaped must restore baseline (cleaned={cleaned} baseline={baseline}): {stdout}"
    );
}

#[test]
fn task_hosts_wasm_real_meta() {
    let toml = include_str!("../../../../packages/internal/host_fn_capabilities.toml");
    for name in [
        "TaskLocalSet",
        "TaskLocalGet",
        "TaskSetDeadlineMs",
        "TaskLiveCount",
    ] {
        let block = host_block(toml, name);
        assert!(
            block.contains("llvm = \"real\""),
            "{name} must keep llvm=real: {block}"
        );
        assert!(
            block.contains("status = \"partial\""),
            "{name} must keep status=partial: {block}"
        );
        assert!(
            block.contains("wasm = \"real\""),
            "{name} must declare wasm=real (ADR-619): {block}"
        );
        assert!(
            block.contains("ADR-619") && block.contains("ADR-605"),
            "{name} notes must cite ADR-619+605: {block}"
        );
    }
}
