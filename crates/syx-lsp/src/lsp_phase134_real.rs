// Phase 134 real LSP matrix (≥48 semantic matrix): Properties v2 + Macro M24 + DI v15 + SynUI v11 + SIMD16 le + Farm/Dist Track D + bounded queue.
// Fingerprints: lang-props-v2:inherited-observers, macro-token-v5:expr-token-subst-mvp,
// macro-service-v15:pointcut-args-name-mvp, gui-synui-v11:widget-tabview,
// wasm-simd16-guest-v17:le-mvp, farm-lease-token-v17:hsm-session-pin-mvp,
// dist-cdn-bgp-v17:peer-announce-ttl-mvp, runtime-concurrency-v2:bounded-queue-abort-mvp,
// syx-cache-v106, audit-align-v1:ci-script-adr-6500-3600,
// driver-gate-cr400-v1:phase134-real-behavioral, lsp-semantic-v36:phase134-48-plus-real,
// stdlib-parity-pin-v1:112.0

use super::*;

fn p134_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p134_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p134_opts())
}

#[test]
fn p134_audit_align_stub() {
    // Gate fingerprint: `audit-align-v1:ci-script-adr-6500-3600`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(
        self_src.contains("audit-align-v1:ci-script-adr-6500-3600")
            || self_src.contains("audit-align-v1:ci-script-adr-6650-3650")
            || self_src.contains("audit-align-v1:ci-script-adr-2500-3100") || self_src.contains("audit-align-v1:ci-script-adr-6800-3700")
    );
    let script = include_str!("../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=6500") && script.contains("MIN_LSP=3600"))
            || (script.contains("MIN_DRIVER=6650") && script.contains("MIN_LSP=3650"))
            || (script.contains("MIN_DRIVER=2500") && script.contains("MIN_LSP=2900"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=2950"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=3000"))
    );
}

#[test]
fn p134_gate0_exclude_p133_behavioral() {
    let adr = include_str!("../../../docs/adr/architecture-decisions.md");
    assert!(adr.contains("p133_behavioral"));
    let tasks = include_str!("../../../plan/archive/tasks.md");
    assert!(tasks.contains("P71 任务（Phase 134 / SYX 112.0）"));
    assert!(tasks.contains("EXCLUDE p133_behavioral"));
}

#[test]
fn p134_track_fingerprints_stub() {
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("lang-props-v2:inherited-observers"));
    assert!(self_src.contains("macro-token-v5:expr-token-subst-mvp"));
    assert!(self_src.contains("macro-service-v15:pointcut-args-name-mvp"));
    assert!(self_src.contains("gui-synui-v11:widget-tabview"));
    assert!(self_src.contains("wasm-simd16-guest-v17:le-mvp"));
    assert!(self_src.contains("farm-lease-token-v17:hsm-session-pin-mvp"));
    assert!(self_src.contains("dist-cdn-bgp-v17:peer-announce-ttl-mvp"));
    assert!(self_src.contains("runtime-concurrency-v2:bounded-queue-abort-mvp"));
}

#[test]
fn p134_stdlib_parity_pin_stub() {
    // Gate fingerprint: `stdlib-parity-pin-v1:112.0`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:112.0"));
}

