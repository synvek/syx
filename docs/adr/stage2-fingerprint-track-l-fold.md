# ADR: Stage-2 fingerprint track_l aggregator fold (S2-I87)

**Status:** Accepted (S2-I87)  
**Date:** 2026-11-04  
**Context:** After S2-I86, on-aggregator tips are only `heap_v100_phase128_track_c_wasm_fingerprint` and `heap_v111_phase139_track_l_language_fingerprint`. Both [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md) and [stage2-fingerprint-foundation-fold](stage2-fingerprint-foundation-fold.md) listed `track_l` as MUST STAY pending a **separate gate**. Soft-nail surface for `track_l` is confined to `cache_v111_invalidation.rs`.

## Decision

1. **Authorize fold** of `heap_v111_phase139_track_l_language_fingerprint` under this ADR (merge hasher input statements into the kept successor; delete the folded def; align `compute_cache_version_tag` `format!` in the same window). **Bare peel is forbidden.**
2. **S2-I87 pack Δ1** — merge into `heap_v100_phase128_track_c_wasm_fingerprint` (kept name); delete:
   - `heap_v111_phase139_track_l_language_fingerprint`
3. **Byte order:** original v100_c → track_l.
4. Soft-nail retarget → merge target: `cache_v111_invalidation` (`cache_v111_tag_active` + `cache_v111_lang_surface_fingerprints`).
5. **Expected census:** `fingerprint_fn_matches` **165 → 164 (−1)**; report **0**; inventory **0**.
6. **MUST STAY:** `farm_lease_*_fingerprint` product APIs; capabilities product API; `lang_props_v*`; five B-09 product flags; `llvm_stub_audit_report`; product-sea fingerprints (gui/vm/…).
7. After this pack, on-aggregator fingerprint tips are a **single** kept helper (`v100_c`). Further product-sea carve requires a **separate** ADR — opened by [stage2-fingerprint-product-sea-cdn-bgp-rename](stage2-fingerprint-product-sea-cdn-bgp-rename.md) (S2-I88).
8. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ product-sea rename in the track_l window.

## Consequences

- Outer cache tag loses the `track_l` slot; `v100_c` hex changes (expected invalidation).
- Aggregator/foundation MUST STAY wording for `track_l` is superseded by this gate.
- **S2-I88:** product-sea cdn_bgp sticky/peer-announce ×11 renamed off census (`*_digest`).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md)
- [stage2-fingerprint-foundation-fold](stage2-fingerprint-foundation-fold.md)
- [stage2-fingerprint-v111-fold](stage2-fingerprint-v111-fold.md)
- [stage2-fingerprint-product-sea-cdn-bgp-rename](stage2-fingerprint-product-sea-cdn-bgp-rename.md) · S2-I88
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
