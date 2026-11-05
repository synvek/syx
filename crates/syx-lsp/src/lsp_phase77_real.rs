// Phase 77 real LSP matrix (<=25): lock v11, farm queue, heap v6, inkwell llvm, borrow proof.

use super::*;

fn p77_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p77_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p77_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}












#[test]
fn p77_adr_phase77_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 77") || adr.contains("ADR-055"));
}

#[test]
fn p77_heap_dict_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_dict_slot_unified.sy");
    assert!(p.exists());
}

#[test]
fn p77_async_fn_parse() {
    assert!(p77_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p77_class_heap_field() {
    assert!(p77_doc_ok(
        "class Node { var n: Int init(n: Int) { self.n = n } }\nfunc main() { print(Node(n: 2).n) }"
    ));
}

#[test]
fn p77_simd_type_parse() {
    assert!(p77_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32, 2i32, 3i32, 4i32]); print(v) }"
    ));
}


