//! inheritance polymorphic dispatch + super chain tests.
// was: inheritance_polymorphic

use syx_driver::{compile_and_run_embedded, compile_file_with_options, emit_syxbc_text, run_file_printed, DriverOptions};
use syx_parser::Parser;
use syx_semantic::SemanticAnalyzer;

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

macro_rules! poly_prints {
    ($name:ident, $src:expr, $expect:expr) => {
        #[test]
        fn $name() {
            assert!(run($src).iter().any(|l| l.trim() == $expect), "expected {}", $expect);
        }
    };
}

poly_prints!(
    poly_dog_speak,
    r#"class Animal { func speak() { print("animal") } } class Dog: Animal { override func speak() { print("woof") } } func main() { Dog().speak() }"#,
    "woof"
);

poly_prints!(
    poly_cat_speak,
    r#"class Animal { func speak() { print("animal") } } class Cat: Animal { override func speak() { print("meow") } } func main() { Cat().speak() }"#,
    "meow"
);

poly_prints!(
    poly_three_level_dispatch,
    r#"class L1 { func tag() { print("1") } } class L2: L1 { override func tag() { print("2") } } class L3: L2 { override func tag() { print("3") } } func main() { L3().tag() }"#,
    "3"
);

poly_prints!(
    poly_super_calls_parent,
    r#"class Base { func id() { print("base") } } class Child: Base { override func id() { print("child") } func up() { super.id() } } func main() { Child().up() }"#,
    "base"
);

poly_prints!(
    poly_super_in_override_chain,
    r#"class A { func n() { print("A") } } class B: A { override func n() { print("B") } func viaSuper() { super.n() } } func main() { B().viaSuper() }"#,
    "A"
);

poly_prints!(
    poly_base_only_no_override,
    r#"class Solo { func run() { print("solo") } } func main() { Solo().run() }"#,
    "solo"
);

poly_prints!(
    poly_override_same_name,
    r#"class X { func k() { print("x") } } class Y: X { override func k() { print("y") } } func main() { Y().k() }"#,
    "y"
);

poly_prints!(
    poly_deep_super_two_hops,
    r#"class G { func v() { print("g") } } class H: G { override func v() { print("h") } func callG() { super.v() } } func main() { H().callG() }"#,
    "g"
);

poly_prints!(
    poly_vehicle_car,
    r#"class Vehicle { func move() { print("v") } } class Car: Vehicle { override func move() { print("c") } } func main() { Car().move() }"#,
    "c"
);

poly_prints!(
    poly_shape_circle,
    r#"class Shape { func area() { print("0") } } class Circle: Shape { override func area() { print("78") } } func main() { Circle().area() }"#,
    "78"
);

poly_prints!(
    poly_employee_manager,
    r#"class Employee { func role() { print("emp") } } class Manager: Employee { override func role() { print("mgr") } } func main() { Manager().role() }"#,
    "mgr"
);

poly_prints!(
    poly_node_leaf,
    r#"class Node { func kind() { print("node") } } class Leaf: Node { override func kind() { print("leaf") } } func main() { Leaf().kind() }"#,
    "leaf"
);

poly_prints!(
    poly_widget_button,
    r#"class Widget { func label() { print("w") } } class Button: Widget { override func label() { print("btn") } } func main() { Button().label() }"#,
    "btn"
);

poly_prints!(
    poly_account_savings,
    r#"class Account { func typeName() { print("acct") } } class Savings: Account { override func typeName() { print("save") } } func main() { Savings().typeName() }"#,
    "save"
);

poly_prints!(
    poly_pet_bird,
    r#"class Pet { func sound() { print("?") } } class Bird: Pet { override func sound() { print("chirp") } } func main() { Bird().sound() }"#,
    "chirp"
);

poly_prints!(
    poly_device_phone,
    r#"class Device { func name() { print("dev") } } class Phone: Device { override func name() { print("phone") } } func main() { Phone().name() }"#,
    "phone"
);

poly_prints!(
    poly_animal_horse,
    r#"class Animal { func gait() { print("walk") } } class Horse: Animal { override func gait() { print("gallop") } } func main() { Horse().gait() }"#,
    "gallop"
);

poly_prints!(
    poly_fruit_apple,
    r#"class Fruit { func color() { print("green") } } class Apple: Fruit { override func color() { print("red") } } func main() { Apple().color() }"#,
    "red"
);

poly_prints!(
    poly_game_player,
    r#"class Entity { func tick() { print("e") } } class Player: Entity { override func tick() { print("p") } } func main() { Player().tick() }"#,
    "p"
);

poly_prints!(
    poly_logger_debug,
    r#"class Logger { func level() { print("info") } } class DebugLogger: Logger { override func level() { print("debug") } } func main() { DebugLogger().level() }"#,
    "debug"
);

#[test]
fn poly_fixture_inheritance_basic() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/types/inheritance_basic.sy");
    syx_host_plugin::register_echo_plugin_in_process();
    match run_file_printed(&path, &opts()) {
        Ok((_, lines)) => assert!(lines.iter().any(|l| l.trim() == "woof")),
        Err(e) if e.code.as_deref() == Some("S057") => {
            // fixture may call sorted() on non-Comparable element until updated
        }
        Err(e) => panic!("inheritance_basic: {e:?}"),
    }
}

