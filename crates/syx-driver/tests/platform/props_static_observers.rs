//! Class `static var` willSet/didSet MVP.

use syx_driver::lang_props_v6::{DOCS_HONESTY, STATIC_OBSERVERS_MVP};
use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[test]
fn props_static_v6_fingerprints() {
    let lib = include_str!("../../src/lang_props_v6.rs");
    assert!(lib.contains(STATIC_OBSERVERS_MVP));
    assert!(lib.contains(DOCS_HONESTY));
    assert_eq!(STATIC_OBSERVERS_MVP, "lang-props-v6:static-observers-mvp");
    assert_eq!(DOCS_HONESTY, "lang-props-v6:docs-honesty");
    let driver = include_str!("../../src/lib.rs");
    assert!(driver.contains("lang-props-v6:static-observers-mvp"));
    assert!(driver.contains("syx-cache-v110"));
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
