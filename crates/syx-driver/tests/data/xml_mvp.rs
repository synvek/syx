//! Phase 91: XML pull parser MVP E2E.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_xml_mvp_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("xml-mvp-v1"));
}

#[test]
fn p98_xml_mvp_host_surface() {
    let reg = include_str!("../../../syx-stdlib/src/registry.rs");
    assert!(reg.contains("XmlParse") && reg.contains("__syx_xml_parse"));
}

#[test]
fn p98_xml_mvp_parse_stdout() {
    let out = run(
        r#"func main() {
    let doc = __syx_xml_parse("<root><item>88</item></root>")
    print(doc["root"]["item"])
}"#,
    );
    assert!(out.contains("88"), "{out}");
}

#[test]
fn p98_xml_mvp_fixture_exists() {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/data/xml_parse.sy");
    assert!(p.is_file());
}
