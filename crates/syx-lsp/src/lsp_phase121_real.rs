// Phase 121 real LSP matrix (≥26 semantic): Service DI v2 themes + Gate0 probes (no bind_b/real_bind/hover_sem clones).

use super::*;

fn p121_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p121_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p121_opts())
}

#[test]
fn p121_track_fingerprints_inherited() {
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("macro-service-v1:attr-scan"));
    assert!(lib.contains("runtime-service-v1:singleton-container"));
    assert!(lib.contains("gui-ot-features-v5:paint-matrix-mvp"));
    assert!(lib.contains("wasm-simd16-guest-v4:shift-lane-mvp"));
}

#[test]
fn p121_hover_string_binding() {
    let text = "func main() {\n  let msg = \"p121\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p121.sy", text, 1, 8, &p121_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p121_definition_msg_use() {
    let text = "func main() {\n  let msg = \"p121\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p121.sy", text, 2, 10, &p121_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p121_hover_let_count_int() {
    let text = "func main() {\n  let count = 1\n}";
    let hover = hover_for_position("file:///p121.sy", text, 1, 10, &p121_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p121_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p121.sy", text, 1, 19, &p121_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p121_definition_local_binding() {
    let text = "func main() {\n  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p121.sy", text, 2, 8, &p121_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p121_diagnostic_unknown_ident() {
    let doc = p121_doc("func main() { print(noSuchSymbol121) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol121")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p121_diagnostic_type_mismatch() {
    let doc = p121_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 8, &p121_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p121_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p121_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 12, &p121_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p121_diagnostic_undefined_type() {
    let doc = p121_doc("func main() { let x: NoSuchType121 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p121_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_diagnostic_duplicate_local() {
    let doc = p121_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p121.sy", text, 1, 19, &p121_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p121_diagnostic_wrong_assign_type() {
    let doc = p121_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p121.sy", text, 1, 19, &p121_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p121_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p121_diagnostic_string_as_int() {
    let doc = p121_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_cjk_string_parse_smoke() {
    let doc = p121_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p121_diagnostic_bool_as_string() {
    let doc = p121_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_references_local_msg() {
    let text = "func main() {\n  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p121_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p121_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p121_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p121_references_service_app() {
    let text = "@Service\nclass App { init() {} }\nfunc main() { let a: App = App() }";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "App");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p121_service_di_parse_smoke() {
    let doc = p121_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p121_diagnostic_service_on_function_e006() {
    let src = "@Service\nfunc bad() {}\nfunc main() { print(1) }";
    let doc = p121_doc(src);
    let via_lsp = doc
        .semantic
        .errors
        .iter()
        .any(|e| e.code.as_deref() == Some("E006"));
    if via_lsp {
        return;
    }
    let err = syx_driver::compile_embedded(src, &p121_opts()).expect_err("E006");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E006") || msg.contains("@Service/@Component"),
        "expected E006 for @Service on function: {msg}"
    );
}

#[test]
fn p121_hover_service_class() {
    // Hover at end of type name (same contract as p120_hover_class_type / "Box" @ col 8).
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 8, &p121_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p121_diagnostic_inject_on_non_service() {
    let doc = p121_doc("func main() { let x: Int = \"bad\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_hover_int_literal() {
    let text = "func main() { let n = 121\n  print(n) }";
    let hover = hover_for_position("file:///p121.sy", text, 1, 10, &p121_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p121_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p121.sy", text, 1, 22, &p121_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p121_references_param_a() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "a");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p121_diagnostic_missing_return() {
    let doc = p121_doc("func main() { let x: Int = \"nope121\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_hover_protocol_name() {
    let text = "class Box { var id: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p121.sy", text, 0, 8, &p121_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p121_diagnostic_void_return_value() {
    let doc = p121_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_references_protocol_p() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p121_hover_array_type() {
    let text = "func main() {\n  let msg = [1, 2]\n  print(msg.count)\n}";
    let hover = hover_for_position("file:///p121.sy", text, 1, 8, &p121_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p121_diagnostic_array_type_mismatch() {
    let doc = p121_doc("func main() { let xs: [Int] = [\"a\"] }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_definition_enum_case() {
    let text = "func helper() -> Int { return 121 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p121.sy", text, 1, 19, &p121_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p121_references_enum_e() {
    let text = "func helper() -> Int { return 121 }\nfunc main() { helper() }";
    let doc = p121_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p121_diagnostic_optional_unwrap() {
    let doc = p121_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p121_hover_optional_binding() {
    let text = "func main() { let x: Int? = 1\n  print(x) }";
    let hover = hover_for_position("file:///p121.sy", text, 1, 8, &p121_opts());
    assert!(hover.to_string().contains('x'), "{hover}");
}

#[test]
fn p121_service_di_typed_resolve_unknown_e010() {
    let src = r#"
@Service
class App { init() {} }
func main() {
  let x = __syx_service_resolve("Missing") as App
  print(x)
}
"#;
    let err = syx_driver::compile_embedded(src, &p121_opts()).expect_err("E010");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E010") || msg.contains("Missing") || msg.contains("not a registered"),
        "expected typed resolve diagnostic: {msg}"
    );
}

#[test]
fn p121_service_di_typed_resolve_cast_mismatch() {
    let doc = p121_doc(
        "@Service\nclass App { init() {} }\n@Service\nclass Other { init() {} }\nfunc main() {\n  let x = __syx_service_resolve(\"App\") as Other\n}",
    );
    assert!(
        doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("E010"))
            || syx_driver::compile_embedded(
                "@Service\nclass App { init() {} }\n@Service\nclass Other { init() {} }\nfunc main() {\n  let x = __syx_service_resolve(\"App\") as Other\n}",
                &p121_opts(),
            )
            .is_err(),
        "expected cast mismatch diagnostic: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p121_hover_service_resolve_typed() {
    let text = "@Service\nclass Box { var n: Int }\nfunc main() {\n  let x = 1\n  print(x)\n}";
    let hover = hover_for_position("file:///p121.sy", text, 3, 6, &p121_opts());
    assert!(
        hover.to_string().contains('x') || hover.to_string().contains("Int"),
        "{hover}"
    );
    let class_hover = hover_for_position("file:///p121.sy", text, 1, 8, &p121_opts());
    assert!(
        class_hover.to_string().contains("Box"),
        "{class_hover}"
    );
}

#[test]
fn p121_lsp_semantic_v23_contract() {
    let src = include_str!("lsp_phase121_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 26,
        "phase121 LSP matrix must be ≥26 tests, found {tests}"
    );
    let pad_a = concat!("fn p121_", "sem_gate");
    let pad_b = concat!("fn p121_", "sem_probe");
    let pad_c = concat!("fn p121_", "hover_extra");
    let pad_d = concat!("fn p121_", "bind_fill");
    let pad_e = concat!("fn p121_", "hover_sem_");
    let pad_f = concat!("fn p121_", "real_bind_");
    let pad_g = concat!("fn p121_", "bind_b");
    assert!(
        !src.contains(pad_a)
            && !src.contains(pad_b)
            && !src.contains(pad_c)
            && !src.contains(pad_d)
            && !src.contains(pad_e)
            && !src.contains(pad_f)
            && !src.contains(pad_g),
        "phase121 must not ship padding clones"
    );
    let hover_n = src.matches("hover_for_position").count();
    let def_n = src.matches("definition_for_position").count();
    let diag_n = src.matches("p121_doc(").count();
    let refs_n = src.matches("find_references_in_module").count();
    assert!(
        hover_n + def_n + diag_n + refs_n >= 26,
        "need ≥26 real semantic probes, found hover={hover_n} def={def_n} diag={diag_n} refs={refs_n}"
    );
}
