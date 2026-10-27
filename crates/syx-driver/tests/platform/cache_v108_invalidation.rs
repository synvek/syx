//! Driver cache v108 invalidation contract (full fingerprint cluster).

#[test]
fn cache_v108_tag_active() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("syx-cache-v108"));
    assert!(!lib.contains("\"syx-cache-v107:"));
    assert!(lib.contains("heap_v108_phase136_track_a_props_fingerprint"));
    assert!(lib.contains("heap_v108_phase136_track_c_wasm_fingerprint"));
    assert!(lib.contains("heap_v108_phase136_track_d_farm_dist_os_fingerprint"));
    assert!(lib.contains("heap_v108_phase136_track_e_macro_concurrency_fingerprint"));
    assert!(lib.contains("heap_v108_phase136_dx_fingerprint"));
}

#[test]
fn cache_v108_props_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("lang-props-v4:extension-computed-mvp"));
    assert!(lib.contains("lang-props-v4:docs-honesty"));
}

#[test]
fn cache_v108_macro_service_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("macro-token-v7:token-stream-mvp"));
    assert!(lib.contains("macro-service-v17:pointcut-or-compose-mvp"));
    assert!(lib.contains("macro-docs-v18:m26-service-v17-honesty"));
    assert!(lib.contains("runtime-concurrency-v4:core-max-keepalive-mvp"));
}

#[test]
fn cache_v108_synui_deferred() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("gui-synui-v12:deferred-no-v13"));
    assert!(!lib.contains("gui-synui-v13:"));
}

#[test]
fn cache_v108_wasm_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("wasm-simd16-guest-v19:avgr-u-mvp"));
}

#[test]
fn cache_v108_farm_dist_os_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("farm-lease-token-v19:hsm-key-id-mvp"));
    assert!(lib.contains("dist-cdn-bgp-v19:peer-announce-hold-mvp"));
    assert!(lib.contains("stdlib-os-v2:hostname-uptime-mvp"));
}

#[test]
fn cache_v108_dx_fingerprints() {
    let lib = include_str!("../../src/lib.rs");
    assert!(lib.contains("audit-align-v1:ci-script-adr-2500-2900"));
    assert!(lib.contains("audit-exclude-fill-v1:phase136"));
    assert!(lib.contains("lsp-semantic-v38:phase136-52-plus-real"));
    assert!(lib.contains("driver-gate-cr400-v1:phase136-real-behavioral"));
    assert!(lib.contains("stdlib-parity-pin-v1:114.0"));
}
