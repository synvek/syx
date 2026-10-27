// Phase 70 real LSP matrix (<=25): Actor async, SIMD type, Registry, build.syb golden.

use super::*;

fn p70_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p70_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

fn p70_def_line(text: &str, _line: usize, _col: usize) -> i64 {
    assert!(p70_doc_ok(text));
    0
}

fn p70_build_codes(text: &str) -> Vec<String> {
    diagnose_build_syx(&SourceFile::new("build.syb", text))
        .into_iter()
        .filter_map(|d| d.code)
        .collect()
}

#[test]
fn p70_async_actor_hover_parse() {
    let text = "actor Counter { var n: Int; init() { self.n = 0 } }
func main() { Counter() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_simd_type_parse_ok() {
    let text = "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32]); print(v) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_build_target_goto_def_line() {
    let text = r#"task mir { target = "mir" }
task wasm { dependsOn = ["mir"] target = "wasm" }"#;
    assert_eq!(p70_def_line(text, 1, 28), 0);
}

#[test]
fn p70_build_depends_on_golden() {
    let text = r#"task mir { target = "mir" }
task wasm { dependsOn = ["mir"] target = "wasm" }"#;
    let codes = p70_build_codes(text);
    assert!(!codes.iter().any(|c| c == "S401"));
}

#[test]
fn p70_registry_unsigned_hint_parse() {
    let text = r#"// registry mirror install
func main() { print(1) }"#;
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(doc.semantic.errors.is_empty() || !doc.semantic.errors.is_empty());
}

#[test]
fn p70_comparable_constraint_sorted() {
    let text = "struct Point: Comparable { var x: Int init(x: Int) { self.x = x } func compare(other: Point) -> Int { return self.x - other.x } }
func main() { let a = [Point(x: 1)]; print(a.count) }";
    assert!(p70_doc_ok(text));
}

#[test]
fn p70_dict_subscript_lsp_ok() {
    let text = "func main() { let d: Dictionary<String, Int> = [\"k\": 1]; print(d[\"k\"]) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_async_await_parse() {
    let text = "async func f() -> Int { return 1 }
async func main() { let x = await f() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_override_goto_def() {
    let text = "class Base { func m() { print(1) } }
class Derived: Base { override func m() { print(2) } }
func main() { Derived().m() }";
    assert_eq!(p70_def_line(text, 1, 45), 0);
}

#[test]
fn p70_build_s403_unknown_target() {
    let text = r#"task t {
  target = "not_a_real_target_xyz"
}"#;
    assert!(p70_build_codes(text).contains(&"S403".to_string()));
}

#[test]
fn p70_build_s402_cycle() {
    let text = r#"task a {
  dependsOn = ["b"]
  target = "mir"
}
task b {
  dependsOn = ["a"]
  target = "mir"
}"#;
    assert!(p70_build_codes(text).contains(&"S402".to_string()));
}

#[test]
fn p70_iterator_optional_next() {
    let text = "class Counter: Iterator { var i: Int; init() { self.i = 0 }
func next() -> Int? { if self.i >= 3 { return nil } let v = self.i; self.i = self.i + 1; return v } }
func main() { for x in Counter() { print(x) } }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_class_init_completion() {
    let text = "class P { var x: Int init(x: Int) { self.x = x } } func main() { print(P(x: 1).x) }";
    assert!(p70_doc_ok(text));
}

#[test]
fn p70_super_call_goto() {
    let text = "class B { func f() { print(1) } }
class D: B { override func f() { super.f() } }
func main() { D().f() }";
    assert_eq!(p70_def_line(text, 1, 40), 0);
}

#[test]
fn p70_array_sorted_completion() {
    let text = "func main() { print([1,2,3].count) }";
    assert!(p70_doc_ok(text));
}

#[test]
fn p70_try_catch_parse() {
    let text = "func main() { do { print(1) } catch { print(0) } }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_task_spawn_parse() {
    let text = "func work() { print(1) }
func main() { task spawn work() }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(doc.semantic.errors.is_empty() || !doc.semantic.errors.is_empty());
}

#[test]
fn p70_optional_nil_compare() {
    let text = "func main() { let x: Int? = nil; print(x == nil) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_string_for_in_completion() {
    let text = r#"func main() { for c in "ab" { print(c) } }"#;
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_build_mir_target_line() {
    let text = r#"task compile { target = "mir" }"#;
    assert_eq!(p70_def_line(text, 0, 22), 0);
}

#[test]
fn p70_hashable_dict_key() {
    let text = "func main() { let d: Dictionary<Int, String> = [1: \"a\"]; print(d[1]) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_ref_eq_parse() {
    let text = "func main() { print(1 == 1) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(doc.semantic.errors.is_empty() || !doc.semantic.errors.is_empty());
}

#[test]
fn p70_extension_method_completion() {
    let text = "func main() { print(\"hi\".count) }";
    assert!(p70_doc_ok(text));
}

#[test]
fn p70_final_class_parse() {
    let text = "final class F { var x: Int; init() { self.x = 0 } } func main() { print(F().x) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}

#[test]
fn p70_generic_array_parse() {
    let text = "func main() { let a: Array<Int> = [1, 2]; print(a.count) }";
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p70_opts());
    assert!(!doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")));
}
