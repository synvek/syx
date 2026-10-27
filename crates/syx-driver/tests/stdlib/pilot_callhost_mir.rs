//! CallHost MirOp llvm+wasm inventory nails (Suite-1 narrow subset).

use syx_driver::{
    run_native_mir_file_printed, run_wasm_source_printed, DriverOptions,
};

fn mir_op_block<'a>(toml: &'a str, name: &str) -> &'a str {
    let needle = format!("name = \"{name}\"");
    let rest = toml
        .split(&needle)
        .nth(1)
        .unwrap_or_else(|| panic!("missing MirOp override {name}"));
    rest.split("[[op]]").next().unwrap_or(rest)
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn write_src(label: &str, src: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w209_{}_{}",
        label,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    std::fs::write(&path, src).unwrap();
    path
}

fn run_native_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native exit {code}: {lines:?}");
    lines
}

fn run_wasm_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src(label, src);
    let (code, lines) =
        run_wasm_source_printed(&path, "main", &opts()).expect("wasm");
    assert_eq!(code, 0, "wasm exit {code}: {lines:?}");
    lines
}

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

const SCALAR_COUNT_CALLHOST: &str = r#"
func main() {
  print(__syx_str_unicode_scalar_count("ab"))
}
"#;

#[test]
fn m2_s1_callhost_scalar_count_native() {
    let out = lines_joined(&run_native_src("ch_native", SCALAR_COUNT_CALLHOST));
    assert_eq!(out, "2", "native CallHost scalar count must exact: {out}");
}

#[test]
fn m2_s1_callhost_scalar_count_wasm() {
    let out = lines_joined(&run_wasm_src("ch_wasm", SCALAR_COUNT_CALLHOST));
    assert_eq!(out, "2", "wasm CallHost scalar count must exact: {out}");
}

#[test]
fn m2_s1_mir_op_callhost_inventory_real() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "CallHost");
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "CallHost product backends must be real: {block}"
    );
    assert!(
        block.contains("ADR-553"),
        "CallHost notes must cite ADR-553: {block}"
    );
}
