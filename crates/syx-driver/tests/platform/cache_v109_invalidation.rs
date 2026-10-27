//! Driver cache v109 invalidation contract (full fingerprint cluster).

#[test]
fn cache_v109_tag_active() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v109"));
    assert!(!lib.contains("\"syx-cache-v108:"));
    assert!(lib.contains("heap_v109_phase137_track_a_props_fingerprint"));
    assert!(lib.contains("heap_v109_phase137_track_c_wasm_fingerprint"));
    assert!(lib.contains("heap_v109_phase137_track_d_os_disk_load_fingerprint"));
    assert!(lib.contains("heap_v109_phase137_track_e_macro_concurrency_fingerprint"));
    assert!(lib.contains("heap_v109_phase137_dx_fingerprint"));
}

#[test]
fn cache_v109_props_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("lang-props-v5:published-mvp"));
    assert!(lib.contains("lang-props-v5:docs-honesty"));
}

#[test]
fn cache_v109_macro_service_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("macro-token-v8:decl-expand-mvp"));
    assert!(lib.contains("macro-service-v18:pointcut-not-within-mvp"));
    assert!(lib.contains("macro-docs-v19:m27-service-v18-honesty"));
    assert!(lib.contains("runtime-concurrency-v5:discard-oldest-mvp"));
}

#[test]
fn cache_v109_synui_deferred_farm_dist_frozen() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("gui-synui-v12:deferred-no-v13"));
    assert!(!lib.contains("gui-synui-v13:"));
    assert!(lib.contains("farm-dist-v19:frozen-no-v20"));
    assert!(!lib.contains("farm-lease-token-v20:"));
    assert!(!lib.contains("dist-cdn-bgp-v20:"));
}

#[test]
fn cache_v109_wasm_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("wasm-simd16-guest-v20:popcnt-mvp"));
}

#[test]
fn cache_v109_os_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("stdlib-os-v3:disk-load-mvp"));
}

#[test]
fn cache_v109_dx_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("audit-align-v1:ci-script-adr-2600-2950"));
    assert!(lib.contains("audit-exclude-fill-v1:phase137"));
    assert!(lib.contains("lsp-semantic-v39:phase137-54-plus-real"));
    assert!(lib.contains("driver-gate-cr410-v1:phase137-real-behavioral"));
    assert!(lib.contains("stdlib-parity-pin-v1:115.0"));
}
