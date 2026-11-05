// Phase 75 real LSP matrix (<=25): heap v4 compact, inkwell default, native deps link.

use super::*;

fn p75_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p75_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p75_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

#[test]
fn p75_heap_v4_class_parse() {
    assert!(p75_doc_ok(
        "class Box { var n: Int init(n: Int) { self.n = n } }
func main() { print(Box(n: 1).n) }"
    ));
}





#[test]
fn p75_public_actor_parse() {
    assert!(p75_doc_ok(
        "public actor Counter { var n: Int init() { self.n = 0 } }\nfunc main() { Counter() }"
    ));
}

#[test]
fn p75_actor_send_parse() {
    assert!(p75_doc_ok(
        "actor A { func ping() { print(1) } }
func main() { let a = A(); a.ping() }"
    ));
}

#[test]
fn p75_borrow_async_fn() {
    assert!(p75_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p75_try_catch() {
    assert!(p75_doc_ok("func main() { try { print(1) } catch e { print(e) } }"));
}

#[test]
fn p75_task_spawn() {
    assert!(p75_doc_ok("func work() { print(1) }\nfunc main() { task spawn work() }"));
}

#[test]
fn p75_simd_type() {
    assert!(p75_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32]); print(v) }"
    ));
}

#[test]
fn p75_adr_phase75_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 75") || adr.contains("53.0") || adr.contains("ADR-051"));
}



#[test]
fn p75_class_inheritance() {
    assert!(p75_doc_ok(
        "class B { var x: Int init(x: Int) { self.x = x } }
class D: B { init(x: Int) { super.init(x: x) } }
func main() { print(D(x: 2).x) }"
    ));
}

#[test]
fn p75_dict_subscript() {
    assert!(p75_doc_ok(
        r#"func main() { let d: Dictionary<String, Int> = ["k": 1]; print(d["k"]) }"#
    ));
}

#[test]
fn p75_for_in_heap() {
    assert!(p75_doc_ok("func main() { for x in [1, 2] { print(x) } }"));
}

#[test]
fn p75_string_concat() {
    assert!(p75_doc_ok(r#"func main() { print("a" + "b") }"#));
}

#[test]
fn p75_deinit_parse() {
    assert!(p75_doc_ok(
        "class R { deinit { print(1) } init() {} }
func main() { R() }"
    ));
}

#[test]
fn p75_comparable_struct() {
    assert!(p75_doc_ok(
        "struct P: Comparable { var x: Int init(x: Int) { self.x = x } func compare(o: P) -> Int { return self.x - o.x } }
func main() { print(P(x: 1).x) }"
    ));
}

#[test]
fn p75_build_mir_target() {
    let text = r#"task compile {
  target = "mir"
}"#;
    use syx_diagnostics::SourceFile;
    use syx_driver::build_syx::diagnose_build_syx;
    assert!(diagnose_build_syx(&SourceFile::new("build.syb", text)).is_empty());
}

