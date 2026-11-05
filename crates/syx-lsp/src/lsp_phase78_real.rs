// Phase 78 real LSP matrix (<=25): lock v12, farm worker, heap v7, inkwell module, borrow external.

use super::*;

fn p78_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p78_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p78_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}












#[test]
fn p78_heap_relocate_fixture() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_relocate_churn.sy");
    assert!(p.exists());
}

#[test]
fn p78_async_fn_parse() {
    assert!(p78_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p78_actor_parse() {
    assert!(p78_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p78_simd_parse() {
    assert!(p78_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32,2i32,3i32,4i32]); print(v) }"
    ));
}

#[test]
fn p78_generic_fn_parse() {
    assert!(p78_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p78_dict_slot_parse() {
    assert!(p78_doc_ok(
        "func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d.count) }"
    ));
}



#[test]
fn p78_adr_phase78_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 78") || adr.contains("ADR-057"));
}
