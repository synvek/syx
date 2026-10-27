//! Character distinct scalar MVP (`lang-character-v1:distinct-scalar-mvp`).

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
fn character_run_file_printed() {
    let printed = run_fixture("tests/fixtures/lang/character.sy");
    let lines: Vec<&str> = printed.iter().map(String::as_str).collect();
    assert_eq!(lines[0].trim(), "h", "string subscript: {printed:?}");
    assert_eq!(lines[1].trim(), "true", "isSingleScalar h: {printed:?}");
    assert_eq!(lines[2].trim(), "Z", "char literal: {printed:?}");
    assert_eq!(lines[3].trim(), "true", "isSingleScalar Z: {printed:?}");
    assert_eq!(lines[4].trim(), "a", "for-in first: {printed:?}");
    assert_eq!(lines[5].trim(), "b", "for-in second: {printed:?}");
    assert_eq!(lines[6].trim(), "true", "isSingleScalar a: {printed:?}");
}

#[test]
fn character_fingerprint_strings() {
    let surface = include_str!("../../src/lang_surface_v1.rs");
    assert!(surface.contains("lang-character-v1:distinct-scalar-mvp"));
    assert!(surface.contains("lang-character-v1:docs-honesty"));
}
