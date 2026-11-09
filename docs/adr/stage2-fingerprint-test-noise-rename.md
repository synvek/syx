# ADR: Stage-2 fingerprint test-noise rename / probe retarget (S2-I100)

**Status:** Accepted (S2-I100)  
**Date:** 2026-11-04  
**Context:** After [S2-I99 orphan tip rename](stage2-fingerprint-product-sea-orphan-rename.md), six remaining census hits are test-noise: two real matrix helpers and four driver_gate string false-positives. Clearing them lands census on the MUST-STAY floor (farm_lease×36 + C-ABI×19 + heap_v100×1).

## Decision

1. **Authorize** clearing test-noise×6 from census without touching MUST-STAY product symbols.
2. **S2-I100 pack Δ6**:
   - Rename matrix helpers: `matrix_wave_and_fingerprint` → `matrix_wave_and_digest`; `assert_live_matrix_fingerprint` → `assert_live_matrix_digest` (+ hostfn soft-nails).
   - Retarget driver_gate probe needles from deleted CR `fn …_fingerprint` strings to existing `fn …_count` tests (same file).
3. **Do not rename:** `farm_lease_*`; `syx_gui_dylib_*`; `heap_v100_phase128_track_c_wasm_fingerprint`; `lang_props_v*`; TOML field name `fingerprint`.
4. **Do not** restore deleted CR fingerprint tests or add deprecated old-name wrappers.
5. **Expected census:** `fingerprint_fn_matches` **62 → 56 (−6)**; report **0**; inventory **0**.
6. Later packs for MUST-STAY (farm_lease / C-ABI / heap_v100) require sibling ADRs with a different strategy — not this rename axis — **S2-I101:** [farm-lease digest rename](stage2-fingerprint-farm-lease-digest-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c`.

## Consequences

- Census drops six test-noise matches; remaining 56 are MUST STAY only.
- **S2-I101:** farm_lease ×36 renamed off census (`*_digest`); C-ABI×19 + heap×1 remain.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-orphan-rename](stage2-fingerprint-product-sea-orphan-rename.md) · S2-I99
- [stage2-fingerprint-farm-lease-digest-rename](stage2-fingerprint-farm-lease-digest-rename.md) · S2-I101
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

