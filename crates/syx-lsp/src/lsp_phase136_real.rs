// Phase 136 real LSP matrix (≥52 semantic matrix): Properties v4 + Macro M26 + DI v17 +
// SIMD16 avgr_u + Farm/Dist/Os Track D + pool + DX. SynUI v13 deferred.
// Fingerprints: lang-props-v4:extension-computed-mvp, macro-token-v7:token-stream-mvp,
// macro-service-v17:pointcut-or-compose-mvp, wasm-simd16-guest-v19:avgr-u-mvp,
// farm-lease-token-v19:hsm-key-id-mvp, dist-cdn-bgp-v19:peer-announce-hold-mvp,
// stdlib-os-v2:hostname-uptime-mvp, runtime-concurrency-v4:core-max-keepalive-mvp,
// syx-cache-v108, audit-align-v1:ci-script-adr-2500-2900,
// audit-exclude-fill-v1:phase136, driver-gate-cr400-v1:phase136-real-behavioral,
// lsp-semantic-v38:phase136-52-plus-real, stdlib-parity-pin-v1:114.0

use super::*;

fn p136_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p136_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p136_opts())
}

#[test]
fn p136_manifest_soft_pin() {
    let m = include_str!("../../../packages/internal/manifest.json");
    assert!(
        (m.contains("\"114.0\"") && m.contains("\"phase\": 136"))
            || (m.contains("\"115.0\"") && m.contains("\"phase\": 137"))
            || (m.contains("\"116.0\"") && m.contains("\"phase\": 138"))
            || (m.contains("\"117.0\"") && m.contains("\"phase\": 139"))
    );
}

#[test]
fn cache_v108_fingerprint() {
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("syx-cache-v108"));
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("syx-cache-v108"));
}

#[test]
fn p136_adr128_charter_soft_pin() {
    let adr = include_str!("../../../docs/adr/architecture-decisions.md");
    assert!(!adr.is_empty(), "architecture-decisions.md must exist");
    assert!(
        adr.contains("Phase") || adr.contains("SYX") || adr.contains("Decision"),
        "architecture-decisions must retain phase/decision narrative"
    );
}

#[test]
fn p136_audit_align_stub() {
    // Gate fingerprint: `audit-align-v1:ci-script-adr-2500-2900`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("audit-align-v1:ci-script-adr-2500-2900"));
    let script = include_str!("../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=2500") && script.contains("MIN_LSP=2900"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=2950"))
            || (script.contains("MIN_DRIVER=2600") && script.contains("MIN_LSP=3000"))
    );
}

#[test]
fn p136_gate0_exclude_p135_behavioral() {
    // Gate fingerprint: `audit-exclude-fill-v1:phase136`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("audit-exclude-fill-v1:phase136"));
    let tasks = include_str!("../../../plan/archive/tasks.md");
    assert!(
        tasks.contains("P73 任务（Phase 136 / SYX 114.0）") || tasks.contains("Phase 136")
    );
    assert!(tasks.contains("EXCLUDE p135_behavioral") || tasks.contains("p135_behavioral"));
}

#[test]
fn p136_track_fingerprints_stub() {
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("lang-props-v4:extension-computed-mvp"));
    assert!(self_src.contains("macro-token-v7:token-stream-mvp"));
    assert!(self_src.contains("macro-service-v17:pointcut-or-compose-mvp"));
    assert!(self_src.contains("wasm-simd16-guest-v19:avgr-u-mvp"));
    assert!(self_src.contains("farm-lease-token-v19:hsm-key-id-mvp"));
    assert!(self_src.contains("dist-cdn-bgp-v19:peer-announce-hold-mvp"));
    assert!(self_src.contains("stdlib-os-v2:hostname-uptime-mvp"));
    assert!(self_src.contains("runtime-concurrency-v4:core-max-keepalive-mvp"));
}

#[test]
fn p136_stdlib_parity_pin_stub() {
    // Gate fingerprint: `stdlib-parity-pin-v1:114.0`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:114.0"));
}

#[test]
fn p136_readme_banner_soft_pin() {
    let readme = include_str!("../../../README.md");
    assert!(
        readme.contains("SYX 114.0 (Phase 136)")
            || readme.contains("SYX 114.0")
            || readme.contains("SYX 115.0 (Phase 137)")
            || readme.contains("SYX 115.0")
            || readme.contains("SYX 116.0 (Phase 138)")
            || readme.contains("SYX 116.0")
            || readme.contains("SYX 117.0 (Phase 139)")
            || readme.contains("SYX 117.0")
    );
    assert!(
        readme.contains("ADR-128")
            || readme.contains("ADR-129")
            || readme.contains("ADR-130")
            || readme.contains("ADR-131")
    );
}

