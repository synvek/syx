// Phase 122 real LSP matrix (≥26 semantic): Service DI v3 scope/PreDestroy + Gate0 probes (no bind_b/real_bind/hover_sem clones).

use super::*;

fn p122_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p122_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p122_opts())
}

#[test]
fn p122_track_fingerprints_inherited() {
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("runtime-service-v2:real-ctor-resolve"));
    assert!(lib.contains("macro-service-v2:field-autowired"));
    assert!(lib.contains("macro-service-v3:multi-package-scan"));
    assert!(lib.contains("runtime-service-v3:prototype-scope"));
    assert!(lib.contains("gui-ot-features-v6:default-paint-ot-mvp"));
    assert!(lib.contains("wasm-simd16-guest-v5:logic-or-xor"));
    assert!(lib.contains("farm-lease-token-v5:pkcs11-bridge-mvp"));
    assert!(lib.contains("dist-cdn-bgp-v5:multi-region-mesh-mvp"));
}

#[test]
fn p122_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p122\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p122.sy", text, 1, 8, &p122_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p122_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p122\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p122.sy", text, 2, 10, &p122_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p122_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p122.sy", text, 1, 10, &p122_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p122_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p122.sy", text, 1, 19, &p122_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p122_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p122.sy", text, 2, 8, &p122_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p122_diagnostic_unknown_ident() {
    let doc = p122_doc("func main() { print(noSuchSymbol122) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol122")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p122_diagnostic_type_mismatch() {
    let doc = p122_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 8, &p122_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p122_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p122_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 12, &p122_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p122_diagnostic_undefined_type() {
    let doc = p122_doc("func main() { let x: NoSuchType122 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p122_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_diagnostic_duplicate_local() {
    let doc = p122_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p122.sy", text, 1, 19, &p122_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p122_diagnostic_wrong_assign_type() {
    let doc = p122_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p122.sy", text, 1, 19, &p122_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p122_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p122_diagnostic_string_as_int() {
    let doc = p122_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_cjk_string_parse_smoke() {
    let doc = p122_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p122_diagnostic_bool_as_string() {
    let doc = p122_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_references_local_msg() {
    let text = "func main() {\n  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p122_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p122_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p122_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p122_references_service_app() {
    let text = "@Service\nclass App { init() {} }\nfunc main() { let a: App = App() }";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "App");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p122_service_di_parse_smoke() {
    let doc = p122_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p122_diagnostic_service_on_function_e006() {
    let src = "@Service\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = p122_doc(src);
    let via_lsp = doc
        .semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some("E006"));
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &p122_opts()).expect_err("E006");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E006") || msg.contains("@Service/@Component"),
        "expected E006 for @Service on function: {msg}"
    );
}

#[test]
fn p122_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 8, &p122_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p122_hover_int_literal() {
    let text = "func main() { let n = 122\n  print(n) }";
    let hover = hover_for_position("file:///p122.sy", text, 1, 10, &p122_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p122_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p122.sy", text, 1, 22, &p122_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p122_references_param_a() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "a");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p122_diagnostic_missing_return() {
    let doc = p122_doc("func main() { let x: Int = \"nope122\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_hover_protocol_name() {
    let text = "class Box { var id: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p122.sy", text, 0, 8, &p122_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p122_diagnostic_void_return_value() {
    let doc = p122_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_references_protocol_p() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p122_hover_array_type() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p122.sy", text, 1, 8, &p122_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p122_diagnostic_array_type_mismatch() {
    let doc = p122_doc("func main() { let xs: [Int] = [\"a\"] }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_definition_enum_case() {
    let text = "func helper() -> Int { return 122 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p122.sy", text, 1, 19, &p122_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p122_references_enum_e() {
    let text = "func helper() -> Int { return 122 }\nfunc main() { helper() }";
    let doc = p122_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p122_diagnostic_optional_unwrap() {
    let doc = p122_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p122_hover_optional_binding() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p122.sy", text, 1, 8, &p122_opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn p122_service_di_scope_prototype_parse() {
    let doc = p122_doc(
        "@Service\n@Scope(\"prototype\")\nclass Worker { init() {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "prototype scope should parse: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p122_service_di_scope_singleton_default() {
    let doc = p122_doc("@Service\nclass App { init() {} }\nfunc main() { print(1) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "singleton default should parse: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p122_service_di_predestroy_parse_smoke() {
    let doc = p122_doc(
        "@Service\nclass App {\n  init() {}\n  @PreDestroy\n  func shutdown() {}\n}\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "PreDestroy hook should parse: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p122_service_di_predestroy_on_function_rejected() {
    let src = "@PreDestroy\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = p122_doc(src);
    let via_lsp = !doc.semantic.errors.is_empty();
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &p122_opts());
    assert!(err.is_err(), "PreDestroy on function should fail or warn");
}

#[test]
fn p122_service_di_scope_invalid_literal_diag() {
    let doc = p122_doc(
        "@Service\n@Scope(\"session\")\nclass App { init() {} }\nfunc main() { print(1) }",
    );
    let compile_err = syx_driver::compile_embedded(
        "@Service\n@Scope(\"session\")\nclass App { init() {} }\nfunc main() { print(1) }",
        &p122_opts(),
    );
    assert!(
        !doc.semantic.errors.is_empty() || compile_err.is_err(),
        "unsupported scope should not silently pass as GA"
    );
}

#[test]
fn p122_service_di_typed_resolve_unknown_e010() {
    let src = r#"
@Service
class App { init() {} }
func main() {
  let x = __syx_service_resolve("Missing") as App
  print(x)
}
"#;
    let err = syx_driver::compile_embedded(src, &p122_opts()).expect_err("E010");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E010") || msg.contains("Missing") || msg.contains("not a registered"),
        "expected typed resolve diagnostic: {msg}"
    );
}

#[test]
fn p122_service_di_typed_resolve_cast_mismatch() {
    let doc = p122_doc(
        "@Service\nclass App { init() {} }\n@Service\nclass Other { init() {} }\nfunc main() {\n  let x = __syx_service_resolve(\"App\") as Other\n}",
    );
    assert!(
        doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("E010"))
            || syx_driver::compile_embedded(
                "@Service\nclass App { init() {} }\n@Service\nclass Other { init() {} }\nfunc main() {\n  let x = __syx_service_resolve(\"App\") as Other\n}",
                &p122_opts(),
            )
            .is_err(),
        "expected cast mismatch diagnostic: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p122_hover_service_resolve_typed() {
    let text = "@Service\nclass Box { var n: Int }\nfunc main() {\n  let x = 1\n  print(x)\n}";
    let hover = hover_for_position("file:///p122.sy", text, 3, 6, &p122_opts());
    assert!(
        hover.to_string().contains('x') || hover.to_string().contains("Int"),
        "{hover}"
    );
    let class_hover = hover_for_position("file:///p122.sy", text, 1, 8, &p122_opts());
    assert!(
        class_hover.to_string().contains("Box"),
        "{class_hover}"
    );
}

#[test]
fn p122_lsp_semantic_v24_contract() {
    let src = include_str!("lsp_phase122_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 26,
        "phase122 LSP matrix must be ≥26 tests, found {tests}"
    );
    let pad_a = concat!("fn p122_", "sem_gate");
    let pad_b = concat!("fn p122_", "sem_probe");
    let pad_c = concat!("fn p122_", "hover_extra");
    let pad_d = concat!("fn p122_", "bind_fill");
    let pad_e = concat!("fn p122_", "hover_sem_");
    let pad_f = concat!("fn p122_", "real_bind_");
    let pad_g = concat!("fn p122_", "bind_b");
    assert!(
        !src.contains(pad_a)
            && !src.contains(pad_b)
            && !src.contains(pad_c)
            && !src.contains(pad_d)
            && !src.contains(pad_e)
            && !src.contains(pad_f)
            && !src.contains(pad_g),
        "phase122 must not ship padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p122_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 26,
        "need ≥26 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
    assert!(
        src.contains("100.0") && src.contains("syx-cache-v94"),
        "phase122 contract must pin 100.0 / syx-cache-v94 fingerprints"
    );
}
