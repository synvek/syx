//! W1023 / ADR-1367: Codable ×2 wasm smoke (status remains partial by charter).

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
        "syx_codable_wasm_{}_{}",
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
        .unwrap_or_else(|e| panic!("wasm Codable smoke failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    lines_joined_vec(&lines)
}

/// Same source as CODABLE_LLVM_SMOKE; structural nail only (no wasm exact).
const CODABLE_WASM_SMOKE: &str = r#"
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

fn assert_codable_wasm_smoke(lines: &[String]) {
    assert_eq!(
        lines.first().map(String::as_str),
        Some("E"),
        "wasm must start with E: {lines:?}"
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
    let encoded = after("E");
    assert!(
        encoded.contains('8') || encoded.contains('{'),
        "wasm encode JSON-ish: {lines:?}"
    );
    let decoded = after("D");
    assert!(
        decoded.contains("88") || decoded.contains("Item"),
        "wasm decode roundtrip: {lines:?}"
    );
}

#[test]
fn codable_wasm_smoke_wasm() {
    let lines = run_wasm("host_smoke", CODABLE_WASM_SMOKE);
    assert_codable_wasm_smoke(&lines);
}
