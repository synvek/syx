//! LLVM stub source audit (B-06 product floor; ≠ stub rewrite).
//!
//! Classifies `llvm_stubs.rs` text into Wired / WiredNoop(POP) / Noop / Unknown / Cold.
//! W313: audit report. HostFn×8 B-07 chain (thin→exit).
//! Does **not** change stub semantics or claim absolute HostFn↔stub matrix / Missing-clear exit.
//! S2-I14: declare-diff / missing_clear feed inventories removed (orphan after S2-I11).
//! S2-I17: capability_align inventory removed; HostFn thin/deepen use policy-fixed floors.
//! S2-I134: accepted as **B-06 product capability** (not S2-01 process-governance) —
//! see `docs/adr/stage2-llvm-stub-audit-product-accept.md`.

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
}
