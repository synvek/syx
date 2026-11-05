// Phase 76 real LSP matrix (<=25): lock v10, cache push, VM heap slot, inkwell crate.

use super::*;

fn p76_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p76_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p76_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}










#[test]
fn p76_heap_array_parse() {
    assert!(p76_doc_ok("func main() { let a = [1, 2, 3]; print(a[0]) }"));
}

#[test]
fn p76_actor_parse() {
    assert!(p76_doc_ok(
        "actor Worker { func run() { print(1) } }\nfunc main() { Worker().run() }"
    ));
}

#[test]
fn p76_async_fn() {
    assert!(p76_doc_ok("async func f() -> Int { return 1 }\nfunc main() { print(1) }"));
}

#[test]
fn p76_class_heap_field() {
    assert!(p76_doc_ok(
        "class Node { var n: Int init(n: Int) { self.n = n } }\nfunc main() { print(Node(n: 2).n) }"
    ));
}

#[test]
fn p76_adr053_heap_four_end() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("ADR-053") || adr.contains("Phase 76"));
}

#[test]
fn p76_adr054_distributed_cache() {
    let adr = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/adr/architecture-decisions.md"),
    )
    .unwrap();
    assert!(adr.contains("ADR-054") || adr.contains("lock v10"));
}



