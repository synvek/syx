# ADR: Stage-2 S2-02 driver_gate exists soft-nail retire (S2-I110)

**Status:** Accepted (S2-I110) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I109 cache soft-nail retire](stage2-cache-soft-nail-retire.md), remaining pure soft-nails include `driver_gate` `fixture(...).exists()` proxies and empty charter bodies. Several `browser_tier2*` fixtures are already gone from disk.

## Decision

1. **Authorize deletion** of ≈28 `*_gate_exists` / `*_gate_files_exist` / `*_crate_exists` tests, 3 empty charter bodies (`adr089` / `adr091` / `manifest_79_historical`), and `p98_driver_browser_tier2u_meta_allowable_charter` (dead `browser_tier2u.rs` fixture nail) from [`driver_gate.rs`](../../crates/syx-driver/tests/platform/driver_gate.rs).
2. **Do not** delete heap/cache `*_invalidation_exact`, other `include_str` surface soft, `cache_v111_invalidation.rs`, hasher bytes, or `lang_surface_v1` in this window.
3. **Expected:** S2-01 census KPIs remain **0**; ≈ −32 soft-nail tests; `#[test]` count stays above the ≥500 floor.
4. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Pure path-exists proxies and empty shells drop; behavioral `*_exact_exec` / `run()` remain.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-cache-soft-nail-retire](stage2-cache-soft-nail-retire.md) · S2-I109
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
