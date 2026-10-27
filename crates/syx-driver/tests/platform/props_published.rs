//! `@Published` class stored property MVP.

use syx_driver::lang_props_v5::{DOCS_HONESTY, PUBLISHED_MVP};
use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[test]
fn props_published_v5_fingerprints() {
    let lib = include_str!("../../src/lang_props_v5.rs");
    assert!(lib.contains(PUBLISHED_MVP));
    assert!(lib.contains(DOCS_HONESTY));
    assert_eq!(PUBLISHED_MVP, "lang-props-v5:published-mvp");
    assert_eq!(DOCS_HONESTY, "lang-props-v5:docs-honesty");
    let driver = include_str!("../../src/lib.rs");
    assert!(driver.contains("lang-props-v5:published-mvp"));
    assert!(driver.contains("syx-cache-v109"));
}

#[test]
fn props_published_notify_on_assign() {
    let src = r#"
class Counter {
    @Published var value: Int
    init(start: Int) { self.value = start }
}
func main() {
    var c = Counter(start: 0)
    c.value = 3
    print(c.value)
}
"#;
    let printed = run(src);
    assert!(
        printed.iter().any(|l| l.contains("published:value")),
        "expected published notify, got {printed:?}"
    );
    assert_eq!(printed.last().map(|l| l.trim()).unwrap_or(""), "3");
}

#[test]
fn props_published_docs_honesty_marker() {
    let docs = include_str!("../../../../books/language-guide/11-properties.md");
    assert!(docs.contains("@Published") || docs.contains("Published"));
    assert!(
        docs.contains("138+") || docs.contains("完整 Swift"),
        "docs must keep full Swift property model → 138+ honesty"
    );
}
