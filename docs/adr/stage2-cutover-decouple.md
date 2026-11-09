# ADR: Stage-2 cutover product decouple (S2-I11)

**Status:** Accepted (S2-I11)  
**Date:** 2026-11-01  
**Context:** After S2-I10, S2-01 easy unused inventory packs are exhausted (census **113**). Remaining mass is mostly `gate_b09`, cutover/`clear_align` feed chains, and HostFn inventories. `wave_b_tri_axis_horizon_readiness_inventory` was the **only product reader** of three cutover rollups (`aot_short_horizon_cutover_readiness`, `mir_module_medium_horizon_cutover_readiness`, `llvm_stub_clear_align_horizon_readiness`), blocking further S2-01 knives without a strategy change. Precedent: S2-I7 hard-coded long-horizon flags then deleted the long inventory chain.

## Decision

1. **Product path no longer walks cutover inventories.** In `wave_b_horizon.rs`, `relational_b04_short_horizon_ok` / `relational_b05_medium_horizon_ok` / `relational_b06_clear_align_horizon_ok` are **policy-fixed `true`** (matching prior live assertions), same honesty pattern as post-I7 long flags.
2. **This window deletes the three rollup inventories** (and their dedicated unit tests / re-exports). Expected census **113 → 110 (−3)**.
3. **Feed chains stay** (short export/blob/absolute; medium metadata/dual_blob_*; llvm `declare_diff` / `missing_clear_readiness` / `capability_align` + HostFn×8; `gate_b09`) for later windows.
4. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ Wave B GA / ≠ B-04/B-05/B-06 axis `[x]` / ≠ deleting feed or `gate_b09` / ≠ HostFn+1.

## Consequences

- Later S2-01 windows may delete orphaned cutover feed inventories without touching `wave_b` product semantics.
- Historical docs may still name deleted rollup symbols as evidence narrative.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
