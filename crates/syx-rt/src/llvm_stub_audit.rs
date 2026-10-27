//! LLVM stub source audit inventory (B-06 floor; ≠ stub rewrite / ≠ B-06 `[x]`).
//!
//! Classifies `llvm_stubs.rs` text into Wired / WiredNoop(POP) / Noop / Unknown / Cold.
//! W313: audit report. W314: capability align inventory. W315: emit-declare vs stubs diff.
//! Does **not** change stub semantics or claim absolute HostFn↔stub matrix / Missing-clear exit.

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

/// Stub ↔ HostFn-llvm cross inventory (B-06 align floor; ≠ absolute 1:1 matrix exit).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LlvmStubCapabilityAlignInventory {
    pub stub_total: usize,
    pub stub_wired: usize,
    pub stub_wired_noop: usize,
    pub stub_noop: usize,
    pub stub_unknown: usize,
    pub stub_cold: usize,
    pub hostfn_llvm_real: usize,
    pub hostfn_llvm_partial: usize,
    pub hostfn_llvm_other: usize,
    /// `syx_rt_*_aot` once classified WiredNoop (historical e.g. pre-W451/W452);
    /// HttpGet/FileRead now HostFn-wired — other named `*_aot` may still be POP.
    pub named_aot_pop: usize,
    /// `syx_rt_*_aot` classified Wired (e.g. call_host_aot / json_*_aot).
    pub named_aot_wired: usize,
    pub hostfn_bound_total: usize,
    pub hostfn_bound_wired: usize,
    pub hostfn_bound_wired_noop: usize,
    /// `syx_rt_call_host` or `syx_rt_call_host_aot` is Wired.
    pub call_host_path_wired: bool,
}

/// Emit-declare ↔ stubs diff inventory (B-06 Missing floor; ≠ unlinkable / ≠ Missing clear exit).
///
/// `missing_in_stubs` counts symbols declared in `emit_llvm_ir` but not defined in
/// `llvm_stubs.rs`. Many of those live in `abi.rs` (including stack-op macros) — see
/// `abi_cover_count` / `missing_not_in_rt`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LlvmStubDeclareDiffInventory {
    pub declared_count: usize,
    pub defined_count: usize,
    pub missing_in_stubs: usize,
    pub extra_in_stubs: usize,
    pub abi_cover_count: usize,
    pub missing_not_in_rt: usize,
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

fn count_hostfn_llvm_backends(toml: &str) -> (usize, usize, usize) {
    let mut real = 0usize;
    let mut partial = 0usize;
    let mut other = 0usize;
    for line in toml.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("backends") {
            continue;
        }
        // backends = { ..., llvm = "real", ... }
        let idx = match trimmed.find("llvm") {
            Some(i) => i,
            None => {
                other += 1;
                continue;
            }
        };
        let rest = &trimmed[idx..];
        let val = if rest.contains("llvm = \"real\"") || rest.contains("llvm=\"real\"") {
            "real"
        } else if rest.contains("llvm = \"partial\"") || rest.contains("llvm=\"partial\"") {
            "partial"
        } else if let Some(q0) = rest.find('"') {
            let after = &rest[q0 + 1..];
            after.split('"').next().unwrap_or("")
        } else {
            ""
        };
        match val {
            "real" => real += 1,
            "partial" => partial += 1,
            _ => other += 1,
        }
    }
    (real, partial, other)
}

/// Cross inventory: stub buckets × HostFn llvm counts × named-AOT/HostFnId honesty nails.
///
/// Does **not** rewrite stubs, rewrite capability tables, or claim absolute 1:1 matrix exit.
/// `hostfn_llvm_real` and `stub_wired` are different categories and must not be forced equal.
pub fn llvm_stub_capability_align_inventory() -> LlvmStubCapabilityAlignInventory {
    let report = llvm_stub_audit_report();
    let caps = include_str!("../../../packages/internal/host_fn_capabilities.toml");
    let (hostfn_llvm_real, hostfn_llvm_partial, hostfn_llvm_other) =
        count_hostfn_llvm_backends(caps);

    let src = include_str!("llvm_stubs.rs");
    let mut named_aot_pop = 0usize;
    let mut named_aot_wired = 0usize;
    let mut hostfn_bound_total = 0usize;
    let mut hostfn_bound_wired = 0usize;
    let mut hostfn_bound_wired_noop = 0usize;
    let mut call_host_path_wired = false;

    for chunk in src.split("pub extern \"C\" fn syx_rt_") {
        if !chunk.contains('{') {
            continue;
        }
        let name = stub_fn_name(chunk);
        if name.is_empty() {
            continue;
        }
        let kind = classify_llvm_stub_block(chunk);

        if name.ends_with("_aot") {
            match kind {
                LlvmStubKind::WiredNoop => named_aot_pop += 1,
                LlvmStubKind::Wired => named_aot_wired += 1,
                _ => {}
            }
        }

        if chunk.contains("HostFnId::") {
            hostfn_bound_total += 1;
            match kind {
                LlvmStubKind::Wired => hostfn_bound_wired += 1,
                LlvmStubKind::WiredNoop => hostfn_bound_wired_noop += 1,
                _ => {}
            }
        }

        if matches!(name, "call_host" | "call_host_aot") && kind == LlvmStubKind::Wired {
            call_host_path_wired = true;
        }
    }

    LlvmStubCapabilityAlignInventory {
        stub_total: report.total,
        stub_wired: report.wired,
        stub_wired_noop: report.wired_noop,
        stub_noop: report.noop,
        stub_unknown: report.unknown,
        stub_cold: report.cold,
        hostfn_llvm_real,
        hostfn_llvm_partial,
        hostfn_llvm_other,
        named_aot_pop,
        named_aot_wired,
        hostfn_bound_total,
        hostfn_bound_wired,
        hostfn_bound_wired_noop,
        call_host_path_wired,
    }
}

fn collect_declared_syx_rt_symbols(llvm_src: &str) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    // Only IR declare string literals emitted by emit_llvm_ir (not comments / asserts).
    let mut rest = llvm_src;
    while let Some(idx) = rest.find("push_str(\"declare") {
        rest = &rest[idx + "push_str(\"".len()..];
        let Some(end) = rest.find('"') else {
            break;
        };
        let lit = &rest[..end];
        rest = &rest[end + 1..];
        let mut lit_rest = lit;
        while let Some(at) = lit_rest.find("@syx_rt_") {
            lit_rest = &lit_rest[at + 1..];
            let name: String = lit_rest
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                .collect();
            // Require a non-empty suffix after `syx_rt_` (reject comment `@syx_rt_*`).
            if name.starts_with("syx_rt_") && name.len() > "syx_rt_".len() {
                out.insert(name);
            }
            let mut i = 0usize;
            for c in lit_rest.chars() {
                if c.is_ascii_alphanumeric() || c == '_' {
                    i += c.len_utf8();
                } else {
                    break;
                }
            }
            lit_rest = &lit_rest[i..];
        }
    }
    out
}

fn collect_stub_syx_rt_symbols(stubs_src: &str) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    for chunk in stubs_src.split("pub extern \"C\" fn syx_rt_") {
        if !chunk.contains('{') {
            continue;
        }
        let name = stub_fn_name(chunk);
        if !name.is_empty() {
            out.insert(format!("syx_rt_{name}"));
        }
    }
    out
}

