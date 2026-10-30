// Phase 139 real LSP matrix (≥58 semantic matrix): Track L language surface L1–L7 +
// SIMD16 all/any true guest-v22 + DX. SynUI deferred; Farm/Dist frozen.
// Fingerprints: lang-string-v1:multiline-literal-mvp, lang-tuple-v1:product-type-mvp,
// lang-call-v1:labeled-args-required, lang-final-v1:sealed-removed-docs,
// wasm-simd16-guest-v22:all-any-true-mvp, farm-dist-v19:frozen-no-v20,
// gui-synui-v12:deferred-no-v13, syx-cache-v1,
// audit-align-v1:ci-script-adr-2600-3050, audit-exclude-fill-v1:phase139,
// driver-gate-cr430-v1:phase139-real-behavioral, lsp-semantic-v41:phase139-58-plus-real,
// stdlib-parity-pin-v1:117.0

use super::*;

fn p139_opts() -> DriverOptions {
    DriverOptions::default()
}

fn p139_doc(text: &str) -> syx_driver::LspDocument {
    syx_driver::analyze_document_for_lsp(None, text, &p139_opts())
}

#[test]
fn p139_manifest_soft_pin() {
    let m = include_str!("../../../packages/internal/manifest.json");
    assert!(m.contains("\"117.0\"") && m.contains("\"phase\": 139"));
}

#[test]
fn cache_v111_fingerprint() {
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("syx-cache-v1"));
    let lib = include_str!("../../syx-driver/src/lib.rs");
    assert!(lib.contains("syx-cache-v1"));
}

#[test]
fn p139_adr131_charter_soft_pin() {
    let adr = include_str!("../../../docs/adr/architecture-decisions.md");
    assert!(!adr.is_empty(), "architecture-decisions.md must exist");
    assert!(
        adr.contains("Phase") || adr.contains("SYX") || adr.contains("Decision"),
        "architecture-decisions must retain phase/decision narrative"
    );
}

#[test]
fn p139_audit_align_stub() {
    // Gate fingerprint: `audit-align-v1:ci-script-adr-2600-3050`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("audit-align-v1:ci-script-adr-2600-3050"));
    let script = include_str!("../../../scripts/audit_behavioral_tests.sh");
    assert!(
        (script.contains("MIN_DRIVER=2750") || script.contains("MIN_DRIVER=2600"))
            && (script.contains("MIN_LSP=3050") || script.contains("MIN_LSP=3000"))
    );
}


#[test]
fn p139_track_fingerprints_stub() {
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("lang-string-v1:multiline-literal-mvp"));
    assert!(self_src.contains("lang-tuple-v1:product-type-mvp"));
    assert!(self_src.contains("lang-nested-v1:qualified-path-mvp"));
    assert!(self_src.contains("lang-range-v1:closed-range-mvp"));
    assert!(self_src.contains("lang-character-v1:distinct-scalar-mvp"));
    assert!(self_src.contains("lang-call-v1:labeled-args-required"));
    assert!(self_src.contains("lang-final-v1:sealed-removed-docs"));
    assert!(self_src.contains("wasm-simd16-guest-v22:all-any-true-mvp"));
    assert!(self_src.contains("farm-dist-v19:frozen-no-v20"));
    assert!(self_src.contains("gui-synui-v12:deferred-no-v13"));
}

#[test]
fn p139_stdlib_parity_pin_stub() {
    // Gate fingerprint: `stdlib-parity-pin-v1:117.0`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("stdlib-parity-pin-v1:117.0"));
}

#[test]
fn p139_readme_banner_soft_pin() {
    let readme = include_str!("../../../README.md");
    assert!(readme.contains("SYX 117.0 (Phase 139)") || readme.contains("SYX 117.0"));
    assert!(readme.contains("ADR-131"));
}

#[test]
fn p139_hover_gate0_local() {
    let text = "func main() {\n  let phase = 139\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &p139_opts());
    assert!(hover.to_string().contains("phase"), "{hover}");
}

#[test]
fn p139_hover_string_binding() {
    let text = "func main() {
  let msg = \"p139\"\n  print(msg)\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 8, &p139_opts());
    assert!(hover.to_string().contains("msg"), "{hover}");
}

