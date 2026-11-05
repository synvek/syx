# ADR: Stage-2 fingerprint_fn heap tip digest rename (S2-I105)

**Status:** Accepted (S2-I105)  
**Date:** 2026-11-05  
**Context:** After [S2-I104 SELECT const rename](stage2-fingerprint-const-select-rename.md), `fingerprint_const` is **0**. Remaining non–C-ABI `fingerprint_fn` tip is the on-agg merge target `heap_v100_phase128_track_c_wasm_fingerprint` (private Rust; no dlsym). Sibling pack: rename identifier only.

## Decision

1. **Authorize rename** of `heap_v100_phase128_track_c_wasm_fingerprint` → `heap_v100_phase128_track_c_wasm_digest` **without** changing hasher input bytes, cache tag `format!` contract, or tip body.
2. **S2-I105 pack Δ1** — `crates/syx-driver/src/lib.rs` (def + `compute_cache_version_tag` caller) + soft-nail string probes.
3. **Do not rename:** `syx_gui_dylib_*_fingerprint` C-ABI×19 / dlsym (deferred); **Do not** fold/delete tip body (≠ fold `v100_c`); `lang_props_v*`.
4. **Expected census:** `fingerprint_fn_matches` **20 → 19 (−1)**; `fingerprint_const` **0**; report **0**; inventory **0**.
5. Later packs (C-ABI ABI/dlsym) under sibling ADRs without claiming S2-01 `[x]` — **S2-I106:** [fingerprint C-ABI dylib digest rename](stage2-fingerprint-cabi-dylib-digest-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c`.

## Consequences

- Census drops the sole non–C-ABI `*_fingerprint` fn; floor = C-ABI×19.
- **S2-I106:** C-ABI×19 + in-tree dlsym coordinated to `*_digest`; expected fn **19→0**.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-const-select-rename](stage2-fingerprint-const-select-rename.md) · S2-I104
- [stage2-fingerprint-cabi-dylib-digest-rename](stage2-fingerprint-cabi-dylib-digest-rename.md) · S2-I106
- [stage2-fingerprint-farm-lease-digest-rename](stage2-fingerprint-farm-lease-digest-rename.md) · S2-I101
- [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