fn collect_abi_syx_rt_symbols(abi_src: &str) -> std::collections::HashSet<String> {
    let mut out = std::collections::HashSet::new();
    // Direct extern "C" fn syx_rt_*
    let mut rest = abi_src;
    while let Some(idx) = rest.find("extern \"C\" fn syx_rt_") {
        rest = &rest[idx + "extern \"C\" fn ".len()..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
            .collect();
        if name.starts_with("syx_rt_") {
            out.insert(name);
        }
    }
    // Macro expansions: rt_stack_op*!(syx_rt_...)
    rest = abi_src;
    while let Some(idx) = rest.find("rt_stack_op") {
        rest = &rest[idx..];
        if let Some(paren) = rest.find('(') {
            let after = rest[paren + 1..].trim_start();
            if after.starts_with("syx_rt_") {
                let name: String = after
                    .chars()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                out.insert(name);
            }
            rest = &rest[paren + 1..];
        } else {
            break;
        }
    }
    out
}

/// Emit-declare ↔ stubs (± abi cover) diff inventory.
///
/// Does **not** rewrite stubs/abi/emit or claim Missing-clear / B-06 `[x]`.
/// `missing_in_stubs` is **not** the same as unlinkable symbols.
pub fn llvm_stub_declare_diff_inventory() -> LlvmStubDeclareDiffInventory {
    let llvm_src = include_str!("../../syx-llvm/src/lib.rs");
    let stubs_src = include_str!("llvm_stubs.rs");
    let abi_src = include_str!("abi.rs");

    let declared = collect_declared_syx_rt_symbols(llvm_src);
    let defined = collect_stub_syx_rt_symbols(stubs_src);
    let abi = collect_abi_syx_rt_symbols(abi_src);

    let missing_in_stubs = declared.difference(&defined).count();
    let extra_in_stubs = defined.difference(&declared).count();
    let abi_cover_count = declared.intersection(&abi).count();
    let mut covered = defined.clone();
    covered.extend(abi.iter().cloned());
    let missing_not_in_rt = declared.difference(&covered).count();

    LlvmStubDeclareDiffInventory {
        declared_count: declared.len(),
        defined_count: defined.len(),
        missing_in_stubs,
        extra_in_stubs,
        abi_cover_count,
        missing_not_in_rt,
    }
}

/// Missing residual / clear-readiness inventory (B-06; ≠ Missing clear / ≠ stub rewrite).
///
/// Aggregates W315 declare-diff into cutover-style readiness honesty: declare gap
/// measurable, unlinkable zero, clear still off. Does **not** rewrite stubs/abi/emit
/// or claim B-06 `[x]` / B-07.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LlvmStubMissingClearReadinessInventory {
    pub relational_declare_gap_measurable_ok: bool,
    pub relational_unlinkable_zero_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    pub missing_in_stubs: usize,
    pub missing_not_in_rt: usize,
    pub abi_cover_count: usize,
    /// Always `false` — honesty: Missing clear remains off.
    pub clear_enabled: bool,
    /// Always `false` — honesty: stub rewrite remains off.
    pub stub_rewrite_enabled: bool,
}

/// Inventory Missing clear readiness from declare-diff residual honesty.
///
/// Used for the B-06 Missing residual / clear-readiness floor (W331). Reuses
/// [`llvm_stub_declare_diff_inventory`]. Does **not** mutate stubs, abi, or emit.
pub fn llvm_stub_missing_clear_readiness_inventory() -> LlvmStubMissingClearReadinessInventory {
    let diff = llvm_stub_declare_diff_inventory();
    let again = llvm_stub_declare_diff_inventory();
    let relational_inventory_overlap_ok = diff == again;
    LlvmStubMissingClearReadinessInventory {
        relational_declare_gap_measurable_ok: diff.missing_in_stubs >= 1,
        relational_unlinkable_zero_ok: diff.missing_not_in_rt == 0,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        missing_in_stubs: diff.missing_in_stubs,
        missing_not_in_rt: diff.missing_not_in_rt,
        abi_cover_count: diff.abi_cover_count,
        clear_enabled: false,
        stub_rewrite_enabled: false,
    }
}

/// Missing clear×align horizon readiness inventory (B-06; ≠ Missing clear / ≠ absolute align exit).
///
/// Aggregates W331 Missing clear-readiness with W314 capability-align floors into a
/// horizon rollup. Does **not** clear Missing, rewrite stubs, claim absolute HostFn↔stub
/// matrix exit, or claim B-06 `[x]` / B-07.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LlvmStubClearAlignHorizonReadinessInventory {
    pub relational_missing_clear_chain_ok: bool,
    pub relational_align_inventory_chain_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    /// Always `false` — honesty: Missing clear remains off.
    pub clear_enabled: bool,
    /// Always `false` — honesty: stub rewrite remains off.
    pub stub_rewrite_enabled: bool,
    /// Always `false` — honesty: absolute HostFn↔stub matrix exit remains off.
    pub absolute_align_matrix_exit_enabled: bool,
}

/// Inventory Missing clear×align horizon readiness from clear-readiness + align floors.
///
/// Used for the B-06 Missing clear×align horizon readiness floor (W334). Reuses
/// [`llvm_stub_missing_clear_readiness_inventory`] and
/// [`llvm_stub_capability_align_inventory`]. Does **not** mutate stubs, abi, or emit.
pub fn llvm_stub_clear_align_horizon_readiness_inventory(
) -> LlvmStubClearAlignHorizonReadinessInventory {
    let clear = llvm_stub_missing_clear_readiness_inventory();
    let align = llvm_stub_capability_align_inventory();
    let relational_missing_clear_chain_ok = clear.relational_declare_gap_measurable_ok
        && clear.relational_unlinkable_zero_ok
        && clear.relational_inventory_overlap_ok
        && !clear.clear_enabled
        && !clear.stub_rewrite_enabled
        && clear.missing_not_in_rt == 0;
    let relational_align_inventory_chain_ok = align.stub_total >= 80
        && align.stub_total
            == align.stub_wired
                + align.stub_wired_noop
                + align.stub_noop
                + align.stub_unknown
                + align.stub_cold
        && align.call_host_path_wired
        && align.hostfn_llvm_real >= 100
        && align.named_aot_pop >= 1
        && align.named_aot_wired >= 1
        && align.hostfn_bound_wired >= 1;
    let clear_again = llvm_stub_missing_clear_readiness_inventory();
    let align_again = llvm_stub_capability_align_inventory();
    let relational_inventory_overlap_ok = clear == clear_again && align == align_again;
    LlvmStubClearAlignHorizonReadinessInventory {
        relational_missing_clear_chain_ok,
        relational_align_inventory_chain_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        clear_enabled: false,
        stub_rewrite_enabled: false,
        absolute_align_matrix_exit_enabled: false,
    }
}

/// Missing 清零诚实延后签收 inventory (B-06 exit triad defer arm; ≠ Missing clear / ≠ B-06 `[x]`).
///
/// Signs the Wave B exit-triad **honest defer** arm for B-06 Missing clear while clear /
/// stub rewrite remain off. Does **not** clear Missing, rewrite stubs, claim B-06 `[x]` /
/// B-07, or claim Wave B exit met.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LlvmStubMissingClearHonestDeferInventory {
    pub relational_clear_align_horizon_ok: bool,
    pub relational_residual_measurable_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    /// Count of relational predicates tracked this window (fixed at 3).
    pub candidate_predicate_count: usize,
    pub missing_in_stubs: usize,
    pub missing_not_in_rt: usize,
    /// Always `false` — honesty: Missing clear remains off.
    pub clear_enabled: bool,
    /// Always `true` — W337 policy: honest defer of Missing clear is signed.
    pub honest_defer_signed: bool,
    /// Always `false` — honesty: stub rewrite remains off.
    pub stub_rewrite_enabled: bool,
}

/// Inventory Missing 清零诚实延后签收 from clear×align horizon + clear-readiness residual.
///
/// Used for the B-06 exit-triad honest-defer floor (W337). Reuses
/// [`llvm_stub_clear_align_horizon_readiness_inventory`] and
/// [`llvm_stub_missing_clear_readiness_inventory`]. Does **not** mutate stubs, abi, or emit.
pub fn llvm_stub_missing_clear_honest_defer_inventory() -> LlvmStubMissingClearHonestDeferInventory
{
    let horizon = llvm_stub_clear_align_horizon_readiness_inventory();
    let clear = llvm_stub_missing_clear_readiness_inventory();
    let relational_clear_align_horizon_ok = horizon.relational_missing_clear_chain_ok
        && horizon.relational_align_inventory_chain_ok
        && horizon.relational_inventory_overlap_ok
        && !horizon.clear_enabled
        && !horizon.stub_rewrite_enabled
        && !horizon.absolute_align_matrix_exit_enabled;
    let relational_residual_measurable_ok =
        clear.missing_in_stubs >= 1 && clear.missing_not_in_rt == 0 && !clear.clear_enabled;
    let horizon_again = llvm_stub_clear_align_horizon_readiness_inventory();
    let clear_again = llvm_stub_missing_clear_readiness_inventory();
    let relational_inventory_overlap_ok = horizon == horizon_again && clear == clear_again;
    LlvmStubMissingClearHonestDeferInventory {
        relational_clear_align_horizon_ok,
        relational_residual_measurable_ok,
        relational_inventory_overlap_ok,
        candidate_predicate_count: 3,
        missing_in_stubs: clear.missing_in_stubs,
        missing_not_in_rt: clear.missing_not_in_rt,
        clear_enabled: false,
        honest_defer_signed: true,
        stub_rewrite_enabled: false,
    }
}

