# ADR: Stage-2 fingerprint product-sea gui shape-core rename (S2-I91)

**Status:** Accepted (S2-I91)  
**Date:** 2026-11-04  
**Context:** After [S2-I90 select-channel rename](stage2-fingerprint-product-sea-select-channel-rename.md), remaining product-sea mass is dominated by gui (`shape` / façade / leaf / C-ABI) and `farm_lease_*`. Sibling pack under the product-sea rename family: carve **shape-core** only (pure Rust helpers), leaving `gui_*_fingerprint` wrappers and `syx_gui_dylib_*` C-ABI untouched.

## Decision

1. **Authorize rename** of shape-core helpers from `*_fingerprint` → `*_digest` **without deleting** product APIs or changing return values/behavior beyond the symbol name.
2. **S2-I91 pack Δ9** — rename defs in `crates/syx-gui/src/shape.rs` and update `shape::*_fingerprint()` call sites in `crates/syx-gui/src/lib.rs` only:
   - `dylib_ot_vf_parity_fingerprint` → `dylib_ot_vf_parity_digest`
   - `vf_paint_variations_fingerprint` → `vf_paint_variations_digest`
   - `vf_axes_ge3_fingerprint` → `vf_axes_ge3_digest`
   - `vf_multi_axis_fingerprint` → `vf_multi_axis_digest`
   - `vf_wght_fingerprint` → `vf_wght_digest`
   - `ot_features_fingerprint` → `ot_features_digest`
   - `harfbuzz_fingerprint` → `harfbuzz_digest`
   - `hb_paint_fingerprint` → `hb_paint_digest`
   - `font_fallback_fingerprint` → `font_fallback_digest`
3. **Do not rename** this window: `gui_*_fingerprint` façade wrappers; `syx_gui_dylib_*_fingerprint` C exports; `gui_dlopen` dlsym strings; leaf modules (cjk/emoji/skia/wgpu); `stdlib_phase97` hosts.
4. **Expected census:** `fingerprint_fn_matches` **131 → 122 (−9)**; report **0**; inventory **0**.
5. **MUST STAY:** `heap_v100_…`; `farm_lease_*_fingerprint`; capabilities API; `lang_props_v*`; gui C-ABI / dlopen; `gui_*_fingerprint` façades (cleared in S2-I92 sibling); leaf fingerprints; five B-09 flags; `llvm_stub_audit_report`.
6. Later packs (gui façade / leaf / generate) under sibling ADRs without claiming S2-01 `[x]` — **S2-I92:** [gui-shape-facade rename](stage2-fingerprint-product-sea-gui-shape-facade-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops nine shape-core matches; public `gui_*_fingerprint` names remain on-census until a façade pack.
- **S2-I92:** shape façades ×9 renamed off census (`gui_*_digest`).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-select-channel-rename](stage2-fingerprint-product-sea-select-channel-rename.md) · S2-I90
- [stage2-fingerprint-product-sea-gui-shape-facade-rename](stage2-fingerprint-product-sea-gui-shape-facade-rename.md) · S2-I92
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