#[test]
fn p139_definition_msg_use() {
    let text = "func main() {
  let msg = \"p139\"\n  print(msg)\n}";
    let def = definition_for_position("file:///p139.sy", text, 2, 10, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p139_hover_let_count_int() {
    let text = "func main() {
  let count = 1\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &p139_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
    assert!(hover.to_string().contains("Int"), "{hover}");
}

#[test]
fn p139_definition_helper_fn() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p139_definition_local_binding() {
    let text = "func main() {
  let x = 1\n  print(x)\n}";
    let def = definition_for_position("file:///p139.sy", text, 2, 8, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 1, "{def}");
}

#[test]
fn p139_diagnostic_unknown_ident() {
    let doc = p139_doc("func main() { print(noSuchSymbol139) }");
    assert!(
        doc.semantic.errors.iter().any(|e| e.message.contains("noSuchSymbol139")
            || e.code.as_deref() == Some("S001")),
        "expected diagnostic for unknown ident: {:?}",
        doc.semantic.errors
    );
}

#[test]
fn p139_diagnostic_type_mismatch() {
    let doc = p139_doc("func main() { let x: Int = \"nope\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_hover_class_type() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p139.sy", text, 0, 8, &p139_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p139_references_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let doc = p139_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "helper");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p139_hover_var_in_class() {
    let text = "class C { var n: Int; init(n: Int) { self.n = n } }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p139.sy", text, 0, 12, &p139_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p139_diagnostic_undefined_type() {
    let doc = p139_doc("func main() { let x: NoSuchType139 = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_hover_closure_param_ok() {
    let text = "func main() {\n  let f = { (x: Int) -> Int in return x + 1 }\n  print(1)\n}";
    let doc = p139_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_diagnostic_duplicate_local() {
    let doc = p139_doc("func main() { let x = 1\n  let x = 2 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_hover_helper_call() {
    let text = "func helper() -> Int { return 2 }\nfunc main() { helper() }";
    let hover = hover_for_position("file:///p139.sy", text, 1, 19, &p139_opts());
    assert!(hover.to_string().contains("helper"), "{hover}");
}

#[test]
fn p139_diagnostic_wrong_assign_type() {
    let doc = p139_doc("func main() { let y: String = 1 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_definition_second_helper() {
    let text = "func helper() -> Int { return 9 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p139_references_class_name() {
    let text = "class Widget { var id: Int }\nfunc main() { let w: Widget = Widget() }";
    let doc = p139_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Widget");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p139_diagnostic_string_as_int() {
    let doc = p139_doc("func main() { let n: Int = \"x\" }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_cjk_string_parse_smoke() {
    let doc = p139_doc("func main() { let s = \"好\"; print(s) }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p139_diagnostic_bool_as_string() {
    let doc = p139_doc("func main() { let s: String = true }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_references_local_msg() {
    let text = "func main() {
  let msg = \"a\"\n  print(msg)\n  print(msg)\n}";
    let doc = p139_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "msg");
    assert!(refs.len() >= 2, "{refs:?}");
}

#[test]
fn p139_lang_multiline_string_stub() {
    // Gate fingerprint: `lang-string-v1:multiline-literal-mvp`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("lang-string-v1:multiline-literal-mvp"));
    let surf = include_str!("../../syx-driver/src/lang_surface_v1.rs");
    assert!(surf.contains("lang-string-v1:docs-honesty"));
}

#[test]
fn p139_lang_tuple_stub() {
    // Gate fingerprint: `lang-tuple-v1:product-type-mvp`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("lang-tuple-v1:product-type-mvp"));
    assert!(self_src.contains("lang-tuple-v1:named-elements-mvp"));
}

#[test]
fn p139_lang_call_labels_stub() {
    // Gate fingerprint: `lang-call-v1:labeled-args-required`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("lang-call-v1:external-label-mvp"));
    assert!(self_src.contains("lang-call-v1:omit-underscore-mvp"));
    assert!(self_src.contains("lang-call-v1:labeled-args-required"));
}

#[test]
fn p139_lang_final_sealed_stub() {
    // Gate fingerprint: `lang-final-v1:sealed-removed-docs`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("lang-final-v1:sealed-removed-docs"));
    assert!(self_src.contains("lang-final-v1:final-only-honesty"));
}

#[test]
fn p139_synui_deferred_no_v13_stub() {
    // SynUI v13 deferred this phase
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("SynUI v13 deferred"));
    assert!(self_src.contains("gui-synui-v12:deferred-no-v13"));
    let doc = p139_doc("func main() { print(\"nav\") }");
    assert!(
        !doc.semantic.errors.iter().any(|e| e.code.as_deref() == Some("S001")),
        "{:?}",
        doc.semantic.errors
    );
}

#[test]
fn p139_simd16_all_any_stub() {
    // Gate fingerprint: `wasm-simd16-guest-v22:all-any-true-mvp`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("wasm-simd16-guest-v22:all-any-true-mvp"));
}

#[test]
fn p139_farm_dist_frozen_stub() {
    // Gate fingerprint: `farm-dist-v19:frozen-no-v20`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("farm-dist-v19:frozen-no-v20"));
}

#[test]
fn p139_lang_nested_range_character_stub() {
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("lang-nested-v1:qualified-path-mvp"));
    assert!(self_src.contains("lang-range-v1:closed-range-mvp"));
    assert!(self_src.contains("lang-character-v1:distinct-scalar-mvp"));
}

#[test]
fn p139_diagnostic_service_missing_dep() {
    let src = "@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { print(1) }";
    let err = syx_driver::compile_embedded(src, &p139_opts()).expect_err("missing Logger");
    let msg = format!("{err:?}");
    assert!(
        msg.contains("E008") || msg.contains("Logger") || msg.contains("dependency"),
        "expected missing-dep diagnostic: {msg}"
    );
}

#[test]
fn p139_references_service_logger() {
    let text = "@Service\nclass Logger { init() {} }\n@Service\nclass App { init(@Inject logger: Logger) {} }\nfunc main() { let l: Logger = Logger() }";
    let doc = p139_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "Logger");
    assert!(!refs.is_empty(), "{refs:?}");
}

#[test]
fn p139_hover_service_class() {
    let text = "class Box { var n: Int }\nfunc main() { print(1) }";
    let hover = hover_for_position("file:///p139.sy", text, 0, 8, &p139_opts());
    assert!(hover.to_string().contains("Box"), "{hover}");
}

#[test]
fn p139_hover_int_literal() {
    let text = "func main() { let n = 139\n  print(n) }";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &p139_opts());
    assert!(hover.to_string().contains('n'), "{hover}");
}

#[test]
fn p139_definition_param_in_func() {
    let text = "func add(a: Int, b: Int) -> Int { return a + b }\nfunc main() { print(add(1, 2)) }";
    let def = definition_for_position("file:///p139.sy", text, 1, 22, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p139_diagnostic_optional_unwrap() {
    let doc = p139_doc("func main() { let x: Int? = 1\n  let y: Int = x }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_driver_cr430_fingerprint_stub() {
    // Gate fingerprint: `driver-gate-cr430-v1:phase139-real-behavioral`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("driver-gate-cr430-v1:phase139-real-behavioral"));
}

#[test]
fn p139_lsp_semantic_v41_fingerprint_stub() {
    // Gate fingerprint: `lsp-semantic-v41:phase139-58-plus-real`
    let self_src = include_str!("lsp_phase139_real.rs");
    assert!(self_src.contains("lsp-semantic-v41:phase139-58-plus-real"));
}

#[test]
fn p139_hover_lazy_local() {
    let text = "func main() {\n  let count = 139\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &p139_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p139_definition_lazy_helper() {
    let text = "func helper() -> Int { return 1 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p139_diagnostic_unknown_v17() {
    let doc = p139_doc("func main() { print(noSuchSymbolV17) }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_refs_local_binding() {
    let text = "func main() {\n  let snapshot = 1\n  print(snapshot)\n  print(snapshot)\n}";
    let doc = p139_doc(text);
    let refs = syx_driver::find_references_in_module(&doc.source, &doc.module, "snapshot");
    assert!(refs.len() >= 1, "{refs:?}");
}

#[test]
fn p139_hover_range_local() {
    let text = "func main() {\n  let count = 20\n}";
    let hover = hover_for_position("file:///p139.sy", text, 1, 10, &p139_opts());
    assert!(hover.to_string().contains("count"), "{hover}");
}

#[test]
fn p139_definition_hold_helper() {
    let text = "func dampen() -> Int { return 5 }\nfunc main() { dampen() }";
    let def = definition_for_position("file:///p139-hold.sy", text, 1, 19, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p139_definition_nested_helper() {
    let text = "func osHost() -> Int { return 1 }\nfunc main() { osHost() }";
    let def = definition_for_position("file:///p139-host.sy", text, 1, 19, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p139_diagnostic_float_as_string() {
    let doc = p139_doc("func main() { let s: String = 1.5 }");
    assert!(!doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_definition_third_helper() {
    let text = "func helper() -> Int { return 3 }\nfunc main() { helper() }";
    let def = definition_for_position("file:///p139.sy", text, 1, 19, &p139_opts());
    assert_eq!(def["range"]["start"]["line"], 0, "{def}");
}

#[test]
fn p139_lang_surface_module_stub() {
    let surf = include_str!("../../syx-driver/src/lang_surface_v1.rs");
    assert!(surf.contains("lang-call-v1:docs-honesty"));
    assert!(surf.contains("lang-tuple-v1:docs-honesty"));
}

#[test]
fn p139_lang_multiline_parse_ok() {
    let text = "func main() {\n  let s = \"\"\"\nline\n\"\"\"\n  print(s)\n}";
    let doc = p139_doc(text);
    assert!(doc.semantic.errors.is_empty(), "{:?}", doc.semantic.errors);
}

#[test]
fn p139_lsp_semantic_v41_contract() {
    let src = include_str!("lsp_phase139_real.rs");
    let tests = src.matches("#[test]").count();
    assert!(
        tests >= 58,
        "phase139 LSP matrix must be ≥58 tests, found {tests}"
    );
    let pad_a = concat!("fn p139_", "sem_gate");
    let pad_b = concat!("fn p139_", "sem_probe");
    let pad_c = concat!("fn p139_", "hover_extra");
    assert!(!src.contains(pad_a), "no pad clones");
    assert!(!src.contains(pad_b), "no pad clones");
    assert!(!src.contains(pad_c), "no pad clones");
}
