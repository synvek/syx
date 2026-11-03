//! B-09 product flags (S2-I18 policy-fixed; S2-I47: orphan thin inventory retired).
//!
//! Five readonly product flags remain. Orphan `gate_b09_gc_narrative_thin_inventory`
//! was deleted under [stage2-hostfn-construction-tip-knife](../../../docs/adr/stage2-hostfn-construction-tip-knife.md).
//! ≠ rewriting ManagedHeap / ObjectRef / GC collect product behavior.
//! ≠ B-09 roadmap `[x]` beyond historical W558 narrow claim narrative.

/// Wave A W555 / ADR-899: public readonly migrate product flag (S2-I18 policy-fixed).
pub fn objectref_rc_migrate_enabled() -> bool {
    true
}

/// Wave A W556 / ADR-900: public readonly storage-cleanup product flag (S2-I18 policy-fixed).
pub fn storage_cleanup_enabled() -> bool {
    true
}

/// Wave A W557 / ADR-901: public readonly unified MirRuntime heap enable flag (S2-I18 policy-fixed).
/// Independent of `ManagedHeap::enabled()` / `SYX_UNIFIED_HEAP` env gate.
pub fn unified_mir_runtime_heap_enabled() -> bool {
    true
}

/// Wave A W558 / ADR-902: public readonly B-09 axis exit flag (S2-I18 policy-fixed).
pub fn b09_axis_exit_enabled() -> bool {
    true
}

/// Wave A W558 / ADR-902: public readonly narrow GC GA claim flag (S2-I18 policy-fixed).
pub fn gc_ga_claimed() -> bool {
    true
}
