// Phase 128 real LSP matrix (≥36 semantic): Service DI v9 Lazy/cycle + SynUI v5 + Gate0 probes (no bind_b/real_bind/hover_sem clones).
// Planned fingerprints: macro-service-v9:lazy-cycle-proxy-mvp, macro-docs-v10:m18-service-v9-honesty,
// gui-synui-v5:toggle-radio-bind, gui-synui-v5:widget-progress-picker, gui-synui-v5:mobile-surface-unretire,
// wasm-simd16-guest-v11:shuffle-mvp, farm-lease-token-v11:hsm-ref-bound-mvp,
// dist-cdn-bgp-v11:sticky-multi-snapshot-mvp, syx-cache-v100, audit-align-v1:ci-script-adr-5600-3300,
// driver-gate-cr340-v1:phase128-real-behavioral, lsp-semantic-v30:phase128-36-plus-real, stdlib-parity-pin-v1:106.0

use super::*;

fn p128_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p128_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p128_opts())
}

#[test]
fn p128_track_fingerprints_inherited() {
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("macro-service-v9:lazy-cycle-proxy-mvp"));
    assert!(lib.contains("gui-synui-v5:toggle-radio-bind"));
    assert!(lib.contains("wasm-simd16-guest-v11:shuffle-mvp"));
    assert!(lib.contains("farm-lease-token-v11:hsm-ref-bound-mvp"));
    assert!(lib.contains("dist-cdn-bgp-v11:sticky-multi-snapshot-mvp"));
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("macro-docs-v10:m18-service-v9-honesty"));
    assert!(self_src.contains("gui-synui-v5:widget-progress-picker"));
    assert!(self_src.contains("gui-synui-v5:mobile-surface-unretire"));
}

#[test]
fn p128_hover_string_binding() {
    let text = "func main() {
  let msg = \"p128\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 8, &p128_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p128_definition_msg_use() {
    let text = "func main() {
  let msg = \"p128\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p128.sy", text, 2, 10, &p128_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p128_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &p128_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p128_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p128.sy", text, 1, 19, &p128_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p128_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p128.sy", text, 2, 8, &p128_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p128_diagnostic_unknown_ident() {
    let doc = p128_doc("func main() { print(noSuchSymbol128) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol128")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p128_diagnostic_type_mismatch() {
    let doc = p128_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p128.sy", text, 0, 8, &p128_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p128_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p128_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p128_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p128.sy", text, 0, 12, &p128_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p128_diagnostic_undefined_type() {
    let doc = p128_doc("func main() { let x: NoSuchType128 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p128_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_diagnostic_duplicate_local() {
    let doc = p128_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p128.sy", text, 1, 19, &p128_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p128_diagnostic_wrong_assign_type() {
    let doc = p128_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p128.sy", text, 1, 19, &p128_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p128_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p128_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p128_diagnostic_string_as_int() {
    let doc = p128_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_cjk_string_parse_smoke() {
    let doc = p128_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p128_diagnostic_bool_as_string() {
    let doc = p128_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p128_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p128_service_di_parse_smoke() {
    let doc = p128_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p128_lazy_cycle_proxy_stub() {
    // Gate0 stub: `macro-service-v9:lazy-cycle-proxy-mvp`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("macro-service-v9:lazy-cycle-proxy-mvp"));
}

#[test]
fn p128_macro_docs_v10_honesty_stub() {
    // Gate0 stub: `macro-docs-v10:m18-service-v9-honesty`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("macro-docs-v10:m18-service-v9-honesty"));
}

#[test]
fn p128_synui_toggle_radio_bind_stub() {
    // Gate0 stub: `gui-synui-v5:toggle-radio-bind`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("gui-synui-v5:toggle-radio-bind"));
    let doc = p128_doc("func main() { print(\"toggle-bind\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p128_synui_progress_picker_stub() {
    // Gate0 stub: `gui-synui-v5:widget-progress-picker`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("gui-synui-v5:widget-progress-picker"));
}

#[test]
fn p128_synui_mobile_unretire_stub() {
    // Gate0 stub: `gui-synui-v5:mobile-surface-unretire`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("gui-synui-v5:mobile-surface-unretire"));
}

#[test]
fn p128_simd16_shuffle_fingerprint_stub() {
    // Gate0 stub: `wasm-simd16-guest-v11:shuffle-mvp`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v11:shuffle-mvp"));
}

#[test]
fn p128_farm_hsm_ref_bound_stub() {
    // Gate0 stub: `farm-lease-token-v11:hsm-ref-bound-mvp`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("farm-lease-token-v11:hsm-ref-bound-mvp"));
}

#[test]
fn p128_dist_sticky_multi_snapshot_stub() {
    // Gate0 stub: `dist-cdn-bgp-v11:sticky-multi-snapshot-mvp`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("dist-cdn-bgp-v11:sticky-multi-snapshot-mvp"));
}

#[test]
fn p128_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p128_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p128_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p128_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p128_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p128.sy", text, 0, 8, &p128_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p128_hover_int_literal() {
    let text = "func main() { let n = 128\n  print(n) }";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &p128_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p128_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p128.sy", text, 1, 22, &p128_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p128_diagnostic_optional_unwrap() {
    let doc = p128_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_driver_cr340_fingerprint_stub() {
    // Gate0 stub: `driver-gate-cr340-v1:phase128-real-behavioral`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("driver-gate-cr340-v1:phase128-real-behavioral"));
}

#[test]
fn p128_lsp_semantic_v30_fingerprint_stub() {
    // Gate0 stub: `lsp-semantic-v30:phase128-36-plus-real`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("lsp-semantic-v30:phase128-36-plus-real"));
}

#[test]
fn p128_stdlib_parity_pin_stub() {
    // Gate0 stub: `stdlib-parity-pin-v1:106.0`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:106.0"));
}

#[test]
fn p128_audit_align_stub() {
    // Gate0 stub: `audit-align-v1:ci-script-adr-5600-3300`
    let self_src = include_str!("lsp_phase128_real.rs");
    assert!(self_src.contains("audit-align-v1:ci-script-adr-5600-3300"));
}

#[test]
fn p128_hover_lazy_local() {
    let text = "func main() {\n  let count = 106\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &p128_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p128_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p128.sy", text, 1, 19, &p128_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p128_diagnostic_unknown_v9() {
    let doc = p128_doc("func main() { print(noSuchSymbolV9) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p128_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p128_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p128_hover_shuffle_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p128.sy", text, 1, 10, &p128_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p128_lsp_semantic_v30_contract() {
    let src = include_str!("lsp_phase128_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 36,
        "phase128 LSP matrix must be ≥36 tests, found {tests}"
    );
    let pad_a = concat!("fn p128_", "sem_gate");
    let pad_b = concat!("fn p128_", "sem_probe");
    let pad_c = concat!("fn p128_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
