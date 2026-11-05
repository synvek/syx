//! Class `static var` willSet/didSet MVP.

use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[test]
fn props_static_observers_fire_after_init() {
    let src = r#"
class Counter {
    static var value: Int = 0
        willSet { print("will:" + newValue) }
        didSet { print("did:" + oldValue) }
    init() {}
}
func main() {
    Counter.value = 7
    print(Counter.value)
}
"#;
    let printed = run(src);
    assert!(
        printed.iter().any(|l| l.contains("will:7")),
        "expected willSet with newValue, got {printed:?}"
    );
    assert!(
        printed.iter().any(|l| l.contains("did:0")),
        "expected didSet with oldValue 0, got {printed:?}"
    );
    assert_eq!(printed.last().map(|l| l.trim()).unwrap_or(""), "7");
}

#[test]
fn props_static_observers_docs_honesty_marker() {
    let docs = include_str!("../../../../books/language-guide/11-properties.md");
    assert!(docs.contains("static") && docs.contains("willSet"));
    assert!(
        docs.contains("139+") || docs.contains("完整 Swift"),
        "docs must keep full Swift property model → 139+ honesty"
    );
}
