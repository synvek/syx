//! W1029 / ADR-1373: ActorReentrancyPolicy ×1 llvm smoke (status remains partial).

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
        "syx_actor_llvm_{}_{}_{}",
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

/// W1029 / ADR-1373 — ACTOR_LLVM_SMOKE via native llvm AOT.
#[cfg(unix)]
fn run_native_llvm(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_actor_llvm_aot_{}_{}_{}",
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
        .unwrap_or_else(|e| panic!("native llvm ActorReentrancyPolicy build failed: {e}"));
    let output = Command::new(&exe)
        .output()
        .expect("run native llvm ActorReentrancyPolicy");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        code, 0,
        "native llvm ActorReentrancyPolicy exit {code}: stdout={stdout} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    lines_joined_vec(
        &stdout
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect::<Vec<_>>(),
    )
}

/// Marker lines keep llvm consecutive-line fold from collapsing the policy line.
const ACTOR_LLVM_SMOKE: &str = r#"
func main() {
  print("ACTOR")
  print(__syx_actor_reentrancy_policy())
}
"#;

fn assert_actor_llvm_smoke(lines: &[String]) {
    assert_eq!(
        lines.first().map(String::as_str),
        Some("ACTOR"),
        "must start with ACTOR: {lines:?}"
    );
    let idx = lines
        .iter()
        .position(|l| l == "ACTOR")
        .unwrap_or_else(|| panic!("missing ACTOR: {lines:?}"));
    let policy = lines
        .get(idx + 1)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("missing value after ACTOR: {lines:?}"));
    assert!(
        policy.contains("serial-no-reenter"),
        "ActorReentrancyPolicy fingerprint: {lines:?}"
    );
}

#[test]
fn actor_llvm_smoke_native_mir() {
    let lines = run_native_mir("host_smoke", ACTOR_LLVM_SMOKE);
    assert_actor_llvm_smoke(&lines);
}

#[cfg(unix)]
#[test]
fn actor_llvm_smoke_native_llvm() {
    let lines = run_native_llvm("host_smoke", ACTOR_LLVM_SMOKE);
    assert_actor_llvm_smoke(&lines);
}
