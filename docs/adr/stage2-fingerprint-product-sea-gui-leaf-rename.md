# ADR: Stage-2 fingerprint product-sea gui render leaf rename (S2-I93)

**Status:** Accepted (S2-I93)  
**Date:** 2026-11-04  
**Context:** After [S2-I92 gui shape façade rename](stage2-fingerprint-product-sea-gui-shape-facade-rename.md), remaining gui product-sea mass includes render **leaf** helpers (cjk/emoji/skia/wgpu) plus matching `gui_*` façades. Leaf-only Δ7 is below the 8–16 knife band; this pack renames leaf-core + façades together (census **Δ14**). C-ABI / dlsym / VM hosts stay.

## Decision

1. **Authorize rename** of render-leaf helpers and their `lib.rs` façades from `*_fingerprint` → `*_digest` **without deleting** product APIs or changing return values beyond the symbol name.
2. **S2-I93 pack Δ14** — leaf defs:
   - `ttf_subset_fingerprint` → `ttf_subset_digest` (`cjk_font.rs`)
   - `emoji_fingerprint` → `emoji_digest` (`emoji.rs`)
   - `skia_glyph_fingerprint` → `skia_glyph_digest` (`skia_glyph.rs`)
   - `skia_layout_fingerprint` → `skia_layout_digest` (`skia_layout.rs`)
   - `font_system_fingerprint` → `font_system_digest` (`skia_layout.rs`)
   - `wgpu_present_fingerprint` → `wgpu_present_digest` (`wgpu_present.rs`)
   - `wgpu_composite_fingerprint` → `wgpu_composite_digest` (`wgpu_present.rs`)
   Façades in `lib.rs` (incl. dual-cfg `gui_font_system_*` = 2 census rows):
   - `gui_font_cjk_subset_fingerprint` → `gui_font_cjk_subset_digest`
   - `gui_emoji_fingerprint` → `gui_emoji_digest`
   - `gui_skia_glyph_fingerprint` → `gui_skia_glyph_digest`
   - `gui_skia_layout_fingerprint` → `gui_skia_layout_digest`
   - `gui_font_system_fingerprint` → `gui_font_system_digest`
   - `gui_wgpu_composite_fingerprint` → `gui_wgpu_composite_digest`
3. **Do not rename:** `syx_gui_dylib_*_fingerprint`; `gui_dlopen` dlsym strings; VM `stdlib_phase97` hosts; non-leaf façades (ime/egui/doctor/vertical…); `farm_lease_*`; `shape` / shape façades already digest.
4. **Expected census:** `fingerprint_fn_matches` **113 → 99 (−14)**; report **0**; inventory **0**.
5. **MUST STAY:** `heap_v100_…`; capabilities; `lang_props_v*`; five B-09 flags; `llvm_stub_audit_report`.
6. Later packs (VM leaf-aligned host / remaining façades / generate) under sibling ADRs without claiming S2-01 `[x]` — **S2-I94:** [vm-stdlib97-host rename](stage2-fingerprint-product-sea-vm-stdlib97-host-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops fourteen matches confined to `syx-gui` leaf + façades.
- **S2-I94:** VM stdlib_phase97 hosts ×9 renamed off census (`gui_*_digest`); dlopen wrappers deferred.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-gui-shape-facade-rename](stage2-fingerprint-product-sea-gui-shape-facade-rename.md) · S2-I92
- [stage2-fingerprint-product-sea-vm-stdlib97-host-rename](stage2-fingerprint-product-sea-vm-stdlib97-host-rename.md) · S2-I94
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
