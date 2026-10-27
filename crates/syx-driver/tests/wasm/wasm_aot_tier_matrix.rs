//! Phase 91: TIER3_NAMES matrix coverage.

use syx_wasm_emit::{TIER3_NAMES, tier_for_import};

#[test]
fn p98_tier3_matrix_has_entries() {
    assert!(TIER3_NAMES.len() >= 20);
}

#[test]
fn p98_tier3_matrix_no_exec_op() {
    assert!(!TIER3_NAMES.contains(&"__syx_aot_exec_op"));
}

#[test]
fn p98_tier3_matrix_tier_for_import() {
    let closure = tier_for_import("__syx_aot_call_closure");
    assert!(
        matches!(closure, Some("tier1") | Some("tier2b") | Some("tier3")),
        "unexpected tier for call_closure: {closure:?}"
    );
    let load_g = tier_for_import("__syx_aot_load_global");
    assert!(
        matches!(load_g, Some("tier1") | Some("tier2b") | Some("tier3")),
        "unexpected tier for load_global: {load_g:?}"
    );
}

#[test]
fn p98_tier3_fingerprint() {
    assert!(include_str!("../../src/lib.rs").contains("wasm-aot-zero-run-op-v1"));
}
