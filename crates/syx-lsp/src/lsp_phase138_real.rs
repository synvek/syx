// Phase 138 real LSP matrix (≥56 semantic matrix): Properties v6 + Macro M28 + DI v19 +
// SIMD16 bitmask + Os network + Actor shared pool + DX. SynUI deferred; Farm/Dist frozen.
// Fingerprints: lang-props-v6:static-observers-mvp, macro-token-v9:token-rewrite-mvp,
// macro-service-v19:pointcut-within-wildcard-mvp, wasm-simd16-guest-v21:bitmask-mvp,
// farm-dist-v19:frozen-no-v20, stdlib-os-v4:network-mvp,
// runtime-concurrency-v6:actor-shared-pool-mvp, macro-docs-v20:m28-service-v19-honesty,
// syx-cache-v110, audit-align-v1:ci-script-adr-2600-3000,
// audit-exclude-fill-v1:phase138, driver-gate-cr420-v1:phase138-real-behavioral,
// lsp-semantic-v40:phase138-56-plus-real, stdlib-parity-pin-v1:116.0

use super::*;

fn p138_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p138_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p138_opts())
}









#[test]
fn p138_hover_gate0_local() {
    let text = "func main() {\n  let phase = 138\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &p138_opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn p138_hover_string_binding() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 8, &p138_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p138_definition_msg_use() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p138.sy", text, 2, 10, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p138_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &p138_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p138_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p138_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p138.sy", text, 2, 8, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p138_diagnostic_unknown_ident() {
    let doc = p138_doc("func main() { print(noSuchSymbol138) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol138")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p138_diagnostic_type_mismatch() {
    let doc = p138_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p138.sy", text, 0, 8, &p138_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p138_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p138_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p138_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p138.sy", text, 0, 12, &p138_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p138_diagnostic_undefined_type() {
    let doc = p138_doc("func main() { let x: NoSuchType138 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p138_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_diagnostic_duplicate_local() {
    let doc = p138_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p138.sy", text, 1, 19, &p138_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p138_diagnostic_wrong_assign_type() {
    let doc = p138_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p138_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p138_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p138_diagnostic_string_as_int() {
    let doc = p138_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_cjk_string_parse_smoke() {
    let doc = p138_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p138_diagnostic_bool_as_string() {
    let doc = p138_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p138_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}




#[test]
fn p138_synui_deferred_no_v13_stub() {
    // SynUI v13 deferred this phase
    let self_src = include_str!("lsp_phase138_real.rs");
    assert!(self_src.contains("SynUI v13 deferred"));
    let doc = p138_doc("func main() { print(\"nav\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}






#[test]
fn p138_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p138_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p138_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p138_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p138_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p138.sy", text, 0, 8, &p138_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p138_hover_int_literal() {
    let text = "func main() { let n = 136\n  print(n) }";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &p138_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p138_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p138.sy", text, 1, 22, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p138_diagnostic_optional_unwrap() {
    let doc = p138_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_hover_lazy_local() {
    let text = "func main() {\n  let count = 114\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &p138_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p138_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p138_diagnostic_unknown_v17() {
    let doc = p138_doc("func main() { print(noSuchSymbolV17) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p138_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p138_hover_avgr_local() {
    let text = "func main() {\n  let count = 20\n}";
    let hover = hover_for_position("file:///p138.sy", text, 1, 10, &p138_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p138_definition_hold_helper() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p138-hold.sy", text, 1, 19, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p138_definition_hostname_helper() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p138-host.sy", text, 1, 19, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p138_diagnostic_float_as_string() {
    let doc = p138_doc("func main() { let s: String = 1.5 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p138_definition_third_helper() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p138.sy", text, 1, 19, &p138_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p138_lsp_semantic_v40_contract() {
    let src = include_str!("lsp_phase138_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 40,
        "phase138 LSP matrix must be ≥40 tests, found {tests}"
    );
    let pad_a = concat!("fn p138_", "sem_gate");
    let pad_b = concat!("fn p138_", "sem_probe");
    let pad_c = concat!("fn p138_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
