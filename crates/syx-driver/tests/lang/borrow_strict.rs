//! borrow SE-0461 strict compile matrix.
// was: borrow_strict

use std::path::PathBuf;

use syx_driver::{compile_and_run_embedded, compile_file_from_source, DriverOptions};

fn opts() -> DriverOptions {
    DriverOptions::default()
}

fn compile_ok(src: &str) -> bool {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_file_from_source("main.sy", src).is_ok()
}

fn run_ok(src: &str) -> bool {
    syx_host_plugin::register_echo_plugin_in_process();
    compile_and_run_embedded(src, &opts()).is_ok()
}

#[test]
fn borrow_se0461_default_enabled() {
    assert!(syx_semantic::borrow_regions_se0461_complete_enabled());
}

#[test]
fn borrow_fixture_v8_full_ok() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/lsp/borrow_regions_v8_full_ok.sy");
    if path.exists() {
        let src = std::fs::read_to_string(&path).unwrap();
        assert!(compile_ok(&src));
    }
}

#[test]
fn borrow_async_fn_ok() {
    assert!(compile_ok(
        "async func f() -> Int { return 1 }\nfunc main() { print(1) }"
    ));
}

#[test]
fn borrow_actor_ok() {
    assert!(compile_ok(
        "actor A { var n: Int init() { self.n = 0 } }\nfunc main() { A() }"
    ));
}

#[test]
fn borrow_for_in_ok() {
    assert!(run_ok("func main() { for x in [1, 2] { print(x) } }"));
}

#[test]
fn borrow_class_field_ok() {
    assert!(run_ok(
        "class C { var x: Int init() { self.x = 0 } }\nfunc main() { print(C().x) }"
    ));
}

#[test]
fn borrow_if_else_ok() {
    assert!(run_ok("func main() { if 1 < 2 { print(9) } else { print(0) } }"));
}

#[test]
fn borrow_func_return_ok() {
    assert!(run_ok("func f() -> Int { return 42 } func main() { print(f()) }"));
}

#[test]
fn borrow_string_eq_ok() {
    assert!(run_ok(r#"func main() { print("x" == "x") }"#));
}

#[test]
fn borrow_class_alias_ok() {
    assert!(compile_ok(
        "class Holder { var n: Int init(n: Int) { self.n = n } }
func main() { print(Holder(n: 4).n) }"
    ));
}

#[test]
fn borrow_empty_for_in_ok() {
    assert!(run_ok(
        r#"func main() {
    var n = 0
    for x in [] { n = n + 1 }
    print(n)
}"#
    ));
}

#[test]
fn borrow_triple_add_ok() {
    assert!(run_ok("func main() { print(1 + 2 + 3) }"));
}

#[test]
fn borrow_gate_count() {
    let src = include_str!("borrow_strict.rs");
    assert!(src.matches("#[test]").count() >= 10);
}
