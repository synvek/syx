//! SYX driver Cranelift retirement stubs + macro warning drain.
//! Extracted in S2-I234 (zero semantics).

use syx_diagnostics::{SyxDiagnostic, SyxResult};

/// Phase 59: Cranelift backend physically removed; LLVM is the sole native backend.
pub fn cranelift_backend_retired() -> bool {
    true
}

/// Retired Cranelift bench stubs (p37 legacy tests).
pub fn cranelift_p20_bench_enabled() -> bool {
    false
}

pub fn cranelift_p20_bench_target_bytes() -> u64 {
    0
}

pub fn cranelift_gc_intrinsics_parity_enabled() -> bool {
    false
}

pub fn cranelift_unified_gc_promotion_codegen_enabled() -> bool {
    false
}

pub fn compile_module_to_object(_module: &syx_mir::MirModule) -> SyxResult<Vec<u8>> {
    Err(SyxDiagnostic::error(
        "Cranelift backend removed in Phase 59; use LLVM (--backend llvm)",
    ))
}

pub fn emit_clif_module_text(_module: &syx_mir::MirModule) -> String {
    "; Cranelift backend retired (Phase 59)\n".into()
}

pub fn take_macro_warnings() -> Vec<SyxDiagnostic> {
    syx_macro::take_macro_warnings()
}
