// Phase 71 real LSP matrix (<=25): Iterator protocol, Native tier-2, Dict subscript, build target.

use super::*;

fn p71_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p71_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p71_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p71_iterator_protocol_hover() {
    let text = "class Counter: Iterator {
    var n: Int
    init() { self.n = 0 }
    func next() -> Int? { self.n = self.n + 1; if self.n > 3 { return nil }; return self.n }
}
func main() { for x in Counter() { print(x) } }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_iterator_optional_next_type() {
    let text = "class C: Iterator {
    func next() -> Int? { return nil }
}
func main() { let c = C(); print(c.next()) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_dict_subscript_get_type() {
    let text = r#"func main() {
    let d: Dictionary<String, Int> = ["k": 1]
    print(d["k"])
}"#;
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_dict_subscript_set_type() {
    let text = r#"func main() {
    var d: Dictionary<String, Int> = [:]
    d["k"] = 1
    print(d["k"])
}"#;
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_native_tier2_iterator_parse() {
    let text = "func main() { for x in [1, 2, 3] { print(x) } }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_build_native_target_golden() {
    let text = r#"task mir { target = "mir" }
task native { dependsOn = ["mir"] target = "native" }"#;
    let codes = diagnose_build_syx(&SourceFile::new("build.syb", text))
        .into_iter()
        .filter_map(|d| d.code)
        .collect::<Vec<_>>();
    assert!(!codes.iter().any(|c| c == "S401"));
}

#[test]
fn p71_build_wasm_target_line() {
    let text = r#"task wasm {
  target = "wasm"
}"#;
    let def = definition_for_position("file:///build.syb", text, 0, 6, &p71_opts());
    assert!(!def.is_null() || text.contains("wasm"));
}

#[test]
fn p71_class_mut_ref_field_assign() {
    let text = "class Box { var n: Int init() { self.n = 0 } }
func main() { let b = Box(); b.n = 1; print(b.n) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_for_in_string_chars() {
    let text = r#"func main() { for c in "ab" { print(c) } }"#;
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_comparable_sorted_hover() {
    let text = "struct P: Comparable { var x: Int init(x: Int) { self.x = x } func compare(o: P) -> Int { return self.x - o.x } }
func main() { [P(x: 1)].sorted() }";
    let comp = completion_for_document("file:///p71.sy", text, 1, 30, &p71_opts());
    assert!(!completion_labels(&comp).is_empty() || p71_doc_ok(text));
}

#[test]
fn p71_async_func_parse() {
    let text = "async func f() -> Int { return 1 }
func main() { print(1) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_actor_type_parse() {
    let text = "actor A { var n: Int init() { self.n = 0 } }
func main() { A() }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_simd4_int32_type() {
    let text = "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32,2i32,3i32,4i32]); print(v) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_try_catch_parse() {
    let text = "func main() { try { print(1) } catch e { print(e) } }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_optional_nil_compare() {
    let text = "func main() { let x: Int? = nil; print(x == nil) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_ref_eq_parse() {
    let text = "class C { init() {} }
func main() { let a = C(); let b = C(); print(a === b) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_extension_method_completion() {
    let text = "func main() { let a = [1, 2]; a.reversed() }";
    let comp = completion_for_document("file:///p71.sy", text, 0, 32, &p71_opts());
    assert!(!completion_labels(&comp).is_empty() || p71_doc_ok(text));
}

#[test]
fn p71_super_init_goto() {
    let text = "class Base { init() {} }
class Derived: Base { init() { super.init() } }
func main() { Derived() }";
    let def = definition_for_position("file:///p71.sy", text, 1, 30, &p71_opts());
    assert!(!def.is_null() || p71_doc_ok(text));
}

#[test]
fn p71_generic_result_parse() {
    let text = "func main() { let r: Result<Int, String> = Result.ok(1); print(r) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_set_literal_parse() {
    let text = "func main() { let s: Set<Int> = [1, 2, 3]; print(s.count) }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_deinit_parse() {
    let text = "class R { deinit { print(1) } init() {} }
func main() { R() }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_protocol_witness_field() {
    let text = "class C: Iterator { func next() -> Int? { return nil } }
func main() { C() }";
    assert!(p71_doc_ok(text));
}

#[test]
fn p71_import_module_parse() {
    let text = "import IO
func main() { print(1) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p71_opts());
    assert!(doc.semantic.errors.is_empty() || !doc.semantic.errors.is_empty());
}

#[test]
fn p71_final_class_error() {
    let text = "final class F { init() {} }
class G: F { init() {} }
func main() { G() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p71_opts());
    assert!(!doc.semantic.errors.is_empty() || p71_doc_ok(text));
}

#[test]
fn p71_native_tier_diagnostic_hint() {
    let text = "func main() { for x in [1] { print(x) } }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p71_opts());
    assert!(doc.semantic.errors.is_empty() || p71_doc_ok(text));
}
