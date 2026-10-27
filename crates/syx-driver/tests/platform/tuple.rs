//! Tuple type/literal/named elements MVP (`lang-tuple-v1:product-type-mvp`).

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
fn tuple_run_file_printed() {
    let printed = run_fixture("tests/fixtures/lang/tuple.sy");
    let lines: Vec<&str> = printed.iter().map(String::as_str).collect();
    assert_eq!(lines[0].trim(), "1", "pair.0: {printed:?}");
    assert_eq!(lines[1].trim(), "a", "pair.1: {printed:?}");
    assert_eq!(lines[2].trim(), "10", "named.x: {printed:?}");
    assert_eq!(lines[3].trim(), "20", "named.y: {printed:?}");
    assert_eq!(lines[4].trim(), "3", "destructure a: {printed:?}");
    assert_eq!(lines[5].trim(), "4", "destructure b: {printed:?}");
    assert_eq!(lines[6].trim(), "1", "switch tuple x: {printed:?}");
    assert_eq!(lines[7].trim(), "2", "switch tuple y: {printed:?}");
}

#[test]
fn tuple_fingerprint_strings() {
    let surface = include_str!("../../src/lang_surface_v1.rs");
    assert!(surface.contains("lang-tuple-v1:product-type-mvp"));
    assert!(surface.contains("lang-tuple-v1:named-elements-mvp"));
    assert!(surface.contains("lang-tuple-v1:docs-honesty"));
}
