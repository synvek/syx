//! Typed error / catch fixtures (SYX 39.0 P1-4).
// was: errors

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

#[test]
fn typed_catch_struct_error() {
    let (code, lines) = run("errors/typed_catch.sy");
    assert_eq!(code, 0);
    assert!(lines.iter().any(|l| l.trim() == "boom"));
    assert!(lines.iter().any(|l| l.trim() == "42"));
}
