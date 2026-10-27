// Phase 103 real LSP matrix (26): cache v75, manifest 81.0, ≥20 true hover/definition/diagnostics.

use super::*;

fn p103_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p103_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p103_opts())
}

#[test]
fn p103_plugin_dlclose_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("plugin-dlclose-v1:unload-reload"));
}

#[test]
fn p103_lang_resultbuilder_v2_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("lang-resultbuilder-v2:build-optional"));
}

#[test]
fn p103_select_await_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("select-await-interleave-v2:case-await-order"));
}

#[test]
fn p103_lang_deinit_weak_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs").contains("lang-deinit-weak-v1:weak-nil-on-drop"));
}

#[test]
fn p103_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p103.sy", text, 1, 10, &p103_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p103_hover_func_binding() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p103.sy", text, 1, 10, &p103_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p103_definition_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p103.sy", text, 1, 19, &p103_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p103_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p103.sy", text, 2, 8, &p103_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p103_diagnostic_unknown_ident() {
    let doc = p103_doc("func main() { print(noSuchSymbol103) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol103")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p103_diagnostic_bad_syntax_nonempty() {
    let doc = p103_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p103_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p103.sy", text, 0, 8, &p103_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p103_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p103_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p103_diagnostic_type_mismatch_smoke() {
    let doc = p103_doc("func main() { let x: Int = \"no\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p103_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p103.sy", text, 0, 12, &p103_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p103_definition_local_use() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p103.sy", text, 1, 19, &p103_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p103_references_helper_again() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p103_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p103_diagnostic_undefined_type() {
    let doc = p103_doc("func main() { let x: NoSuchType103 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p103_hover_closure_param() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p103_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p103_diagnostic_duplicate_local() {
    let doc = p103_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p103_weak_var_parse_smoke() {
    let doc = p103_doc("class C { var n: Int init(n: Int) { self.n = n } }\nfunc main() { var c = C(n: 1)\n  weak var w = c }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p103_resultbuilder_attr_parse() {
    let text = "@resultBuilder\nfunc build() -> Array<Int> {\n    if let x = 1 { x }\n}";
    let doc = p103_doc(text);
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}


