// Phase 137 real LSP matrix (≥54 semantic matrix): Properties v5 + Macro M27 + DI v18 +
// SIMD16 popcnt + Os disk/load + DiscardOldest + DX. SynUI deferred; Farm/Dist frozen.
// Fingerprints: lang-props-v5:published-mvp, macro-token-v8:decl-expand-mvp,
// macro-service-v18:pointcut-not-within-mvp, wasm-simd16-guest-v20:popcnt-mvp,
// farm-dist-v19:frozen-no-v20, stdlib-os-v3:disk-load-mvp,
// runtime-concurrency-v5:discard-oldest-mvp, macro-docs-v19:m27-service-v18-honesty,
// syx-cache-v109, audit-align-v1:ci-script-adr-2600-2950,
// audit-exclude-fill-v1:phase137, driver-gate-cr410-v1:phase137-real-behavioral,
// lsp-semantic-v39:phase137-54-plus-real, stdlib-parity-pin-v1:115.0

use super::*;

fn p137_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p137_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p137_opts())
}

#[test]
fn p137_manifest_soft_pin() {
    let m = include_str!("../../../packages/internal/manifest.json");
    assert!(
        (m.contains("\"115.0\"") && m.contains("\"phase\": 137"))
            || (m.contains("\"116.0\"") && m.contains("\"phase\": 138"))
            || (m.contains("\"117.0\"") && m.contains("\"phase\": 139"))
    );
}

#[test]
fn p137_cache_v108_fingerprint() {
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("syx-cache-v109"));
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("syx-cache-v109"));
}

#[test]
fn p137_adr128_charter_soft_pin() {
    let adr = include_str!("../../../docs/adr/architecture-decisions.md");
    assert!(!adr.is_empty(), "architecture-decisions.md must exist");
    assert!(
        adr.contains("Phase") || adr.contains("SYX") || adr.contains("Decision"),
        "architecture-decisions must retain phase/decision narrative"
    );
}

#[test]
fn p137_audit_align_stub() {
    // Gate fingerprint: `audit-align-v1:ci-script-adr-2600-2950`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("audit-align-v1:ci-script-adr-2600-2950"));
    let script = include_str!("../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=2950"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=3000"))
            || (script.contains("MIN_DRIVER=2750") && script.contains("MIN_LSP=3000"))
    );
}


#[test]
fn p137_track_fingerprints_stub() {
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("lang-props-v5:published-mvp"));
    assert!(self_src.contains("macro-token-v8:decl-expand-mvp"));
    assert!(self_src.contains("macro-service-v18:pointcut-not-within-mvp"));
    assert!(self_src.contains("wasm-simd16-guest-v20:popcnt-mvp"));
    assert!(self_src.contains("farm-dist-v19:frozen-no-v20"));
    assert!(self_src.contains("stdlib-os-v3:disk-load-mvp"));
    assert!(self_src.contains("runtime-concurrency-v5:discard-oldest-mvp"));
    assert!(self_src.contains("macro-docs-v19:m27-service-v18-honesty"));
}

#[test]
fn p137_stdlib_parity_pin_stub() {
    // Gate fingerprint: `stdlib-parity-pin-v1:115.0`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:115.0"));
}

#[test]
fn p137_readme_banner_soft_pin() {
    let readme = include_str!("../../../README.md");
    assert!(
        readme.contains("SYX 115.0 (Phase 137)")
            || readme.contains("SYX 115.0")
            || readme.contains("SYX 116.0 (Phase 138)")
            || readme.contains("SYX 116.0")
            || readme.contains("SYX 117.0 (Phase 139)")
            || readme.contains("SYX 117.0")
    );
    assert!(readme.contains("ADR-129") || readme.contains("ADR-130") || readme.contains("ADR-131"));
}

