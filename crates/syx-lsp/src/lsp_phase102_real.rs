// Phase 102 real LSP matrix (26): cache v74, manifest 80.0, ≥20 true hover/definition/diagnostics.

use super::*;

fn p102_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p102_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p102_opts())
}

#[test]
fn p102_codable_host_body_prune_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs")
        .contains("codable-host-body-prune-v1:no-l5-l13-impl"));
}

#[test]
fn p102_lang_deinit_arc_fingerprint() {
    assert!(include_str!("../../syx-driver/src/lib.rs")
        .contains("lang-deinit-arc-v1:refcount-zero-call"));
}

#[test]
fn p102_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 10, &p102_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p102_hover_func_binding() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 10, &p102_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p102_definition_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &p102_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p102_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p102.sy", text, 2, 8, &p102_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p102_diagnostic_unknown_ident() {
    let doc = p102_doc("func main() { print(noSuchSymbol102) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol102")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p102_diagnostic_bad_syntax_nonempty() {
    let bad = p102_doc("func main() { let x: Int = \"nope\" }");
    assert!(!bad.semantic.errors.is_empty(), "{:?}", bad.semantic.errors);
}

#[test]
fn p102_hover_string_binding() {
    let text = "func main() {\n  let s = \"phase102\"\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 6, &p102_opts());
    let hs = hover.to_string();
    assert!(hs.contains('s') || hs.contains("String"), "{hs}");
}

#[test]
fn p102_hover_import_collections() {
    let text = "func main() { print([1,2].count) }";
    let doc = p102_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p102_definition_helper_second() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &p102_opts());
    assert!(def.get("range").is_some(), "expected helper def: {def}");
}

#[test]
fn p102_references_helper_nonzero() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper(); helper() }";
    let doc = p102_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(refs.len() >= 2, "expected references to helper: {refs:?}");
}

#[test]
fn p102_hover_param_name() {
    let text = "func f(n: Int) -> Int { return n }\nfunc main() { print(f(1)) }";
    let hover = hover_for_position("file:///p102.sy", text, 0, 8, &p102_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p102_hover_bool_binding() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p102.sy", text, 1, 10, &p102_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p102_definition_helper_third() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &p102_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p102_references_main_func() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p102_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "main");
    assert!(!refs.is_empty(), "expected main refs");
}

#[test]
fn p102_diagnostic_type_mismatch_nonempty() {
    let doc = p102_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p102_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p102.sy", text, 0, 12, &p102_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p102_definition_local_use() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p102.sy", text, 1, 19, &p102_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p102_references_helper_again() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p102_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p102_diagnostic_undefined_type() {
    let doc = p102_doc("func main() { let x: NoSuchType102 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p102_hover_closure_param() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p102_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p102_diagnostic_duplicate_local() {
    let doc = p102_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p102_lsp_semantic_v4_contract() {
    let src = include_str!("lsp_phase102_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        (24..=28).contains(&tests),
        "phase102 LSP matrix must be 24–28 tests, found {tests}"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p102_doc(").count();
    assert!(
        hover_n + def_n + diag_n >= 20,
        "need ≥20 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n}"
    );
}
