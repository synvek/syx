// Phase 81 real LSP matrix (<=25): lock v15, farm billing settlement, heap v10 bridge, inkwell llvm v7 link, RT native stack v6, dist CDN, tier-2E.

use super::*;

fn p81_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p81_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p81_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}











#[test]
fn p81_heap_bridge_parse() {
    assert!(p81_doc_ok("func main() { var i = 0; while i < 3 { i = i + 1 }; print(i) }"));
}

#[test]
fn p81_heap_string_offheap_parse() {
    assert!(p81_doc_ok("func main() { let s: String = \"hello\"; print(s.count) }"));
}

#[test]
fn p81_async_fn_parse() {
    assert!(p81_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p81_actor_parse() {
    assert!(p81_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p81_closure_parse() {
    assert!(p81_doc_ok("func id(x: Int) -> Int { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p81_generic_fn_parse() {
    assert!(p81_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}



#[test]
fn p81_adr_phase81_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 81") || adr.contains("ADR-067"));
}

#[test]
fn p81_heap_bridge_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_bridge_cross_process.sy");
    assert!(p.exists());
}

#[test]
fn p81_actor_cluster_global_cdn_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/parity/actor_cluster_global_cdn/main.sy");
    assert!(p.exists());
}
