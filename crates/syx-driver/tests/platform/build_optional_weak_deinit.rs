//! buildOptional + weak deinit language gates.

use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

fn fixture(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

#[test]
fn build_optional_lang_resultbuilder_v2_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("lang-resultbuilder-v2:build-optional"));
}

#[test]
fn build_optional_lang_deinit_weak_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("lang-deinit-weak-v1:weak-nil-on-drop"));
}

#[test]
fn build_optional_resultbuilder_build_optional_stdout() {
    // Wave A W2 / ADR-346 / A-03: exact lines (mirror resultbuilder_ga.rs)
    let src = std::fs::read_to_string(fixture("tests/fixtures/lang/resultbuilder_build_optional.sy")).unwrap();
    let out = run(&src);
    assert_eq!(out, "2\n42\n99", "buildOptional exact stdout: {out}");
}

#[test]
fn build_optional_resultbuilder_build_optional_host_registered() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("__syx_result_builder_buildOptional"));
}

#[test]
fn build_optional_weak_nil_on_drop_stdout() {
    // Wave A W3 / ADR-347 / A-03: exact line vector (weak alive → deinit → dead).
    let src = std::fs::read_to_string(fixture("tests/fixtures/lang/weak_nil_on_drop.sy")).unwrap();
    let out = run(&src);
    assert_eq!(
        out,
        "true\ndeinit:\n1\nfalse\ndone\ndeinit:\n2",
        "weak nil-on-drop exact stdout: {out}"
    );
}

#[test]
fn build_optional_weak_parser_desugar_host() {
    let src = r#"class C { var n: Int init(n: Int) { self.n = n } }
func main() {
    var c = C(n: 1)
    weak var w = c
    print(__syx_weak_alive(w))
}"#;
    assert_eq!(run(src), "true");
}

#[test]
fn build_optional_weak_hosts_registry() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("__syx_weak_from"));
    assert!(reg.contains("__syx_weak_alive"));
}
