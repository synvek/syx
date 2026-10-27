//! Driver cache v111 invalidation contract (full fingerprint cluster).

#[test]
fn cache_v111_tag_active() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v1"));
    assert!(!lib.contains("\"syx-cache-v110:"));
    assert!(lib.contains("heap_v111_phase139_track_l_language_fingerprint"));
    assert!(lib.contains("heap_v111_phase139_track_c_wasm_fingerprint"));
    assert!(lib.contains("heap_v111_phase139_track_d_frozen_fingerprint"));
    assert!(lib.contains("heap_v111_phase139_track_inherit_fingerprint"));
    assert!(lib.contains("heap_v111_phase139_dx_fingerprint"));
}

#[test]
fn cache_v111_lang_surface_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    let surf = include_str!("../../src/lang_surface_v1.rs");
    assert!(lib.contains("heap_v111_phase139_track_l_language_fingerprint"));
    assert!(surf.contains("lang-string-v1:multiline-literal-mvp"));
    assert!(surf.contains("lang-tuple-v1:product-type-mvp"));
    assert!(surf.contains("lang-nested-v1:qualified-path-mvp"));
    assert!(surf.contains("lang-range-v1:closed-range-mvp"));
    assert!(surf.contains("lang-character-v1:distinct-scalar-mvp"));
    assert!(surf.contains("lang-call-v1:external-label-mvp"));
    assert!(surf.contains("lang-call-v1:omit-underscore-mvp"));
    assert!(surf.contains("lang-call-v1:labeled-args-required"));
    assert!(surf.contains("lang-final-v1:sealed-removed-docs"));
}

#[test]
fn cache_v111_lang_surface_module() {
    let surf = include_str!("../../src/lang_surface_v1.rs");
    assert!(surf.contains("lang-string-v1:docs-honesty"));
    assert!(surf.contains("lang-tuple-v1:named-elements-mvp"));
    assert!(surf.contains("lang-final-v1:final-only-honesty"));
}

#[test]
fn cache_v111_wasm_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("wasm-simd16-guest-v22:all-any-true-mvp"));
}

#[test]
fn cache_v111_synui_deferred_farm_dist_frozen() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("gui-synui-v12:deferred-no-v13"));
    assert!(!lib.contains("gui-synui-v13:"));
    assert!(lib.contains("farm-dist-v19:frozen-no-v20"));
    assert!(!lib.contains("farm-lease-token-v20:"));
    assert!(!lib.contains("dist-cdn-bgp-v20:"));
}

#[test]
fn cache_v111_inherit_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("lang-props-v6:static-observers-mvp"));
    assert!(lib.contains("wasm-simd16-guest-v21:bitmask-mvp"));
}

#[test]
fn cache_v111_dx_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("audit-align-v1:ci-script-adr-2600-3050"));
    assert!(lib.contains("audit-exclude-fill-v1:phase139"));
    assert!(lib.contains("lsp-semantic-v41:phase139-58-plus-real"));
    assert!(lib.contains("driver-gate-cr430-v1:phase139-real-behavioral"));
    assert!(lib.contains("stdlib-parity-pin-v1:117.0"));
}
