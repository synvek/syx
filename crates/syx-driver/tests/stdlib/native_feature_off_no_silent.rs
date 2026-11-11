//! 04 native feature-off must not silent-empty-succeed.
//! native/wasm behavioral nails for collections.feature_off.
// was: m2_s1_04_native_feature_off_no_silent

use std::fs;
use std::path::PathBuf;

use syx_driver::{
    compile_and_run_embedded, run_native_mir_file_printed, run_wasm_source_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run_err(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let err = compile_and_run_embedded(src, &opts()).expect_err("expected fail-closed HOF error");
    format!("{err}")
}

fn write_src_tmp(label: &str, src: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("syx_w205_{}_{}", label, std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    fs::write(&path, src).unwrap();
    path
}

fn lines_joined(lines: &[String]) -> String {
    lines
        .iter()
        .map(|l| l.trim_end_matches('\r').trim_end_matches('\n'))
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn run_native_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    match run_native_mir_file_printed(&path, &opts()) {
        Ok((code, lines)) if code == 0 => lines_joined(&lines),
        Ok((_, lines)) if !lines.is_empty() => lines_joined(&lines),
        Ok(_) => String::new(),
        Err(e) => format!("{e}"),
    }
}

fn run_wasm_err(label: &str, src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = write_src_tmp(label, src);
    match run_wasm_source_printed(&path, "main", &opts()) {
        Ok((code, lines)) if code == 0 => lines_joined(&lines),
        Ok((_, lines)) if !lines.is_empty() => lines_joined(&lines),
        Ok(_) => String::new(),
        Err(e) => format!("{e}"),
    }
}

fn assert_hof_fail_closed(src: &str, label: &str, expected: &str) {
    // Wave A W92 / ADR-436 / A-03: HOF feature-off fail-closed product exact Display.
    let msg = run_err(src);
    assert_eq!(msg, expected, "{label}: HOF fail-closed must exact: {msg}");
    assert!(
        !msg.lines().any(|l| l.trim() == "[]"),
        "{label}: must not silent-empty-succeed with []: {msg}"
    );
}

const MAP_U001: &str =
    "[U001] unsupported array HOF 'map': requires Array receiver and Closure (feature-off or illegal shape)";
const FILTER_U001: &str =
    "[U001] unsupported array HOF 'filter': requires Array receiver and Closure (feature-off or illegal shape)";
const REDUCE_U001: &str =
    "[U001] unsupported array HOF 'reduce': requires Array receiver and Closure (feature-off or illegal shape)";

#[test]
fn m2_s1_04_map_missing_closure_fails() {
    assert_hof_fail_closed(
        r#"func main() { print(__syx_array_map([1, 2, 3], 0)) }"#,
        "map missing Closure",
        MAP_U001,
    );
}

#[test]
fn m2_s1_04_filter_missing_closure_fails() {
    assert_hof_fail_closed(
        r#"func main() { print(__syx_array_filter([1, 2, 3], 0)) }"#,
        "filter missing Closure",
        FILTER_U001,
    );
}

#[test]
fn m2_s1_04_map_illegal_receiver_fails() {
    // Same Display as map missing Closure (illegal shape); keep exact, not a second W92 nail.
    assert_hof_fail_closed(
        r#"func main() { print(__syx_array_map(0, { (x: Int) -> Int in return x })) }"#,
        "map illegal receiver",
        MAP_U001,
    );
}

#[test]
fn m2_s1_04_filter_illegal_receiver_fails() {
    // Wave A W94 / ADR-438 / A-03: HOF filter illegal receiver product exact.
    assert_hof_fail_closed(
        r#"func main() { print(__syx_array_filter(0, { (x: Int) -> Bool in return true })) }"#,
        "filter illegal receiver",
        FILTER_U001,
    );
}

#[test]
fn m2_s1_04_reduce_missing_closure_fails() {
    // Wave A W93 / ADR-437 / A-03: HOF reduce missing Closure product exact.
    assert_hof_fail_closed(
        r#"func main() { print(__syx_array_reduce([1, 2, 3], 0)) }"#,
        "reduce missing Closure",
        REDUCE_U001,
    );
}

#[test]
fn m2_s1_04_reduce_illegal_receiver_fails() {
    // Wave A W96 / ADR-440 / A-03: HOF reduce illegal receiver product exact.
    assert_hof_fail_closed(
        r#"func main() { print(__syx_array_reduce(0, { (a: Int, b: Int) -> Int in return a + b })) }"#,
        "reduce illegal receiver",
        REDUCE_U001,
    );
}

#[test]
fn m2_s1_04_hof_map_u001_native() {
    let err = run_native_err(
        "map_n",
        r#"func main() { print(__syx_array_map([1, 2, 3], 0)) }"#,
    );
    assert_eq!(err, MAP_U001, "native map U001 must exact: {err}");
    assert!(
        !err.lines().any(|l| l.trim() == "[]"),
        "native must not silent-empty-succeed: {err}"
    );
}

#[test]
fn m2_s1_04_hof_map_u001_wasm() {
    let err = run_wasm_err(
        "map_w",
        r#"func main() { print(__syx_array_map([1, 2, 3], 0)) }"#,
    );
    assert_eq!(err, MAP_U001, "wasm map U001 must exact: {err}");
    assert!(
        !err.lines().any(|l| l.trim() == "[]"),
        "wasm must not silent-empty-succeed: {err}"
    );
}

#[test]
fn m2_s1_04_hof_filter_u001_native() {
    let err = run_native_err(
        "filter_n",
        r#"func main() { print(__syx_array_filter([1, 2, 3], 0)) }"#,
    );
    assert_eq!(err, FILTER_U001, "native filter U001 must exact: {err}");
}

#[test]
fn m2_s1_04_hof_filter_u001_wasm() {
    let err = run_wasm_err(
        "filter_w",
        r#"func main() { print(__syx_array_filter([1, 2, 3], 0)) }"#,
    );
    assert_eq!(err, FILTER_U001, "wasm filter U001 must exact: {err}");
}

#[test]
fn m2_s1_04_hof_reduce_u001_native() {
    let err = run_native_err(
        "reduce_n",
        r#"func main() { print(__syx_array_reduce([1, 2, 3], 0)) }"#,
    );
    assert_eq!(err, REDUCE_U001, "native reduce U001 must exact: {err}");
}

#[test]
fn m2_s1_04_hof_reduce_u001_wasm() {
    let err = run_wasm_err(
        "reduce_w",
        r#"func main() { print(__syx_array_reduce([1, 2, 3], 0)) }"#,
    );
    assert_eq!(err, REDUCE_U001, "wasm reduce U001 must exact: {err}");
}

#[test]
fn m2_s1_04_inventory_meta() {
    let inv = include_str!("../../../../packages/internal/error_model_migration.toml");
    assert!(
        inv.contains("m2_s1_04_native_feature_off_no_silent_tier"),
        "inventory must contain m2_s1_04_native_feature_off_no_silent_tier"
    );
    assert!(
        inv.contains("ADR-225") || inv.contains("signed-ADR-225"),
        "inventory must note ADR-225"
    );
}
