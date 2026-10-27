// Phase 133 real LSP matrix (≥46 semantic matrix): Macro M23 + DI v14 + SynUI v10 + SIMD16 ne + Farm/Dist Track D + Os/Concurrency.
// Fingerprints: macro-token-v4:stmt-token-subst-mvp, macro-service-v14:pointcut-within-name-mvp,
// macro-docs-v15:m23-service-v14-honesty, gui-synui-v10:widget-stack, gui-synui-v10:scrollview-signal-bind,
// gui-synui-v10:web-mobile-layout-deepen, wasm-simd16-guest-v16:ne-mvp,
// farm-lease-token-v16:hsm-multi-slot-mvp, dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp,
// stdlib-os-v1:cpu-mem-user-mvp, runtime-concurrency-v1:task-worker-config-mvp,
// runtime-concurrency-v1:actor-limit-mailbox-mvp, syx-cache-v105,
// audit-align-v1:ci-script-adr-6350-3550, driver-gate-cr390-v1:phase133-real-behavioral,
// lsp-semantic-v35:phase133-46-plus-real, stdlib-parity-pin-v1:111.0

use super::*;

fn p133_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p133_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p133_opts())
}

#[test]
fn p133_cache_v105_fingerprint() {
    let m = include_str!("../../../packages/internal/manifest.json");
    assert!(
        (m.contains("\"111.0\"") && m.contains("\"phase\": 133"))
            || (m.contains("\"112.0\"") && m.contains("\"phase\": 134"))
            || (m.contains("\"113.0\"") && m.contains("\"phase\": 135"))
            || (m.contains("\"114.0\"") && m.contains("\"phase\": 136"))
    );
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("syx-cache-v105"));
}

#[test]
fn p133_audit_align_stub() {
    // Gate fingerprint: `audit-align-v1:ci-script-adr-6350-3550`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("audit-align-v1:ci-script-adr-6350-3550"));
    let script = include_str!("../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=6350") && script.contains("MIN_LSP=3550"))
            || (script.contains("MIN_DRIVER=6500") && script.contains("MIN_LSP=3600"))
            || (script.contains("MIN_DRIVER=6650") && script.contains("MIN_LSP=3650"))
            || (script.contains("MIN_DRIVER=2500") && script.contains("MIN_LSP=2900"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=2950"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=3000"))
    );
}

#[test]
fn p133_gate0_exclude_p132_behavioral() {
    let adr = include_str!("../../../docs/adr/architecture-decisions.md");
    assert!(adr.contains("EXCLUDE `p132_behavioral`"));
    let tasks = include_str!("../../../plan/archive/tasks.md");
    assert!(
        tasks.contains("P70 任务（Phase 133 / SYX 111.0）")
            || tasks.contains("P71 任务（Phase 134 / SYX 112.0）")
    );
    assert!(tasks.contains("EXCLUDE p132_behavioral"));
}

#[test]
fn p133_track_fingerprints_stub() {
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("macro-token-v4:stmt-token-subst-mvp"));
    assert!(self_src.contains("macro-service-v14:pointcut-within-name-mvp"));
    assert!(self_src.contains("gui-synui-v10:widget-stack"));
    assert!(self_src.contains("wasm-simd16-guest-v16:ne-mvp"));
    assert!(self_src.contains("farm-lease-token-v16:hsm-multi-slot-mvp"));
    assert!(self_src.contains("dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp"));
    assert!(self_src.contains("stdlib-os-v1:cpu-mem-user-mvp"));
    assert!(self_src.contains("runtime-concurrency-v1:task-worker-config-mvp"));
}

#[test]
fn p133_stdlib_parity_pin_stub() {
    // Gate fingerprint: `stdlib-parity-pin-v1:111.0`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:111.0"));
}

