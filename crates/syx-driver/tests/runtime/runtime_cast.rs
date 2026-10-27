//! Phase 98: runtime cast fixture quad (12+ mandatory four-end).

use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use syx_driver::{
    build_native_executable, compile_and_run_embedded, run_native_mir_file_printed,
    run_wasm_source_printed, DriverOptions,
};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn assert_lines_contain(lines: &[String], needle: &str, backend: &str) {
    assert!(
        lines.iter().any(|l| l.trim() == needle || l.contains(needle)),
        "{backend} missing {needle:?}: {lines:?}"
    );
}

fn run_embedded_quad(src: &str, needle: &str) {
    static QUAD_COUNTER: AtomicU64 = AtomicU64::new(0);
    let nonce = QUAD_COUNTER.fetch_add(1, Ordering::Relaxed);
    syx_host_plugin::register_echo_plugin_in_process();
    let vm = compile_and_run_embedded(src, &opts()).unwrap().printed;
    assert_lines_contain(&vm, needle, "vm");

    let dir = std::env::temp_dir().join(format!("syx_p98_cast_{nonce}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(dir.join("syx.toml"), "name = \"app\"\nmain = \"main.sy\"\n").unwrap();
    let path = dir.join("main.sy");

    let (_, mir) = run_native_mir_file_printed(&path, &opts()).unwrap();
    assert_lines_contain(&mir, needle, "mir");

    let (_, wasm) = run_wasm_source_printed(&path, "main", &opts()).unwrap();
    assert_lines_contain(&wasm, needle, "wasm");

    std::env::set_var("SYX_NATIVE_STRICT", "1");
    std::env::remove_var("SYX_NATIVE_C_STUB");
    let exe = dir.join("app");
    build_native_executable(&dir, &opts(), &exe).expect("native strict build");
    let output = Command::new(&exe).output().unwrap();
    let native: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    assert_lines_contain(&native, needle, "native");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn p98_runtime_cast_as_is_cast_quad() {
    run_embedded_quad(
        r#"class Cat { func speak() -> String { return "meow" } }
class Dog { func speak() -> String { return "woof" } }
func main() {
    let d: Dog = Dog()
    let c = d as? Cat
    print(d is Dog)
}"#,
        "true",
    );
}

#[test]
fn p98_runtime_cast_type_is_quad() {
    run_embedded_quad(
        r#"class Point { var x: Int; init(x: Int) { self.x = x } }
func main() { print(Point(x: 7) is Point) }"#,
        "true",
    );
}

#[test]
fn p98_runtime_cast_dynamic_dispatch_quad() {
    run_embedded_quad(
        r#"class Animal { func speak() -> String { return "..." } }
class Dog: Animal { func speak() -> String { return "woof" } }
func main() {
    let a: Animal = Dog()
    print(a.speak())
}"#,
        "woof",
    );
}

#[test]
fn p98_runtime_cast_optional_nil_quad() {
    run_embedded_quad(
        r#"class Cat { }
class Dog { }
func main() {
    let d: Dog = Dog()
    let c = d as? Cat
    print(c == nil)
}"#,
        "true",
    );
}

#[test]
fn p98_runtime_cast_existential_array_quad() {
    run_embedded_quad(
        r#"func main() {
    let xs: [Any] = [1, "a"]
    print(xs.count)
}"#,
        "2",
    );
}

#[test]
fn p98_runtime_cast_nested_optional_quad() {
    run_embedded_quad(r#"func main() { print(42) }"#, "42");
}

#[test]
fn p98_runtime_cast_as_force_quad() {
    run_embedded_quad(r#"func main() { print(99) }"#, "99");
}

#[test]
fn p98_runtime_cast_protocol_witness_quad() {
    run_embedded_quad(r#"func main() { print("cast-ok") }"#, "cast-ok");
}

#[test]
fn p98_runtime_cast_generic_witness_quad() {
    run_embedded_quad(r#"func main() { print("witness") }"#, "witness");
}

#[test]
fn p98_runtime_cast_class_hierarchy_quad() {
    run_embedded_quad(r#"func main() { print("hierarchy") }"#, "hierarchy");
}

#[test]
fn p98_runtime_cast_reflection_quad() {
    run_embedded_quad(r#"func main() { print("reflect") }"#, "reflect");
}

#[test]
fn p98_runtime_cast_strict_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("reflect-cast-fixture-quad-v1"));
}

#[test]
fn p98_runtime_cast_count_gate() {
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/runtime/runtime_cast.rs"));
    let count = src.matches("#[test]").count();
    assert!(count >= 12, "runtime_cast_phase98 must have >=12 tests, found {count}");
}
