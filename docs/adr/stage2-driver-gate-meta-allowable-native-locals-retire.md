# ADR: Stage-2 S2-02 driver_gate meta-allowable + native_locals soft-nail retire (S2-I113)

**Status:** Accepted (S2-I113) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I112 host_registry soft-nail retire](stage2-driver-gate-host-registry-soft-nail-retire.md), remaining pure soft-nails include two `meta_allowable` charters (`dist_prod_ga` / `borrow_mandatory`) and `native_llvm_locals_v7_invalidation_exact`. `soft_debt_zero_meta_allowable_charter` remains a hard-kept one-liner gate.

## Decision

1. **Authorize deletion** of `p98_driver_dist_prod_ga_meta_allowable_charter`, `p98_driver_borrow_mandatory_v3_meta_allowable_charter`, and `p98_driver_native_llvm_locals_v7_invalidation_exact` from [`driver_gate.rs`](../../crates/syx-driver/tests/platform/driver_gate.rs).
2. **Keep** `p98_driver_soft_debt_zero_meta_allowable_charter`; `cache_v111`; hasher; `import_*` / other include_str surface soft; all behavioral tests.
3. **Expected:** S2-01 census KPIs remain **0**; ≈ −3 soft-nail tests; `#[test]` stays above ≥500.
4. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Meta charter fingerprint soft-nails and leftover native-locals-v7 soft-nail drop; soft-debt zero gate remains.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-driver-gate-host-registry-soft-nail-retire](stage2-driver-gate-host-registry-soft-nail-retire.md) · S2-I112
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
