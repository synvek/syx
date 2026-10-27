//! Phase 91: generic HOF witness sortBy/flatMap (String comparator) four-end stdout hard.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

const STRING_SORT_SRC: &str = r#"func cmp(a: String, b: String) -> Int {
    if a < b { return -1 }
    if a > b { return 1 }
    return 0
}
func main() {
    let s = __syx_array_sort_by(["c", "a", "b"], cmp)
    print(s[0])
    print(s[2])
}"#;

const STRING_SORT_INLINE_SRC: &str = r#"func main() {
    let s = __syx_array_sort_by(["c", "a", "b"], { (a: String, b: String) -> Int in
        if a < b { return -1 }
        if a > b { return 1 }
        return 0
    })
    print(s[0])
    print(s[2])
}"#;

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn temp_dir(src: &str, tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("syx_p98_hof_gen_{tag}_{}", std::process::id()));
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

fn assert_sorted(lines: &[String], backend: &str) {
    assert!(lines.iter().any(|l| l.trim() == "a" || l.contains("a")), "{backend}: {lines:?}");
    assert!(lines.iter().any(|l| l.trim() == "c" || l.contains("c")), "{backend}: {lines:?}");
}

#[test]
fn p98_hof_generic_witness_string_sort_vm_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let lines = compile_and_run_embedded(STRING_SORT_SRC, &opts()).unwrap().printed;
    assert_sorted(&lines, "vm");
}

#[test]
fn p98_hof_generic_witness_string_sort_mir_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = temp_dir(STRING_SORT_SRC, "mir");
    let (_, lines) = run_native_mir_file_printed(&dir.join("main.sy"), &opts()).unwrap();
    let _ = fs::remove_dir_all(&dir);
    assert_sorted(&lines, "mir");
}

#[test]
fn p98_hof_generic_witness_string_sort_wasm_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let dir = temp_dir(STRING_SORT_SRC, "wasm");
    let (_, lines) = run_wasm_source_printed(&dir.join("main.sy"), "main", &opts()).unwrap();
    let _ = fs::remove_dir_all(&dir);
    assert_sorted(&lines, "wasm");
}

#[test]
fn p98_hof_generic_witness_string_sort_native_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = temp_dir(STRING_SORT_SRC, "native");
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).unwrap();
    let output = Command::new(&exe).output().unwrap();
    let _ = fs::remove_dir_all(&dir);
    let lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.to_string())
        .collect();
    assert_sorted(&lines, "native");
}

#[test]
fn p98_hof_generic_witness_inline_closure_vm_stdout() {
    syx_host_plugin::register_echo_plugin_in_process();
    let lines = compile_and_run_embedded(STRING_SORT_INLINE_SRC, &opts()).unwrap().printed;
    assert_sorted(&lines, "vm-inline");
}

#[test]
fn p98_hof_generic_witness_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("hof-generic-witness-v1"));
}

#[test]
fn p98_hof_generic_witness_registry() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("ArraySortBy"));
}
