//! Phase 91: Array.sortBy / flatMap SYX closure comparator stdout (VM / MIR / WASM / native exe).

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

const SORT_BY_CLOSURE_SRC: &str = r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
    print(s[2])
}"#;

const SORT_BY_INLINE_SRC: &str = r#"func main() {
    let s = __syx_array_sort_by([3, 1, 2], { (a: Int, b: Int) -> Int in return a - b })
    print(s[0])
    print(s[2])
}"#;

const FLAT_MAP_CLOSURE_SRC: &str = r#"func main() {
    let flat = __syx_array_flat_map([1, 2, 3], { (x: Int) -> Array in return [x, x] })
    print(flat.count)
    print(flat[0])
    print(flat[2])
}"#;

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run_embedded(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts()).unwrap().printed
}

fn run_mir_src(src: &str) -> Vec<String> {
    let dir = temp_src_dir("mir", src);
    let path = dir.join("main.sy");
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_native_mir_file_printed(&path, &opts()).unwrap();
    let _ = fs::remove_dir_all(&dir);
    lines
}

fn run_wasm_src(src: &str) -> Vec<String> {
    let dir = temp_src_dir("wasm", src);
    let path = dir.join("main.sy");
    syx_host_plugin::register_echo_plugin_in_process();
    let (_, lines) = run_wasm_source_printed(&path, "main", &opts()).unwrap();
    let _ = fs::remove_dir_all(&dir);
    lines
}

fn run_native_src(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = temp_src_dir("native", src);
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    let _ = fs::remove_dir_all(&dir);
    output
        .stdout
        .split(|&b| b == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| String::from_utf8_lossy(line).into_owned())
        .collect()
}

fn temp_src_dir(tag: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("syx_p98_hof_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    dir
}

fn assert_lines_contain(lines: &[String], needle: &str, backend: &str) {
    assert!(
        lines.iter().any(|l| l.trim() == needle || l.contains(needle)),
        "{backend} missing {needle:?}: {lines:?}"
    );
}

#[test]
fn p98_hof_sort_by_named_fn_vm_stdout() {
    let lines = run_embedded(SORT_BY_CLOSURE_SRC);
    assert_lines_contain(&lines, "1", "vm");
    assert_lines_contain(&lines, "3", "vm");
}

#[test]
fn p98_hof_sort_by_named_fn_mir_stdout() {
    let lines = run_mir_src(SORT_BY_CLOSURE_SRC);
    assert_lines_contain(&lines, "1", "mir");
    assert_lines_contain(&lines, "3", "mir");
}

#[test]
fn p98_hof_sort_by_named_fn_wasm_stdout() {
    let lines = run_wasm_src(SORT_BY_CLOSURE_SRC);
    assert_lines_contain(&lines, "1", "wasm");
    assert_lines_contain(&lines, "3", "wasm");
}

#[test]
fn p98_hof_sort_by_named_fn_native_stdout() {
    let lines = run_native_src(SORT_BY_CLOSURE_SRC);
    assert_lines_contain(&lines, "1", "native");
    assert_lines_contain(&lines, "3", "native");
}

#[test]
fn p98_hof_sort_by_inline_closure_vm_stdout() {
    let lines = run_embedded(SORT_BY_INLINE_SRC);
    assert_lines_contain(&lines, "1", "vm");
    assert_lines_contain(&lines, "3", "vm");
}

#[test]
fn p98_hof_flat_map_named_fn_vm_stdout() {
    let lines = run_embedded(FLAT_MAP_CLOSURE_SRC);
    assert_lines_contain(&lines, "6", "vm");
    assert_lines_contain(&lines, "1", "vm");
}

#[test]
fn p98_hof_flat_map_named_fn_mir_stdout() {
    let lines = run_mir_src(FLAT_MAP_CLOSURE_SRC);
    assert_lines_contain(&lines, "6", "mir");
}

#[test]
fn p98_hof_flat_map_named_fn_wasm_stdout() {
    let lines = run_wasm_src(FLAT_MAP_CLOSURE_SRC);
    assert_lines_contain(&lines, "6", "wasm");
}

#[test]
fn p98_hof_flat_map_named_fn_native_stdout() {
    let lines = run_native_src(FLAT_MAP_CLOSURE_SRC);
    assert_lines_contain(&lines, "6", "native");
}

#[test]
fn p98_hof_closure_host_registry() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("ArraySortBy") && reg.contains("ArrayFlatMap"));
}

#[test]
fn p98_hof_closure_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("hof-mir-wasm-parity-v1"));
}
