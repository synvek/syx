//! `@Codable` L2 associated-value enum full auto synthesis E2E.

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
fn codable_l2_associated_enum_roundtrip() {
    let printed = run_fixture("tests/fixtures/macros/codable_associated_enum.sy");
    assert!(
        printed.iter().any(|l| l.contains("\"__tag\":\"ok\"") && l.contains("42")),
        "ok encode: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "ok decode code: {printed:?}"
    );
    assert!(
        printed
            .iter()
            .any(|l| l.contains("\"__tag\":\"err\"") && l.contains("boom")),
        "err encode: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("boom")),
        "err decode message: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("\"__tag\":\"plain\"")),
        "plain encode: {printed:?}"
    );
}

#[test]
fn codable_l2_fingerprint_strings() {
    let attached = include_str!("../../../syx-macro/src/attached.rs");
    let lib = include_str!("../../src/lib.rs");
    assert!(attached.contains("macro-codable-l2:associated-enum-auto"));
    assert!(lib.contains("macro-codable-l2:associated-enum-auto"));
    assert!(lib.contains("heap_v101_phase129_track_a_macro_service_fingerprint"));
}