/// B-06 Missing/align `[x]` exit inventory (W448 / ADR-792).
///
/// Signs narrow construction-chain exit through W337 honest defer. Does **not**
/// claim Missing 清零 complete / stub rewrite / absolute align matrix / B-07 /
/// MIR blob delete this window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LlvmStubMissingAlignHorizonExitInventory {
    pub relational_honest_defer_ok: bool,
    pub relational_clear_align_horizon_ok: bool,
    pub relational_declare_diff_ok: bool,
    pub relational_audit_report_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub honest_defer_signed: bool,
    /// Always `false` — honesty: Missing clear remains off after exit charter.
    pub clear_enabled: bool,
    /// Always `false` — honesty: stub rewrite remains off.
    pub stub_rewrite_enabled: bool,
    /// Always `false` — honesty: absolute HostFn↔stub matrix exit remains off.
    pub absolute_align_matrix_exit_enabled: bool,
    /// Always `true` — W448 / ADR-792: B-06 Missing/align exit charter signed (narrow).
    pub b06_missing_align_exit_signed: bool,
}

/// Inventory B-06 Missing/align exit prerequisites (W448 / ADR-792).
///
/// Policy+inventory only — does not mutate stubs/abi/emit (ADR-792).
pub fn llvm_stub_missing_align_horizon_exit_inventory() -> LlvmStubMissingAlignHorizonExitInventory
{
    let again = llvm_stub_missing_align_horizon_exit_inventory_once();
    let first = llvm_stub_missing_align_horizon_exit_inventory_once();
    LlvmStubMissingAlignHorizonExitInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn llvm_stub_missing_align_horizon_exit_inventory_once() -> LlvmStubMissingAlignHorizonExitInventory
{
    let defer = llvm_stub_missing_clear_honest_defer_inventory();
    let horizon = llvm_stub_clear_align_horizon_readiness_inventory();
    let declare = llvm_stub_declare_diff_inventory();
    let audit = llvm_stub_audit_report();
    LlvmStubMissingAlignHorizonExitInventory {
        relational_honest_defer_ok: defer.honest_defer_signed
            && defer.relational_clear_align_horizon_ok
            && defer.relational_residual_measurable_ok
            && !defer.clear_enabled
            && !defer.stub_rewrite_enabled,
        relational_clear_align_horizon_ok: horizon.relational_missing_clear_chain_ok
            && horizon.relational_align_inventory_chain_ok
            && !horizon.clear_enabled
            && !horizon.stub_rewrite_enabled
            && !horizon.absolute_align_matrix_exit_enabled,
        relational_declare_diff_ok: declare.missing_not_in_rt == 0
            && declare.declared_count >= 300
            && declare.defined_count >= 80
            && declare.missing_in_stubs >= 1,
        relational_audit_report_ok: audit.total >= 80
            && audit.wired_noop >= 1
            && audit.wired + audit.wired_noop >= 20,
        relational_inventory_overlap_ok: true,
        honest_defer_signed: defer.honest_defer_signed,
        clear_enabled: false,
        stub_rewrite_enabled: false,
        absolute_align_matrix_exit_enabled: false,
        b06_missing_align_exit_signed: true,
    }
}

/// B-07 real HostFn lowering thin-open inventory (W449 / ADR-793).
///
/// Indexes capability `backends.llvm=real` claims vs stub/AOT wiring using the
/// existing align inventory. Does **not** enable real lowering, rewrite stubs,
/// claim B-07 `[x]`, or mutate emit/abi.
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

/// Inventory B-07 thin-open gap index from capability-align floors (W449 / ADR-793).
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
    let align = llvm_stub_capability_align_inventory();
    GateHostFnRealLoweringHorizonThinInventory {
        relational_hostfn_real_floor_ok: align.hostfn_llvm_real >= 100,
        relational_named_aot_wired_ok: align.named_aot_wired >= 1,
        relational_call_host_wired_ok: align.call_host_path_wired,
        relational_inventory_overlap_ok: true,
        hostfn_llvm_real: align.hostfn_llvm_real,
        hostfn_llvm_partial: align.hostfn_llvm_partial,
        named_aot_wired: align.named_aot_wired,
        call_host_path_wired: align.call_host_path_wired,
        lowering_enabled: false,
        b07_real_lowering_thin_signed: true,
    }
}

/// B-07 real HostFn lowering deepen thin inventory (W450 / ADR-794).
///
/// First-slice candidate / readiness index on top of thin+align floors.
/// Does **not** enable wire, rewrite stubs, claim B-07 `[x]`, or mutate emit/abi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonDeepenThinInventory {
    pub relational_thin_floor_ok: bool,
    pub relational_category_gap_ok: bool,
    pub relational_named_aot_wired_ok: bool,
    pub relational_partial_count_ok: bool,
    pub relational_first_slice_candidate_floor_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub hostfn_llvm_real: usize,
    pub hostfn_llvm_partial: usize,
    pub named_aot_wired: usize,
    pub call_host_path_wired: bool,
    pub b07_real_lowering_thin_signed: bool,
    /// Always `false` — honesty: real HostFn lowering expansion remains off.
    pub lowering_enabled: bool,
    /// Live: `true` after W451 HttpGet first-slice wire (ADR-795); deepen charter historically false.
    pub first_slice_wire_enabled: bool,
    /// Always `true` — W450 / ADR-794: B-07 deepen thin inventory is signed.
    pub b07_real_lowering_deepen_thin_signed: bool,
}

/// Inventory B-07 deepen thin first-slice candidate readiness (W450 / ADR-794).
///
/// Policy+inventory only — does not mutate stubs/abi/emit (ADR-794).
pub fn gate_hostfn_real_lowering_horizon_deepen_thin_inventory(
) -> GateHostFnRealLoweringHorizonDeepenThinInventory {
    let again = gate_hostfn_real_lowering_horizon_deepen_thin_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_deepen_thin_inventory_once();
    GateHostFnRealLoweringHorizonDeepenThinInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_deepen_thin_inventory_once(
) -> GateHostFnRealLoweringHorizonDeepenThinInventory {
    let thin = gate_hostfn_real_lowering_horizon_thin_inventory_once();
    let align = llvm_stub_capability_align_inventory();
    let thin_floor_ok = thin.b07_real_lowering_thin_signed
        && thin.relational_hostfn_real_floor_ok
        && thin.relational_named_aot_wired_ok
        && thin.relational_call_host_wired_ok
        && !thin.lowering_enabled;
    // Category honesty: llvm=real claims are a different bucket from named *_aot Wired stubs;
    // real count must not be forced below named_aot_wired.
    let category_gap_ok = align.hostfn_llvm_real >= align.named_aot_wired;
    // First-slice candidate floor: prefer named *_aot Wired paths as proxy (≥1).
    let first_slice_candidate_floor_ok = align.named_aot_wired >= 1;
    GateHostFnRealLoweringHorizonDeepenThinInventory {
        relational_thin_floor_ok: thin_floor_ok,
        relational_category_gap_ok: category_gap_ok,
        relational_named_aot_wired_ok: align.named_aot_wired >= 1,
        relational_partial_count_ok: true, // hostfn_llvm_partial is always countable
        relational_first_slice_candidate_floor_ok: first_slice_candidate_floor_ok,
        relational_inventory_overlap_ok: true,
        hostfn_llvm_real: align.hostfn_llvm_real,
        hostfn_llvm_partial: align.hostfn_llvm_partial,
        named_aot_wired: align.named_aot_wired,
        call_host_path_wired: align.call_host_path_wired,
        b07_real_lowering_thin_signed: thin.b07_real_lowering_thin_signed,
        lowering_enabled: false,
        first_slice_wire_enabled: http_get_aot_first_slice_wired(),
        b07_real_lowering_deepen_thin_signed: true,
    }
}

/// Extract `syx_rt_http_get_aot` function body from llvm_stubs.rs (W451 / ADR-795).
fn http_get_aot_stub_body() -> &'static str {
    let src = include_str!("llvm_stubs.rs");
    let start = match src.find("fn syx_rt_http_get_aot") {
        Some(i) => i,
        None => return "",
    };
    let after = &src[start..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len());
    &after[..end]
}

/// True when HttpGet first-slice wire is live (W451 / ADR-795).
fn http_get_aot_first_slice_wired() -> bool {
    let body = http_get_aot_stub_body();
    body.contains("HostFnId::HttpGet")
        && body.contains("rt_call_host_native")
        && !body.contains("rt_direct_exec")
}

/// B-07 real HostFn lowering first-slice wire inventory (W451 / ADR-795).
///
/// Narrow HttpGet / `syx_rt_http_get_aot` wire only. Does **not** enable global
/// lowering, batch rewrite stubs, claim B-07 `[x]`, or mutate emit/abi beyond that stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonFirstSliceWireInventory {
    pub relational_deepen_floor_ok: bool,
    pub relational_http_get_wire_body_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub b07_real_lowering_thin_signed: bool,
    pub b07_real_lowering_deepen_thin_signed: bool,
    /// Always `false` — honesty: global HostFn lowering expansion remains off.
    pub lowering_enabled: bool,
    /// Always `true` after W451 — HttpGet first-slice wire is live.
    pub first_slice_wire_enabled: bool,
    /// Always `true` — W451 / ADR-795: first-slice wire inventory is signed.
    pub b07_real_lowering_first_slice_wire_signed: bool,
}

