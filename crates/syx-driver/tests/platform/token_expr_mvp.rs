//! Expression-site `#macro` token MVP (`macro-token-v1:expr-token-mvp`).

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
fn token_expr_evaluates_reproducibly() {
    let printed = run_fixture("tests/fixtures/macros/token_expr.sy");
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "double(21) => 42: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("hi:world")),
        "wrap: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "14"),
        "double(n=7) => 14: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "double(inc(20)) => 42: {printed:?}"
    );
}

#[test]
fn token_expr_fingerprint_strings() {
    let user_rs = include_str!("../../../syx-macro/src/user.rs");
    let lib = include_str!("../../src/lib.rs");
    assert!(user_rs.contains("macro-token-v1:expr-token-mvp"));
    assert!(lib.contains("macro-token-v1:expr-token-mvp"));
    assert!(lib.contains("heap_v102_phase130_track_a_macro_service_fingerprint"));
}
