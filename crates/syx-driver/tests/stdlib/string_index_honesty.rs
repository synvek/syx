//! 02 String Int grapheme offset O(n) honesty (no opaque Index).
//! native/wasm behavioral nails for text.index_honesty.
//! StringCharAt HostFn llvm=real (native AOT nail).
//! StringCharAt HostFn wasm=real (text HostFn wasm8).
// was: m2_s2_02_string_index_honesty

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts())
        .expect("run")
        .printed
        .join("\n")
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "syx_w210_{}_{}",
        label,
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

fn run_native_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    let (code, lines) =
        run_native_mir_file_printed(&path, &opts()).expect("native mir");
    assert_eq!(code, 0, "native exit {code}: {lines:?}");
    lines
}

fn run_wasm_src(label: &str, src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
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

fn run_native_strict(src: &str) -> (i32, String) {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!(
        "syx_w239_char_at_llvm_{}",
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

const ZWJ_SUBSCRIPT: &str = r#"
func main() {
  let s = "👨‍👩‍👧"
  print(String.count(s))
  print(__syx_value_description(s[0]))
}
"#;

const ASCII_OFFSET: &str = r#"
func main() {
  let s = "ab"
  print(String.count(s))
  print(__syx_value_description(s[0]))
  print(__syx_value_description(s[1]))
}
"#;

const CHAR_AT_AB0: &str = r#"
func main() {
  print(__syx_value_description(__syx_string_char_at("ab", 0)))
}
"#;

#[test]
fn m2_s2_02_zwj_subscript_native() {
    let out = lines_joined(&run_native_src("zwj_native", ZWJ_SUBSCRIPT));
    assert_eq!(out, "1\n👨‍👩‍👧", "native ZWJ subscript must exact: {out}");
}

#[test]
fn m2_s2_02_zwj_subscript_wasm() {
    let out = lines_joined(&run_wasm_src("zwj_wasm", ZWJ_SUBSCRIPT));
    assert_eq!(out, "1\n👨‍👩‍👧", "wasm ZWJ subscript must exact: {out}");
}

#[test]
fn m2_s2_02_ascii_offset_native() {
    let out = lines_joined(&run_native_src("ascii_native", ASCII_OFFSET));
    assert_eq!(out, "2\na\nb", "native ASCII offset must exact: {out}");
}

#[test]
fn m2_s2_02_ascii_offset_wasm() {
    let out = lines_joined(&run_wasm_src("ascii_wasm", ASCII_OFFSET));
    assert_eq!(out, "2\na\nb", "wasm ASCII offset must exact: {out}");
}

#[test]
fn m2_s2_02_zwj_family_subscript_still_one_grapheme() {
    let out = run(ZWJ_SUBSCRIPT);
    // Wave A W86 / ADR-430 / A-03: ZWJ family subscript still one grapheme product exact.
    assert_eq!(out, "1\n👨‍👩‍👧", "ZWJ subscript grapheme must exact: {out}");
}

#[test]
fn m2_s2_02_string_char_at_wasm() {
    let out = lines_joined(&run_wasm_src("char_at_wasm", CHAR_AT_AB0));
    assert_eq!(out, "a", "wasm string_char_at(ab,0) must exact: {out}");
}

#[test]
fn m2_s2_02_string_char_at_native_llvm() {
    let (code, stdout) = run_native_strict(CHAR_AT_AB0);
    let out = stdout
        .lines()
        .map(|l| l.trim_end_matches('\r'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(code, 0, "native llvm exit {code}: {stdout}");
    assert_eq!(out, "a", "native llvm string_char_at(ab,0) must exact: {out}");
}

#[test]
fn m2_s2_02_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s2_02_string_index_honesty_tier"),
        "inventory must contain m2_s2_02_string_index_honesty_tier"
    );
    assert!(
        inv.contains("ADR-227") || inv.contains("signed-ADR-227"),
        "inventory must note ADR-227"
    );
}