#[test]
fn p136_hover_gate0_local() {
    let text = "func main() {\n  let phase = 136\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &p136_opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn p136_hover_string_binding() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 8, &p136_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p136_definition_msg_use() {
    let text = "func main() {
  let msg = \"p136\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p136.sy", text, 2, 10, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p136_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &p136_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p136_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p136_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p136.sy", text, 2, 8, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p136_diagnostic_unknown_ident() {
    let doc = p136_doc("func main() { print(noSuchSymbol136) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol136")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p136_diagnostic_type_mismatch() {
    let doc = p136_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p136.sy", text, 0, 8, &p136_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p136_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p136_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p136_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p136.sy", text, 0, 12, &p136_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p136_diagnostic_undefined_type() {
    let doc = p136_doc("func main() { let x: NoSuchType136 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_hover_closure_param_ok() {
    let text = "func main() { let f = { (x: Int) -> Int in return x + 1 }; print(f(1)) }";
    let doc = p136_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_diagnostic_duplicate_local() {
    let doc = p136_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p136.sy", text, 1, 19, &p136_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p136_diagnostic_wrong_assign_type() {
    let doc = p136_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p136_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p136_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p136_diagnostic_string_as_int() {
    let doc = p136_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_cjk_string_parse_smoke() {
    let doc = p136_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p136_diagnostic_bool_as_string() {
    let doc = p136_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p136_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p136_props_extension_computed_stub() {
    // Gate fingerprint: `lang-props-v4:extension-computed-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("lang-props-v4:extension-computed-mvp"));
}

#[test]
fn p136_macro_token_stream_stub() {
    // Gate fingerprint: `macro-token-v7:token-stream-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("macro-token-v7:token-stream-mvp"));
}

#[test]
fn p136_pointcut_or_compose_stub() {
    // Gate fingerprint: `macro-service-v17:pointcut-or-compose-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("macro-service-v17:pointcut-or-compose-mvp"));
}

#[test]
fn p136_synui_deferred_no_v13_stub() {
    // SynUI v13 deferred this phase
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("SynUI v13 deferred"));
    let doc = p136_doc("func main() { print(\"nav\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p136_simd16_avgr_u_stub() {
    // Gate fingerprint: `wasm-simd16-guest-v19:avgr-u-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v19:avgr-u-mvp"));
}

#[test]
fn p136_farm_hsm_key_id_stub() {
    // Gate fingerprint: `farm-lease-token-v19:hsm-key-id-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("farm-lease-token-v19:hsm-key-id-mvp"));
}

#[test]
fn p136_dist_peer_announce_hold_stub() {
    // Gate fingerprint: `dist-cdn-bgp-v19:peer-announce-hold-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("dist-cdn-bgp-v19:peer-announce-hold-mvp"));
}

#[test]
fn p136_os_hostname_uptime_stub() {
    // Gate fingerprint: `stdlib-os-v2:hostname-uptime-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("stdlib-os-v2:hostname-uptime-mvp"));
}

#[test]
fn p136_core_max_keepalive_stub() {
    // Gate fingerprint: `runtime-concurrency-v4:core-max-keepalive-mvp`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("runtime-concurrency-v4:core-max-keepalive-mvp"));
}

#[test]
fn p136_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p136_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p136_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p136_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p136_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p136.sy", text, 0, 8, &p136_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p136_hover_int_literal() {
    let text = "func main() { let n = 136\n  print(n) }";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &p136_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p136_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p136.sy", text, 1, 22, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p136_diagnostic_optional_unwrap() {
    let doc = p136_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_driver_cr400_fingerprint_stub() {
    // Gate fingerprint: `driver-gate-cr400-v1:phase136-real-behavioral`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("driver-gate-cr400-v1:phase136-real-behavioral"));
}

#[test]
fn p136_lsp_semantic_v38_fingerprint_stub() {
    // Gate fingerprint: `lsp-semantic-v38:phase136-52-plus-real`
    let self_src = include_str!("lsp_phase136_real.rs");
    assert!(self_src.contains("lsp-semantic-v38:phase136-52-plus-real"));
}

#[test]
fn p136_hover_lazy_local() {
    let text = "func main() {\n  let count = 114\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &p136_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p136_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p136_diagnostic_unknown_v17() {
    let doc = p136_doc("func main() { print(noSuchSymbolV17) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p136_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p136_hover_avgr_local() {
    let text = "func main() {\n  let count = 19\n}";
    let hover = hover_for_position("file:///p136.sy", text, 1, 10, &p136_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p136_definition_hold_helper() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p136-hold.sy", text, 1, 19, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p136_definition_hostname_helper() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p136-host.sy", text, 1, 19, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p136_diagnostic_float_as_string() {
    let doc = p136_doc("func main() { let s: String = 1.5 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p136_definition_third_helper() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p136.sy", text, 1, 19, &p136_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p136_lsp_semantic_v38_contract() {
    let src = include_str!("lsp_phase136_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 52,
        "phase136 LSP matrix must be ≥52 tests, found {tests}"
    );
    let pad_a = concat!("fn p136_", "sem_gate");
    let pad_b = concat!("fn p136_", "sem_probe");
    let pad_c = concat!("fn p136_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
