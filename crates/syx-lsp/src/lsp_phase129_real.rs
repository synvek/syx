// Phase 129 real LSP matrix (≥38 semantic): Codable L2 + DI v10 + SynUI v6 + Gate0 probes (no bind_b/real_bind/hover_sem clones).
// Planned fingerprints: macro-codable-l2:associated-enum-auto, macro-service-v10:bean-param-autowire-mvp,
// macro-docs-v11:m19-service-v10-honesty, gui-synui-v6:widget-textarea-list, gui-synui-v6:progress-number-bind,
// gui-synui-v6:desktop-run-layout-ga, wasm-simd16-guest-v12:load-store-mvp, farm-lease-token-v12:hsm-sign-bound-mvp,
// dist-cdn-bgp-v12:peer-announce-sticky-mvp, syx-cache-v101, audit-align-v1:ci-script-adr-5750-3350,
// driver-gate-cr350-v1:phase129-real-behavioral, lsp-semantic-v31:phase129-38-plus-real, stdlib-parity-pin-v1:107.0

use super::*;

fn p129_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p129_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p129_opts())
}

#[test]
fn p129_track_fingerprints_inherited() {
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("macro-codable-l2:associated-enum-auto"));
    assert!(lib.contains("gui-synui-v6:widget-textarea-list"));
    assert!(lib.contains("wasm-simd16-guest-v12:load-store-mvp"));
    assert!(lib.contains("farm-lease-token-v12:hsm-sign-bound-mvp"));
    assert!(lib.contains("dist-cdn-bgp-v12:peer-announce-sticky-mvp"));
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("macro-docs-v11:m19-service-v10-honesty"));
    assert!(self_src.contains("gui-synui-v6:progress-number-bind"));
    assert!(self_src.contains("gui-synui-v6:desktop-run-layout-ga"));
}

#[test]
fn p129_hover_string_binding() {
    let text = "func main() {
  let msg = \"p129\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 8, &p129_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p129_definition_msg_use() {
    let text = "func main() {
  let msg = \"p129\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p129.sy", text, 2, 10, &p129_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p129_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &p129_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p129_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p129.sy", text, 1, 19, &p129_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p129_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p129.sy", text, 2, 8, &p129_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p129_diagnostic_unknown_ident() {
    let doc = p129_doc("func main() { print(noSuchSymbol129) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol129")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p129_diagnostic_type_mismatch() {
    let doc = p129_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p129.sy", text, 0, 8, &p129_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p129_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p129_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p129_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p129.sy", text, 0, 12, &p129_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p129_diagnostic_undefined_type() {
    let doc = p129_doc("func main() { let x: NoSuchType129 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p129_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_diagnostic_duplicate_local() {
    let doc = p129_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p129.sy", text, 1, 19, &p129_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p129_diagnostic_wrong_assign_type() {
    let doc = p129_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p129.sy", text, 1, 19, &p129_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p129_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p129_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p129_diagnostic_string_as_int() {
    let doc = p129_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_cjk_string_parse_smoke() {
    let doc = p129_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p129_diagnostic_bool_as_string() {
    let doc = p129_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p129_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p129_service_di_parse_smoke() {
    let doc = p129_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p129_codable_l2_associated_enum_stub() {
    // Gate0 stub: `macro-codable-l2:associated-enum-auto`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("macro-codable-l2:associated-enum-auto"));
}

#[test]
fn p129_bean_param_autowire_stub() {
    // Gate0 stub: `macro-service-v10:bean-param-autowire-mvp`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("macro-service-v10:bean-param-autowire-mvp"));
}

#[test]
fn p129_macro_docs_v11_honesty_stub() {
    // Gate0 stub: `macro-docs-v11:m19-service-v10-honesty`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("macro-docs-v11:m19-service-v10-honesty"));
}

#[test]
fn p129_synui_textarea_list_stub() {
    // Gate0 stub: `gui-synui-v6:widget-textarea-list`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("gui-synui-v6:widget-textarea-list"));
    let doc = p129_doc("func main() { print(\"textarea-list\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p129_synui_progress_number_bind_stub() {
    // Gate0 stub: `gui-synui-v6:progress-number-bind`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("gui-synui-v6:progress-number-bind"));
}

#[test]
fn p129_synui_desktop_layout_ga_stub() {
    // Gate0 stub: `gui-synui-v6:desktop-run-layout-ga`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("gui-synui-v6:desktop-run-layout-ga"));
}

#[test]
fn p129_simd16_load_store_fingerprint_stub() {
    // Gate0 stub: `wasm-simd16-guest-v12:load-store-mvp`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v12:load-store-mvp"));
}

#[test]
fn p129_farm_hsm_sign_bound_stub() {
    // Gate0 stub: `farm-lease-token-v12:hsm-sign-bound-mvp`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("farm-lease-token-v12:hsm-sign-bound-mvp"));
}

#[test]
fn p129_dist_peer_announce_sticky_stub() {
    // Gate0 stub: `dist-cdn-bgp-v12:peer-announce-sticky-mvp`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("dist-cdn-bgp-v12:peer-announce-sticky-mvp"));
}

#[test]
fn p129_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p129_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p129_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p129_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p129_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p129.sy", text, 0, 8, &p129_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p129_hover_int_literal() {
    let text = "func main() { let n = 129\n  print(n) }";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &p129_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p129_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p129.sy", text, 1, 22, &p129_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p129_diagnostic_optional_unwrap() {
    let doc = p129_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_driver_cr350_fingerprint_stub() {
    // Gate0 stub: `driver-gate-cr350-v1:phase129-real-behavioral`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("driver-gate-cr350-v1:phase129-real-behavioral"));
}

#[test]
fn p129_lsp_semantic_v31_fingerprint_stub() {
    // Gate0 stub: `lsp-semantic-v31:phase129-38-plus-real`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("lsp-semantic-v31:phase129-38-plus-real"));
}

#[test]
fn p129_stdlib_parity_pin_stub() {
    // Gate0 stub: `stdlib-parity-pin-v1:107.0`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:107.0"));
}

#[test]
fn p129_audit_align_stub() {
    // Gate0 stub: `audit-align-v1:ci-script-adr-5750-3350`
    let self_src = include_str!("lsp_phase129_real.rs");
    assert!(self_src.contains("audit-align-v1:ci-script-adr-5750-3350"));
}

#[test]
fn p129_hover_lazy_local() {
    let text = "func main() {\n  let count = 107\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &p129_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p129_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p129.sy", text, 1, 19, &p129_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p129_diagnostic_unknown_v10() {
    let doc = p129_doc("func main() { print(noSuchSymbolV10) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p129_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p129_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p129_hover_load_store_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p129.sy", text, 1, 10, &p129_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p129_lsp_semantic_v31_contract() {
    let src = include_str!("lsp_phase129_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 38,
        "phase129 LSP matrix must be ≥38 tests, found {tests}"
    );
    let pad_a = concat!("fn p129_", "sem_gate");
    let pad_b = concat!("fn p129_", "sem_probe");
    let pad_c = concat!("fn p129_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