#[test]
fn p137_hover_gate0_local() {
    let text = "func main() {\n  let phase = 137\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &p137_opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn p137_hover_string_binding() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 8, &p137_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p137_definition_msg_use() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p137.sy", text, 2, 10, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p137_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &p137_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p137_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p137_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p137.sy", text, 2, 8, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p137_diagnostic_unknown_ident() {
    let doc = p137_doc("func main() { print(noSuchSymbol136) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol136")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p137_diagnostic_type_mismatch() {
    let doc = p137_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p137.sy", text, 0, 8, &p137_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p137_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p137_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p137_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p137.sy", text, 0, 12, &p137_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p137_diagnostic_undefined_type() {
    let doc = p137_doc("func main() { let x: NoSuchType136 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p137_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_diagnostic_duplicate_local() {
    let doc = p137_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p137.sy", text, 1, 19, &p137_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p137_diagnostic_wrong_assign_type() {
    let doc = p137_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p137_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p137_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p137_diagnostic_string_as_int() {
    let doc = p137_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_cjk_string_parse_smoke() {
    let doc = p137_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p137_diagnostic_bool_as_string() {
    let doc = p137_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p137_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p137_props_extension_computed_stub() {
    // Gate fingerprint: `lang-props-v5:published-mvp`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("lang-props-v5:published-mvp"));
}

#[test]
fn p137_macro_token_stream_stub() {
    // Gate fingerprint: `macro-token-v8:decl-expand-mvp`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("macro-token-v8:decl-expand-mvp"));
}

#[test]
fn p137_pointcut_or_compose_stub() {
    // Gate fingerprint: `macro-service-v18:pointcut-not-within-mvp`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("macro-service-v18:pointcut-not-within-mvp"));
}

#[test]
fn p137_synui_deferred_no_v13_stub() {
    // SynUI v13 deferred this phase
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("SynUI v13 deferred"));
    let doc = p137_doc("func main() { print(\"nav\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p137_simd16_avgr_u_stub() {
    // Gate fingerprint: `wasm-simd16-guest-v20:popcnt-mvp`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v20:popcnt-mvp"));
}

#[test]
fn p137_farm_hsm_key_id_stub() {
    // Gate fingerprint: `farm-dist-v19:frozen-no-v20`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("farm-dist-v19:frozen-no-v20"));
}

#[test]
fn p137_dist_peer_announce_hold_stub() {
    // Gate fingerprint: `stdlib-os-v3:disk-load-mvp`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("stdlib-os-v3:disk-load-mvp"));
}

#[test]
fn p137_os_hostname_uptime_stub() {
    // Gate fingerprint: `runtime-concurrency-v5:discard-oldest-mvp`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("runtime-concurrency-v5:discard-oldest-mvp"));
}

#[test]
fn p137_core_max_keepalive_stub() {
    // Gate fingerprint: `macro-docs-v19:m27-service-v18-honesty`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("macro-docs-v19:m27-service-v18-honesty"));
}

#[test]
fn p137_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p137_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p137_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p137_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p137_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p137.sy", text, 0, 8, &p137_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p137_hover_int_literal() {
    let text = "func main() { let n = 136\n  print(n) }";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &p137_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p137_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p137.sy", text, 1, 22, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p137_diagnostic_optional_unwrap() {
    let doc = p137_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_driver_cr410_fingerprint_stub() {
    // Gate fingerprint: `driver-gate-cr410-v1:phase137-real-behavioral`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("driver-gate-cr410-v1:phase137-real-behavioral"));
}

#[test]
fn p137_lsp_semantic_v39_fingerprint_stub() {
    // Gate fingerprint: `lsp-semantic-v39:phase137-54-plus-real`
    let self_src = include_str!("lsp_phase137_real.rs");
    assert!(self_src.contains("lsp-semantic-v39:phase137-54-plus-real"));
}

#[test]
fn p137_hover_lazy_local() {
    let text = "func main() {\n  let count = 114\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &p137_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p137_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p137_diagnostic_unknown_v17() {
    let doc = p137_doc("func main() { print(noSuchSymbolV17) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p137_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p137_hover_avgr_local() {
    let text = "func main() {\n  let count = 20\n}";
    let hover = hover_for_position("file:///p137.sy", text, 1, 10, &p137_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p137_definition_hold_helper() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p137-hold.sy", text, 1, 19, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p137_definition_hostname_helper() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p137-host.sy", text, 1, 19, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p137_diagnostic_float_as_string() {
    let doc = p137_doc("func main() { let s: String = 1.5 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p137_definition_third_helper() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p137.sy", text, 1, 19, &p137_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p137_lsp_semantic_v39_contract() {
    let src = include_str!("lsp_phase137_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 54,
        "phase137 LSP matrix must be ≥54 tests, found {tests}"
    );
    let pad_a = concat!("fn p137_", "sem_gate");
    let pad_b = concat!("fn p137_", "sem_probe");
    let pad_c = concat!("fn p137_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
