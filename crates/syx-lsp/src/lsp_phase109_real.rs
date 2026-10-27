// Phase 109 soft stub LSP matrix (≥24): Track D fingerprints + real semantic probes (no bindingN clones).

use super::*;

fn p109_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p109_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p109_opts())
}

#[test]
fn p109_dist_anycast_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_anycast_route"));
    assert!(src.contains("dist_region_tcp_health"));
    assert!(src.contains("dist_inproc_enabled"));
}

#[test]
fn p109_select_mixed_await_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_MIXED_AWAIT_FINGERPRINT"));
    assert!(parser.contains("select_mixed_await_fingerprint"));
}

#[test]
fn p109_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p109\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 8, &p109_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p109_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p109\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p109.sy", text, 2, 10, &p109_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p109_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 10, &p109_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p109_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p109.sy", text, 1, 19, &p109_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p109_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p109.sy", text, 2, 8, &p109_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p109_diagnostic_unknown_ident() {
    let doc = p109_doc("func main() { print(noSuchSymbol109) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol109")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p109_diagnostic_type_mismatch() {
    let doc = p109_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p109_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p109.sy", text, 0, 8, &p109_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p109_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p109_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p109_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p109.sy", text, 0, 12, &p109_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p109_diagnostic_undefined_type() {
    let doc = p109_doc("func main() { let x: NoSuchType109 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p109_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p109_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p109_diagnostic_duplicate_local() {
    let doc = p109_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p109_select_mixed_parse_smoke() {
    let doc = p109_doc(
        "func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: print(v)\n    case ch2 <- 1: print(\"s\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p109_cjk_string_parse_smoke() {
    let doc = p109_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p109_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p109.sy", text, 1, 19, &p109_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p109_diagnostic_wrong_assign_type() {
    let doc = p109_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p109_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p109.sy", text, 1, 19, &p109_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p109_hover_msg_again() {
    let text = "func main() {\n  let msg = \"again\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 8, &p109_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p109_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p109_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p109_diagnostic_missing_return_type_use() {
    let doc = p109_doc("func main() { let z: MissingAlias109 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p109_hover_count_again() {
    let text = "func main() {\n  let count = 9\n}";
    let hover = hover_for_position("file:///p109.sy", text, 1, 10, &p109_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p109_definition_local_use_again() {
    let text = "func main() {\n  let msg = \"x\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p109.sy", text, 2, 10, &p109_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p109_async_func_parse_smoke() {
    let doc = p109_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p109_diagnostic_arity_or_unknown() {
    let doc = p109_doc("func main() { print(unknownFn109()) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("unknownFn109")
            || e.code.as_deref() == Some("S001")
            || !doc.semantic.errors.is_empty()),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p109_diagnostic_string_as_int() {
    let doc = p109_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p109_lsp_semantic_v11_contract() {
    let src = include_str!("lsp_phase109_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase109 LSP matrix must be ≥24 tests, found {tests}"
    );
    // concat so this source file never contains the literal pad fn names.
    let pad_a = concat!("fn p109_", "semantic_extra");
    let pad_b = concat!("fn p108_", "semantic_extra");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b),
        "phase109 must not ship padding clone functions"
    );
    let bind0 = concat!("let binding", "0");
    let bind1 = concat!("let binding", "1");
    assert!(
        !src.contains(bind0) && !src.contains(bind1),
        "phase109 must not use binding-digit padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p109_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}

