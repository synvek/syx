//! ConstBool / ConstNil MirOp llvm+wasm RT push nails.
// was: pilot inventory deepen for Suite-1 ConstBool/ConstNil

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed,
    DriverOptions,
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
        "syx_w203_{}_{}",
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

fn run_vm(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts())
        .expect("vm")
        .printed
        .join("\n")
}

const BOOL_PRINT: &str = r#"
func main() {
  print(true)
  print(false)
}
"#;

const BOOL_BRANCH: &str = r#"
func main() {
  if false {
    print("X")
  } else {
    print("Y")
  }
}
"#;

const NIL_PRINT: &str = r#"
func main() {
  print(nil)
}
"#;

#[test]
fn m2_s1_const_bool_print_vm() {
    assert_eq!(run_vm(BOOL_PRINT), "true\nfalse");
}

#[test]
fn m2_s1_const_bool_print_native() {
    let lines = run_native_src("bool_print", BOOL_PRINT);
    assert_eq!(
        lines_joined(&lines),
        "true\nfalse",
        "native ConstBool print must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_const_bool_print_wasm() {
    let lines = run_wasm_src("bool_print", BOOL_PRINT);
    assert_eq!(
        lines_joined(&lines),
        "true\nfalse",
        "wasm ConstBool print must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_const_bool_branch_vm() {
    assert_eq!(run_vm(BOOL_BRANCH), "Y");
}

#[test]
fn m2_s1_const_bool_branch_native() {
    let lines = run_native_src("bool_br", BOOL_BRANCH);
    assert_eq!(
        lines_joined(&lines),
        "Y",
        "native ConstBool branch must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_const_bool_branch_wasm() {
    let lines = run_wasm_src("bool_br", BOOL_BRANCH);
    assert_eq!(
        lines_joined(&lines),
        "Y",
        "wasm ConstBool branch must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_const_nil_print_vm() {
    assert_eq!(run_vm(NIL_PRINT), "nil");
}

#[test]
fn m2_s1_const_nil_print_native() {
    let lines = run_native_src("nil_print", NIL_PRINT);
    assert_eq!(
        lines_joined(&lines),
        "nil",
        "native ConstNil print must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_const_nil_print_wasm() {
    let lines = run_wasm_src("nil_print", NIL_PRINT);
    assert_eq!(
        lines_joined(&lines),
        "nil",
        "wasm ConstNil print must exact: {lines:?}"
    );
}

#[test]
fn m2_s1_mir_op_const_bool_inventory_real() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "ConstBool");
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "ConstBool product backends must be real: {block}"
    );
    assert!(
        block.contains("ADR-547"),
        "ConstBool notes must cite ADR-547: {block}"
    );
}

#[test]
fn m2_s1_mir_op_const_nil_inventory_real() {
    let toml = include_str!("../../../../packages/internal/mir_op_capabilities.toml");
    let block = mir_op_block(toml, "ConstNil");
    assert!(
        block.contains("llvm = \"real\"") && block.contains("wasm = \"real\""),
        "ConstNil product backends must be real: {block}"
    );
    assert!(
        block.contains("ADR-547"),
        "ConstNil notes must cite ADR-547: {block}"
    );
}
