# ADR: Stage-2 Wave B readiness inventory retire (S2-I16)

**Status:** Accepted (S2-I16)  
**Date:** 2026-11-02  
**Context:** After S2-I15, census is **85** (product-surface). Orphan cutover feeds and driver test-name false positives are gone. Remaining mass is `gate_b09`≈74, HostFn/`capability_align`≈9, and Wave B readiness inventories×2. Those two (`wave_b_tri_axis_horizon_readiness_inventory`, `wave_b_structural_exit_readiness_inventory`) are pure policy fixtures after S2-I11 hardcodes, have **no product flag readers**, and Wave B exit charter (W340) plus live B-04/B-05/B-06 `[x]` (W446–W448) are already signed. Precedent: [stage2-cutover-decouple](stage2-cutover-decouple.md).

## Decision

1. **Retire Wave B readiness inventories from product `src/`.** Delete `crates/syx-rt/src/wave_b_horizon.rs` (tri + structural) and companion driver thin tests that only `include_str`/`.contains` those symbols.
2. **Expected census:** **85 → 83 (−2)**.
3. **Docs soft-archive:** `wave-b-horizon-readiness.md` / `wave-b-exit-readiness.md` / readiness floor line in `wave-b-exit.md` mark product inventory retired; W340 charter narrative stays.
4. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ rewriting Wave B GA product semantics / ≠ deleting `gate_b09` / HostFn×8 / `capability_align` / ≠ HostFn+1 / ≠ product emit·ABI change.

## Consequences

- Later S2-01 windows still need a heavier ADR for HostFn←align decouple or `gate_b09` pack migration.
- Historical Wave docs may still name deleted inventory symbols as archival evidence.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-cutover-decouple](stage2-cutover-decouple.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [wave-b-exit.md](../lang/wave-b-exit.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
