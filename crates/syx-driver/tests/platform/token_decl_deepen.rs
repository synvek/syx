//! Declaration-site `#macro` param subst deepen (`macro-token-v6:decl-token-subst-mvp`).

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
fn token_decl_deepen_token_run_file_printed() {
    let printed = run_fixture("tests/fixtures/macros/decl_token.sy");
    assert!(
        printed.iter().any(|l| l.contains("decl-token-ok")),
        "greet msg: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("decl-token-bye")),
        "bye msg: {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.trim() == "42"),
        "compute() => 42: {printed:?}"
    );
}

#[test]
fn token_decl_deepen_token_fingerprint_strings() {
    let user_rs = include_str!("../../../syx-macro/src/user.rs");
    let lib = include_str!("../../src/lib.rs");
    assert!(user_rs.contains("macro-token-v6:decl-token-subst-mvp"));
    assert!(lib.contains("macro-token-v6:decl-token-subst-mvp"));
}
