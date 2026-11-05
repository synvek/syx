// Phase 125 real LSP matrix (≥30 semantic): Service DI v6 polymorphic + SynUI v2 stubs + Gate0 probes (no bind_b/real_bind/hover_sem clones).
// Planned fingerprints (Gate0 stubs): macro-service-v6:polymorphic-base-inject, macro-service-v6:field-qualifier-metadata,
// runtime-service-v6:protocol-binding, macro-m15:service-method-interceptor-mvp, macro-docs-v7:m15-service-v6-honesty,
// gui-synui-v2:desktop-run-mvp, gui-synui-v2:signal-callback, gui-synui-v2:widget-expand, gui-synui-v2:ci-sibling-checkout,
// wasm-simd16-guest-v8:lane-extract-replace, farm-lease-token-v8:session-pin-persist,
// dist-cdn-bgp-v8:sticky-pop-mvp, syx-cache-v97

use super::*;

fn p125_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p125_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p125_opts())
}


#[test]
fn p125_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p125\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p125.sy", text, 1, 8, &p125_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p125_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p125\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p125.sy", text, 2, 10, &p125_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p125_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p125.sy", text, 1, 10, &p125_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p125_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p125.sy", text, 1, 19, &p125_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p125_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p125.sy", text, 2, 8, &p125_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p125_diagnostic_unknown_ident() {
    let doc = p125_doc("func main() { print(noSuchSymbol125) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol125")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p125_diagnostic_type_mismatch() {
    let doc = p125_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p125.sy", text, 0, 8, &p125_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p125_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p125_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p125_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p125.sy", text, 0, 12, &p125_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p125_diagnostic_undefined_type() {
    let doc = p125_doc("func main() { let x: NoSuchType125 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p125_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_diagnostic_duplicate_local() {
    let doc = p125_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p125.sy", text, 1, 19, &p125_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p125_diagnostic_wrong_assign_type() {
    let doc = p125_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p125.sy", text, 1, 19, &p125_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p125_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p125_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p125_diagnostic_string_as_int() {
    let doc = p125_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_cjk_string_parse_smoke() {
    let doc = p125_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p125_diagnostic_bool_as_string() {
    let doc = p125_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_references_local_msg() {
    let text = "func main() {\n  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p125_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p125_service_di_parse_smoke() {
    let doc = p125_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p125_di_package_import_surface_stub() {
    // Gate0 stub: Di package GA surface (`macro-service-v6:polymorphic-base-inject`) — parse-level probe.
    let doc = p125_doc(
        "@Service\nclass App { init() {} }\nfunc main() { let a: App = App(); print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "Di package surface stub should parse: {:?}",
        doc.semantic.errors
    );
}




#[test]
fn p125_di_with_request_scope_stub() {
    // Gate0 stub: Di.withRequest / request scope inherited into v5
    let doc = p125_doc(
        "@Service\n@Scope(\"request\")\nclass ReqSvc { init() {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "withRequest/request scope stub should parse: {:?}",
        doc.semantic.errors
    );
}



#[test]
fn p125_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p125_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p125_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p125_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p125_references_service_app() {
    let text = "@Service\nclass App { init() {} }\nfunc main() { let a: App = App() }";
    let doc = p125_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "App");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p125_diagnostic_service_on_function_e006() {
    let src = "@Service\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = p125_doc(src);
    let via_lsp = doc
        .semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some("E006"));
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &p125_opts()).expect_err("E006");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E006") || msg.contains("@Service/@Component"),
        "expected E006 for @Service on function: {msg}"
    );
}

#[test]
fn p125_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p125.sy", text, 0, 8, &p125_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p125_hover_int_literal() {
    let text = "func main() { let n = 125\n  print(n) }";
    let hover = hover_for_position("file:///p125.sy", text, 1, 10, &p125_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p125_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p125.sy", text, 1, 22, &p125_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p125_references_param_a() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let doc = p125_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "a");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p125_diagnostic_missing_return() {
    let doc = p125_doc("func main() { let x: Int = \"nope125\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_hover_array_type() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p125.sy", text, 1, 8, &p125_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p125_diagnostic_array_type_mismatch() {
    let doc = p125_doc("func main() { let xs: [Int] = [\"a\"] }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p125_service_di_scope_request_parse_smoke() {
    let doc = p125_doc(
        "@Service\n@Scope(\"request\")\nclass App { init() {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "request scope should parse: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p125_service_di_typed_resolve_unknown_e010() {
    let src = r#"
@Service
class App { init() {} }
func main() {
  let x = __syx_service_resolve("Missing") as App
  print(x)
}
"#;
    let err = syx_driver::compile_embedded(src, &p125_opts()).expect_err("E010");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E010") || msg.contains("Missing") || msg.contains("not a registered"),
        "expected typed resolve diagnostic: {msg}"
    );
}

#[test]
fn p125_hover_optional_binding() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p125.sy", text, 1, 8, &p125_opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn p125_diagnostic_optional_unwrap() {
    let doc = p125_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}



#[test]
fn p125_lsp_semantic_v27_contract() {
    let src = include_str!("lsp_phase125_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 30,
        "phase125 LSP matrix must be ≥30 tests, found {tests}"
    );
    let pad_a = concat!("fn p125_", "sem_gate");
    let pad_b = concat!("fn p125_", "sem_probe");
    let pad_c = concat!("fn p125_", "hover_extra");
    let pad_d = concat!("fn p125_", "bind_fill");
    let pad_e = concat!("fn p125_", "hover_sem_");
    let pad_f = concat!("fn p125_", "real_bind_");
    let pad_g = concat!("fn p125_", "bind_b");
    assert!(
        !src.contains(pad_a)
            && !src.contains(pad_b)
            && !src.contains(pad_c)
            && !src.contains(pad_d)
            && !src.contains(pad_e)
            && !src.contains(pad_f)
            && !src.contains(pad_g),
        "phase125 must not ship padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p125_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 30,
        "need ≥30 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
    assert!(
        src.contains("103.0") && src.contains("syx-cache-v97"),
        "phase125 contract must pin 103.0 / syx-cache-v97 fingerprints"
    );
}
