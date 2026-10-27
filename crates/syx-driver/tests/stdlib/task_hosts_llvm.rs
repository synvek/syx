//! TaskLocalSet/Get + TaskSetDeadlineMs + TaskLiveCount HostFn llvm=real.
//! same four HostFn wasm=real (see task_hosts_wasm.rs).

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, DriverOptions};

fn host_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing host override {name}"));
    rest.split("[[host]]").next().unwrap_or(rest)
}

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w261_task_hosts_llvm_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe)
        .unwrap_or_else(|e| panic!("native build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native exe");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

fn stdout_lines(stdout: &str) -> String {
    stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

const TASK_LOCAL_SET_GET: &str = r#"
func main() {
  __syx_task_local_set("k", "V")
  print(__syx_task_local_get("k"))
}
"#;

const TASK_LOCAL_MISSING: &str = r#"
func main() {
  print(__syx_task_local_get("missing"))
}
"#;

const TASK_SET_DEADLINE: &str = r#"
func main() {
  __syx_task_set_deadline_ms(5000)
  print("DEADLINE_SET")
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
}
"#;

#[test]
fn task_local_set_get_native_llvm() {
    let (code, stdout) = run_native_strict(TASK_LOCAL_SET_GET);
    let out = stdout_lines(&stdout);
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "V", "same-thread set→get must exact V: {out:?} stdout={stdout:?}");
}

#[test]
fn task_local_get_missing_native_llvm() {
    let (code, stdout) = run_native_strict(TASK_LOCAL_MISSING);
    let out = stdout_lines(&stdout);
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert!(
        out.lines().any(|l| l.trim() == "nil" || l.trim() == "Nil"),
        "missing key must print as nil: {out:?} stdout={stdout:?}"
    );
}

#[test]
fn task_set_deadline_ms_native_llvm() {
    let (code, stdout) = run_native_strict(TASK_SET_DEADLINE);
    let out = stdout_lines(&stdout);
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(
        out, "DEADLINE_SET",
        "set deadline must exact DEADLINE_SET (no sleep/C004): {out:?}"
    );
}

#[test]
fn task_live_count_orphan_join_native_llvm() {
    let (code, stdout) = run_native_strict(TASK_LIVE_COUNT_ORPHAN_JOIN);
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    // Parse labeled counts in Rust — avoid native AOT Int I32/I64 `==`/`+` pitfalls.
    let mut baseline = None;
    let mut after = None;
    let mut cleaned = None;
    let mut lines = stdout.lines().map(|l| l.trim_end_matches('\r'));
    while let Some(line) = lines.next() {
        match line.trim() {
            "BASE" => {
                baseline = lines
                    .next()
                    .and_then(|s| s.trim().parse::<i64>().ok())
            }
            "AFTER" => {
                after = lines
                    .next()
                    .and_then(|s| s.trim().parse::<i64>().ok())
            }
            "CLEAN" => {
                cleaned = lines
                    .next()
                    .and_then(|s| s.trim().parse::<i64>().ok())
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
fn task_hosts_llvm_real_meta() {
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
            "{name} must declare llvm=real (ADR-605): {block}"
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
            block.contains("ADR-605"),
            "{name} notes must cite ADR-605: {block}"
        );
        assert!(
            block.contains("ADR-619"),
            "{name} notes must cite ADR-619: {block}"
        );
    }
}
