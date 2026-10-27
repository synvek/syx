//! Statement-site `#macro` token subst MVP (`macro-token-v4:stmt-token-subst-mvp`).

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
fn token_stmt_run_file_printed() {
    let printed = run_fixture("tests/fixtures/macros/stmt_token.sy");
    assert!(
        printed.iter().any(|l| l.contains("stmt-token-ok")),
        "say msg: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "logSum(20,22) => 42: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "7"),
        "sayNested(n=7): {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("hi:world")),
        "nested wrap: {printed:?}"
    );
}

#[test]
fn token_stmt_example_run_file() {
    let printed = run_fixture("../../examples/stmt_token_macro/main.sy");
    assert!(
        printed.iter().any(|l| l.contains("stmt-token-ok")),
        "example say: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "example sum: {printed:?}"
    );
}

#[test]
fn token_stmt_fingerprint_strings() {
    let user_rs = include_str!("../../../syx-macro/src/user.rs");
    let lib = include_str!("../../src/lib.rs");
    assert!(user_rs.contains("macro-token-v4:stmt-token-subst-mvp"));
    assert!(lib.contains("macro-token-v4:stmt-token-subst-mvp"));
}
