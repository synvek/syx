# ADR: Stage-2 fingerprint product-sea gui shape façade rename (S2-I92)

**Status:** Accepted (S2-I92)  
**Date:** 2026-11-04  
**Context:** After [S2-I91 gui shape-core rename](stage2-fingerprint-product-sea-gui-shape-rename.md), nine public `gui_*_fingerprint` wrappers in `syx-gui` `lib.rs` still forward to `shape::*_digest()` and remain on-census. Sibling pack: rename façades only; leave C-ABI, dlsym, VM hosts, and leaf modules untouched.

## Decision

1. **Authorize rename** of the nine shape façades from `gui_*_fingerprint` → `gui_*_digest` **without deleting** product APIs or changing return values/behavior beyond the symbol name.
2. **S2-I92 pack Δ9** — rename in `crates/syx-gui/src/lib.rs` (defs + same-file tests only):
   - `gui_harfbuzz_fingerprint` → `gui_harfbuzz_digest`
   - `gui_hb_paint_fingerprint` → `gui_hb_paint_digest`
   - `gui_font_fallback_fingerprint` → `gui_font_fallback_digest`
   - `gui_ot_features_fingerprint` → `gui_ot_features_digest`
   - `gui_dylib_ot_vf_parity_fingerprint` → `gui_dylib_ot_vf_parity_digest`
   - `gui_vf_wght_fingerprint` → `gui_vf_wght_digest`
   - `gui_vf_multi_axis_fingerprint` → `gui_vf_multi_axis_digest`
   - `gui_vf_axes_ge3_fingerprint` → `gui_vf_axes_ge3_digest`
   - `gui_vf_paint_variations_fingerprint` → `gui_vf_paint_variations_digest`
3. **Do not rename** this window: `syx_gui_dylib_*_fingerprint`; `gui_dlopen` dlsym strings; VM `stdlib_phase97` `gui_*_fingerprint` hosts; leaf modules; other non-shape `gui_*_fingerprint`; `shape.rs`.
4. **Expected census:** `fingerprint_fn_matches` **122 → 113 (−9)**; report **0**; inventory **0**.
5. **MUST STAY:** `heap_v100_…`; `farm_lease_*_fingerprint`; capabilities API; `lang_props_v*`; gui C-ABI / dlopen / leaf (cleared in S2-I93 sibling); five B-09 flags; `llvm_stub_audit_report`.
6. Later packs (gui leaf / generate) under sibling ADRs without claiming S2-01 `[x]` — **S2-I93:** [gui-leaf rename](stage2-fingerprint-product-sea-gui-leaf-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops nine façade matches confined to `syx-gui` `lib.rs`.
- **S2-I93:** render leaf ×14 renamed off census (`*_digest`).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-gui-shape-rename](stage2-fingerprint-product-sea-gui-shape-rename.md) · S2-I91
- [stage2-fingerprint-product-sea-gui-leaf-rename](stage2-fingerprint-product-sea-gui-leaf-rename.md) · S2-I93
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

