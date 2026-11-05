# ADR: Stage-2 S2-02 driver_gate invalidation-exact soft-nail retire (S2-I111)

**Status:** Accepted (S2-I111) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I110 driver_gate exists soft-nail retire](stage2-driver-gate-exists-soft-nail-retire.md), remaining pure soft-nails include historical heap/cache `*_invalidation_exact` and adjacent cache-tag `include_str!(lib.rs)+contains` tests in `driver_gate.rs`, superseded by [`cache_v111_invalidation`](../../crates/syx-driver/tests/platform/cache_v111_invalidation.rs).

## Decision

1. **Authorize deletion** of 8 heap/cache `*_invalidation_exact` tests and 3 adjacent cache-tag soft tests (`driver_cache_v75_tag` / `p104_driver_cache_v76_tag` / `p105_cache_v77_track_d_fingerprints`) from [`driver_gate.rs`](../../crates/syx-driver/tests/platform/driver_gate.rs).
2. **Keep** `cache_v111_invalidation.rs`, hasher bytes, historical tags comment, `lang_surface_v1`, `p98_driver_native_llvm_locals_v7_invalidation_exact`, host_registry / meta_allowable / other include_str soft, and all behavioral `*_exact_exec` / `run()` tests.
3. **Expected:** S2-01 census KPIs remain **0**; ≈ −11 soft-nail tests; `#[test]` count stays above the ≥500 floor.
4. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Historical invalidation / tag soft-nails in `driver_gate` drop; tip remains in `cache_v111`.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-cache-soft-nail-retire](stage2-cache-soft-nail-retire.md) · S2-I109
- [stage2-driver-gate-exists-soft-nail-retire](stage2-driver-gate-exists-soft-nail-retire.md) · S2-I110
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
