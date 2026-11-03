# ADR: Stage-2 fingerprint tip knife (S2-I57+)

**Status:** Accepted (S2-I57; amended S2-I58)  
**Date:** 2026-11-03  
**Context:** After S2-I56, doctor-report tip/nail packs are exhausted except product `capabilities_report`. Remaining S2-01 mass is dominated by fingerprint towers (`fingerprint_fn`≈655 / `fingerprint_const`≈241). Many historical `heap_v*_fingerprint` helpers are **orphan** (definition only; masked by `#![allow(dead_code)]`; not on `compute_cache_version_tag`). Precedent: [stage2-b09-construction-tip-knife](stage2-b09-construction-tip-knife.md), [stage2-doctor-report-run-test-nail-knife](stage2-doctor-report-run-test-nail-knife.md).

## Decision

1. **Orphan `*_fingerprint` helpers may be retired** when they have **no** product/runtime callers. Window-end must leave **no** test `include_str`/`contains` nails of the deleted fn names — pure orphans delete the body alone; helpers pinned only by `driver_gate`/`include_str` nails may be retired **in the same window** by deleting the function **and** removing those nails (parallel to doctor-report run-test-nail).
2. **S2-I57 deletes orphan tip packs×4** (not on cache aggregator):
   - `heap_v90_phase118_track_a_gui_fingerprint`
   - `heap_v90_phase118_track_b_wasm_fingerprint`
   - `heap_v90_phase118_track_c_farm_fingerprint`
   - `heap_v90_phase118_track_d_dist_select_fingerprint`
3. **Expected census (S2-I57):** `fingerprint_fn_matches` **655 → 651 (−4)**; `syx_driver_src_report_rs` stays **1**; `inventory_fn_matches` stays **0**.
4. **Later windows may continue packs under this ADR** (default Δ3–4) without a new decision document. Peeling `compute_cache_version_tag` aggregator tips requires same-window `format!` `{}`/arg alignment and is allowed under this ADR only when the tip has no external readers beyond the aggregator.
5. **S2-I58 deletes nail-co-delete tip packs×4** (not on cache aggregator; same-window narrow of `p104_driver_heap_v76_fingerprint`):
   - `heap_v76_phase104_track_a_gui_fingerprint`
   - `heap_v76_phase104_track_b_wasm_fingerprint`
   - `heap_v76_phase104_track_c_fingerprint`
   - `heap_v76_phase104_track_d_fingerprint`
6. **Expected census (S2-I58):** `fingerprint_fn_matches` **651 → 647 (−4)**; report **1**; inventory **0**. Keeps `heap_v76_phase104_dx_fingerprint` and `heap_v90_phase118_dx_fingerprint`.
7. **MUST STAY:** `capabilities_report`; fingerprints on the cache aggregator that remain product cache surface; `farm_lease_*_fingerprint` product APIs; near-tip `heap_v111_*`; five B-09 product flags; `llvm_stub_audit_report`. Do not confuse with wasm const `PHASE118_TRACK_B_WASM_FINGERPRINTS`.
8. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ emit·ABI rewrite.

## Consequences

- **S2-I57:** orphan packs×4 deleted; `fingerprint_fn` **655 → 651**; cache aggregator unchanged.
- **S2-I58:** v76 phase104 track a–d×4 deleted + `p104_driver_heap_v76_fingerprint` narrowed to `dx` only; `fingerprint_fn` **651 → 647**; cache aggregator unchanged.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
