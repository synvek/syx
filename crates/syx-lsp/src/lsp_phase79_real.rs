// Phase 79 real LSP matrix (<=25): lock v13, farm multitenant, heap v8, inkwell object, RT dispatch, actor IPC.

use super::*;

fn p79_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p79_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p79_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}












#[test]
fn p79_dict_slot_parse() {
    assert!(p79_doc_ok(
        "func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d.count) }"
    ));
}

#[test]
fn p79_heap_string_inline_parse() {
    assert!(p79_doc_ok("func main() { let s: String = \"hi\"; print(s.count) }"));
}

#[test]
fn p79_async_fn_parse() {
    assert!(p79_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p79_actor_parse() {
    assert!(p79_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p79_simd_parse() {
    assert!(p79_doc_ok(
        "func main() { let v: SIMD4<Int32> = SIMD4.fromArray([1i32,2i32,3i32,4i32]); print(v) }"
    ));
}

#[test]
fn p79_generic_fn_parse() {
    assert!(p79_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}



#[test]
fn p79_adr_phase79_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 79") || adr.contains("ADR-059"));
}

#[test]
fn p79_heap_dict_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_dict_slot_unified.sy");
    assert!(p.exists());
}
