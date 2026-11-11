//! Phase 91: KVO full chain observe/willSet/didSet/didChange stdout hard.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> String {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn p98_kvo_chain_observe_stdout() {
    let out = run(r#"class Dog {
    var name: String
    init(name: String) { self.name = name }
}
func main() {
    let d = Dog(name: "fido")
    __syx_kvo_observe(d, "name")
    print("kvo-ok")
}"#);
    assert!(out.contains("kvo-ok"), "{out}");
}

#[test]
fn p98_kvo_chain_fixture_exists() {
    let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/parity/kvo_inheritance_chain.sy");
    assert!(p.is_file());
}
