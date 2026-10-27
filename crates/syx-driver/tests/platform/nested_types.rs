//! Nested qualified type path MVP (`lang-nested-v1:qualified-path-mvp`).

use std::path::PathBuf;

use syx_driver::{run_file_printed, DriverOptions};

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn run_fixture(rel: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let options = DriverOptions::default();
    let (code, printed) = run_file_printed(&path, &options).expect("run fixture");
    assert_eq!(code, 0, "{printed:?}");
    printed
}

#[test]
fn nested_types_run_file_printed() {
    let printed = run_fixture("tests/fixtures/lang/nested_types.sy");
    let lines: Vec<&str> = printed.iter().map(String::as_str).collect();
    assert_eq!(lines[0].trim(), "7", "Outer.Inner direct: {printed:?}");
    assert_eq!(lines[1].trim(), "7", "Holder external ref: {printed:?}");
    assert_eq!(lines[2].trim(), "9", "Outer.Middle.Deep: {printed:?}");
}

#[test]
fn nested_types_fingerprint_strings() {
    let surface = include_str!("../../src/lang_surface_v1.rs");
    assert!(surface.contains("lang-nested-v1:qualified-path-mvp"));
    assert!(surface.contains("lang-nested-v1:docs-honesty"));
}
