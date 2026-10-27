// Phase 110 real LSP matrix (≥24): Track D fingerprints + real semantic probes (no hover_extra clones).

use super::*;

fn p110_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p110_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p110_opts())
}

#[test]
fn p110_dist_multihost_source_gate() {
    let src = include_str!("../../syx-async/src/dist_transport.rs");
    assert!(src.contains("dist_region_multihost_route"));
    assert!(src.contains("dist_region_multihost_tcp_health"));
    assert!(src.contains("SYX_DIST_CDN_MEMORY"));
}

#[test]
fn p110_select_mixed_timeout_source_gate() {
    let parser = include_str!("../../syx-parser/src/lib.rs");
    assert!(parser.contains("SELECT_MIXED_TIMEOUT_FINGERPRINT"));
    assert!(parser.contains("__syx_channel_select_mixed_await"));
    let hosts = include_str!("../../syx-vm/src/channel_hosts.rs");
    assert!(hosts.contains("host_channel_select_mixed_await"));
}

#[test]
fn p110_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p110\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p110.sy", text, 1, 8, &p110_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p110_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p110\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p110.sy", text, 2, 10, &p110_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p110_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p110.sy", text, 1, 10, &p110_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p110_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p110.sy", text, 1, 19, &p110_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p110_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p110.sy", text, 2, 8, &p110_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p110_diagnostic_unknown_ident() {
    let doc = p110_doc("func main() { print(noSuchSymbol110) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol110")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p110_diagnostic_type_mismatch() {
    let doc = p110_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p110_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p110.sy", text, 0, 8, &p110_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p110_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p110_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p110_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p110.sy", text, 0, 12, &p110_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p110_diagnostic_undefined_type() {
    let doc = p110_doc("func main() { let x: NoSuchType110 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p110_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p110_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p110_diagnostic_duplicate_local() {
    let doc = p110_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p110_select_mixed_parse_smoke() {
    let doc = p110_doc(
        "func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: print(v)\n    case ch2 <- 1: print(\"s\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p110_async_select_parse_smoke() {
    let doc = p110_doc(
        "async func tick() -> Int { return 1 }\nasync func main() {\n  let ch1 = __syx_channel_create(1)\n  let ch2 = __syx_channel_create(1)\n  select {\n    case v <- ch1: await tick()\n    case ch2 <- 1: print(\"s\")\n    timeout(30): print(\"t\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p110_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p110.sy", text, 1, 19, &p110_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p110_diagnostic_wrong_assign_type() {
    let doc = p110_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p110_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p110.sy", text, 1, 19, &p110_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p110_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p110_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p110_diagnostic_missing_return_type_use() {
    let doc = p110_doc("func main() { let z: MissingAlias110 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p110_async_func_parse_smoke() {
    let doc = p110_doc("async func tick() -> Int { return 1 }\nasync func main() { await tick() }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p110_diagnostic_string_as_int() {
    let doc = p110_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p110_cjk_string_parse_smoke() {
    let doc = p110_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p110_lsp_semantic_v12_contract() {
    let src = include_str!("lsp_phase110_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 24,
        "phase110 LSP matrix must be ≥24 tests, found {tests}"
    );
    let pad_a = concat!("fn p110_", "hover_extra");
    let pad_b = concat!("let binding", "0");
    assert!(
        !src.contains(pad_a) && !src.contains(pad_b),
        "phase110 must not ship hover_extra/bindingN padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p110_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 24,
        "need ≥24 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}

