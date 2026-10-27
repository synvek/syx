//! Phase 104 Track D: locals v10 — frame-owned Values, no TLS `FRAME_VALUE_SLABS`.

#[test]
fn p104_native_llvm_locals_v10_fingerprint() {
    let abi = include_str!("../../../syx-rt/src/abi.rs");
    assert!(
        abi.contains("native-llvm-locals-v10:no-tls-slab"),
        "locals v10 fingerprint must be in abi.rs"
    );
    assert!(
        !abi.contains("thread_local!") || !abi.contains("FRAME_VALUE"),
        "hot path must not use TLS frame value slabs"
    );
    assert!(
        abi.contains("locals_v10_hard_fail_legacy_local_bits"),
        "legacy *_local_bits must hard-fail"
    );
    assert!(include_str!("../../src/lib.rs").contains("native-llvm-locals-v10:no-tls-slab"));
}

#[test]
fn p104_native_llvm_locals_v10_llvm_surface() {
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(!llvm.contains("syx_rt_pop_to_local_bits"));
    assert!(!llvm.contains("syx_rt_frame_local_bits"));
    assert!(llvm.contains("syx_rt_frame_local_ptr"));
}