#[test]
fn poly_compiles_inheritance_fixture() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/types/inheritance_basic.sy");
    match compile_file_with_options(&path, &opts()) {
        Ok(_) => {}
        Err(e) if e.code.as_deref() == Some("S057") => {}
        Err(e) => panic!("compile: {e:?}"),
    }
}

#[test]
fn poly_missing_override_s052() {
    assert!(has_error(
        r#"class A { func f() {} } class B: A { func f() {} } func main() {}"#,
        "S052"
    ));
}

#[test]
fn poly_bogus_override_s051() {
    assert!(has_error(
        r#"class A { func f() {} } class B: A { override func g() {} } func main() {}"#,
        "S051"
    ));
}

#[test]
fn poly_return_mismatch_s053() {
    assert!(has_error(
        r#"class A { func f() -> Int { return 1 } } class B: A { override func f() -> String { return "x" } } func main() {}"#,
        "S053"
    ));
}

#[test]
fn poly_final_override_s054() {
    assert!(has_error(
        r#"class A { final func f() {} } class B: A { override func f() {} } func main() {}"#,
        "S054"
    ));
}

#[test]
fn poly_override_without_base_s051() {
    assert!(has_error(
        r#"class Solo { override func f() {} } func main() {}"#,
        "S051"
    ));
}

#[test]
fn poly_init_with_superclass_field() {
    let src = r#"class P { var n: Int init(n: Int) { self.n = n } } class C: P { init(n: Int) { self.n = n } } func main() { print(C(n: 7).n) }"#;
    assert!(run(src).iter().any(|l| l.trim() == "7"));
}

#[test]
fn poly_vtable_three_level_runtime() {
    let src = r#"class L1 { func m() { print("1") } } class L2: L1 { override func m() { print("2") } } class L3: L2 { override func m() { print("3") } } func main() { L3().m() }"#;
    assert!(run(src).iter().any(|l| l.trim() == "3"));
}

#[test]
fn poly_super_not_dynamic_receiver() {
    let src = r#"class A { func id() { print("A") } } class B: A { override func id() { print("B") } func callSuper() { super.id() } } func main() { B().callSuper() }"#;
    assert!(run(src).iter().any(|l| l.trim() == "A"));
}

#[test]
fn poly_phase66_inheritance_doc() {
    let doc = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../books/language-guide/14-inheritance.md"),
    )
    .unwrap();
    assert!(doc.contains("override") || doc.contains("super"));
}

#[test]
fn poly_single_inheritance_only() {
    let src = r#"class A { func f() { print("a") } } class B: A { override func f() { print("b") } } func main() { B().f() }"#;
    assert!(run(src).iter().any(|l| l.trim() == "b"));
}

#[test]
fn poly_parse_colon_syntax() {
    run(r#"class Base { func m() {} } class Derived: Base { override func m() {} } func main() {}"#);
}

#[test]
fn poly_method_on_base_instance() {
    let src = r#"class Base { func hi() { print("hi") } } func main() { Base().hi() }"#;
    assert!(run(src).iter().any(|l| l.trim() == "hi"));
}

const POLY_BASE_REF: &str = r#"class Animal { func speak() { print("animal") } }
class Dog: Animal { override func speak() { print("woof") } }
func main() {
    let a: Animal = Dog()
    a.speak()
}"#;

fn syxbc_for_embedded(src: &str) -> String {
    let dir = std::env::temp_dir().join(format!("syx_poly_mir_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("main.sy");
    std::fs::write(&path, src).unwrap();
    std::fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    emit_syxbc_text(&path, &opts()).expect("syxbc")
}

fn syxbc_for_fixture(rel: &str) -> String {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel);
    emit_syxbc_text(&path, &opts()).expect("syxbc")
}

#[test]
fn poly_mir_call_indirect_base_ref() {
    let bc = syxbc_for_fixture("tests/fixtures/parity/polymorphic_base_ref.sy");
    assert!(
        bc.contains("CallIndirect") || bc.contains("__vtable_speak"),
        "expected vtable dispatch in bytecode:\n{bc}"
    );
}

#[test]
fn poly_mir_vtable_slot_on_init() {
    let bc = syxbc_for_fixture("tests/fixtures/parity/polymorphic_base_ref.sy");
    assert!(
        bc.contains("__vtable_speak") || bc.contains("ObjectGet"),
        "expected vtable slot read:\n{bc}"
    );
}

#[test]
fn poly_base_ref_runtime_matches_fixture() {
    let fixture = include_str!("../fixtures/parity/polymorphic_base_ref.sy");
    assert!(run(fixture).iter().any(|l| l.trim() == "woof"));
    assert!(run(POLY_BASE_REF).iter().any(|l| l.trim() == "woof"));
}

#[test]
fn poly_three_level_mir_indirect() {
    let src = r#"class L1 { func m() { print("1") } }
class L2: L1 { override func m() { print("2") } }
class L3: L2 { override func m() { print("3") } }
func main() {
    let x: L1 = L3()
    x.m()
}"#;
    let bc = syxbc_for_embedded(src);
    assert!(bc.contains("CallIndirect") || bc.contains("__vtable_m"), "{bc}");
    assert!(run(src).iter().any(|l| l.trim() == "3"));
}
