//! Driver cache v110 invalidation contract (full fingerprint cluster).

#[test]
fn cache_v110_tag_active() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v110"));
    assert!(!lib.contains("\"syx-cache-v109:"));
    assert!(lib.contains("heap_v110_phase138_track_a_props_fingerprint"));
    assert!(lib.contains("heap_v110_phase138_track_c_wasm_fingerprint"));
    assert!(lib.contains("heap_v110_phase138_track_d_os_network_fingerprint"));
    assert!(lib.contains("heap_v110_phase138_track_e_macro_concurrency_fingerprint"));
    assert!(lib.contains("heap_v110_phase138_dx_fingerprint"));
}

#[test]
fn cache_v110_props_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("lang-props-v6:static-observers-mvp"));
    assert!(lib.contains("lang-props-v6:docs-honesty"));
}

#[test]
fn cache_v110_macro_service_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("macro-token-v9:token-rewrite-mvp"));
    assert!(lib.contains("macro-service-v19:pointcut-within-wildcard-mvp"));
    assert!(lib.contains("macro-docs-v20:m28-service-v19-honesty"));
    assert!(lib.contains("runtime-concurrency-v6:actor-shared-pool-mvp"));
}

#[test]
fn cache_v110_synui_deferred_farm_dist_frozen() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("gui-synui-v12:deferred-no-v13"));
    assert!(!lib.contains("gui-synui-v13:"));
    assert!(lib.contains("farm-dist-v19:frozen-no-v20"));
    assert!(!lib.contains("farm-lease-token-v20:"));
    assert!(!lib.contains("dist-cdn-bgp-v20:"));
}

#[test]
fn cache_v110_wasm_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("wasm-simd16-guest-v21:bitmask-mvp"));
}

#[test]
fn cache_v110_os_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("stdlib-os-v4:network-mvp"));
}

#[test]
fn cache_v110_dx_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("audit-align-v1:ci-script-adr-2600-3000"));
    assert!(lib.contains("audit-exclude-fill-v1:phase138"));
    assert!(lib.contains("lsp-semantic-v40:phase138-56-plus-real"));
    assert!(lib.contains("driver-gate-cr420-v1:phase138-real-behavioral"));
    assert!(lib.contains("stdlib-parity-pin-v1:116.0"));
}
