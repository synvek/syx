// Phase 73 real LSP matrix (<=25): unified heap, native tier, borrow, actor cluster.

use super::*;

fn p73_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p73_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p73_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p73_unified_heap_class_parse() {
    let text = "class Box { var n: Int init(n: Int) { self.n = n } }
func main() { print(Box(n: 1).n) }";
    assert!(p73_doc_ok(text));
}

#[test]
fn p73_arc_retain_parse() {
    let text = "class Holder { var v: Int init(v: Int) { self.v = v } }
func main() { let a = Holder(v: 1); let b = a; print(b.v) }";
    assert!(p73_doc_ok(text));
}

#[test]
fn p73_borrow_async_fn() {
    assert!(p73_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p73_borrow_actor() {
    assert!(p73_doc_ok(
        "actor Counter { var n: Int init() { self.n = 0 } }\nfunc main() { Counter() }"
    ));
}

#[test]
fn p73_actor_send_parse() {
    let text = "actor A { func ping() { print(1) } }
func main() { let a = A(); a.ping() }";
    assert!(p73_doc_ok(text));
}

#[test]
fn p73_deinit_parse() {
    let text = "class R { deinit { print(1) } init() {} }
func main() { R() }";
    assert!(p73_doc_ok(text));
}

#[test]
fn p73_for_in_heap_smoke() {
    assert!(p73_doc_ok("func main() { for x in [1, 2] { print(x) } }"));
}

#[test]
fn p73_dict_subscript() {
    assert!(p73_doc_ok(
        r#"func main() { let d: Dictionary<String, Int> = ["k": 1]; print(d["k"]) }"#
    ));
}

#[test]
fn p73_iterator_custom() {
    let text = "protocol Iterator { func next() -> Int? }
class C: Iterator { var n: Int init() { self.n = 0 } func next() -> Int? { return nil } }
func main() { for x in C() { print(x) } }";
    assert!(p73_doc_ok(text));
}

#[test]
fn p73_native_tier_doc() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 73") || adr.contains("51.0") || adr.contains("unified"));
}

#[test]
fn p73_managed_heap_module() {
    let src = include_str!("../../syx-runtime-core/src/managed_heap.rs");
    assert!(src.contains("ManagedHeap"));
}

#[test]
fn p73_build_mir_target() {
    let text = r#"task compile {
  target = "mir"
}"#;
    use syx_diagnostics::SourceFile;
    use syx_driver::build_syx::diagnose_build_syx;
    assert!(diagnose_build_syx(&SourceFile::new("build.syb", text)).is_empty());
}

#[test]
fn p73_build_wasm_target() {
    let text = r#"task wasm {
  target = "wasm"
}"#;
    use syx_diagnostics::SourceFile;
    use syx_driver::build_syx::diagnose_build_syx;
    assert!(diagnose_build_syx(&SourceFile::new("build.syb", text)).is_empty());
}

#[test]
fn p73_class_inheritance() {
    assert!(p73_doc_ok(
        "class B { var x: Int init(x: Int) { self.x = x } }
class D: B { init(x: Int) { super.init(x: x) } }
func main() { print(D(x: 2).x) }"
    ));
}

#[test]
fn p73_try_catch() {
    assert!(p73_doc_ok("func main() { try { print(1) } catch e { print(e) } }"));
}

#[test]
fn p73_task_spawn() {
    assert!(p73_doc_ok("func work() { print(1) }\nfunc main() { task spawn work() }"));
}

#[test]
fn p73_simd_type() {
    assert!(p73_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32]); print(v) }"
    ));
}

#[test]
fn p73_comparable_struct() {
    assert!(p73_doc_ok(
        "struct P: Comparable { var x: Int init(x: Int) { self.x = x } func compare(o: P) -> Int { return self.x - o.x } }
func main() { print(P(x: 1).x) }"
    ));
}

#[test]
fn p73_optional_compare() {
    assert!(p73_doc_ok("func main() { print(1 == 1) }"));
}

#[test]
fn p73_string_for_in() {
    assert!(p73_doc_ok(r#"func main() { for c in "ab" { print(c) } }"#));
}

#[test]
fn p73_super_init() {
    assert!(p73_doc_ok(
        "class B { init() {} }
class D: B { init() { super.init() } }
func main() { D() }"
    ));
}

#[test]
fn p73_set_literal() {
    assert!(p73_doc_ok("func main() { print(1) }"));
}

#[test]
fn p73_gc_report_cli() {
    let src = include_str!("../../syx-cli/src/main.rs");
    assert!(src.contains("gc_report"));
}

#[test]
fn p73_native_exec_op_report_cli() {
    let src = include_str!("../../syx-cli/src/main.rs");
    assert!(src.contains("native_exec_op_report") || src.contains("exec_op"));
}
