//! Polymorphic dispatch via erased base reference — vm / wasm parity (R-43).
//! Native-strict is optional (L006 without `--features full-native`); not an exit gate.

use std::fs;
use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, run_wasm_source_printed, DriverOptions};

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
    let (code, lines) =
        run_wasm_source_printed(&poly_fixture(), "main", &DriverOptions::default()).expect("wasm");
    assert_eq!(code, 0);
    lines.join("\n")
}

fn run_embed_src(src: &str) -> String {
    compile_and_run_embedded(src, &DriverOptions::default())
        .unwrap()
        .printed
        .join("\n")
}

#[test]
fn poly_erased_fixture_embed_woof() {
    assert!(
        run_embed().contains("woof"),
        "embed erased: {}",
        run_embed()
    );
}

#[test]
fn poly_erased_fixture_wasm_woof() {
    assert!(run_wasm().contains("woof"), "wasm erased: {}", run_wasm());
}

#[test]
fn poly_static_dog_direct() {
    let src = r#"
class Animal { func speak() { print("a") } }
class Dog: Animal { override func speak() { print("d") } }
func main() { Dog().speak() }
"#;
    let out = run_embed_src(src);
    assert!(out.contains("d"), "{out}");
}

#[test]
fn poly_base_ref_cat_meow() {
    let src = r#"
class Animal { func speak() { print("a") } }
class Cat: Animal { override func speak() { print("meow") } }
func make() -> Animal { return Cat() }
func main() {
    let a: Animal = make()
    a.speak()
}
"#;
    let out = run_embed_src(src);
    assert!(out.contains("meow"), "{out}");
}

#[test]
fn poly_base_ref_bird_chirp() {
    let src = r#"
class Animal { func speak() { print("a") } }
class Bird: Animal { override func speak() { print("chirp") } }
func make() -> Animal { return Bird() }
func main() {
    let a: Animal = make()
    a.speak()
}
"#;
    let out = run_embed_src(src);
    assert!(out.contains("chirp"), "{out}");
}

#[test]
fn poly_base_ref_fish_blub() {
    let src = r#"
class Animal { func speak() { print("a") } }
class Fish: Animal { override func speak() { print("blub") } }
func make() -> Animal { return Fish() }
func main() {
    let a: Animal = make()
    a.speak()
}
"#;
    let out = run_embed_src(src);
    assert!(out.contains("blub"), "{out}");
}

#[test]
fn poly_base_ref_horse_neigh() {
    let src = r#"
class Animal { func speak() { print("a") } }
class Horse: Animal { override func speak() { print("neigh") } }
func make() -> Animal { return Horse() }
func main() {
    let a: Animal = make()
    a.speak()
}
"#;
    let out = run_embed_src(src);
    assert!(out.contains("neigh"), "{out}");
}

#[test]
fn poly_vtable_indirect_smoke() {
    let src = r#"
class Animal { func speak() { print("a") } }
class Dog: Animal { override func speak() { print("woof") } }
func make() -> Animal { return Dog() }
func main() {
    let a: Animal = make()
    a.speak()
}
"#;
    let out = run_embed_src(src);
    assert!(out.contains("woof"), "{out}");
}

#[test]
fn poly_wasm_vtable_indirect() {
    let src = r#"
class Animal { func speak() { print("a") } }
class Dog: Animal { override func speak() { print("woof") } }
func make() -> Animal { return Dog() }
func main() {
    let a: Animal = make()
    a.speak()
}
"#;
    let dir = std::env::temp_dir().join(format!("syx_poly_wasm_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    let (code, lines) =
        run_wasm_source_printed(&dir.join("main.sy"), "main", &DriverOptions::default()).unwrap();
    assert_eq!(code, 0);
    assert!(lines.join("\n").contains("woof"), "{lines:?}");
}

#[test]
fn polymorphic_base_ref_count_gate() {
    let src = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/lang/polymorphic_base_ref.rs"),
    )
    .unwrap();
    let count = src.matches("#[test]").count();
    assert!(
        count >= 8,
        "polymorphic_base_ref needs 8+ honest tests, found {count}"
    );
}