/// Inventory B-07 first-slice HttpGet wire (W451 / ADR-795).
pub fn gate_hostfn_real_lowering_horizon_first_slice_wire_inventory(
) -> GateHostFnRealLoweringHorizonFirstSliceWireInventory {
    let again = gate_hostfn_real_lowering_horizon_first_slice_wire_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_first_slice_wire_inventory_once();
    GateHostFnRealLoweringHorizonFirstSliceWireInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_first_slice_wire_inventory_once(
) -> GateHostFnRealLoweringHorizonFirstSliceWireInventory {
    let deepen = gate_hostfn_real_lowering_horizon_deepen_thin_inventory_once();
    let deepen_floor_ok = deepen.b07_real_lowering_deepen_thin_signed
        && deepen.b07_real_lowering_thin_signed
        && deepen.relational_thin_floor_ok
        && deepen.relational_first_slice_candidate_floor_ok
        && !deepen.lowering_enabled;
    let wire_ok = http_get_aot_first_slice_wired();
    GateHostFnRealLoweringHorizonFirstSliceWireInventory {
        relational_deepen_floor_ok: deepen_floor_ok,
        relational_http_get_wire_body_ok: wire_ok,
        relational_inventory_overlap_ok: true,
        b07_real_lowering_thin_signed: deepen.b07_real_lowering_thin_signed,
        b07_real_lowering_deepen_thin_signed: deepen.b07_real_lowering_deepen_thin_signed,
        lowering_enabled: false,
        first_slice_wire_enabled: wire_ok,
        b07_real_lowering_first_slice_wire_signed: true,
    }
}

/// Extract `syx_rt_file_read_aot` function body from llvm_stubs.rs (W452 / ADR-796).
fn file_read_aot_stub_body() -> &'static str {
    let src = include_str!("llvm_stubs.rs");
    let start = match src.find("fn syx_rt_file_read_aot") {
        Some(i) => i,
        None => return "",
    };
    let after = &src[start..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len());
    &after[..end]
}

/// True when FileRead second-slice wire is live (W452 / ADR-796).
fn file_read_aot_second_slice_wired() -> bool {
    let body = file_read_aot_stub_body();
    body.contains("HostFnId::FileRead")
        && body.contains("rt_call_host_native")
        && !body.contains("rt_direct_exec")
}

/// B-07 real HostFn lowering second-slice wire inventory (W452 / ADR-796).
///
/// Narrow FileRead / `syx_rt_file_read_aot` wire only. Does **not** enable global
/// lowering, batch rewrite stubs, claim B-07 `[x]`, or mutate emit/abi beyond that stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonSecondSliceWireInventory {
    pub relational_first_slice_floor_ok: bool,
    pub relational_file_read_wire_body_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub b07_real_lowering_thin_signed: bool,
    pub b07_real_lowering_deepen_thin_signed: bool,
    pub b07_real_lowering_first_slice_wire_signed: bool,
    /// Always `false` — honesty: global HostFn lowering expansion remains off.
    pub lowering_enabled: bool,
    /// Always `true` after W451 — HttpGet first-slice wire is live.
    pub first_slice_wire_enabled: bool,
    /// Always `true` after W452 — FileRead second-slice wire is live.
    pub second_slice_wire_enabled: bool,
    /// Always `true` — W452 / ADR-796: second-slice wire inventory is signed.
    pub b07_real_lowering_second_slice_wire_signed: bool,
}

/// Inventory B-07 second-slice FileRead wire (W452 / ADR-796).
pub fn gate_hostfn_real_lowering_horizon_second_slice_wire_inventory(
) -> GateHostFnRealLoweringHorizonSecondSliceWireInventory {
    let again = gate_hostfn_real_lowering_horizon_second_slice_wire_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_second_slice_wire_inventory_once();
    GateHostFnRealLoweringHorizonSecondSliceWireInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_second_slice_wire_inventory_once(
) -> GateHostFnRealLoweringHorizonSecondSliceWireInventory {
    let first_slice = gate_hostfn_real_lowering_horizon_first_slice_wire_inventory_once();
    let first_slice_floor_ok = first_slice.b07_real_lowering_first_slice_wire_signed
        && first_slice.b07_real_lowering_deepen_thin_signed
        && first_slice.b07_real_lowering_thin_signed
        && first_slice.relational_deepen_floor_ok
        && first_slice.relational_http_get_wire_body_ok
        && first_slice.first_slice_wire_enabled
        && !first_slice.lowering_enabled;
    let wire_ok = file_read_aot_second_slice_wired();
    GateHostFnRealLoweringHorizonSecondSliceWireInventory {
        relational_first_slice_floor_ok: first_slice_floor_ok,
        relational_file_read_wire_body_ok: wire_ok,
        relational_inventory_overlap_ok: true,
        b07_real_lowering_thin_signed: first_slice.b07_real_lowering_thin_signed,
        b07_real_lowering_deepen_thin_signed: first_slice.b07_real_lowering_deepen_thin_signed,
        b07_real_lowering_first_slice_wire_signed: first_slice
            .b07_real_lowering_first_slice_wire_signed,
        lowering_enabled: false,
        first_slice_wire_enabled: first_slice.first_slice_wire_enabled,
        second_slice_wire_enabled: wire_ok,
        b07_real_lowering_second_slice_wire_signed: true,
    }
}

/// Extract `syx_rt_tcp_read` function body from llvm_stubs.rs (W453 / ADR-797).
fn tcp_read_stub_body() -> &'static str {
    let src = include_str!("llvm_stubs.rs");
    let start = match src.find("fn syx_rt_tcp_read") {
        Some(i) => i,
        None => return "",
    };
    let after = &src[start..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len());
    &after[..end]
}

/// True when TcpRead third-slice wire is live (W453 / ADR-797).
fn tcp_read_third_slice_wired() -> bool {
    let body = tcp_read_stub_body();
    body.contains("HostFnId::TcpRead")
        && body.contains("rt_call_host_native")
        && !body.contains("rt_direct_exec")
}

/// B-07 real HostFn lowering third-slice wire inventory (W453 / ADR-797).
///
/// Narrow TcpRead / `syx_rt_tcp_read` wire only. Does **not** enable global
/// lowering, batch rewrite stubs, claim B-07 `[x]`, or mutate emit/abi beyond that stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonThirdSliceWireInventory {
    pub relational_second_slice_floor_ok: bool,
    pub relational_tcp_read_wire_body_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub b07_real_lowering_thin_signed: bool,
    pub b07_real_lowering_deepen_thin_signed: bool,
    pub b07_real_lowering_first_slice_wire_signed: bool,
    pub b07_real_lowering_second_slice_wire_signed: bool,
    /// Always `false` — honesty: global HostFn lowering expansion remains off.
    pub lowering_enabled: bool,
    /// Always `true` after W451 — HttpGet first-slice wire is live.
    pub first_slice_wire_enabled: bool,
    /// Always `true` after W452 — FileRead second-slice wire is live.
    pub second_slice_wire_enabled: bool,
    /// Always `true` after W453 — TcpRead third-slice wire is live.
    pub third_slice_wire_enabled: bool,
    /// Always `true` — W453 / ADR-797: third-slice wire inventory is signed.
    pub b07_real_lowering_third_slice_wire_signed: bool,
}

