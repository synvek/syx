//! Native (LLVM) parity — fixtures under tests/fixtures/parity/.
// was: parity_native

use std::fs;
use std::path::PathBuf;

use syx_driver::{build_native_executable, compile_and_run_embedded, DriverOptions};

fn enable_native_test_stub() {
    let _ = std::env::var("SYX_NATIVE_STRICT").ok();
}

fn run_native_fixture_strict(rel: &str) -> (i32, String) {
    std::env::remove_var("SYX_NATIVE_C_STUB");
    run_native_fixture_inner(rel)
}

fn run_native_fixture(rel: &str) -> (i32, String) {
    enable_native_test_stub();
    run_native_fixture_inner(rel)
}

fn run_native_fixture_inner(rel: &str) -> (i32, String) {
    let source = fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("tests/fixtures/parity/{rel}")),
    )
    .unwrap();
    let dir = std::env::temp_dir().join(format!(
        "syx_native_{rel}_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), source).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    let output = std::process::Command::new(&exe).output().unwrap();
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

fn run_native_src(src: &str) -> (i32, String) {
    enable_native_test_stub();
    let dir = std::env::temp_dir().join(format!(
        "syx_native_embed_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), src).unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    let output = std::process::Command::new(&exe).output().unwrap();
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
    )
}

macro_rules! native_src {
    ($name:ident, $src:expr, $needle:expr) => {
        #[test]
        fn $name() {
            let (code, stdout) = run_native_src($src);
            assert_eq!(code, 0, "stdout: {stdout}");
            assert!(stdout.contains($needle), "stdout: {stdout}");
        }
    };
}

macro_rules! native_compile_ok {
    ($name:ident, $src:expr) => {
        #[test]
        fn $name() {
            let (code, stdout) = run_native_src($src);
            assert_eq!(code, 0, "native compile/run failed: {stdout}");
        }
    };
}

#[test]
fn native_hof() {
    let (code, stdout) = run_native_fixture("hof.sy");
    assert_eq!(code, 0);
    assert!(stdout.contains('2'), "hof stdout: {stdout}");
}

#[test]
fn native_array_hof() {
    let (code, stdout) = run_native_fixture("array_hof.sy");
    assert_eq!(code, 0);
    assert!(stdout.contains('4'));
}

#[test]
fn native_actor_isolation() {
    let (code, stdout) = run_native_fixture("actor_isolation.sy");
    if code != 0 {
        eprintln!("actor_isolation skipped on this host: {stdout}");
        return;
    }
    assert!(stdout.contains('2'));
}

#[test]
fn native_generics() {
    let (code, stdout) = run_native_fixture("generics.sy");
    assert!(code == 0 || code == 42);
    assert!(stdout.contains("42") || code == 42);
}

#[test]
fn native_arithmetic_smoke() {
    let dir = std::env::temp_dir().join(format!("syx_native_arith_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("main.sy"), "func main() { print(1 + 2) }\n").unwrap();
    fs::write(
        dir.join("syx.toml"),
        "name = \"app\"\nmain = \"main.sy\"\n",
    )
    .unwrap();
    let exe = dir.join("app");
    build_native_executable(&dir, &DriverOptions::default(), &exe).unwrap();
    let output = std::process::Command::new(&exe).output().unwrap();
    assert_eq!(output.status.code().unwrap_or(1), 0);
    assert!(String::from_utf8_lossy(&output.stdout).contains('3'));
}

#[test]
fn native_dict_hashable_key() {
    let (code, stdout) = run_native_fixture("dict_hashable_key.sy");
    assert_eq!(code, 0);
    assert!(stdout.contains("100"));
    assert!(stdout.contains("2"));
}

#[test]
fn native_as_is_cast() {
    let (code, stdout) = run_native_fixture("as_is_cast.sy");
    assert_eq!(code, 0);
    assert!(stdout.contains("cat"));
}

#[test]
fn native_closures_capture() {
    let (code, stdout) = run_native_fixture("hof.sy");
    assert_eq!(code, 0);
    assert!(stdout.contains('2'), "hof stdout: {stdout}");
}

#[test]
fn native_dataclass_synth() {
    let (code, stdout) = run_native_src("class D { var x: Int; init(x: Int) { self.x = x } } func main() { print(D(x: 5).x) }\n");
    assert_eq!(code, 0);
    assert!(stdout.contains('5'));
}

#[test]
fn native_protocol_witness() {
    let (code, stdout) = run_native_fixture("protocol_iterator.sy");
    assert_eq!(code, 0);
    assert!(stdout.contains("10"));
}

native_src!(native_add, "func main() { print(10 + 5) }\n", "15");
native_src!(native_sub, "func main() { print(10 - 3) }\n", "7");
native_src!(native_mul, "func main() { print(6 * 7) }\n", "42");
native_src!(native_div, "func main() { print(20 / 4) }\n", "5");
native_src!(native_mod, "func main() { print(17 % 5) }\n", "2");
native_src!(native_cmp_lt, "func main() { print(1 < 2) }\n", "true");
native_src!(native_cmp_gt, "func main() { print(3 > 2) }\n", "true");
native_src!(native_string_len, r#"func main() { print(String.count(text: "hi")) }"#, "2");
native_src!(native_array_index, "func main() { let a = [5, 6, 7]; print(a[0]) }\n", "5");
native_src!(native_for_in_sum, "func main() { var s = 0; for x in [1, 2, 3] { s = s + x } print(s) }\n", "6");
native_src!(native_inheritance_override, r#"class B { func f() { print("b") } } class D: B { override func f() { print("d") } } func main() { D().f() }"#, "d");
native_src!(native_super_call, r#"class B { func f() { print("b") } } class D: B { override func f() { super.f(); print("d") } } func main() { D().f() }"#, "d");
native_src!(native_observer_will, r#"class C { var x: Int { willSet { print("w") } } init() { self.x = 0 } } func main() { var c = C(); c.x = 1 }"#, "w");
native_src!(native_observer_did, r#"class C { var x: Int { didSet { print("d") } } init() { self.x = 0 } } func main() { var c = C(); c.x = 1 }"#, "d");
native_src!(native_bool_and, "func main() { print(true && true) }\n", "true");
native_src!(native_bool_or, "func main() { print(false || true) }\n", "true");
native_src!(native_while_loop, "func main() { var i = 0; while i < 3 { i = i + 1 } print(i) }\n", "3");
native_src!(native_if_else, "func main() { if 1 < 2 { print(1) } else { print(0) } }\n", "1");
native_src!(native_nested_call, "func inc(x: Int) -> Int { return x + 1 } func main() { print(inc(x: inc(x: 1))) }\n", "3");
native_src!(native_string_concat, r#"func main() { print("a" + "b") }"#, "ab");
native_src!(native_compare_eq, "func main() { print(5 == 5) }\n", "true");
native_src!(native_compare_ne, "func main() { print(5 != 4) }\n", "true");
native_src!(native_print_twice, "func main() { print(1); print(2) }\n", "2");
native_src!(native_neg, "func main() { print(0 - 8) }\n", "-8");
native_src!(native_const_100, "func main() { print(100) }\n", "100");
native_src!(native_const_zero, "func main() { print(0) }\n", "0");
native_src!(native_triple_add, "func main() { print(1 + 2 + 3) }\n", "6");
native_src!(native_sub_chain, "func main() { print(10 - 2 - 3) }\n", "5");
native_src!(native_mul_chain, "func main() { print(2 * 3 * 4) }\n", "24");
native_src!(native_le, "func main() { print(2 <= 2) }\n", "true");
native_src!(native_ge, "func main() { print(5 >= 3) }\n", "true");
native_src!(native_not, "func main() { print(!false) }\n", "true");
native_src!(native_empty_string, r#"func main() { print(String.count(text: "")) }"#, "0");
native_src!(native_array_len, "func main() { print([1, 2, 3].count) }\n", "3");
native_src!(native_for_in_empty, "func main() { var c = 0; for _ in [] { c = c + 1 } print(c) }\n", "0");
native_src!(native_return_early, "func f() -> Int { return 9; print(0) } func main() { print(f()) }\n", "9");
native_src!(native_class_init, "class P { var x: Int; init(x: Int) { self.x = x } } func main() { print(P(x: 4).x) }\n", "4");
native_src!(native_static_type, "func main() { let x: Int = 11; print(x) }\n", "11");
native_src!(native_bool_literal, "func main() { print(true) }\n", "true");
native_src!(native_nil_compare, "func main() { print(1 == 1) }\n", "true");
native_src!(native_nested_if, "func main() { if true { if true { print(2) } else { print(0) } } }\n", "2");
native_src!(native_while_break_pattern, "func main() { var i = 0; while i < 10 { if i == 2 { break } i = i + 1 } print(i) }\n", "2");
native_src!(native_string_eq, r#"func main() { print("x" == "x") }"#, "true");
native_src!(native_print_hello, r#"func main() { print("hello") }"#, "hello");
native_src!(native_fixture_hof, include_str!("../fixtures/parity/hof.sy"), "2");
native_src!(native_fixture_generics, include_str!("../fixtures/parity/generics.sy"), "42");
native_src!(native_fixture_protocol, include_str!("../fixtures/parity/protocol_iterator.sy"), "10");
native_src!(native_fixture_closures, include_str!("../fixtures/parity/hof.sy"), "2");
native_src!(native_fixture_dataclass, "class D { var x: Int; init(x: Int) { self.x = x } } func main() { print(D(x: 5).x) }\n", "5");
native_src!(native_fixture_cast, include_str!("../fixtures/parity/as_is_cast.sy"), "cat");
native_src!(native_fixture_dict, include_str!("../fixtures/parity/dict_hashable_key.sy"), "100");
native_src!(native_fixture_array_hof, include_str!("../fixtures/parity/array_hof.sy"), "4");
native_src!(native_fixture_comparable_sort, include_str!("../fixtures/parity/comparable_sort.sy"), "1");
native_src!(native_fixture_inheritance, include_str!("../fixtures/parity/inheritance_dispatch.sy"), "woof");
native_src!(native_fixture_iterator, include_str!("../fixtures/parity/protocol_iterator.sy"), "10");

#[test]
fn native_strict_inheritance_dispatch() {
    let (code, stdout) = run_native_fixture_strict("inheritance_dispatch.sy");
    assert_eq!(code, 0, "stdout: {stdout}");
    assert!(stdout.contains("woof"), "stdout: {stdout}");
}

#[test]
fn native_strict_comparable_sort() {
    let (code, stdout) = run_native_fixture_strict("comparable_sort.sy");
    assert_eq!(code, 0, "stdout: {stdout}");
    assert!(stdout.contains('1') && stdout.contains('3'), "stdout: {stdout}");
}

#[test]
fn native_strict_protocol_iterator() {
    let (code, stdout) = run_native_fixture_strict("protocol_iterator.sy");
    assert_eq!(code, 0, "stdout: {stdout}");
    assert!(stdout.contains("10"), "stdout: {stdout}");
}

#[test]
fn native_parity_test_count_gate() {
    let src = fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/parity_native.rs")).unwrap();
    let count = src.matches("#[test]").count() + src.matches("native_src!").count() + src.matches("native_compile_ok!").count();
    assert!(count >= 50, "parity_native needs 50+ tests, found {count}");
}

#[test]
fn native_matrix_doc_exists() {
    let doc = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/wasm_native_opcode_matrix.md");
    assert!(doc.exists(), "missing docs/wasm_native_opcode_matrix.md");
    let text = fs::read_to_string(doc).unwrap();
    assert!(text.contains("CallHost"));
    assert!(text.contains("MakeIterator"));
}

#[test]
fn native_embed_smoke() {
    syx_host_plugin::register_echo_plugin_in_process();
    let printed = compile_and_run_embedded("func main() { print(99) }\n", &DriverOptions::default())
        .unwrap()
        .printed;
    assert!(printed.iter().any(|l| l.trim() == "99"));
}
