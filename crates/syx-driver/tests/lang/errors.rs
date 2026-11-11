//! Typed error / catch fixtures (SYX 39.0 P1-4).
//! R-30: cross-function `do { try g() } catch` must enter catch (not print error as value).
// was: errors

use std::sync::{Mutex, OnceLock};

use syx_driver::{compile_file_with_options, run_file_printed, DriverOptions};

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn run(rel: &str) -> (i32, Vec<String>) {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let options = DriverOptions::default();
    compile_file_with_options(&path, &options).expect("compile");
    run_file_printed(&path, &options).expect("run")
}

fn mir_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn run_mir(rel: &str) -> (i32, Vec<String>) {
    let _guard = mir_lock();
    std::env::set_var("SYX_USE_MIR", "1");
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let out = run_file_printed(&path, &DriverOptions::default())
        .unwrap_or_else(|e| panic!("MIR run failed for {rel}: {e}"));
    std::env::remove_var("SYX_USE_MIR");
    out
}

fn assert_caught_not_value(lines: &[String], label: &str) {
    assert!(
        lines.iter().any(|l| l.trim() == "caught"),
        "{label}: expected caught, got {lines:?}"
    );
    assert!(
        !lines.iter().any(|l| l.trim() == "value"),
        "{label}: must not print value after throw, got {lines:?}"
    );
    assert!(
        !lines
            .iter()
            .any(|l| l.contains("{case:") || l.contains("case: bad")),
        "{label}: error must not print as return value, got {lines:?}"
    );
}

#[test]
fn typed_catch_struct_error() {
    let (code, lines) = run("errors/typed_catch.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "boom"));
    assert!(lines.iter().any(|l| l.trim() == "42"));
}

#[test]
fn try_cross_fn_enum_error_vm() {
    let (code, lines) = run("errors/try_cross_fn.sy");
    assert_eq!(code, 0, "vm exit: {lines:?}");
    assert_caught_not_value(&lines, "vm enum");
}

#[test]
fn try_cross_fn_string_error_vm() {
    let (code, lines) = run("errors/try_cross_fn_string.sy");
    assert_eq!(code, 0, "vm exit: {lines:?}");
    assert_caught_not_value(&lines, "vm string");
}

#[test]
fn try_cross_fn_enum_error_mir() {
    let (code, lines) = run_mir("errors/try_cross_fn.sy");
    assert_eq!(code, 0, "mir exit: {lines:?}");
    assert_caught_not_value(&lines, "mir enum");
}

#[test]
fn try_cross_fn_string_error_mir() {
    let (code, lines) = run_mir("errors/try_cross_fn_string.sy");
    assert_eq!(code, 0, "mir exit: {lines:?}");
    assert_caught_not_value(&lines, "mir string");
}