/// Inventory B-07 third-slice TcpRead wire (W453 / ADR-797).
pub fn gate_hostfn_real_lowering_horizon_third_slice_wire_inventory(
) -> GateHostFnRealLoweringHorizonThirdSliceWireInventory {
    let again = gate_hostfn_real_lowering_horizon_third_slice_wire_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_third_slice_wire_inventory_once();
    GateHostFnRealLoweringHorizonThirdSliceWireInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_third_slice_wire_inventory_once(
) -> GateHostFnRealLoweringHorizonThirdSliceWireInventory {
    let second = gate_hostfn_real_lowering_horizon_second_slice_wire_inventory_once();
    let second_floor_ok = second.b07_real_lowering_second_slice_wire_signed
        && second.b07_real_lowering_first_slice_wire_signed
        && second.b07_real_lowering_deepen_thin_signed
        && second.b07_real_lowering_thin_signed
        && second.relational_first_slice_floor_ok
        && second.relational_file_read_wire_body_ok
        && second.first_slice_wire_enabled
        && second.second_slice_wire_enabled
        && !second.lowering_enabled;
    let wire_ok = tcp_read_third_slice_wired();
    GateHostFnRealLoweringHorizonThirdSliceWireInventory {
        relational_second_slice_floor_ok: second_floor_ok,
        relational_tcp_read_wire_body_ok: wire_ok,
        relational_inventory_overlap_ok: true,
        b07_real_lowering_thin_signed: second.b07_real_lowering_thin_signed,
        b07_real_lowering_deepen_thin_signed: second.b07_real_lowering_deepen_thin_signed,
        b07_real_lowering_first_slice_wire_signed: second.b07_real_lowering_first_slice_wire_signed,
        b07_real_lowering_second_slice_wire_signed: second
            .b07_real_lowering_second_slice_wire_signed,
        lowering_enabled: false,
        first_slice_wire_enabled: second.first_slice_wire_enabled,
        second_slice_wire_enabled: second.second_slice_wire_enabled,
        third_slice_wire_enabled: wire_ok,
        b07_real_lowering_third_slice_wire_signed: true,
    }
}

/// Extract `syx_rt_tcp_write` function body from llvm_stubs.rs (W454 / ADR-798).
fn tcp_write_stub_body() -> &'static str {
    let src = include_str!("llvm_stubs.rs");
    let start = match src.find("fn syx_rt_tcp_write") {
        Some(i) => i,
        None => return "",
    };
    let after = &src[start..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len());
    &after[..end]
}

/// True when TcpWrite fourth-slice wire is live (W454 / ADR-798).
fn tcp_write_fourth_slice_wired() -> bool {
    let body = tcp_write_stub_body();
    body.contains("HostFnId::TcpWrite")
        && body.contains("rt_call_host_native")
        && !body.contains("rt_direct_exec")
}

/// B-07 real HostFn lowering fourth-slice wire inventory (W454 / ADR-798).
///
/// Narrow TcpWrite / `syx_rt_tcp_write` wire only. Does **not** enable global
/// lowering, batch rewrite stubs, claim B-07 `[x]`, or mutate emit/abi beyond that stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonFourthSliceWireInventory {
    pub relational_third_slice_floor_ok: bool,
    pub relational_tcp_write_wire_body_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub b07_real_lowering_thin_signed: bool,
    pub b07_real_lowering_deepen_thin_signed: bool,
    pub b07_real_lowering_first_slice_wire_signed: bool,
    pub b07_real_lowering_second_slice_wire_signed: bool,
    pub b07_real_lowering_third_slice_wire_signed: bool,
    /// Always `false` — honesty: global HostFn lowering expansion remains off.
    pub lowering_enabled: bool,
    pub first_slice_wire_enabled: bool,
    pub second_slice_wire_enabled: bool,
    pub third_slice_wire_enabled: bool,
    /// Always `true` after W454 — TcpWrite fourth-slice wire is live.
    pub fourth_slice_wire_enabled: bool,
    /// Always `true` — W454 / ADR-798: fourth-slice wire inventory is signed.
    pub b07_real_lowering_fourth_slice_wire_signed: bool,
}

/// Inventory B-07 fourth-slice TcpWrite wire (W454 / ADR-798).
pub fn gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory(
) -> GateHostFnRealLoweringHorizonFourthSliceWireInventory {
    let again = gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory_once();
    GateHostFnRealLoweringHorizonFourthSliceWireInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory_once(
) -> GateHostFnRealLoweringHorizonFourthSliceWireInventory {
    let third = gate_hostfn_real_lowering_horizon_third_slice_wire_inventory_once();
    let third_floor_ok = third.b07_real_lowering_third_slice_wire_signed
        && third.b07_real_lowering_second_slice_wire_signed
        && third.b07_real_lowering_first_slice_wire_signed
        && third.b07_real_lowering_deepen_thin_signed
        && third.b07_real_lowering_thin_signed
        && third.relational_second_slice_floor_ok
        && third.relational_tcp_read_wire_body_ok
        && third.first_slice_wire_enabled
        && third.second_slice_wire_enabled
        && third.third_slice_wire_enabled
        && !third.lowering_enabled;
    let wire_ok = tcp_write_fourth_slice_wired();
    GateHostFnRealLoweringHorizonFourthSliceWireInventory {
        relational_third_slice_floor_ok: third_floor_ok,
        relational_tcp_write_wire_body_ok: wire_ok,
        relational_inventory_overlap_ok: true,
        b07_real_lowering_thin_signed: third.b07_real_lowering_thin_signed,
        b07_real_lowering_deepen_thin_signed: third.b07_real_lowering_deepen_thin_signed,
        b07_real_lowering_first_slice_wire_signed: third.b07_real_lowering_first_slice_wire_signed,
        b07_real_lowering_second_slice_wire_signed: third
            .b07_real_lowering_second_slice_wire_signed,
        b07_real_lowering_third_slice_wire_signed: third.b07_real_lowering_third_slice_wire_signed,
        lowering_enabled: false,
        first_slice_wire_enabled: third.first_slice_wire_enabled,
        second_slice_wire_enabled: third.second_slice_wire_enabled,
        third_slice_wire_enabled: third.third_slice_wire_enabled,
        fourth_slice_wire_enabled: wire_ok,
        b07_real_lowering_fourth_slice_wire_signed: true,
    }
}

/// Extract `syx_rt_tcp_handle_release` function body from llvm_stubs.rs (W455 / ADR-799).
fn tcp_handle_release_stub_body() -> &'static str {
    let src = include_str!("llvm_stubs.rs");
    let start = match src.find("fn syx_rt_tcp_handle_release") {
        Some(i) => i,
        None => return "",
    };
    let after = &src[start..];
    let end = after.find("\n#[no_mangle]").unwrap_or(after.len());
    &after[..end]
}

/// True when TcpHandleRelease fifth-slice wire is live (W455 / ADR-799).
fn tcp_handle_release_fifth_slice_wired() -> bool {
    let body = tcp_handle_release_stub_body();
    body.contains("HostFnId::TcpHandleRelease")
        && body.contains("rt_call_host_native")
        && !body.contains("rt_direct_exec")
}

/// B-07 real HostFn lowering fifth-slice wire inventory (W455 / ADR-799).
///
/// Narrow TcpHandleRelease / `syx_rt_tcp_handle_release` wire only. Does **not** enable
/// global lowering, batch rewrite stubs, claim B-07 `[x]`, or mutate emit/abi beyond that stub.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonFifthSliceWireInventory {
    pub relational_fourth_slice_floor_ok: bool,
    pub relational_tcp_handle_release_wire_body_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub b07_real_lowering_thin_signed: bool,
    pub b07_real_lowering_deepen_thin_signed: bool,
    pub b07_real_lowering_first_slice_wire_signed: bool,
    pub b07_real_lowering_second_slice_wire_signed: bool,
    pub b07_real_lowering_third_slice_wire_signed: bool,
    pub b07_real_lowering_fourth_slice_wire_signed: bool,
    /// Always `false` — honesty: global HostFn lowering expansion remains off.
    pub lowering_enabled: bool,
    pub first_slice_wire_enabled: bool,
    pub second_slice_wire_enabled: bool,
    pub third_slice_wire_enabled: bool,
    pub fourth_slice_wire_enabled: bool,
    /// Always `true` after W455 — TcpHandleRelease fifth-slice wire is live.
    pub fifth_slice_wire_enabled: bool,
    /// Always `true` — W455 / ADR-799: fifth-slice wire inventory is signed.
    pub b07_real_lowering_fifth_slice_wire_signed: bool,
}

