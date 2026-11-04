//! Phase 104 Track D: locals v10 — frame-owned Values, no TLS `FRAME_VALUE_SLABS`.


#[test]
fn p104_native_llvm_locals_v10_llvm_surface() {
    let llvm = include_str!("../../../syx-llvm/src/lib.rs");
    assert!(!llvm.contains("syx_rt_pop_to_local_bits"));
    assert!(!llvm.contains("syx_rt_frame_local_bits"));
    assert!(llvm.contains("syx_rt_frame_local_ptr"));
}
