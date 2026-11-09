# ADR: Stage-2 fingerprint v111 aggregator fold (S2-I85)

**Status:** Accepted (S2-I85)  
**Date:** 2026-11-04  
**Context:** After S2-I84, inventory and `*_report.rs` census are **0**; `fingerprint_fn` **181**. Pre-`v111` tip-knife / earlier-tip fold axes are exhausted. Remaining on-aggregator tips include `heap_v111_*`×5, which the general [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md) listed as MUST STAY pending a **new gate**. Precedent: S2-I81 folded contiguous `v110` c/d/e/dx into `track_a`.

## Decision

1. **Authorize fold** of `heap_v111_*` tip helpers under this ADR (same fold semantics as aggregator-fold: merge hasher input statements into one kept successor; delete folded defs; align `compute_cache_version_tag` `format!` in the same window). **Bare peel is forbidden.**
2. **S2-I85 pack Δ4** — merge into `heap_v111_phase139_track_l_language_fingerprint` (kept name); delete:
   - `heap_v111_phase139_track_c_wasm_fingerprint`
   - `heap_v111_phase139_track_d_frozen_fingerprint`
   - `heap_v111_phase139_track_inherit_fingerprint`
   - `heap_v111_phase139_dx_fingerprint`
3. **Byte order:** original track_l → c → d → inherit → dx.
4. Same-window soft-nail: `cache_v111_invalidation` `cache_v111_tag_active` drop c/d/inherit/dx `contains` (keep `track_l` + `syx-cache-v1`).
5. **Expected census:** `fingerprint_fn_matches` **181 → 177 (−4)**; `syx_driver_src_report_rs` **0**; `inventory_fn_matches` **0**.
6. **MUST STAY:** foundation aggregator slots; `heap_v100_phase128_track_c_wasm_fingerprint`; `farm_lease_*_fingerprint` product APIs; capabilities product API; `lang_props_v*`; five B-09 product flags; `llvm_stub_audit_report`.
7. Later windows may continue v111 residual packs under this ADR only if further `heap_v111_*` tips remain; foundation fold still requires a **separate** gate.
8. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ emit·ABI rewrite / ≠ product umbrella / F-08.

## Consequences

- Outer cache tag loses four `heap_v111_*` slots; `track_l` hex changes (expected invalidation).
- General aggregator-fold ADR MUST STAY for `heap_v111_*` is superseded by this gate for folded tips; remaining on-agg helpers stay under that ADR / foundation gate.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
