//! Closed integer range `a...b` MVP (`lang-range-v1:closed-range-mvp`).

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
fn closed_range_run_file_printed() {
    let printed = run_fixture("tests/fixtures/lang/closed_range.sy");
    let lines: Vec<&str> = printed.iter().map(String::as_str).collect();
    assert_eq!(lines[0].trim(), "true", "1...5 contains 5: {printed:?}");
    assert_eq!(lines[1].trim(), "false", "1...5 contains 6: {printed:?}");
    assert_eq!(lines[2].trim(), "true", "1...5 contains 1: {printed:?}");
    assert_eq!(lines[3].trim(), "6", "for-in 1...3 sum: {printed:?}");
    assert_eq!(lines[4].trim(), "small", "switch 1...3 matches 3: {printed:?}");
    assert_eq!(lines[5].trim(), "other", "switch default: {printed:?}");
}

#[test]
fn closed_range_fingerprint_strings() {
    let surface = include_str!("../../src/lang_surface_v1.rs");
    assert!(surface.contains("lang-range-v1:closed-range-mvp"));
    assert!(surface.contains("lang-range-v1:docs-honesty"));
}
