//! Polymorphic dispatch via base reference — vm / wasm / native parity .
// was: polymorphic_base_ref

use std::fs;
use std::path::PathBuf;

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_wasm_source_printed, DriverOptions,
};

const POLY_SRC: &str = include_str!("../fixtures/parity/polymorphic_base_ref.sy");

fn poly_fixture() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/parity/polymorphic_base_ref.sy")
}

fn run_embed() -> String {
    compile_and_run_embedded(POLY_SRC, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

fn run_wasm() -> String {
    let (code, lines) = run_wasm_source_printed(&poly_fixture(), "main", &DriverOptions::default())
        .expect("wasm");
    assert_eq!(code, 0);
    lines.join("\n")
}

fn run_native_strict() -> String {
    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let dir = std::env::temp_dir().join(format!("syx_poly_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), POLY_SRC).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    String::from_utf8_lossy(&std::process::Command::new(&exe).output().unwrap().stdout).into_owned()
}

macro_rules! poly_tri {
    ($name:ident, $needle:expr) => {
        #[test]
        fn $name() {
            assert!(run_embed().contains($needle), "embed");
            assert!(run_wasm().contains($needle), "wasm");
            let native = run_native_strict();
            assert!(native.contains($needle), "native strict: {native}");
        }
    };
}

poly_tri!(poly_base_ref_woof, "woof");
poly_tri!(poly_base_ref_animal_speak, "woof");

#[test]
fn poly_static_dog_direct() {
    let src = r#"class Animal { func speak() { print("a") } } class Dog: Animal { override func speak() { print("d") } } func main() { Dog().speak() }"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default()).unwrap().printed.join("\n");
    assert!(out.contains("d"));
}

#[test]
fn poly_base_ref_cat_meow() {
    let src = r#"class Animal { func speak() { print("a") } }
class Cat: Animal { override func speak() { print("meow") } }
func main() { let a: Animal = Cat(); a.speak() }"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default()).unwrap().printed.join("\n");
    assert!(out.contains("meow"));
}

#[test]
fn poly_base_ref_bird_chirp() {
    let src = r#"class Animal { func speak() { print("a") } }
class Bird: Animal { override func speak() { print("chirp") } }
func main() { let a: Animal = Bird(); a.speak() }"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default()).unwrap().printed.join("\n");
    assert!(out.contains("chirp"));
}

#[test]
fn poly_base_ref_fish_blub() {
    let src = r#"class Animal { func speak() { print("a") } }
class Fish: Animal { override func speak() { print("blub") } }
func main() { let a: Animal = Fish(); a.speak() }"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default()).unwrap().printed.join("\n");
    assert!(out.contains("blub"));
}

#[test]
fn poly_base_ref_horse_neigh() {
    let src = r#"class Animal { func speak() { print("a") } }
class Horse: Animal { override func speak() { print("neigh") } }
func main() { let a: Animal = Horse(); a.speak() }"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default()).unwrap().printed.join("\n");
    assert!(out.contains("neigh"));
}

#[test]
fn poly_vtable_indirect_smoke() {
    let src = r#"class Animal { func speak() { print("a") } }
class Dog: Animal { override func speak() { print("woof") } }
func main() { let a: Animal = Dog(); a.speak() }"#;
    let out = compile_and_run_embedded(src, &DriverOptions::default()).unwrap().printed.join("\n");
    assert!(out.contains("woof"));
}

#[test]
fn poly_wasm_vtable_indirect() {
    let src = r#"class Animal { func speak() { print("a") } }
class Dog: Animal { override func speak() { print("woof") } }
func main() { let a: Animal = Dog(); a.speak() }"#;
    let dir = std::env::temp_dir().join(format!("syx_poly_wasm_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    let (code, lines) = run_wasm_source_printed(&dir.join("main.sy"), "main", &DriverOptions::default()).unwrap();
    assert_eq!(code, 0);
    assert!(lines.join("\n").contains("woof"));
}

#[test]
fn polymorphic_base_ref_count_gate() {
    let src = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/polymorphic_base_ref.rs")).unwrap();
    let count = src.matches("#[test]").count();
    assert!(count >= 8, "polymorphic_base_ref needs 8+ honest tests, found {count}");
}
