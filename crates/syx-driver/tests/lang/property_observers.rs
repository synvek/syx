//! Property observer oldValue/newValue + S056 tests .
// was: property_observers
//! Phase 134: inherited observers + init suppress.

use syx_driver::lang_props_v2::{
    COMPUTED_GET_ONLY, INIT_SUPPRESS_OBSERVERS, INHERITED_OBSERVERS, LAZY_VAR, STATIC_VAR,
};
use syx_driver::lang_props_v3::SUPER_INIT_FIELD_READBACK;
use syx_driver::{compile_and_run_embedded, run_file_printed, DriverOptions};
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
}

fn parity_fixture(name: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(format!("tests/fixtures/parity/{name}"))
}

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn run(src: &str) -> Vec<String> {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts()).unwrap().printed
}

fn has_error(src: &str, code: &str) -> bool {
    let file = syx_diagnostics::SourceFile::new("test.sy", src);
    let module = Parser::parse(file).unwrap();
    match SemanticAnalyzer::analyze(&module) {
        Err(d) => d.code.as_deref() == Some(code),
        Ok(a) => a.errors.iter().any(|e| e.code.as_deref() == Some(code)),
    }
}

#[test]
fn p134_computed_get_only_fingerprint() {
    let lib = include_str!("../../src/lang_props_v2.rs");
    assert!(lib.contains(COMPUTED_GET_ONLY));
    assert_eq!(COMPUTED_GET_ONLY, "lang-props-v2:computed-get-only-mvp");
}

#[test]
fn p134_static_var_mvp_fingerprint() {
    let lib = include_str!("../../src/lang_props_v2.rs");
    assert!(lib.contains(STATIC_VAR));
    assert_eq!(STATIC_VAR, "lang-props-v2:static-var-mvp");
}

#[test]
fn p134_lazy_var_mvp_fingerprint() {
    let lib = include_str!("../../src/lang_props_v2.rs");
    assert!(lib.contains(LAZY_VAR));
    assert_eq!(LAZY_VAR, "lang-props-v2:lazy-var-mvp");
}

#[test]
fn p134_computed_get_only_stdout() {
    let src = r#"
class Box {
    var base: Int
    init(base: Int) { self.base = base }
    var doubled: Int {
        get { return self.base * 2 }
    }
}
func main() {
    let b = Box(base: 21)
    print(b.doubled)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "42");
}

#[test]
fn p134_static_var_mvp_stdout() {
    let src = r#"
class Counter {
    static var n: Int = 0
    init() {}
}
func main() {
    Counter.n = 7
    print(Counter.n)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "7");
}

#[test]
fn p134_lazy_var_mvp_stdout() {
    let src = r#"
class Box {
    lazy var label: String = "lazy-ok"
    init() {}
}
func main() {
    let b = Box()
    print(b.label)
    print(b.label)
}
"#;
    let printed = run(src);
    let lines: Vec<&str> = printed.iter().map(|l| l.trim()).collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "lazy-ok");
    assert_eq!(lines[1], "lazy-ok");
}

#[test]
fn p134_inherited_observers_fingerprint() {
    let lib = include_str!("../../src/lang_props_v2.rs");
    assert!(lib.contains(INHERITED_OBSERVERS));
    assert_eq!(INHERITED_OBSERVERS, "lang-props-v2:inherited-observers");
}

#[test]
fn p134_init_suppress_observers_fingerprint() {
    let lib = include_str!("../../src/lang_props_v2.rs");
    assert!(lib.contains(INIT_SUPPRESS_OBSERVERS));
    assert_eq!(INIT_SUPPRESS_OBSERVERS, "lang-props-v2:init-suppress-observers");
}

#[test]
fn p135_super_init_field_readback_fingerprint() {
    let lib = include_str!("../../src/lang_props_v3.rs");
    assert!(lib.contains(SUPER_INIT_FIELD_READBACK));
    assert_eq!(SUPER_INIT_FIELD_READBACK, "lang-props-v3:super-init-field-readback");
}

