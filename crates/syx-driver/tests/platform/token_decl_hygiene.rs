//! Declaration-site `#macro` hygiene + linked/run_file (`macro-token-v3:decl-hygiene-mvp`).

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
fn token_decl_hygiene_run_file_printed() {
    let printed = run_fixture("tests/fixtures/macros/token_decl.sy");
    assert!(
        printed.iter().any(|l| l.contains("decl-param-ok")),
        "printer msg: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "makeSum(20,22) => 42: {printed:?}"
    );
}

#[test]
fn token_decl_hygiene_example_run_file() {
    let printed = run_fixture("../../examples/decl_param_macro/main.sy");
    assert!(
        printed.iter().any(|l| l.contains("decl-param-ok")),
        "example printer: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "example sum: {printed:?}"
    );
}

#[test]
fn token_decl_hygiene_fingerprint_strings() {
    let user_rs = include_str!("../../../syx-macro/src/user.rs");
    let hygiene_rs = include_str!("../../../syx-macro/src/hygiene.rs");
    let lib = include_str!("../../src/lib.rs");
    assert!(user_rs.contains("macro-token-v3:decl-hygiene-mvp"));
    assert!(hygiene_rs.contains("macro-token-v3:decl-hygiene-mvp"));
    assert!(lib.contains("macro-token-v3:decl-hygiene-mvp"));
}
