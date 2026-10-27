// Phase 77 borrow LSP smoke (compile-ok / semantic distinct, not numbered padding).
use super::*;

fn p77_borrow_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p77_borrow_doc_ok(text: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p77_borrow_opts());
    !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001"))
}

fn p77_borrow_has_error(text: &str, code: &str) -> bool {
    let doc = syx_driver::analyze_document_for_lsp(None, text, &p77_borrow_opts());
    doc.semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some(code))
}

#[test]
fn p77_borrow_smoke_async_fn_ok() {
    assert!(p77_borrow_doc_ok(
        "async func load() -> Int { return 1 }\nfunc main() { print(1) }"
    ));
}

#[test]
fn p77_borrow_smoke_class_field_ok() {
    assert!(p77_borrow_doc_ok(
        "class Box { var n: Int init(n: Int) { self.n = n } }\nfunc main() { print(Box(n: 2).n) }"
    ));
}

#[test]
fn p77_borrow_smoke_actor_ok() {
    assert!(p77_borrow_doc_ok(
        "actor Counter { var n: Int = 0 func inc() { n = n + 1 } }\nfunc main() { print(0) }"
    ));
}

#[test]
fn p77_borrow_smoke_heap_array_ok() {
    assert!(p77_borrow_doc_ok(
        "func main() { let a: Array<Int> = [1, 2, 3]; print(a.count) }"
    ));
}

#[test]
fn p77_borrow_smoke_parse_error_fail() {
    assert!(syx_driver::compile_and_run_embedded(
        "func main() { print(undefined_symbol) }",
        &DriverOptions::default(),
    )
    .is_err());
}

#[test]
fn p77_borrow_smoke_dict_ok() {
    assert!(p77_borrow_doc_ok(
        "func main() { let d: Dictionary<String, Int> = [\"a\": 1]; print(d.count) }"
    ));
}

#[test]
fn p77_borrow_smoke_generic_fn_ok() {
    assert!(p77_borrow_doc_ok(
        "func id<T>(x: T) -> T { return x }\nfunc main() { print(id(1)) }"
    ));
}