#[test]
fn p134_inherited_observers_kvo_chain() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = parity_fixture("kvo_inheritance_chain.sy");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    let trimmed: Vec<&str> = lines.iter().map(|l| l.trim()).collect();
    // Init suppress (A3): only the post-init `d.name = "max"` fires observers once.
    // Inherited lookup (A2): Dog assignment hits Animal willSet/didSet.
    let will_n = trimmed.iter().filter(|l| **l == "animal-will").count();
    let did_n = trimmed.iter().filter(|l| **l == "animal-did").count();
    assert_eq!(will_n, 1, "expected one inherited willSet (init suppressed): {trimmed:?}");
    assert_eq!(did_n, 1, "expected one inherited didSet (init suppressed): {trimmed:?}");
    let value_lines: Vec<&str> = trimmed
        .iter()
        .copied()
        .filter(|l| *l != "animal-will" && *l != "animal-did")
        .collect();
    assert_eq!(
        value_lines,
        vec!["rex", "max"],
        "super.init field readback + assign: {trimmed:?}"
    );
}

#[test]
fn p135_super_init_field_readback_embedded() {
    let src = r#"
class Animal {
    var name: String
    init(name: String) { self.name = name }
}
class Dog: Animal {
    init(name: String) { super.init(name: name) }
}
func main() {
    let d = Dog(name: "rex")
    print(d.name)
    d.name = "max"
    print(d.name)
}
"#;
    let lines = run(src);
    let trimmed: Vec<&str> = lines.iter().map(|l| l.trim()).collect();
    assert_eq!(
        trimmed,
        vec!["rex", "max"],
        "super.init must initialize inherited stored fields: {trimmed:?}"
    );
}

#[test]
fn p134_inherited_observers_embedded_chain() {
    let src = r#"
class Animal {
    var name: String
        willSet { print("animal-will") }
        didSet { print("animal-did") }
    init(name: String) { self.name = name }
}
class Dog: Animal {
    init(name: String) { super.init(name: name) }
}
func main() {
    let d = Dog(name: "rex")
    print(d.name)
    d.name = "max"
    print(d.name)
}
"#;
    let lines = run(src);
    let trimmed: Vec<&str> = lines.iter().map(|l| l.trim()).collect();
    assert_eq!(
        trimmed.iter().filter(|l| **l == "animal-will").count(),
        1,
        "post-init assign should fire inherited willSet once: {trimmed:?}"
    );
    assert_eq!(
        trimmed.iter().filter(|l| **l == "animal-did").count(),
        1,
        "post-init assign should fire inherited didSet once: {trimmed:?}"
    );
    let value_lines: Vec<&str> = trimmed
        .iter()
        .copied()
        .filter(|l| *l != "animal-will" && *l != "animal-did")
        .collect();
    assert_eq!(
        value_lines,
        vec!["rex", "max"],
        "field readback after super.init and assign: {trimmed:?}"
    );
}

#[test]
fn p134_init_suppress_observers_method_assign_prints() {
    // External assign after init (same path as Phase 65 fixtures): init must not
    // fire observers; post-init assign fires exactly once.
    let src = r#"
class Box {
    var n: Int
        willSet { print("init-check-will") }
        didSet { print("init-check-did") }
    init(n: Int) { self.n = n }
}
func main() {
    let b = Box(n: 1)
    b.n = 99
}
"#;
    let lines = run(src);
    let trimmed: Vec<&str> = lines.iter().map(|l| l.trim()).collect();
    assert_eq!(
        trimmed.iter().filter(|l| **l == "init-check-will").count(),
        1,
        "only post-init assign should fire willSet once: {trimmed:?}"
    );
    assert_eq!(
        trimmed.iter().filter(|l| **l == "init-check-did").count(),
        1,
        "only post-init assign should fire didSet once: {trimmed:?}"
    );
}

