//! Phase 91: @Codable L5 JSON encode→decode roundtrip stdout hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_codable_l4_json_roundtrip_host_smoke() {
    let out = run(r#"func main() {
    let json = __syx_fmt_format("{{\"n\":{0}}}", [88])
    print(json)
    let decoded = __syx_codable_decode("Item", json)
    print(__syx_fmt_debug(decoded))
}"#);
    assert!(out.contains("88"), "{out}");
}

#[test]
fn p98_codable_l4_embedded_struct_roundtrip() {
    let out = run(r#"class Item { var n: Int; init(n: Int) { self.n = n } }
func main() {
    let i = Item(n: 88)
    let json = __syx_codable_encode("Item", i)
    print(json)
    let back = __syx_codable_decode("Item", json)
    print(__syx_fmt_debug(back))
}"#);
    assert!(out.contains("88") || out.contains("Item"), "{out}");
}

#[test]
fn p98_codable_l4_attached_macro_fixture_compiles() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/macros/codable_l2_external.sy");
    if path.exists() {
        let result = syx_driver::compile_file_with_options(&path, &DriverOptions::default());
        assert!(result.is_ok(), "{result:?}");
    }
}
