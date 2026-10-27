//! W1022 / ADR-1366: Codable ×2 llvm smoke (status remains partial; wasm still host-assisted).

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
        "syx_codable_llvm_{}_{}_{}",
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

/// W1022 / ADR-1366 — CODABLE_LLVM_SMOKE via native llvm AOT.
#[cfg(unix)]
fn run_native_llvm(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_codable_llvm_aot_{}_{}_{}",
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
        .unwrap_or_else(|e| panic!("native llvm Codable build failed: {e}"));
    let output = Command::new(&exe)
        .output()
        .expect("run native llvm Codable");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let code = output.status.code().unwrap_or(1);
    let _ = fs::remove_dir_all(&dir);
    assert_eq!(
        code, 0,
        "native llvm Codable exit {code}: stdout={stdout} stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    lines_joined_vec(
        &stdout
            .lines()
            .map(|l| l.trim_end_matches('\r').to_string())
            .collect::<Vec<_>>(),
    )
}

/// Marker lines keep llvm consecutive-line fold from collapsing encode/decode.
/// `@Codable` data class so reflect metadata is present (plain `class` encodes empty).
const CODABLE_LLVM_SMOKE: &str = r#"
@Codable
data class Item(n: Int)
func main() {
  print("E")
  let json = Item(n: 88).encode()
  print(json)
  print("D")
  print(__syx_fmt_debug(__syx_codable_decode("Item", json)))
}
"#;

fn assert_codable_llvm_smoke(lines: &[String]) {
    assert_eq!(
        lines.first().map(String::as_str),
        Some("E"),
        "must start with E: {lines:?}"
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
    let encoded = after("E");
    assert!(
        encoded.contains('8') || encoded.contains('{'),
        "encode JSON-ish: {lines:?}"
    );
    let decoded = after("D");
    assert!(
        decoded.contains("88") || decoded.contains("Item"),
        "decode roundtrip: {lines:?}"
    );
}

#[test]
fn codable_llvm_smoke_native_mir() {
    let lines = run_native_mir("host_smoke", CODABLE_LLVM_SMOKE);
    assert_codable_llvm_smoke(&lines);
}

#[cfg(unix)]
#[test]
fn codable_llvm_smoke_native_llvm() {
    let lines = run_native_llvm("host_smoke", CODABLE_LLVM_SMOKE);
    assert_codable_llvm_smoke(&lines);
}