#[test]
fn stored_property_assign_prints() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("types/property_observer.sy");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "will"));
    assert!(lines.iter().any(|l| l.trim() == "did"));
}

#[test]
fn willset_newvalue_binding() {
    let src = r#"
class Box {
    var n: Int
        willSet { print(newValue) }
    init(n: Int) { self.n = n }
}
func main() {
    let b = Box(n: 1)
    b.n = 42
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "42"));
}

#[test]
fn didset_oldvalue_binding() {
    let src = r#"
class Box {
    var n: Int
        didSet { print(oldValue) }
    init(n: Int) { self.n = n }
}
func main() {
    let b = Box(n: 10)
    b.n = 20
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "10"));
}

#[test]
fn willset_didset_old_and_new() {
    let src = r#"
class Box {
    var n: Int
        willSet { print(newValue) }
        didSet { print(oldValue) }
    init(n: Int) { self.n = n }
}
func main() {
    let b = Box(n: 1)
    b.n = 2
}
"#;
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "2"));
    assert!(lines.iter().any(|l| l.trim() == "1"));
}

#[test]
fn p135_main_mir_calls_computed_setter() {
    use syx_mir::{format_mir_module, MirOp};
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
    let b = Box(backing: 10)
    b.value = 20
}
"#;
    let unit = syx_driver::compile_file_from_source("embed.sy", src).unwrap();
    let mir = unit.program.mir_module.as_ref().expect("mir module");
    let text = format_mir_module(mir);
    assert!(
        text.contains("Call Box___set_value"),
        "main should call computed setter; mir:\n{text}"
    );
    let setter = mir
        .functions
        .iter()
        .find(|f| f.name == "Box___set_value")
        .expect("setter");
    assert_eq!(
        setter.param_count, 2,
        "setter expects self + newValue"
    );
    let main_fn = mir.functions.iter().find(|f| f.name == "main").expect("main");
    assert!(
        main_fn.blocks.iter().any(|b| {
            b.instructions
                .iter()
                .any(|i| matches!(&i.op, MirOp::Call(name) if name == "Box___set_value"))
        }),
        "main mir missing Box___set_value call: {text}"
    );
}

#[test]
fn p135_computed_get_set_fingerprint() {
    let lib = include_str!("../../src/lang_props_v3.rs");
    assert!(lib.contains("lang-props-v3:computed-get-set-mvp"));
}

#[test]
fn p135_property_override_fingerprint() {
    let lib = include_str!("../../src/lang_props_v3.rs");
    assert!(lib.contains("lang-props-v3:property-override-mvp"));
}

#[test]
fn p135_direct_backing_assign_works() {
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
    let b = Box(backing: 10)
    b.backing = 20
    print(b.value)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "20");
}

#[test]
fn p135_computed_get_set_stdout() {
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
    let b = Box(backing: 10)
    b.value = 20
    print(b.value)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "20");
}

#[test]
fn p135_property_override_base_ref() {
    let src = r#"
class Animal {
    var tag: String {
        get { return "animal" }
    }
    init() {}
}
class Dog: Animal {
    override var tag: String {
        get { return "dog" }
    }
    init() {}
}
func main() {
    let d = Dog()
    let a: Animal = d
    print(a.tag)
}
"#;
    assert_eq!(run(src).last().map(|l| l.trim()).unwrap_or(""), "dog");
}

#[test]
fn computed_property_observer_rejected_s056() {
    assert!(has_error(
        r#"
class C {
    let x: Int
        willSet { print("bad") }
    init(x: Int) { self.x = x }
}
func main() {}
"#,
        "S056"
    ));
}

#[test]
fn observer_fixture_still_runs() {
    let src = include_str!("../fixtures/types/property_observer.sy");
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "will"));
    assert!(lines.iter().any(|l| l.trim() == "did"));
}
