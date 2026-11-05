# ADR: Stage-2 fingerprint product-sea orphan tip rename (S2-I99)

**Status:** Accepted (S2-I99)  
**Date:** 2026-11-04  
**Context:** After [S2-I98 generate helper rename](stage2-fingerprint-product-sea-generate-rename.md), two orphan `pub fn *_fingerprint` tips remain outside MUST-STAY packs. Sibling pack: rename Rust symbols only; leave driver hasher byte literals, C-ABI, farm_lease, and `heap_v100_*` untouched.

## Decision

1. **Authorize rename** of two orphan tips from `*_fingerprint` → `*_digest` **without deleting** APIs or changing cache hasher inputs.
2. **S2-I99 pack Δ2**:
   - `reflect_witness_honest_fingerprint` → `reflect_witness_honest_digest` (`syx-meta` + same-file test call)
   - `macro_external_plugin_prod_fingerprint` → `macro_external_plugin_prod_digest` (`syx-macro` def + `pub use`)
3. **Do not rename:** `syx_gui_dylib_*_fingerprint`; `farm_lease_*`; `heap_v100_phase128_track_c_wasm_fingerprint`; hasher `b"reflect-witness-honest-v1:…"` / `b"macro-external-v2:plugin-prod"`; driver_gate string probes; matrix×2; `lang_props_v*`.
4. **Do not** keep deprecated old-name wrappers (would add census rows).
5. **Expected census:** `fingerprint_fn_matches` **64 → 62 (−2)**; report **0**; inventory **0**.
6. Later packs (test-noise) under sibling ADRs without claiming S2-01 `[x]` — **S2-I100:** [test-noise rename](stage2-fingerprint-test-noise-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops two orphan tip matches; cache invalidation bytes unchanged.
- **S2-I100:** test-noise×6 cleared (matrix digest + driver_gate needle retarget); MUST STAY floor 56.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-generate-rename](stage2-fingerprint-product-sea-generate-rename.md) · S2-I98
- [stage2-fingerprint-test-noise-rename](stage2-fingerprint-test-noise-rename.md) · S2-I100
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

