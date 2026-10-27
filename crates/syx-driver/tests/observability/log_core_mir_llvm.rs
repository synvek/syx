//! W1005 / ADR-1349: Log core ×4 mir+llvm smoke (status remains partial; ≠ wasm exact).

use std::fs;
use std::process::Command;

use syx_driver::{
    build_native_executable, run_native_mir_file_printed, DriverOptions,
};

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
        "syx_log_core_{}_{}_{}",
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

/// W1005 / ADR-1349 — LOG_CORE_SMOKE via native llvm AOT.
#[cfg(unix)]
fn run_native_llvm(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_log_core_llvm_{}_{}_{}",
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
        .unwrap_or_else(|e| panic!("native llvm Log core build failed: {e}"));
    let output = Command::new(&exe).output().expect("run native llvm Log core");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        code, 0,
        "native llvm Log core exit {code}: stdout={stdout} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    lines_joined(
        &stdout
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect::<Vec<_>>(),
    )
}

const LOG_CORE_SMOKE: &str = r#"
func main() {
  print(__syx_log_min_level())
  print(__syx_log_set_min_level("WARN"))
  print(__syx_log_min_level())
  __syx_log_info("hidden")
  print(__syx_str_utf8_count(__syx_log_take_last()))
  print(__syx_log_set_min_level("DEBUG"))
  __syx_log_info("hello")
  print(__syx_log_take_last())
}
"#;

const LOG_CORE_SMOKE_EXPECTED: &str = "DEBUG\ntrue\nWARN\n0\ntrue\n[INFO] hello";

#[test]
fn log_core_smoke_native_mir() {
    let out = run_native_mir("host_smoke", LOG_CORE_SMOKE);
    assert_eq!(out, LOG_CORE_SMOKE_EXPECTED, "native mir Log core smoke: {out}");
}

#[cfg(unix)]
#[test]
fn log_core_smoke_native_llvm() {
    let out = run_native_llvm("host_smoke", LOG_CORE_SMOKE);
    assert_eq!(out, LOG_CORE_SMOKE_EXPECTED, "native llvm Log core smoke: {out}");
}
