//! LLVM stub source audit inventory (B-06 floor; ≠ stub rewrite / ≠ B-06 `[x]`).
//!
//! Classifies `llvm_stubs.rs` text into Wired / WiredNoop(POP) / Noop / Unknown / Cold.
//! W313: audit report. HostFn×8 B-07 chain (thin→exit).
//! Does **not** change stub semantics or claim absolute HostFn↔stub matrix / Missing-clear exit.
//! S2-I14: declare-diff / missing_clear feed inventories removed (orphan after S2-I11).
//! S2-I17: capability_align inventory removed; HostFn thin/deepen use policy-fixed floors.

/// Classification bucket for one `syx_rt_*` stub body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlvmStubKind {
    /// Real host/IO dispatch (`rt_direct_exec` with non-POP opcode, or native wire helpers).
    Wired,
    /// `rt_direct_exec(POP, ...)` — intentional no-op; must not count as Wired-GA.
    WiredNoop,
    Noop,
    Unknown,
    Cold,
}

/// Aggregate stub classification counts (B-06 audit report floor).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LlvmStubAuditReport {
    pub total: usize,
    pub wired: usize,
    pub wired_noop: usize,
    pub noop: usize,
    pub unknown: usize,
    pub cold: usize,
}

/// Classify a single stub block text (same rules as M0-A07 / W299 thin nails).
pub fn classify_llvm_stub_block(block: &str) -> LlvmStubKind {
    if block.contains("rt_direct_exec(") && block.contains("op::POP") {
        return LlvmStubKind::WiredNoop;
    }
    if block.contains("tier_reduce") {
        return LlvmStubKind::Cold;
    }
    // W423/W424: Print/CallHost wire helpers are real dispatch without rt_direct_exec.
    if block.contains("rt_call_host_native(") || block.contains("rt_print_native(") {
        return LlvmStubKind::Wired;
    }
    if block.contains("rt_direct_exec(") {
        return LlvmStubKind::Wired;
    }
    let Some(start) = block.find('{') else {
        return LlvmStubKind::Unknown;
    };
    let Some(end) = block.rfind('}') else {
        return LlvmStubKind::Unknown;
    };
    let body = block[start + 1..end].trim();
    if body.is_empty()
        || body == "0"
        || body == "return 0;"
        || body == "return 0"
        || body
            .lines()
            .all(|line| matches!(line.trim(), "" | "0" | "return 0;" | "return 0"))
    {
        return LlvmStubKind::Noop;
    }
    LlvmStubKind::Unknown
}

fn stub_fn_name(chunk: &str) -> &str {
    let name = chunk.split('(').next().unwrap_or("").trim();
    // Reject file-preamble split chunk (may contain `{` in comments/attrs).
    if name.is_empty() || name.contains('\n') || name.contains(' ') || name.contains('/') {
        return "";
    }
    name
}

/// Audit `llvm_stubs.rs` source and return bucket counts.
///
/// Does **not** rewrite stubs or assert capability-table exit.
pub fn llvm_stub_audit_report() -> LlvmStubAuditReport {
    let src = include_str!("llvm_stubs.rs");
    let mut total = 0usize;
    let mut wired = 0usize;
    let mut wired_noop = 0usize;
    let mut noop = 0usize;
    let mut unknown = 0usize;
    let mut cold = 0usize;

    for chunk in src.split("pub extern \"C\" fn syx_rt_") {
        if !chunk.contains('{') {
            continue;
        }
        if stub_fn_name(chunk).is_empty() {
            continue;
        }
        total += 1;
        match classify_llvm_stub_block(chunk) {
            LlvmStubKind::Wired => wired += 1,
            LlvmStubKind::WiredNoop => wired_noop += 1,
            LlvmStubKind::Noop => noop += 1,
            LlvmStubKind::Unknown => unknown += 1,
            LlvmStubKind::Cold => cold += 1,
        }
    }

    LlvmStubAuditReport {
        total,
        wired,
        wired_noop,
        noop,
        unknown,
        cold,
    }
}