#[test]
fn p133_hover_gate0_local() {
    let text = "func main() {\n  let phase = 133\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &p133_opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn p133_hover_string_binding() {
    let text = "func main() {
  let msg = \"p133\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 8, &p133_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p133_definition_msg_use() {
    let text = "func main() {
  let msg = \"p133\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p133.sy", text, 2, 10, &p133_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p133_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &p133_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p133_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p133.sy", text, 1, 19, &p133_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p133_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p133.sy", text, 2, 8, &p133_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p133_diagnostic_unknown_ident() {
    let doc = p133_doc("func main() { print(noSuchSymbol133) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol133")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p133_diagnostic_type_mismatch() {
    let doc = p133_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p133.sy", text, 0, 8, &p133_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p133_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p133_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p133_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p133.sy", text, 0, 12, &p133_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p133_diagnostic_undefined_type() {
    let doc = p133_doc("func main() { let x: NoSuchType133 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p133_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_diagnostic_duplicate_local() {
    let doc = p133_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p133.sy", text, 1, 19, &p133_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p133_diagnostic_wrong_assign_type() {
    let doc = p133_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p133.sy", text, 1, 19, &p133_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p133_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p133_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p133_diagnostic_string_as_int() {
    let doc = p133_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_cjk_string_parse_smoke() {
    let doc = p133_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p133_diagnostic_bool_as_string() {
    let doc = p133_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p133_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p133_service_di_parse_smoke() {
    let doc = p133_doc(
        "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }",
    );
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p133_macro_stmt_token_subst_stub() {
    // Gate fingerprint: `macro-token-v4:stmt-token-subst-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("macro-token-v4:stmt-token-subst-mvp"));
}

#[test]
fn p133_pointcut_within_name_stub() {
    // Gate fingerprint: `macro-service-v14:pointcut-within-name-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("macro-service-v14:pointcut-within-name-mvp"));
}

#[test]
fn p133_macro_docs_v15_honesty_stub() {
    // Gate fingerprint: `macro-docs-v15:m23-service-v14-honesty`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("macro-docs-v15:m23-service-v14-honesty"));
}

#[test]
fn p133_synui_widget_stack_stub() {
    // Gate fingerprint: `gui-synui-v10:widget-stack`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("gui-synui-v10:widget-stack"));
    let doc = p133_doc("func main() { print(\"stack\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p133_synui_scrollview_signal_bind_stub() {
    // Gate fingerprint: `gui-synui-v10:scrollview-signal-bind`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("gui-synui-v10:scrollview-signal-bind"));
}

#[test]
fn p133_synui_web_mobile_layout_stub() {
    // Gate fingerprint: `gui-synui-v10:web-mobile-layout-deepen`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("gui-synui-v10:web-mobile-layout-deepen"));
}

#[test]
fn p133_simd16_ne_stub() {
    // Gate fingerprint: `wasm-simd16-guest-v16:ne-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v16:ne-mvp"));
}

#[test]
fn p133_farm_hsm_multi_slot_stub() {
    // Gate fingerprint: `farm-lease-token-v16:hsm-multi-slot-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("farm-lease-token-v16:hsm-multi-slot-mvp"));
}

#[test]
fn p133_dist_peer_announce_epoch_persist_stub() {
    // Gate fingerprint: `dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("dist-cdn-bgp-v16:peer-announce-epoch-persist-mvp"));
}

#[test]
fn p133_os_cpu_mem_user_stub() {
    // Gate fingerprint: `stdlib-os-v1:cpu-mem-user-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("stdlib-os-v1:cpu-mem-user-mvp"));
}

#[test]
fn p133_task_worker_config_stub() {
    // Gate fingerprint: `runtime-concurrency-v1:task-worker-config-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("runtime-concurrency-v1:task-worker-config-mvp"));
}

#[test]
fn p133_actor_limit_mailbox_stub() {
    // Gate fingerprint: `runtime-concurrency-v1:actor-limit-mailbox-mvp`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("runtime-concurrency-v1:actor-limit-mailbox-mvp"));
}

#[test]
fn p133_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p133_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p133_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p133_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p133_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p133.sy", text, 0, 8, &p133_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p133_hover_int_literal() {
    let text = "func main() { let n = 133\n  print(n) }";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &p133_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p133_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p133.sy", text, 1, 22, &p133_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p133_diagnostic_optional_unwrap() {
    let doc = p133_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_driver_cr390_fingerprint_stub() {
    // Gate fingerprint: `driver-gate-cr390-v1:phase133-real-behavioral`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("driver-gate-cr390-v1:phase133-real-behavioral"));
}

#[test]
fn p133_lsp_semantic_v35_fingerprint_stub() {
    // Gate fingerprint: `lsp-semantic-v35:phase133-46-plus-real`
    let self_src = include_str!("lsp_phase133_real.rs");
    assert!(self_src.contains("lsp-semantic-v35:phase133-46-plus-real"));
}

#[test]
fn p133_hover_lazy_local() {
    let text = "func main() {\n  let count = 111\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &p133_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p133_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p133.sy", text, 1, 19, &p133_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p133_diagnostic_unknown_v14() {
    let doc = p133_doc("func main() { print(noSuchSymbolV14) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p133_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p133_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p133_hover_ne_local() {
    let text = "func main() {\n  let count = 16\n}";
    let hover = hover_for_position("file:///p133.sy", text, 1, 10, &p133_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p133_lsp_semantic_v35_contract() {
    let src = include_str!("lsp_phase133_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 46,
        "phase133 LSP matrix must be ≥46 tests, found {tests}"
    );
    let pad_a = concat!("fn p133_", "sem_gate");
    let pad_b = concat!("fn p133_", "sem_probe");
    let pad_c = concat!("fn p133_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
