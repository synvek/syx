//! Declaration-site `#macro` param subst MVP (`macro-token-v2:decl-param-subst-mvp`).

use std::path::PathBuf;

use syx_driver::compile_file_from_source;

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

/// Expand → analyze → codegen → VM run (no linked prelude / unified MIR re-parse).
fn run_source(rel: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture(rel);
    let src = std::fs::read_to_string(&path).expect("read");
    let unit = compile_file_from_source("token_decl.sy", &src).expect("compile");
    syx_vm::reset_service_container();
    let mut vm = syx_vm::WasmVM::default();
    vm.run_program(&unit.program).expect("run");
    vm.printed
}

#[test]
fn token_decl_evaluates_reproducibly() {
    let printed = run_source("tests/fixtures/macros/token_decl.sy");
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
fn token_decl_example_runs() {
    let printed = run_source("../../examples/decl_param_macro/main.sy");
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
fn token_decl_fingerprint_strings() {
    let user_rs = include_str!("../../../syx-macro/src/user.rs");
    assert!(user_rs.contains("macro-token-v2:decl-param-subst-mvp"));
}
