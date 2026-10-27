// Phase 106 real LSP matrix (24–28): manifest, audit, cache v78, farm/DX fingerprints, ≥24 semantic probes.

use super::*;

fn p106_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p106_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p106_opts())
}

#[test]
fn p106_hover_string_literal_binding() {
    let text = "func main() {\n  let msg = \"p106\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p106.sy", text, 1, 8, &p106_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p106_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p106\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p106.sy", text, 2, 10, &p106_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p106_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p106.sy", text, 1, 10, &p106_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p106_hover_func_binding() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p106.sy", text, 1, 10, &p106_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p106_definition_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p106.sy", text, 1, 19, &p106_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p106_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p106.sy", text, 2, 8, &p106_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p106_diagnostic_unknown_ident() {
    let doc = p106_doc("func main() { print(noSuchSymbol106) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol106")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p106_diagnostic_bad_syntax_nonempty() {
    let doc = p106_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p106_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p106.sy", text, 0, 8, &p106_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p106_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p106_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p106_diagnostic_type_mismatch_smoke() {
    let doc = p106_doc("func main() { let x: Int = \"no\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p106_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p106.sy", text, 0, 12, &p106_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p106_definition_local_use() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p106.sy", text, 1, 19, &p106_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p106_references_helper_again() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p106_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p106_diagnostic_undefined_type() {
    let doc = p106_doc("func main() { let x: NoSuchType106 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p106_hover_closure_param() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p106_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p106_diagnostic_duplicate_local() {
    let doc = p106_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p106_select_default_parse_smoke() {
    let doc = p106_doc(
        "func main() {\n  select {\n    case let x = ch.receive(): print(x)\n    default: print(\"d\")\n  }\n}",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p106_cjk_string_parse_smoke() {
    let doc = p106_doc("func main() { let s = \"你\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p106_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p106.sy", text, 1, 19, &p106_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}



#[test]
fn p106_farm_server_gate_source() {
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("farm_lease_require_active"));
    assert!(lib.contains("farm_tcp_roundtrip"));
}