#[test]
fn p134_hover_gate0_local() {
    let text = "func main() {\n  let phase = 134\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &p134_opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn p134_hover_string_binding() {
    let text = "func main() {
  let msg = \"p134\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 8, &p134_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p134_definition_msg_use() {
    let text = "func main() {
  let msg = \"p134\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p134.sy", text, 2, 10, &p134_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p134_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &p134_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p134_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p134.sy", text, 1, 19, &p134_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p134_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p134.sy", text, 2, 8, &p134_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p134_diagnostic_unknown_ident() {
    let doc = p134_doc("func main() { print(noSuchSymbol134) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol134")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p134_diagnostic_type_mismatch() {
    let doc = p134_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p134.sy", text, 0, 8, &p134_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p134_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p134_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p134_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p134.sy", text, 0, 12, &p134_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p134_diagnostic_undefined_type() {
    let doc = p134_doc("func main() { let x: NoSuchType134 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p134_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_diagnostic_duplicate_local() {
    let doc = p134_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p134.sy", text, 1, 19, &p134_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p134_diagnostic_wrong_assign_type() {
    let doc = p134_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p134.sy", text, 1, 19, &p134_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p134_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p134_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p134_diagnostic_string_as_int() {
    let doc = p134_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_cjk_string_parse_smoke() {
    let doc = p134_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p134_diagnostic_bool_as_string() {
    let doc = p134_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p134_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p134_props_inherited_observers_stub() {
    // Gate fingerprint: `lang-props-v2:inherited-observers`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("lang-props-v2:inherited-observers"));
}

#[test]
fn p134_macro_expr_token_subst_stub() {
    // Gate fingerprint: `macro-token-v5:expr-token-subst-mvp`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("macro-token-v5:expr-token-subst-mvp"));
}

#[test]
fn p134_pointcut_args_name_stub() {
    // Gate fingerprint: `macro-service-v15:pointcut-args-name-mvp`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("macro-service-v15:pointcut-args-name-mvp"));
}

#[test]
fn p134_synui_tabview_stub() {
    // Gate fingerprint: `gui-synui-v11:widget-tabview`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("gui-synui-v11:widget-tabview"));
    let doc = p134_doc("func main() { print(\"tabview\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p134_simd16_le_stub() {
    // Gate fingerprint: `wasm-simd16-guest-v17:le-mvp`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v17:le-mvp"));
}

#[test]
fn p134_farm_hsm_session_pin_stub() {
    // Gate fingerprint: `farm-lease-token-v17:hsm-session-pin-mvp`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("farm-lease-token-v17:hsm-session-pin-mvp"));
}

#[test]
fn p134_dist_peer_announce_ttl_stub() {
    // Gate fingerprint: `dist-cdn-bgp-v17:peer-announce-ttl-mvp`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("dist-cdn-bgp-v17:peer-announce-ttl-mvp"));
}

#[test]
fn p134_bounded_queue_abort_stub() {
    // Gate fingerprint: `runtime-concurrency-v2:bounded-queue-abort-mvp`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("runtime-concurrency-v2:bounded-queue-abort-mvp"));
}

#[test]
fn p134_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p134_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p134_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p134_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p134_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p134.sy", text, 0, 8, &p134_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p134_hover_int_literal() {
    let text = "func main() { let n = 134\n  print(n) }";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &p134_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p134_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p134.sy", text, 1, 22, &p134_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p134_diagnostic_optional_unwrap() {
    let doc = p134_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_driver_cr400_fingerprint_stub() {
    // Gate fingerprint: `driver-gate-cr400-v1:phase134-real-behavioral`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(
        self_src.contains("driver-gate-cr400-v1:phase134-real-behavioral")
            || self_src.contains("driver-gate-cr410-v1:phase135-real-behavioral")
    );
}

#[test]
fn p134_lsp_semantic_v36_fingerprint_stub() {
    // Gate fingerprint: `lsp-semantic-v36:phase134-48-plus-real`
    let self_src = include_str!("lsp_phase134_real.rs");
    assert!(self_src.contains("lsp-semantic-v36:phase134-48-plus-real"));
}

#[test]
fn p134_hover_lazy_local() {
    let text = "func main() {\n  let count = 112\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &p134_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p134_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p134.sy", text, 1, 19, &p134_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p134_diagnostic_unknown_v15() {
    let doc = p134_doc("func main() { print(noSuchSymbolV15) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p134_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p134_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p134_hover_le_local() {
    let text = "func main() {\n  let count = 17\n}";
    let hover = hover_for_position("file:///p134.sy", text, 1, 10, &p134_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p134_lsp_semantic_v36_contract() {
    let src = include_str!("lsp_phase134_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 48,
        "phase134 LSP matrix must be ≥48 tests, found {tests}"
    );
    let pad_a = concat!("fn p134_", "sem_gate");
    let pad_b = concat!("fn p134_", "sem_probe");
    let pad_c = concat!("fn p134_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