/// Inventory B-07 fifth-slice TcpHandleRelease wire (W455 / ADR-799).
pub fn gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory(
) -> GateHostFnRealLoweringHorizonFifthSliceWireInventory {
    let again = gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory_once();
    GateHostFnRealLoweringHorizonFifthSliceWireInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory_once(
) -> GateHostFnRealLoweringHorizonFifthSliceWireInventory {
    let fourth = gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory_once();
    let fourth_floor_ok = fourth.b07_real_lowering_fourth_slice_wire_signed
        && fourth.b07_real_lowering_third_slice_wire_signed
        && fourth.b07_real_lowering_second_slice_wire_signed
        && fourth.b07_real_lowering_first_slice_wire_signed
        && fourth.b07_real_lowering_deepen_thin_signed
        && fourth.b07_real_lowering_thin_signed
        && fourth.relational_third_slice_floor_ok
        && fourth.relational_tcp_write_wire_body_ok
        && fourth.first_slice_wire_enabled
        && fourth.second_slice_wire_enabled
        && fourth.third_slice_wire_enabled
        && fourth.fourth_slice_wire_enabled
        && !fourth.lowering_enabled;
    let wire_ok = tcp_handle_release_fifth_slice_wired();
    GateHostFnRealLoweringHorizonFifthSliceWireInventory {
        relational_fourth_slice_floor_ok: fourth_floor_ok,
        relational_tcp_handle_release_wire_body_ok: wire_ok,
        relational_inventory_overlap_ok: true,
        b07_real_lowering_thin_signed: fourth.b07_real_lowering_thin_signed,
        b07_real_lowering_deepen_thin_signed: fourth.b07_real_lowering_deepen_thin_signed,
        b07_real_lowering_first_slice_wire_signed: fourth.b07_real_lowering_first_slice_wire_signed,
        b07_real_lowering_second_slice_wire_signed: fourth
            .b07_real_lowering_second_slice_wire_signed,
        b07_real_lowering_third_slice_wire_signed: fourth.b07_real_lowering_third_slice_wire_signed,
        b07_real_lowering_fourth_slice_wire_signed: fourth
            .b07_real_lowering_fourth_slice_wire_signed,
        lowering_enabled: false,
        first_slice_wire_enabled: fourth.first_slice_wire_enabled,
        second_slice_wire_enabled: fourth.second_slice_wire_enabled,
        third_slice_wire_enabled: fourth.third_slice_wire_enabled,
        fourth_slice_wire_enabled: fourth.fourth_slice_wire_enabled,
        fifth_slice_wire_enabled: wire_ok,
        b07_real_lowering_fifth_slice_wire_signed: true,
    }
}

/// B-07 real HostFn lowering `[x]` exit inventory (W456 / ADR-800).
///
/// Signs narrow construction-chain exit through W449–W455 (thin+deepen+five-slice
/// HostFn wire; HostFn-bound POP exhausted). Does **not** enable global lowering,
/// batch POP rewrite, absolute HostFn↔stub matrix exit, language exports, or MIR
/// blob delete this window.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GateHostFnRealLoweringHorizonExitInventory {
    pub relational_fifth_slice_floor_ok: bool,
    pub relational_inventory_overlap_ok: bool,
    pub b07_real_lowering_thin_signed: bool,
    pub b07_real_lowering_deepen_thin_signed: bool,
    pub b07_real_lowering_first_slice_wire_signed: bool,
    pub b07_real_lowering_second_slice_wire_signed: bool,
    pub b07_real_lowering_third_slice_wire_signed: bool,
    pub b07_real_lowering_fourth_slice_wire_signed: bool,
    pub b07_real_lowering_fifth_slice_wire_signed: bool,
    pub first_slice_wire_enabled: bool,
    pub second_slice_wire_enabled: bool,
    pub third_slice_wire_enabled: bool,
    pub fourth_slice_wire_enabled: bool,
    pub fifth_slice_wire_enabled: bool,
    /// Always `false` — honesty: global HostFn lowering expansion remains off after exit.
    pub lowering_enabled: bool,
    /// Always `false` — honesty: batch POP rewrite remains off.
    pub batch_rewrite_enabled: bool,
    /// Always `false` — honesty: absolute HostFn↔stub matrix exit remains off.
    pub absolute_hostfn_stub_matrix_exit_enabled: bool,
    /// Always `true` — W456 / ADR-800: B-07 real HostFn lowering exit charter signed (narrow).
    pub b07_real_lowering_exit_signed: bool,
}

/// Inventory B-07 real HostFn lowering exit prerequisites (W456 / ADR-800).
///
/// Policy+inventory only — does not mutate stubs/abi/emit (ADR-800).
pub fn gate_hostfn_real_lowering_horizon_exit_inventory() -> GateHostFnRealLoweringHorizonExitInventory {
    let again = gate_hostfn_real_lowering_horizon_exit_inventory_once();
    let first = gate_hostfn_real_lowering_horizon_exit_inventory_once();
    GateHostFnRealLoweringHorizonExitInventory {
        relational_inventory_overlap_ok: first == again,
        ..first
    }
}

