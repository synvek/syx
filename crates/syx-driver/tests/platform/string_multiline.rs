//! Multiline string `"""` literal MVP (`lang-string-v1:multiline-literal-mvp`).

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
fn string_multiline_run_file_printed() {
    let printed = run_fixture("tests/fixtures/lang/string_multiline.sy");
    assert!(
        printed.iter().any(|l| l.trim().parse::<i64>().ok() == Some(19)),
        "multiline literal length: {printed:?}"
    );
    let hi = printed
        .iter()
        .find(|l| l.contains("Hello, World!"))
        .expect("interpolated multiline output");
    assert!(hi.contains('\n'), "expected bare newline in multiline: {hi:?}");
}

#[test]
fn string_multiline_fingerprint_strings() {
    let surface = include_str!("../../src/lang_surface_v1.rs");
    assert!(surface.contains("lang-string-v1:multiline-literal-mvp"));
    assert!(surface.contains("lang-string-v1:docs-honesty"));
}
