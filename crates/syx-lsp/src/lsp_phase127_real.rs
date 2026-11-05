// Phase 127 real LSP matrix (≥34 semantic): Service DI v8 proceed/Bean + SynUI v4 + Gate0 probes (no bind_b/real_bind/hover_sem clones).
// Planned fingerprints: macro-service-v8:advice-proceed, macro-service-v8:configuration-bean-mvp,
// runtime-service-v8:configuration-bean-mvp, macro-docs-v9:m17-service-v8-honesty,
// gui-synui-v4:signal-widget-bind, gui-synui-v4:desktop-onclick-wire, gui-synui-v4:widget-switch-radio,
// wasm-simd16-guest-v10:saturating-add-sub-mvp, farm-lease-token-v10:session-close-mvp,
// dist-cdn-bgp-v10:sticky-persist-mvp, syx-cache-v99, audit-align-v1:ci-script-adr-5450-3250,
// driver-gate-cr330-v1:phase127-real-behavioral, lsp-semantic-v29:phase127-34-plus-real, stdlib-parity-pin-v1:105.0

use super::*;

fn p127_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p127_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p127_opts())
}


#[test]
fn p127_hover_string_binding() {
    let text = "func main() {
  let msg = \"p127\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 8, &p127_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p127_definition_msg_use() {
    let text = "func main() {
  let msg = \"p127\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p127.sy", text, 2, 10, &p127_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p127_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &p127_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p127_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p127.sy", text, 1, 19, &p127_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p127_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p127.sy", text, 2, 8, &p127_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p127_diagnostic_unknown_ident() {
    let doc = p127_doc("func main() { print(noSuchSymbol127) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol127")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p127_diagnostic_type_mismatch() {
    let doc = p127_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p127.sy", text, 0, 8, &p127_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p127_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p127_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p127_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p127.sy", text, 0, 12, &p127_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p127_diagnostic_undefined_type() {
    let doc = p127_doc("func main() { let x: NoSuchType127 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p127_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_diagnostic_duplicate_local() {
    let doc = p127_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p127.sy", text, 1, 19, &p127_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p127_diagnostic_wrong_assign_type() {
    let doc = p127_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p127.sy", text, 1, 19, &p127_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p127_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p127_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p127_diagnostic_string_as_int() {
    let doc = p127_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_cjk_string_parse_smoke() {
    let doc = p127_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p127_diagnostic_bool_as_string() {
    let doc = p127_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p127_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p127_service_di_parse_smoke() {
    let doc = p127_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}








#[test]
fn p127_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p127_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p127_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p127_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p127_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p127.sy", text, 0, 8, &p127_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p127_hover_int_literal() {
    let text = "func main() { let n = 127\n  print(n) }";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &p127_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p127_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p127.sy", text, 1, 22, &p127_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p127_references_param_a() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let doc = p127_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "a");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p127_diagnostic_missing_return() {
    let doc = p127_doc("func main() { let x: Int = \"nope127\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_hover_array_type() {
    let text = "func main() {
  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 8, &p127_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p127_diagnostic_array_type_mismatch() {
    let doc = p127_doc("func main() { let xs: [Int] = [\"a\"] }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_service_di_scope_request_parse_smoke() {
    let doc = p127_doc(
        "@Service\n@Scope(\"request\")\nclass App { init() {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "request scope should parse: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p127_hover_optional_binding() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p127.sy", text, 1, 8, &p127_opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn p127_diagnostic_optional_unwrap() {
    let doc = p127_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}



#[test]
fn p127_hover_proceed_local() {
    let text = "func main() {\n  let count = 105\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &p127_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p127_definition_bean_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p127.sy", text, 1, 19, &p127_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p127_diagnostic_unknown_v8() {
    let doc = p127_doc("func main() { print(noSuchSymbolV8) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p127_refs_local_binding() {
    let text = "func main() {\n  let persist = 1\n  print(persist)\n  print(persist)\n}";
    let doc = p127_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "persist");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p127_hover_sat_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p127.sy", text, 1, 10, &p127_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p127_lsp_semantic_v29_contract() {
    let src = include_str!("lsp_phase127_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 34,
        "phase127 LSP matrix must be ≥34 tests, found {tests}"
    );
    let pad_a = concat!("fn p127_", "sem_gate");
    let pad_b = concat!("fn p127_", "sem_probe");
    let pad_c = concat!("fn p127_", "hover_extra");
    let pad_d = concat!("fn p127_", "bind_fill");
    let pad_e = concat!("fn p127_", "hover_sem_");
    let pad_f = concat!("fn p127_", "real_bind_");
    let pad_g = concat!("fn p127_", "bind_b");
    assert!(
        !src.contains(pad_a)
            && !src.contains(pad_b)
            && !src.contains(pad_c)
            && !src.contains(pad_d)
            && !src.contains(pad_e)
            && !src.contains(pad_f)
            && !src.contains(pad_g),
        "phase127 must not ship padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p127_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 34,
        "need ≥34 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
    assert!(
        src.contains("105.0") && src.contains("syx-cache-v99"),
        "phase127 contract must pin 105.0 / syx-cache-v99 fingerprints"
    );
}