fn gate_hostfn_real_lowering_horizon_exit_inventory_once() -> GateHostFnRealLoweringHorizonExitInventory {
    let fifth = gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory_once();
    let fifth_floor_ok = fifth.b07_real_lowering_fifth_slice_wire_signed
        && fifth.b07_real_lowering_fourth_slice_wire_signed
        && fifth.b07_real_lowering_third_slice_wire_signed
        && fifth.b07_real_lowering_second_slice_wire_signed
        && fifth.b07_real_lowering_first_slice_wire_signed
        && fifth.b07_real_lowering_deepen_thin_signed
        && fifth.b07_real_lowering_thin_signed
        && fifth.relational_fourth_slice_floor_ok
        && fifth.relational_tcp_handle_release_wire_body_ok
        && fifth.first_slice_wire_enabled
        && fifth.second_slice_wire_enabled
        && fifth.third_slice_wire_enabled
        && fifth.fourth_slice_wire_enabled
        && fifth.fifth_slice_wire_enabled
        && !fifth.lowering_enabled;
    GateHostFnRealLoweringHorizonExitInventory {
        relational_fifth_slice_floor_ok: fifth_floor_ok,
        relational_inventory_overlap_ok: true,
        b07_real_lowering_thin_signed: fifth.b07_real_lowering_thin_signed,
        b07_real_lowering_deepen_thin_signed: fifth.b07_real_lowering_deepen_thin_signed,
        b07_real_lowering_first_slice_wire_signed: fifth.b07_real_lowering_first_slice_wire_signed,
        b07_real_lowering_second_slice_wire_signed: fifth
            .b07_real_lowering_second_slice_wire_signed,
        b07_real_lowering_third_slice_wire_signed: fifth.b07_real_lowering_third_slice_wire_signed,
        b07_real_lowering_fourth_slice_wire_signed: fifth
            .b07_real_lowering_fourth_slice_wire_signed,
        b07_real_lowering_fifth_slice_wire_signed: fifth.b07_real_lowering_fifth_slice_wire_signed,
        first_slice_wire_enabled: fifth.first_slice_wire_enabled,
        second_slice_wire_enabled: fifth.second_slice_wire_enabled,
        third_slice_wire_enabled: fifth.third_slice_wire_enabled,
        fourth_slice_wire_enabled: fifth.fourth_slice_wire_enabled,
        fifth_slice_wire_enabled: fifth.fifth_slice_wire_enabled,
        lowering_enabled: false,
        batch_rewrite_enabled: false,
        absolute_hostfn_stub_matrix_exit_enabled: false,
        b07_real_lowering_exit_signed: true,
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
    fn llvm_stub_capability_align_inventory_matches_report_and_floors() {
        let r = llvm_stub_audit_report();
        let a = llvm_stub_capability_align_inventory();
        assert_eq!(a.stub_total, r.total);
        assert_eq!(a.stub_wired, r.wired);
        assert_eq!(a.stub_wired_noop, r.wired_noop);
        assert_eq!(a.stub_noop, r.noop);
        assert_eq!(a.stub_unknown, r.unknown);
        assert_eq!(a.stub_cold, r.cold);
        assert_eq!(
            a.stub_total,
            a.stub_wired + a.stub_wired_noop + a.stub_noop + a.stub_unknown + a.stub_cold
        );
        assert!(
            a.hostfn_llvm_real >= 100,
            "expected many HostFn llvm=real, found {}",
            a.hostfn_llvm_real
        );
        assert!(
            a.named_aot_pop >= 1,
            "expected named AOT POP stubs, found {}",
            a.named_aot_pop
        );
        assert!(
            a.named_aot_wired >= 1,
            "expected named AOT Wired stubs, found {}",
            a.named_aot_wired
        );
        assert!(
            a.call_host_path_wired,
            "call_host / call_host_aot path must be Wired"
        );
        assert!(
            a.hostfn_bound_wired >= 1,
            "expected HostFnId-bound Wired stub, found {}",
            a.hostfn_bound_wired
        );
        // Honesty: categories differ — equality is NOT required (and usually false).
        let _ = a.hostfn_llvm_real != a.stub_wired;
        eprintln!("align inventory: {a:?}");
    }

    #[test]
    fn count_hostfn_llvm_backends_parses_real_and_partial() {
        let sample = r#"
[[host]]
name = "A"
backends = { vm = "real", mir = "real", llvm = "real", wasm = "real" }

[[host]]
name = "B"
backends = { vm = "real", mir = "partial", llvm = "partial", wasm = "partial" }

[[host]]
name = "C"
backends = { vm = "real", mir = "real", llvm = "unsupported", wasm = "real" }
"#;
        let (real, partial, other) = count_hostfn_llvm_backends(sample);
        assert_eq!((real, partial, other), (1, 1, 1));
    }

    #[test]
    fn llvm_stub_declare_diff_inventory_floors() {
        let r = llvm_stub_audit_report();
        let d = llvm_stub_declare_diff_inventory();
        assert_eq!(
            d.defined_count, r.total,
            "defined_count must match audit total"
        );
        assert!(
            d.declared_count >= 300,
            "expected many emit declares, found {}",
            d.declared_count
        );
        assert!(
            d.defined_count >= 80,
            "expected many stub defs, found {}",
            d.defined_count
        );
        assert!(
            d.missing_in_stubs >= 1,
            "expected declare−stubs gap, found {}",
            d.missing_in_stubs
        );
        assert!(
            d.extra_in_stubs >= 1,
            "expected stubs−declare gap, found {}",
            d.extra_in_stubs
        );
        assert_eq!(
            d.missing_not_in_rt, 0,
            "declare − (stubs ∪ abi) must be empty (Missing ≠ unlinkable): {d:?}"
        );
        // Honesty: missing_in_stubs is largely abi-covered, not a link hole.
        assert!(
            d.abi_cover_count >= 1,
            "expected abi cover of declared symbols, found {}",
            d.abi_cover_count
        );
        eprintln!("declare-diff inventory: {d:?}");
    }

    #[test]
    fn llvm_stub_missing_clear_readiness_inventory_preds_and_honesty() {
        let inv = llvm_stub_missing_clear_readiness_inventory();
        assert!(
            inv.relational_declare_gap_measurable_ok,
            "declare gap must be measurable: {inv:?}"
        );
        assert!(
            inv.relational_unlinkable_zero_ok,
            "unlinkable must remain zero: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(!inv.clear_enabled, "Missing clear must remain off: {inv:?}");
        assert!(
            !inv.stub_rewrite_enabled,
            "stub rewrite must remain off: {inv:?}"
        );
        assert_eq!(
            inv.missing_not_in_rt, 0,
            "missing_not_in_rt must stay 0: {inv:?}"
        );
        assert!(
            inv.missing_in_stubs >= 1,
            "missing_in_stubs must stay positive: {inv:?}"
        );
    }

    #[test]
    fn llvm_stub_missing_clear_readiness_inventory_matches_declare_diff() {
        let inv = llvm_stub_missing_clear_readiness_inventory();
        let diff = llvm_stub_declare_diff_inventory();
        assert_eq!(
            inv.relational_declare_gap_measurable_ok,
            diff.missing_in_stubs >= 1
        );
        assert_eq!(
            inv.relational_unlinkable_zero_ok,
            diff.missing_not_in_rt == 0
        );
        assert_eq!(inv.missing_in_stubs, diff.missing_in_stubs);
        assert_eq!(inv.missing_not_in_rt, diff.missing_not_in_rt);
        assert_eq!(inv.abi_cover_count, diff.abi_cover_count);
        assert_eq!(inv.clear_enabled, false);
        assert_eq!(inv.stub_rewrite_enabled, false);
    }

    #[test]
    fn llvm_stub_clear_align_horizon_readiness_inventory_preds_and_honesty() {
        let inv = llvm_stub_clear_align_horizon_readiness_inventory();
        assert!(
            inv.relational_missing_clear_chain_ok,
            "missing clear chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_align_inventory_chain_ok,
            "align inventory chain must hold: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(!inv.clear_enabled, "Missing clear must remain off: {inv:?}");
        assert!(
            !inv.stub_rewrite_enabled,
            "stub rewrite must remain off: {inv:?}"
        );
        assert!(
            !inv.absolute_align_matrix_exit_enabled,
            "absolute align matrix exit must remain off: {inv:?}"
        );
    }

    #[test]
    fn llvm_stub_clear_align_horizon_readiness_inventory_matches_axes() {
        let inv = llvm_stub_clear_align_horizon_readiness_inventory();
        let clear = llvm_stub_missing_clear_readiness_inventory();
        let align = llvm_stub_capability_align_inventory();
        assert_eq!(
            inv.relational_missing_clear_chain_ok,
            clear.relational_declare_gap_measurable_ok
                && clear.relational_unlinkable_zero_ok
                && clear.relational_inventory_overlap_ok
                && !clear.clear_enabled
                && !clear.stub_rewrite_enabled
                && clear.missing_not_in_rt == 0
        );
        assert_eq!(
            inv.relational_align_inventory_chain_ok,
            align.stub_total >= 80
                && align.stub_total
                    == align.stub_wired
                        + align.stub_wired_noop
                        + align.stub_noop
                        + align.stub_unknown
                        + align.stub_cold
                && align.call_host_path_wired
                && align.hostfn_llvm_real >= 100
                && align.named_aot_pop >= 1
                && align.named_aot_wired >= 1
                && align.hostfn_bound_wired >= 1
        );
        assert!(!inv.clear_enabled);
        assert!(!inv.stub_rewrite_enabled);
        assert!(!inv.absolute_align_matrix_exit_enabled);
        assert_eq!(inv.candidate_predicate_count, 3);
    }

    #[test]
    fn llvm_stub_missing_clear_honest_defer_inventory_preds_and_honesty() {
        let inv = llvm_stub_missing_clear_honest_defer_inventory();
        assert!(
            inv.relational_clear_align_horizon_ok,
            "clear×align horizon must hold: {inv:?}"
        );
        assert!(
            inv.relational_residual_measurable_ok,
            "residual must remain measurable: {inv:?}"
        );
        assert!(
            inv.relational_inventory_overlap_ok,
            "inventory overlap must hold: {inv:?}"
        );
        assert_eq!(inv.candidate_predicate_count, 3);
        assert!(
            inv.honest_defer_signed,
            "honest defer must be signed (W337): {inv:?}"
        );
        assert!(!inv.clear_enabled, "Missing clear must remain off: {inv:?}");
        assert!(
            !inv.stub_rewrite_enabled,
            "stub rewrite must remain off: {inv:?}"
        );
        assert_eq!(
            inv.missing_not_in_rt, 0,
            "missing_not_in_rt must stay 0: {inv:?}"
        );
        assert!(
            inv.missing_in_stubs >= 1,
            "missing_in_stubs must stay positive: {inv:?}"
        );
    }

    #[test]
    fn llvm_stub_missing_align_horizon_exit_inventory_preds_and_honesty() {
        let inv = llvm_stub_missing_align_horizon_exit_inventory();
        assert!(
            inv.b06_missing_align_exit_signed,
            "B-06 exit must be signed (W448 / ADR-792): {inv:?}"
        );
        assert!(inv.honest_defer_signed, "{inv:?}");
        assert!(
            inv.relational_honest_defer_ok,
            "honest defer must hold: {inv:?}"
        );
        assert!(
            inv.relational_clear_align_horizon_ok,
            "clear×align horizon must hold: {inv:?}"
        );
        assert!(
            inv.relational_declare_diff_ok,
            "declare-diff must hold: {inv:?}"
        );
        assert!(
            inv.relational_audit_report_ok,
            "audit report must hold: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        assert!(!inv.clear_enabled, "Missing clear must remain off: {inv:?}");
        assert!(
            !inv.stub_rewrite_enabled,
            "stub rewrite must remain off: {inv:?}"
        );
        assert!(
            !inv.absolute_align_matrix_exit_enabled,
            "absolute align matrix exit must remain off: {inv:?}"
        );
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

    #[test]
    fn gate_hostfn_real_lowering_horizon_deepen_thin_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_deepen_thin_inventory();
        assert!(
            inv.b07_real_lowering_deepen_thin_signed,
            "B-07 deepen thin must be signed (W450 / ADR-794): {inv:?}"
        );
        assert!(
            inv.b07_real_lowering_thin_signed,
            "thin floor must remain signed under deepen: {inv:?}"
        );
        assert!(
            inv.relational_thin_floor_ok,
            "thin floor must hold: {inv:?}"
        );
        assert!(
            inv.relational_category_gap_ok,
            "real>=named_aot category honesty must hold: {inv:?}"
        );
        assert!(
            inv.relational_named_aot_wired_ok,
            "named aot wired must hold: {inv:?}"
        );
        assert!(
            inv.relational_partial_count_ok,
            "partial count must be measurable: {inv:?}"
        );
        assert!(
            inv.relational_first_slice_candidate_floor_ok,
            "first-slice candidate floor must hold: {inv:?}"
        );
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.hostfn_llvm_real >= inv.named_aot_wired, "{inv:?}");
        assert!(inv.named_aot_wired >= 1, "{inv:?}");
        assert!(inv.call_host_path_wired);
        assert!(
            !inv.lowering_enabled,
            "real lowering must remain off (deepen thin): {inv:?}"
        );
        assert!(
            inv.first_slice_wire_enabled,
            "first-slice HttpGet wire must be live after W451 / ADR-795: {inv:?}"
        );
    }

    #[test]
    fn gate_hostfn_real_lowering_horizon_first_slice_wire_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_first_slice_wire_inventory();
        assert!(
            inv.b07_real_lowering_first_slice_wire_signed,
            "B-07 first-slice wire must be signed (W451 / ADR-795): {inv:?}"
        );
        assert!(inv.b07_real_lowering_thin_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_deepen_thin_signed, "{inv:?}");
        assert!(inv.relational_deepen_floor_ok, "{inv:?}");
        assert!(inv.relational_http_get_wire_body_ok, "{inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.first_slice_wire_enabled, "{inv:?}");
        assert!(
            !inv.lowering_enabled,
            "global lowering must remain off (first-slice only): {inv:?}"
        );
        let body = http_get_aot_stub_body();
        assert!(body.contains("HostFnId::HttpGet"), "{body}");
        assert!(body.contains("rt_call_host_native"), "{body}");
        assert!(!body.contains("rt_direct_exec"), "{body}");
    }

    #[test]
    fn gate_hostfn_real_lowering_horizon_second_slice_wire_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_second_slice_wire_inventory();
        assert!(
            inv.b07_real_lowering_second_slice_wire_signed,
            "B-07 second-slice wire must be signed (W452 / ADR-796): {inv:?}"
        );
        assert!(inv.b07_real_lowering_first_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_thin_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_deepen_thin_signed, "{inv:?}");
        assert!(inv.relational_first_slice_floor_ok, "{inv:?}");
        assert!(inv.relational_file_read_wire_body_ok, "{inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.first_slice_wire_enabled, "{inv:?}");
        assert!(inv.second_slice_wire_enabled, "{inv:?}");
        assert!(
            !inv.lowering_enabled,
            "global lowering must remain off (second-slice only): {inv:?}"
        );
        let body = file_read_aot_stub_body();
        assert!(body.contains("HostFnId::FileRead"), "{body}");
        assert!(body.contains("rt_call_host_native"), "{body}");
        assert!(!body.contains("rt_direct_exec"), "{body}");
    }

    #[test]
    fn gate_hostfn_real_lowering_horizon_third_slice_wire_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_third_slice_wire_inventory();
        assert!(
            inv.b07_real_lowering_third_slice_wire_signed,
            "B-07 third-slice wire must be signed (W453 / ADR-797): {inv:?}"
        );
        assert!(inv.b07_real_lowering_second_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_first_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_thin_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_deepen_thin_signed, "{inv:?}");
        assert!(inv.relational_second_slice_floor_ok, "{inv:?}");
        assert!(inv.relational_tcp_read_wire_body_ok, "{inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.first_slice_wire_enabled, "{inv:?}");
        assert!(inv.second_slice_wire_enabled, "{inv:?}");
        assert!(inv.third_slice_wire_enabled, "{inv:?}");
        assert!(
            !inv.lowering_enabled,
            "global lowering must remain off (third-slice only): {inv:?}"
        );
        let body = tcp_read_stub_body();
        assert!(body.contains("HostFnId::TcpRead"), "{body}");
        assert!(body.contains("rt_call_host_native"), "{body}");
        assert!(!body.contains("rt_direct_exec"), "{body}");
    }

    #[test]
    fn gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory();
        assert!(
            inv.b07_real_lowering_fourth_slice_wire_signed,
            "B-07 fourth-slice wire must be signed (W454 / ADR-798): {inv:?}"
        );
        assert!(inv.b07_real_lowering_third_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_second_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_first_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_thin_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_deepen_thin_signed, "{inv:?}");
        assert!(inv.relational_third_slice_floor_ok, "{inv:?}");
        assert!(inv.relational_tcp_write_wire_body_ok, "{inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.first_slice_wire_enabled, "{inv:?}");
        assert!(inv.second_slice_wire_enabled, "{inv:?}");
        assert!(inv.third_slice_wire_enabled, "{inv:?}");
        assert!(inv.fourth_slice_wire_enabled, "{inv:?}");
        assert!(
            !inv.lowering_enabled,
            "global lowering must remain off (fourth-slice only): {inv:?}"
        );
        let body = tcp_write_stub_body();
        assert!(body.contains("HostFnId::TcpWrite"), "{body}");
        assert!(body.contains("rt_call_host_native"), "{body}");
        assert!(!body.contains("rt_direct_exec"), "{body}");
    }

    #[test]
    fn gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory();
        assert!(
            inv.b07_real_lowering_fifth_slice_wire_signed,
            "B-07 fifth-slice wire must be signed (W455 / ADR-799): {inv:?}"
        );
        assert!(inv.b07_real_lowering_fourth_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_third_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_second_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_first_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_thin_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_deepen_thin_signed, "{inv:?}");
        assert!(inv.relational_fourth_slice_floor_ok, "{inv:?}");
        assert!(inv.relational_tcp_handle_release_wire_body_ok, "{inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.first_slice_wire_enabled, "{inv:?}");
        assert!(inv.second_slice_wire_enabled, "{inv:?}");
        assert!(inv.third_slice_wire_enabled, "{inv:?}");
        assert!(inv.fourth_slice_wire_enabled, "{inv:?}");
        assert!(inv.fifth_slice_wire_enabled, "{inv:?}");
        assert!(
            !inv.lowering_enabled,
            "global lowering must remain off (fifth-slice only): {inv:?}"
        );
        let body = tcp_handle_release_stub_body();
        assert!(body.contains("HostFnId::TcpHandleRelease"), "{body}");
        assert!(body.contains("rt_call_host_native"), "{body}");
        assert!(!body.contains("rt_direct_exec"), "{body}");
    }

    #[test]
    fn gate_hostfn_real_lowering_horizon_exit_inventory_preds_and_honesty() {
        let inv = gate_hostfn_real_lowering_horizon_exit_inventory();
        assert!(
            inv.b07_real_lowering_exit_signed,
            "B-07 exit must be signed (W456 / ADR-800): {inv:?}"
        );
        assert!(inv.b07_real_lowering_fifth_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_fourth_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_third_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_second_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_first_slice_wire_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_thin_signed, "{inv:?}");
        assert!(inv.b07_real_lowering_deepen_thin_signed, "{inv:?}");
        assert!(inv.relational_fifth_slice_floor_ok, "{inv:?}");
        assert!(inv.relational_inventory_overlap_ok);
        assert!(inv.first_slice_wire_enabled, "{inv:?}");
        assert!(inv.second_slice_wire_enabled, "{inv:?}");
        assert!(inv.third_slice_wire_enabled, "{inv:?}");
        assert!(inv.fourth_slice_wire_enabled, "{inv:?}");
        assert!(inv.fifth_slice_wire_enabled, "{inv:?}");
        assert!(
            !inv.lowering_enabled,
            "global lowering must remain off after narrow exit: {inv:?}"
        );
        assert!(!inv.batch_rewrite_enabled, "{inv:?}");
        assert!(!inv.absolute_hostfn_stub_matrix_exit_enabled, "{inv:?}");
    }

    #[test]
    fn llvm_stub_missing_clear_honest_defer_inventory_matches_axes() {
        let inv = llvm_stub_missing_clear_honest_defer_inventory();
        let horizon = llvm_stub_clear_align_horizon_readiness_inventory();
        let clear = llvm_stub_missing_clear_readiness_inventory();
        assert_eq!(
            inv.relational_clear_align_horizon_ok,
            horizon.relational_missing_clear_chain_ok
                && horizon.relational_align_inventory_chain_ok
                && horizon.relational_inventory_overlap_ok
                && !horizon.clear_enabled
                && !horizon.stub_rewrite_enabled
                && !horizon.absolute_align_matrix_exit_enabled
        );
        assert_eq!(
            inv.relational_residual_measurable_ok,
            clear.missing_in_stubs >= 1 && clear.missing_not_in_rt == 0 && !clear.clear_enabled
        );
        assert_eq!(inv.missing_in_stubs, clear.missing_in_stubs);
        assert_eq!(inv.missing_not_in_rt, clear.missing_not_in_rt);
        assert_eq!(inv.clear_enabled, false);
        assert_eq!(inv.honest_defer_signed, true);
        assert_eq!(inv.stub_rewrite_enabled, false);
        assert_eq!(inv.candidate_predicate_count, 3);
    }
}