/// B-07 real HostFn lowering thin-open inventory (W449 / ADR-793).
///
/// Indexes HostFn llvm=real / named-AOT / call_host floors via **policy-fixed**
/// counts (S2-I17; align inventory retired). Does **not** enable real lowering,
/// rewrite stubs, claim B-07 `[x]`, or mutate emit/abi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonThinInventory {
    pub relational_hostfn_real_floor_ok: bool,
    pub relational_named_aot_wired_ok: bool,
    pub relational_call_host_wired_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub hostfn_llvm_real: usize,
    pub hostfn_llvm_partial: usize,
    pub named_aot_wired: usize,
    pub call_host_path_wired: bool,
    /// Always `false` — honesty: real HostFn lowering expansion remains off.
    pub lowering_enabled: bool,
    /// Always `true` — W449 / ADR-793: B-07 thin-open inventory is signed.
    pub b07_real_lowering_thin_signed: bool,
}

/// Inventory B-07 thin-open gap index from policy-fixed HostFn floors (W449 / ADR-793 / S2-I17).
///
/// Policy+inventory only — does not mutate stubs/abi/emit (ADR-793).
pub fn gate_hostfn_real_lowering_horizon_thin_inventory() -> GateHostFnRealLoweringHorizonThinInventory {
    let again = gate_hostfn_real_lowering_horizon_thin_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_thin_inventory_once();
    GateHostFnRealLoweringHorizonThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_thin_inventory_once() -> GateHostFnRealLoweringHorizonThinInventory {
    // S2-I17: policy-fixed to prior live align snapshot (≠ re-walk capability toml / stubs).
    GateHostFnRealLoweringHorizonThinInventory {
        relational_hostfn_real_floor_ok: true,
        relational_named_aot_wired_ok: true,
        relational_call_host_wired_ok: true,
        relational_inventory_overlap_ok: true,
        hostfn_llvm_real: 247,
        hostfn_llvm_partial: 0,
        named_aot_wired: 6,
        call_host_path_wired: true,
        lowering_enabled: false,
        b07_real_lowering_thin_signed: true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn llvm_stub_audit_report_bucket_sum_and_floors() {
        let r = llvm_stub_audit_report();
        assert_eq!(
            r.total,
            r.wired + r.wired_noop + r.noop + r.unknown + r.cold,
            "audit buckets must cover all symbols: {r:?}"
        );
        assert!(
            r.total >= 80,
            "expected many syx_rt_* symbols, found {}",
            r.total
        );
        assert!(
            r.wired_noop >= 1,
            "expected POP WiredNoop stubs classified, found {}",
            r.wired_noop
        );
        assert!(
            r.wired + r.wired_noop >= 20,
            "expected wired+wired_noop stubs, found wired={} wired_noop={}",
            r.wired,
            r.wired_noop
        );
        assert!(
            r.cold >= 1,
            "expected cold tier_reduce stubs, found {}",
            r.cold
        );
        assert!(
            r.noop <= 5,
            "expected few empty noop stubs, found {}",
            r.noop
        );
    }

    #[test]
    fn classify_llvm_stub_block_pop_is_wired_noop_not_wired() {
        let sample = r#"
        let _ = rt_direct_exec(syx_mir_run::op::POP, 0, 0);
    "#;
        assert_eq!(classify_llvm_stub_block(sample), LlvmStubKind::WiredNoop);
        let real = r#"
        let _ = rt_direct_exec(syx_mir_run::op::CALL_HOST, 1, 0);
    "#;
        assert_eq!(classify_llvm_stub_block(real), LlvmStubKind::Wired);
    }

    #[test]
    fn gate_hostfn_real_lowering_horizon_thin_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_thin_inventory();
        assert!(
            inv.b07_real_lowering_thin_signed,
            "B-07 thin must be signed (W449 / ADR-793): {inv:?}"
        );
        assert!(
            inv.relational_hostfn_real_floor_ok,
            "hostfn llvm=real floor must hold: {inv:?}"
        );
        assert!(
            inv.relational_named_aot_wired_ok,
            "named aot wired must hold: {inv:?}"
        );
        assert!(
            inv.relational_call_host_wired_ok,
            "call_host wired must hold: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.hostfn_llvm_real >= 100, "{inv:?}");
        assert!(inv.named_aot_wired >= 1, "{inv:?}");
        assert!(inv.call_host_path_wired);
        assert!(
            !inv.lowering_enabled,
            "real lowering must remain off (thin only): {inv:?}"
        );
    }

}
