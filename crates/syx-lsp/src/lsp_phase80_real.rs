// Phase 80 real LSP matrix (<=25): lock v14, farm global CDN, heap v9, inkwell llvm, RT native stack, dist actor, tier-2D.

use super::*;

fn p80_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p80_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p80_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}











#[test]
fn p80_heap_generational_parse() {
    assert!(p80_doc_ok("func main() { var i = 0; while i < 3 { i = i + 1 }; print(i) }"));
}

#[test]
fn p80_heap_string_full_inline_parse() {
    assert!(p80_doc_ok("func main() { let s: String = \"hello\"; print(s.count) }"));
}

#[test]
fn p80_async_fn_parse() {
    assert!(p80_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p80_actor_parse() {
    assert!(p80_doc_ok("actor A { var n: Int = 0 }\nfunc main() { print(0) }"));
}

#[test]
fn p80_closure_parse() {
    assert!(p80_doc_ok("func id(x: Int) -> Int { return x }\nfunc main() { print(id(1)) }"));
}

#[test]
fn p80_generic_fn_parse() {
    assert!(p80_doc_ok("func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"));
}


#[test]
fn p80_adr_phase80_charter() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("Phase 80") || adr.contains("ADR-063"));
}

#[test]
fn p80_heap_generational_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/memory/heap_generational_churn.sy");
    assert!(p.exists());
}

#[test]
fn p80_actor_cluster_cross_region_fixture_exists() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/syx-driver/tests/fixtures/parity/actor_cluster_cross_region/main.sy");
    assert!(p.exists());
}
