//! Computed `{ get set }` + property override MVP.

use syx_driver::lang_props_v3::{COMPUTED_GET_SET_MVP, DOCS_HONESTY, PROPERTY_OVERRIDE_MVP};
use syx_driver::{compile_and_run_embedded, DriverOptions};

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
}

#[test]
fn props_override_desugar_emits_setter_method() {
    let src = r#"
class Box {
    var backing: Int
    init(backing: Int) { self.backing = backing }
    var value: Int {
        get { return self.backing }
        set { self.backing = newValue }
    }
}
func main() {}
"#;
    let preview = syx_driver::macro_expand_preview(src).unwrap();
    assert!(
        preview.contains("__set_value"),
        "expected desugared setter in module: {preview}"
    );
}

#[test]
fn props_override_v3_fingerprints_in_driver() {
    let lib = include_str!("../../src/lang_props_v3.rs");
    assert!(lib.contains(COMPUTED_GET_SET_MVP));
    assert!(lib.contains(PROPERTY_OVERRIDE_MVP));
    assert!(lib.contains(DOCS_HONESTY));
    assert_eq!(COMPUTED_GET_SET_MVP, "lang-props-v3:computed-get-set-mvp");
    assert_eq!(PROPERTY_OVERRIDE_MVP, "lang-props-v3:property-override-mvp");
}

#[test]
fn props_override_computed_get_set_stdout() {
    let src = r#"
class Box {
    var backing: Int
    init(backing: Int) { self.backing = backing }
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
fn props_override_computed_get_set_newvalue_binding() {
    let src = r#"
class Box {
    var backing: Int
    init(backing: Int) { self.backing = backing }
    var scaled: Int {
        get { return self.backing }
        set { self.backing = newValue * 2 }
    }
}
func main() {
    let b = Box(backing: 0)
    b.scaled = 3
    print(b.scaled)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "6");
}

#[test]
fn props_override_property_override_get_polymorphic() {
    let src = r#"
class Base {
    var n: Int
    init(n: Int) { self.n = n }
    var label: String {
        get { return "base" }
    }
}
class Derived: Base {
    init(n: Int) { self.n = n }
    override var label: String {
        get { return "derived" }
    }
}
func main() {
    let d = Derived(n: 1)
    let b: Base = d
    print(b.label)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "derived");
}

#[test]
fn props_override_property_override_get_set_polymorphic() {
    let src = r#"
class Base {
    var backing: Int
    init(backing: Int) { self.backing = backing }
    var value: Int {
        get { return self.backing }
        set { self.backing = newValue }
    }
}
class Derived: Base {
    init(backing: Int) { self.backing = backing }
    override var value: Int {
        get { return self.backing + 100 }
        set { self.backing = newValue - 100 }
    }
}
func main() {
    let d = Derived(backing: 1)
    let b: Base = d
    b.value = 150
    print(b.value)
}
"#;
    // Derived setter: backing = 150 - 100 = 50; derived getter: 50 + 100 = 150
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "150");
}
