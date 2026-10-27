//! W1006 / ADR-1350: Log rest ×8 mir+llvm smoke (status remains partial; ≠ wasm exact).

use std::fs;
use std::process::Command;

use syx_driver::{build_native_executable, run_native_mir_file_printed, DriverOptions};

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

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    use std::sync::atomic::{AtomicU64, Ordering};
    static SEQ: AtomicU64 = AtomicU64::new(0);
    let dir = std::env::temp_dir().join(format!(
        "syx_log_rest_{}_{}_{}",
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

fn run_native_mir(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &DriverOptions::default()).expect("native mir");
    assert_eq!(code, 0, "native mir exit {code}: {lines:?}");
    lines_joined(&lines)
}

/// W1006 / ADR-1350 — LOG_REST_SMOKE via native llvm AOT.
#[cfg(unix)]
fn run_native_llvm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_log_rest_llvm_{}_{}_{}",
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
        .unwrap_or_else(|e| panic!("native llvm Log rest build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native llvm Log rest");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        code, 0,
        "native llvm Log rest exit {code}: stdout={stdout} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    lines_joined(
        &stdout
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect::<Vec<_>>(),
    )
}

const LOG_REST_SMOKE: &str = r#"
func main() {
  print(__syx_log_policy())
  print(__syx_log_set_min_level("DEBUG"))
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
  // Restore default min level so in-process native mir does not poison sibling Log smokes.
  let _ = __syx_log_set_min_level("DEBUG")
}
"#;

const LOG_REST_SMOKE_EXPECTED: &str =
    "log-v1-fields\ntrue\n[DEBUG] d\n[WARN] w\n[INFO] hi a=1 actor_id=- b=2 span=- task_id=-\ntrue\n0\n[ERROR] e";

#[test]
fn log_rest_smoke_native_mir() {
    let out = run_native_mir("host_smoke", LOG_REST_SMOKE);
    assert_eq!(out, LOG_REST_SMOKE_EXPECTED, "native mir Log rest smoke: {out}");
}

#[cfg(unix)]
#[test]
fn log_rest_smoke_native_llvm() {
    let out = run_native_llvm("host_smoke", LOG_REST_SMOKE);
    assert_eq!(
        out, LOG_REST_SMOKE_EXPECTED,
        "native llvm Log rest smoke: {out}"
    );
}
