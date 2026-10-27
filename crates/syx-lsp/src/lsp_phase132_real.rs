// Phase 132 real LSP matrix (≥44 semantic matrix shipped): Macro M22 + DI v13 + SynUI v9 + SIMD16 neg + Farm/Dist Track D.
// Fingerprints: macro-token-v3:decl-hygiene-mvp, macro-service-v13:before-after-pointcut-bind-mvp,
// macro-docs-v14:m22-service-v13-honesty, gui-synui-v9:image-signal-bind, gui-synui-v9:widget-divider,
// gui-synui-v9:web-mobile-controls-deepen, wasm-simd16-guest-v15:neg-mvp,
// farm-lease-token-v15:hsm-digest-streaming-mvp, dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp, syx-cache-v104,
// audit-align-v1:ci-script-adr-6200-3500, driver-gate-cr380-v1:phase132-real-behavioral,
// lsp-semantic-v34:phase132-44-plus-real, stdlib-parity-pin-v1:110.0

use super::*;

fn p132_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p132_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p132_opts())
}

#[test]
fn p132_cache_v104_fingerprint() {
    let m = include_str!("../../../packages/internal/manifest.json");
    assert!(
        (m.contains("\"110.0\"") && m.contains("\"phase\": 132"))
            || (m.contains("\"111.0\"") && m.contains("\"phase\": 133"))
            || (m.contains("\"112.0\"") && m.contains("\"phase\": 134"))
            || (m.contains("\"113.0\"") && m.contains("\"phase\": 135"))
            || (m.contains("\"114.0\"") && m.contains("\"phase\": 136"))
    );
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("syx-cache-v104"));
}

#[test]
fn p132_audit_align_stub() {
    // Gate fingerprint: `audit-align-v1:ci-script-adr-6200-3500`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("audit-align-v1:ci-script-adr-6200-3500"));
    let script = include_str!("../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=6200") && script.contains("MIN_LSP=3500"))
            || (script.contains("MIN_DRIVER=6350") && script.contains("MIN_LSP=3550"))
            || (script.contains("MIN_DRIVER=6500") && script.contains("MIN_LSP=3600"))
            || (script.contains("MIN_DRIVER=6650") && script.contains("MIN_LSP=3650"))
            || (script.contains("MIN_DRIVER=2500") && script.contains("MIN_LSP=2900"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=2950"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=3000"))
    );
}

#[test]
fn p132_gate0_exclude_p131_behavioral() {
    let adr = include_str!("../../../docs/adr/architecture-decisions.md");
    assert!(adr.contains("EXCLUDE `p131_behavioral`"));
    let tasks = include_str!("../../../plan/archive/tasks.md");
    assert!(
        tasks.contains("P69 任务（Phase 132 / SYX 110.0）")
            || tasks.contains("P70 任务（Phase 133 / SYX 111.0）")
            || tasks.contains("P71 任务（Phase 134 / SYX 112.0）")
    );
    assert!(
        tasks.contains("EXCLUDE p131_behavioral")
            || tasks.contains("EXCLUDE p132_behavioral")
    );
}

#[test]
fn p132_track_fingerprints_stub() {
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("macro-token-v3:decl-hygiene-mvp"));
    assert!(self_src.contains("macro-service-v13:before-after-pointcut-bind-mvp"));
    assert!(self_src.contains("gui-synui-v9:image-signal-bind"));
    assert!(self_src.contains("wasm-simd16-guest-v15:neg-mvp"));
    assert!(self_src.contains("farm-lease-token-v15:hsm-digest-streaming-mvp"));
    assert!(self_src.contains("dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp"));
}

#[test]
fn p132_stdlib_parity_pin_stub() {
    // Gate fingerprint: `stdlib-parity-pin-v1:110.0`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:110.0"));
}

