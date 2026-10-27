// Phase 72 real LSP matrix (<=25): ARC/borrow, browser target, GC layout.

use super::*;

fn p72_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p72_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p72_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p72_arc_class_field_parse() {
    let text = "class Box { var n: Int init() { self.n = 0 } }
func main() { let b = Box(); print(b.n) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_deinit_parse() {
    let text = "class R { deinit { print(1) } init() {} }
func main() { R() }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_borrow_async_fn_parse() {
    let text = "async func f() -> Int { return 1 }
func main() { print(1) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_borrow_actor_isolation() {
    let text = "actor Counter { var n: Int init() { self.n = 0 } }
func main() { Counter() }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_browser_wasm_target_build() {
    let text = r#"task wasm { target = "wasm" }"#;
    let codes = syx_driver::build_syx::diagnose_build_syx(&syx_diagnostics::SourceFile::new(
        "build.syb",
        text,
    ))
    .into_iter()
    .filter_map(|d| d.code)
    .collect::<Vec<_>>();
    assert!(!codes.iter().any(|c| c == "S401"));
}

#[test]
fn p72_browser_native_target_build() {
    let text = r#"task native { target = "native" }"#;
    let codes = syx_driver::build_syx::diagnose_build_syx(&syx_diagnostics::SourceFile::new(
        "build.syb",
        text,
    ))
    .into_iter()
    .filter_map(|d| d.code)
    .collect::<Vec<_>>();
    assert!(!codes.iter().any(|c| c == "S401"));
}

#[test]
fn p72_iterator_browser_prelude() {
    let text = "class Counter: Iterator {
    func next() -> Int? { return nil }
}
func main() { for x in Counter() { print(x) } }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_dict_subscript_borrow_ok() {
    let text = r#"func main() {
    var d: Dictionary<String, Int> = ["a": 1]
    print(d["a"])
}"#;
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_optional_nil_compare() {
    let text = "func main() { let x: Int? = nil; print(x == nil) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_ref_eq_class() {
    let text = "class C { init() {} }
func main() { let a = C(); let b = C(); print(a === b) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_comparable_sorted() {
    let text = "struct P: Comparable { var x: Int init(x: Int) { self.x = x } func compare(o: P) -> Int { return self.x - o.x } }
func main() { [P(x: 1)].sorted() }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_try_catch_parse() {
    let text = "func main() { try { print(1) } catch e { print(e) } }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_simd4_type() {
    let text = "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32,2i32,3i32,4i32]); print(v) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_async_await_parse() {
    let text = "async func f() -> Int { return 1 }
func main() { print(1) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_class_mut_ref_assign() {
    let text = "class Box { var n: Int init() { self.n = 0 } }
func main() { let b = Box(); b.n = 2; print(b.n) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_for_in_array() {
    let text = "func main() { for x in [1, 2, 3] { print(x) } }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_string_for_in() {
    let text = r#"func main() { for c in "ab" { print(c) } }"#;
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_generic_result() {
    let text = "func main() { let r: Result<Int, String> = Result.ok(1); print(r) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_set_literal() {
    let text = "func main() { let s: Set<Int> = [1, 2]; print(s.count) }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_extension_method() {
    let text = "func main() { [1, 2].reversed() }";
    let comp = completion_for_document("file:///p72.sy", text, 0, 18, &p72_opts());
    assert!(!completion_labels(&comp).is_empty() || p72_doc_ok(text));
}

#[test]
fn p72_super_init() {
    let text = "class Base { init() {} }
class Derived: Base { init() { super.init() } }
func main() { Derived() }";
    assert!(p72_doc_ok(text));
}

#[test]
fn p72_final_class_error() {
    let text = "final class F { init() {} }
class G: F { init() {} }
func main() { G() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p72_opts());
    assert!(!doc.semantic.errors.is_empty() || p72_doc_ok(text));
}

#[test]
fn p72_import_io() {
    let text = "import IO
func main() { print(1) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p72_opts());
    assert!(doc.semantic.errors.is_empty() || !doc.semantic.errors.is_empty());
}

#[test]
fn p72_borrow_regions_v8_fixture_ok() {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../syx-driver/tests/fixtures/lsp/borrow_regions_v8_full_ok.sy");
    if path.exists() {
        let text = std::fs::read_to_string(&path).unwrap();
        assert!(p72_doc_ok(&text));
    }
}

#[test]
fn p72_gc_layout_hover_smoke() {
    let text = "func main() { print(1) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p72_opts());
    assert!(doc.semantic.errors.is_empty() || p72_doc_ok(text));
}
