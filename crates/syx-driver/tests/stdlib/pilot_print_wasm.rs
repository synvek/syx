//! Print MirOp wasm inventory nails (Suite-1 narrow subset).

use syx_driver::{run_wasm_source_printed, DriverOptions};

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
        "syx_w208_{}_{}",
        label,
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    std::fs::write(&path, src).unwrap();
    path
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

const INT_PRINT: &str = r#"
func main() {
  print(10 + 5)
}
"#;

const STRING_PRINT: &str = r#"
func main() {
  print("hi")
}
"#;

#[test]
fn m2_s1_print_int_wasm() {
    let out = lines_joined(&run_wasm_src("int_print", INT_PRINT));
    assert_eq!(out, "15", "wasm Int print must exact: {out}");
}

#[test]
fn m2_s1_print_string_wasm() {
    let out = lines_joined(&run_wasm_src("str_print", STRING_PRINT));
    assert_eq!(out, "hi", "wasm String print must exact: {out}");
}

#[test]
fn m2_s1_mir_op_print_inventory_real() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "Print");
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "Print product backends must be real: {block}"
    );
    assert!(
        block.contains("ADR-552"),
        "Print notes must cite ADR-552: {block}"
    );
}
