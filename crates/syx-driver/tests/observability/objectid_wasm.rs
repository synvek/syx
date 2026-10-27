//! W1027 / ADR-1371: ObjectIdentifier ×1 wasm smoke (status remains partial by charter).

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
        "syx_objectid_wasm_{}_{}",
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
        .unwrap_or_else(|e| panic!("wasm ObjectIdentifier smoke failed: {e:?}"));
    let _ = fs::remove_dir_all(&dir);
    lines_joined_vec(&lines)
}

/// Same source as OBJECTID_LLVM_SMOKE; structural nail only (no wasm exact).
const OBJECTID_WASM_SMOKE: &str = r#"
func main() {
  print("OID")
  print(__syx_fmt_debug(__syx_object_identifier("x")))
}
"#;

fn assert_objectid_wasm_smoke(lines: &[String]) {
    assert_eq!(
        lines.first().map(String::as_str),
        Some("OID"),
        "wasm must start with OID: {lines:?}"
    );
    let idx = lines
        .iter()
        .position(|l| l == "OID")
        .unwrap_or_else(|| panic!("wasm missing OID: {lines:?}"));
    let desc = lines
        .get(idx + 1)
        .map(String::as_str)
        .unwrap_or_else(|| panic!("wasm missing value after OID: {lines:?}"));
    assert!(
        desc.contains("ObjectIdentifier") || desc.contains('('),
        "wasm ObjectIdentifier description: {lines:?}"
    );
}

#[test]
fn objectid_wasm_smoke_wasm() {
    let lines = run_wasm("host_smoke", OBJECTID_WASM_SMOKE);
    assert_objectid_wasm_smoke(&lines);
}
