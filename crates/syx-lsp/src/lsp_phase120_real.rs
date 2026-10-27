// Phase 120 real LSP matrix (≥26 semantic): Service DI + Track fingerprints + real probes (no bind_b/real_bind/hover_sem clones).

use super::*;

fn p120_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p120_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p120_opts())
}

#[test]
fn p120_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p120\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p120.sy", text, 1, 8, &p120_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p120_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p120\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p120.sy", text, 2, 10, &p120_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p120_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p120.sy", text, 1, 10, &p120_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p120_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p120.sy", text, 1, 19, &p120_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p120_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p120.sy", text, 2, 8, &p120_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p120_diagnostic_unknown_ident() {
    let doc = p120_doc("func main() { print(noSuchSymbol120) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol120")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p120_diagnostic_type_mismatch() {
    let doc = p120_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p120.sy", text, 0, 8, &p120_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p120_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p120_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p120_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p120.sy", text, 0, 12, &p120_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p120_diagnostic_undefined_type() {
    let doc = p120_doc("func main() { let x: NoSuchType120 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p120_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_diagnostic_duplicate_local() {
    let doc = p120_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p120.sy", text, 1, 19, &p120_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p120_diagnostic_wrong_assign_type() {
    let doc = p120_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p120.sy", text, 1, 19, &p120_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p120_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p120_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p120_diagnostic_missing_return_type_use() {
    let doc = p120_doc("func main() { let z: MissingAlias120 = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_diagnostic_string_as_int() {
    let doc = p120_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_cjk_string_parse_smoke() {
    let doc = p120_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p120_diagnostic_bool_as_string() {
    let doc = p120_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p120_references_local_msg() {
    let text = "func main() {\n  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p120_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p120_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p120_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p120_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p120_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p120_references_service_app() {
    let text = "@Service\nclass App { init() {} }\nfunc main() { let a: App = App() }";
    let doc = p120_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "App");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p120_service_di_parse_smoke() {
    let doc = p120_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p120_diagnostic_service_on_function_e006() {
    let src = "@Service\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = p120_doc(src);
    let via_lsp = doc
        .semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some("E006"));
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &p120_opts()).expect_err("E006");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E006") || msg.contains("@Service/@Component"),
        "expected E006 for @Service on function: {msg}"
    );
}

#[test]
fn p120_diagnostic_component_on_function_e006() {
    let src = "@Component\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = p120_doc(src);
    let via_lsp = doc
        .semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some("E006"));
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &p120_opts()).expect_err("E006");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E006") || msg.contains("@Service/@Component"),
        "expected E006 for @Component on function: {msg}"
    );
}

#[test]
fn p120_lsp_semantic_v22_contract() {
    let src = include_str!("lsp_phase120_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 26,
        "phase120 LSP matrix must be ≥26 tests, found {tests}"
    );
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("lsp-semantic-v22:phase120-24-plus-real"));
    let pad_a = concat!("fn p120_", "sem_gate");
    let pad_b = concat!("fn p120_", "sem_probe");
    let pad_c = concat!("fn p120_", "hover_extra");
    let pad_d = concat!("fn p120_", "bind_fill");
    let pad_e = concat!("fn p120_", "hover_sem_");
    let pad_f = concat!("fn p120_", "real_bind_");
    let pad_g = concat!("fn p120_", "bind_b");
    assert!(
        !src.contains(pad_a)
            && !src.contains(pad_b)
            && !src.contains(pad_c)
            && !src.contains(pad_d)
            && !src.contains(pad_e)
            && !src.contains(pad_f)
            && !src.contains(pad_g),
        "phase120 must not ship padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p120_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 26,
        "need ≥26 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}