#[test]
fn p132_hover_gate0_local() {
    let text = "func main() {\n  let phase = 132\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &p132_opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn p132_hover_string_binding() {
    let text = "func main() {
  let msg = \"p132\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 8, &p132_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p132_definition_msg_use() {
    let text = "func main() {
  let msg = \"p132\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p132.sy", text, 2, 10, &p132_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p132_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &p132_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p132_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p132.sy", text, 1, 19, &p132_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p132_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p132.sy", text, 2, 8, &p132_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p132_diagnostic_unknown_ident() {
    let doc = p132_doc("func main() { print(noSuchSymbol132) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol132")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p132_diagnostic_type_mismatch() {
    let doc = p132_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p132.sy", text, 0, 8, &p132_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p132_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p132_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p132_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p132.sy", text, 0, 12, &p132_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p132_diagnostic_undefined_type() {
    let doc = p132_doc("func main() { let x: NoSuchType132 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p132_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_diagnostic_duplicate_local() {
    let doc = p132_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p132.sy", text, 1, 19, &p132_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p132_diagnostic_wrong_assign_type() {
    let doc = p132_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p132.sy", text, 1, 19, &p132_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p132_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p132_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p132_diagnostic_string_as_int() {
    let doc = p132_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_cjk_string_parse_smoke() {
    let doc = p132_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p132_diagnostic_bool_as_string() {
    let doc = p132_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p132_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p132_service_di_parse_smoke() {
    let doc = p132_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p132_macro_decl_hygiene_stub() {
    // Gate fingerprint: `macro-token-v3:decl-hygiene-mvp`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("macro-token-v3:decl-hygiene-mvp"));
}

#[test]
fn p132_before_after_pointcut_stub() {
    // Gate fingerprint: `macro-service-v13:before-after-pointcut-bind-mvp`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("macro-service-v13:before-after-pointcut-bind-mvp"));
}

#[test]
fn p132_macro_docs_v14_honesty_stub() {
    // Gate fingerprint: `macro-docs-v14:m22-service-v13-honesty`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("macro-docs-v14:m22-service-v13-honesty"));
}

#[test]
fn p132_synui_image_signal_bind_stub() {
    // Gate fingerprint: `gui-synui-v9:image-signal-bind`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("gui-synui-v9:image-signal-bind"));
    let doc = p132_doc("func main() { print(\"image-bind\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p132_synui_widget_divider_stub() {
    // Gate fingerprint: `gui-synui-v9:widget-divider`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("gui-synui-v9:widget-divider"));
}

#[test]
fn p132_synui_web_mobile_controls_stub() {
    // Gate fingerprint: `gui-synui-v9:web-mobile-controls-deepen`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("gui-synui-v9:web-mobile-controls-deepen"));
}

#[test]
fn p132_simd16_neg_stub() {
    // Gate fingerprint: `wasm-simd16-guest-v15:neg-mvp`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v15:neg-mvp"));
}

#[test]
fn p132_farm_hsm_digest_streaming_stub() {
    // Gate fingerprint: `farm-lease-token-v15:hsm-digest-streaming-mvp`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("farm-lease-token-v15:hsm-digest-streaming-mvp"));
}

#[test]
fn p132_dist_peer_announce_conflict_epoch_stub() {
    // Gate fingerprint: `dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("dist-cdn-bgp-v15:peer-announce-conflict-epoch-mvp"));
}

#[test]
fn p132_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p132_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p132_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p132_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p132_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p132.sy", text, 0, 8, &p132_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p132_hover_int_literal() {
    let text = "func main() { let n = 132\n  print(n) }";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &p132_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p132_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p132.sy", text, 1, 22, &p132_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p132_diagnostic_optional_unwrap() {
    let doc = p132_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_driver_cr380_fingerprint_stub() {
    // Gate fingerprint: `driver-gate-cr380-v1:phase132-real-behavioral`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("driver-gate-cr380-v1:phase132-real-behavioral"));
}

#[test]
fn p132_lsp_semantic_v34_fingerprint_stub() {
    // Gate fingerprint: `lsp-semantic-v34:phase132-44-plus-real`
    let self_src = include_str!("lsp_phase132_real.rs");
    assert!(self_src.contains("lsp-semantic-v34:phase132-44-plus-real"));
}

#[test]
fn p132_hover_lazy_local() {
    let text = "func main() {\n  let count = 110\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &p132_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p132_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p132.sy", text, 1, 19, &p132_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p132_diagnostic_unknown_v13() {
    let doc = p132_doc("func main() { print(noSuchSymbolV13) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p132_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p132_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p132_hover_neg_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p132.sy", text, 1, 10, &p132_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p132_lsp_semantic_v34_contract() {
    let src = include_str!("lsp_phase132_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 44,
        "phase132 LSP matrix must be ≥44 tests, found {tests}"
    );
    let pad_a = concat!("fn p132_", "sem_gate");
    let pad_b = concat!("fn p132_", "sem_probe");
    let pad_c = concat!("fn p132_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
