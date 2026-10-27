//! Inheritance vtable + super + polymorphic dispatch tests .
// was: inheritance

use syx_driver::{compile_and_run_embedded, compile_file_with_options, run_file_printed, DriverOptions};
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

fn fixture(rel: &str) -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/{rel}"))
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

macro_rules! embed_inherit {
    ($name:ident, $src:expr, $expect:expr) => {
        #[test]
        fn $name() {
            assert!(run($src).iter().any(|l| l.trim() == $expect));
        }
    };
}

embed_inherit!(override_prints, r#"class B { func f() { print("b") } } class D: B { override func f() { print("d") } } func main() { D().f() }"#, "d");
embed_inherit!(super_chain, r#"class A { func tag() { print("a") } } class B: A { override func tag() { print("b") } } func main() { B().tag() }"#, "b");
embed_inherit!(base_only, r#"class Base { func id() { print("base") } } func main() { Base().id() }"#, "base");
embed_inherit!(deep_override, r#"class A { func n() { print("1") } } class B: A { override func n() { print("2") } } class C: B { override func n() { print("3") } } func main() { C().n() }"#, "3");
embed_inherit!(animal_dog, r#"class Animal { func s() { print("a") } } class Dog: Animal { override func s() { print("d") } } func main() { Dog().s() }"#, "d");
embed_inherit!(two_level, r#"class Root { func v() { print("r") } } class Leaf: Root { override func v() { print("l") } } func main() { Leaf().v() }"#, "l");
embed_inherit!(same_name, r#"class X { func k() { print("x") } } class Y: X { override func k() { print("y") } } func main() { Y().k() }"#, "y");
embed_inherit!(triple, r#"class T1 { func p() { print("1") } } class T2: T1 { override func p() { print("2") } } func main() { T2().p() }"#, "2");

#[test]
fn super_call_invokes_parent() {
    let src = r#"
class Base { func greet() { print("base") } }
class Child: Base {
    override func greet() { print("child") }
    func callParent() { super.greet() }
}
func main() {
    Child().callParent()
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "base"));
}

#[test]
fn super_call_with_override_chain() {
    let src = r#"
class A { func id() { print("A") } }
class B: A {
    override func id() { print("B") }
    func viaSuper() { super.id() }
}
func main() { B().viaSuper() }
"#;
    assert!(run(src).iter().any(|l| l.trim() == "A"));
}

#[test]
fn polymorphic_animal_dog_via_base_ref() {
    let src = r#"
class Animal { func s() { print("a") } }
class Dog: Animal { override func s() { print("d") } }
func via(base: Animal) { base.s() }
func main() { via(Dog()) }
"#;
    assert!(run(src).iter().any(|l| l.trim() == "d"));
}

#[test]
fn polymorphic_three_level_dispatch() {
    let src = r#"
class A { func m() { print("A") } }
class B: A { override func m() { print("B") } }
class C: B { override func m() { print("C") } }
func main() {
    let c = C()
    let a: A = c
    a.m()
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "C"));
}

#[test]
fn subclass_method_dispatch_fixture() {
    syx_host_plugin::register_echo_plugin_in_process();
    let path = fixture("types/inheritance_basic.sy");
    let (_, lines) = run_file_printed(&path, &opts()).unwrap();
    assert!(lines.iter().any(|l| l.trim() == "woof"));
}

#[test]
fn compiles_inheritance_fixture() {
    compile_file_with_options(&fixture("types/inheritance_basic.sy"), &opts()).expect("compile");
}

#[test]
fn missing_override_s052() {
    assert!(has_error(
        r#"class A { func f() {} } class B: A { func f() {} } func main() {}"#,
        "S052"
    ));
}

#[test]
fn bogus_override_s051() {
    assert!(has_error(
        r#"class A { func f() {} } class B: A { override func g() {} } func main() {}"#,
        "S051"
    ));
}

#[test]
fn override_return_mismatch_s053() {
    assert!(has_error(
        r#"class A { func f() -> Int { return 1 } } class B: A { override func f() -> String { return "x" } } func main() {}"#,
        "S053"
    ));
}

#[test]
fn override_final_method_s054() {
    assert!(has_error(
        r#"class A { final func f() {} } class B: A { override func f() {} } func main() {}"#,
        "S054"
    ));
}

#[test]
fn override_without_superclass_s051() {
    assert!(has_error(
        r#"class Solo { override func f() {} } func main() {}"#,
        "S051"
    ));
}

#[test]
fn parse_colon_base_syntax() {
    run(r#"class Base { func m() {} } class Derived: Base { override func m() {} } func main() {}"#);
}

#[test]
fn inheritance_compiles_with_superclass() {
    run(r#"class P { var n: Int init(n: Int) { self.n = n } } class C: P { init(n: Int) { self.n = n } } func main() { print(C(n: 1).n) }"#);
}

#[test]
fn gate_inheritance_fixture_path() {
    assert!(fixture("types/inheritance_basic.sy").exists());
}

#[test]
fn vtable_smoke_three_level() {
    let src = r#"
class L1 { func m() { print("1") } }
class L2: L1 { override func m() { print("2") } }
class L3: L2 { override func m() { print("3") } }
func main() { L3().m() }
"#;
    assert!(run(src).iter().any(|l| l.trim() == "3"));
}

#[test]
fn polymorphic_mid_chain_base_ref() {
    let src = r#"
class A { func k() { print("a") } }
class B: A { override func k() { print("b") } }
func main() {
    let b = B()
    let ref: A = b
    ref.k()
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "b"));
}

#[test]
fn super_static_not_polymorphic() {
    let src = r#"
class A { func id() { print("A") } }
class B: A {
    override func id() { print("B") }
    func up() { super.id() }
}
func main() { B().up() }
"#;
    assert!(run(src).iter().any(|l| l.trim() == "A"));
}

#[test]
fn dog_animal_polymorphic_prints_d() {
    let src = r#"
class Animal { func speak() { print("animal") } }
class Dog: Animal { override func speak() { print("dog") } }
func main() {
    let pet: Animal = Dog()
    pet.speak()
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "dog"));
}

#[test]
fn base_class_direct_call_unchanged() {
    let src = r#"
class Animal { func speak() { print("animal") } }
class Dog: Animal { override func speak() { print("dog") } }
func main() { Animal().speak() }
"#;
    assert!(run(src).iter().any(|l| l.trim() == "animal"));
}

#[test]
fn subclass_assignable_to_superclass() {
    let src = r#"
class Base { var n: Int init(n: Int) { self.n = n } }
class Derived: Base { init(n: Int) { self.n = n } }
func main() {
    let d = Derived(n: 7)
    let b: Base = d
    print(b.n)
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "7"));
}

macro_rules! inherit_expect {
    ($name:ident, $body:expr, $tag:expr) => {
        #[test]
        fn $name() {
            let src = format!(
                "class A {{ func t() {{ print(\"{}\") }} }} class B: A {{ override func t() {{ print(\"{}\") }} }} func main() {{ B().t() }}",
                $body, $tag
            );
            assert!(run(&src).iter().any(|l| l.trim() == $tag));
        }
    };
}

inherit_expect!(vtable_slot_a, "a", "b");
inherit_expect!(vtable_slot_x, "x", "y");

#[test]
fn four_level_override_chain() {
    let src = r#"
class L0 { func v() { print("0") } }
class L1: L0 { override func v() { print("1") } }
class L2: L1 { override func v() { print("2") } }
class L3: L2 { override func v() { print("3") } }
func main() {
    let x: L0 = L3()
    x.v()
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "3"));
}

#[test]
fn override_without_super_still_vtable() {
    let src = r#"
class P { func m() { print("p") } }
class Q: P { override func m() { print("q") } }
func main() {
    let q = Q()
    let p: P = q
    p.m()
}
"#;
    assert!(run(src).iter().any(|l| l.trim() == "q"));
}

#[test]
fn cache_v38_in_driver_for_inheritance_gate() {
    let src = include_str!("../../src/lib.rs");
    assert!(src.contains("syx-cache-v38"));
    assert!(src.contains("vtable_layout_fingerprint"));
}

#[test]
fn manifest_phase66_inheritance_gate() {
    let m: serde_json::Value =
        serde_json::from_str(syx_driver::embedded_std::internal_manifest_json()).unwrap();
    assert_eq!(m["phase"], 66);
}

#[test]
fn multiple_methods_polymorphic() {
    let src = r#"
class A {
    func one() { print("a1") }
    func two() { print("a2") }
}
class B: A {
    override func one() { print("b1") }
    override func two() { print("b2") }
}
func main() {
    let b: A = B()
    b.one()
    b.two()
}
"#;
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "b1"));
    assert!(lines.iter().any(|l| l.trim() == "b2"));
}

#[test]
fn nested_super_call_chain() {
    let src = r#"
class G { func n() { print("g") } }
class P: G { override func n() { print("p") } func up() { super.n() } }
func main() { P().up() }
"#;
    assert!(run(src).iter().any(|l| l.trim() == "g"));
}

#[test]
fn compile_empty_override_chain_base() {
    run(r#"class Solo { func m() { print("solo") } } func main() { Solo().m() }"#);
}

#[test]
fn polymorphic_with_field_access() {
    let src = r#"
class Animal {
    var tag: String
    init(tag: String) { self.tag = tag }
    func label() { print(self.tag) }
}
class Dog: Animal {
    init(tag: String) { self.tag = tag }
    override func label() { print("dog") }
}
func via(pet: Animal) { pet.label() }
func main() { via(Dog(tag: "x")) }
"#;
    assert!(run(src).iter().any(|l| l.trim() == "dog"));
}

#[test]
fn two_subclasses_same_base() {
    let src = r#"
class Base { func id() { print("b") } }
class One: Base { override func id() { print("1") } }
class Two: Base { override func id() { print("2") } }
func main() {
    let a: Base = One()
    let b: Base = Two()
    a.id()
    b.id()
}
"#;
    let lines = run(src);
    assert!(lines.iter().any(|l| l.trim() == "1"));
    assert!(lines.iter().any(|l| l.trim() == "2"));
}
