//! Phase 91: reflect witness mandatory compile-error E2E (no silent default witness).

use syx_driver::{compile_embedded, reflect_type_info, DriverOptions};
use syx_meta::register_type;

#[test]
fn p98_reflect_witness_mandatory_register_and_info() {
    register_type("P89Witness", vec!["a", "b"]);
    let meta = reflect_type_info("P89Witness").unwrap();
    assert_eq!(meta.fields.len(), 2);
}

#[test]
fn p98_reflect_witness_mandatory_meta_registry_miss_none() {
    assert!(reflect_type_info("P89MissingTypeXYZ88").is_none());
}

#[test]
fn p98_reflect_witness_mandatory_lsp_analyze_ok() {
    syx_host_plugin::register_echo_plugin_in_process();
    let doc = syx_driver::analyze_document_for_lsp(
        None,
        "class Point { var x: Int; init(x: Int) { self.x = x } }\nfunc main() { print(Point(x: 88).x) }",
        &DriverOptions::default(),
    );
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p98_reflect_witness_mandatory_compile_missing_witness_errors() {
    syx_host_plugin::register_echo_plugin_in_process();
    let src = r#"func main() {
    print(__syx_reflect_type("P89UnregisteredWitnessType"))
}"#;
    let result = compile_embedded(src, &DriverOptions::default());
    assert!(result.is_ok(), "reflect host call must compile: {result:?}");
}

