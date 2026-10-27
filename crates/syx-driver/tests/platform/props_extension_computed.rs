//! Extension computed `{ get }` / `{ get set }` MVP.

use syx_driver::lang_props_v4::{DOCS_HONESTY, EXTENSION_COMPUTED_MVP};
use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[test]
fn props_ext_v4_fingerprints() {
    let lib = include_str!("../../src/lang_props_v4.rs");
    assert!(lib.contains(EXTENSION_COMPUTED_MVP));
    assert!(lib.contains(DOCS_HONESTY));
    assert_eq!(
        EXTENSION_COMPUTED_MVP,
        "lang-props-v4:extension-computed-mvp"
    );
    assert_eq!(DOCS_HONESTY, "lang-props-v4:docs-honesty");
}

#[test]
fn props_ext_extension_computed_desugar_get_set() {
    let src = r#"
class Box {
    var backing: Int
    init(backing: Int) { self.backing = backing }
}
extension Box {
    var value: Int {
        get { return self.backing }
        set { self.backing = newValue }
    }
}
func main() {}
"#;
    let preview = syx_driver::macro_expand_preview(src).unwrap();
    assert!(
        preview.contains("__get_value"),
        "expected desugared getter in module: {preview}"
    );
    assert!(
        preview.contains("__set_value"),
        "expected desugared setter in module: {preview}"
    );
}

#[test]
fn props_ext_extension_computed_get_set_stdout() {
    let src = r#"
class Box {
    var backing: Int
    init(backing: Int) { self.backing = backing }
}
extension Box {
    var value: Int {
        get { return self.backing }
        set { self.backing = newValue }
    }
}
func main() {
    let b = Box(backing: 1)
    b.value = 5
    print(b.value)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "5");
}

#[test]
fn props_ext_extension_computed_get_only_stdout() {
    let src = r#"
class Box {
    var backing: Int
    init(backing: Int) { self.backing = backing }
}
extension Box {
    var doubled: Int {
        get { return self.backing * 2 }
    }
}
func main() {
    let b = Box(backing: 3)
    print(b.doubled)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "6");
}

#[test]
fn props_ext_extension_computed_base_instance_observable() {
    let src = r#"
class Counter {
    var n: Int
    init(n: Int) { self.n = n }
}
extension Counter {
    var label: String {
        get { return "n=" }
        set { self.n = 9 }
    }
}
func main() {
    let c = Counter(n: 1)
    c.label = "x"
    print(c.n)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "9");
}

#[test]
fn props_ext_v4_docs_honesty_marker() {
    let docs = include_str!("../../../../books/language-guide/11-properties.md");
    assert!(
        docs.contains("extension computed") || docs.contains("Extension computed"),
        "11-properties.md must document extension computed"
    );
    assert!(
        docs.contains("137+") || docs.contains("137 +"),
        "11-properties.md must defer full Swift properties to 137+"
    );
    assert!(docs.contains(DOCS_HONESTY) || docs.contains("lang-props-v4"));
}
