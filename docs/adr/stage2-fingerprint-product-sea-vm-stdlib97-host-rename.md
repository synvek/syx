# ADR: Stage-2 fingerprint product-sea VM stdlib_phase97 host rename (S2-I94)

**Status:** Accepted (S2-I94)  
**Date:** 2026-11-04  
**Context:** After [S2-I93 gui render leaf rename](stage2-fingerprint-product-sea-gui-leaf-rename.md), VM `stdlib_phase97_hosts` still exposes nine `gui_*_fingerprint` HostFn-facing helpers that wrap `gui_dlopen::dlopen_*_fingerprint`. Sibling pack: rename the **host surface** only; leave dlopen wrappers, dlsym byte strings, and C-ABI untouched.

## Decision

1. **Authorize rename** of the nine stdlib_phase97 gui hosts from `gui_*_fingerprint` → `gui_*_digest` **without deleting** product APIs or changing return values beyond the symbol name.
2. **S2-I94 pack Δ9** — rename in `crates/syx-vm/src/stdlib_phase97_hosts.rs` (+ `lib.rs` re-exports) and soft-nail driver gui tests:
   - `gui_ttf_subset_fingerprint` → `gui_ttf_subset_digest`
   - `gui_ime_preedit_fingerprint` → `gui_ime_preedit_digest`
   - `gui_doctor_coverage_fingerprint` → `gui_doctor_coverage_digest`
   - `gui_wgpu_composite_fingerprint` → `gui_wgpu_composite_digest`
   - `gui_emoji_fingerprint` → `gui_emoji_digest`
   - `gui_vertical_rl_fingerprint` → `gui_vertical_rl_digest`
   - `gui_skia_glyph_fingerprint` → `gui_skia_glyph_digest`
   - `gui_harfbuzz_fingerprint` → `gui_harfbuzz_digest`
   - `gui_font_fallback_fingerprint` → `gui_font_fallback_digest`
3. **Bodies keep calling** `crate::gui_dlopen::dlopen_*_fingerprint()` (unchanged this window).
4. **Do not rename:** `syx_gui_dylib_*_fingerprint`; `dlopen_*_fingerprint` / `read_fingerprint`; dlsym `b"syx_gui_dylib_*_fingerprint"`; syx-gui remaining façades; `farm_lease_*`; `heap_v100_*`; `lang_props_v*`.
5. **Expected census:** `fingerprint_fn_matches` **99 → 90 (−9)**; report **0**; inventory **0**.
6. Later packs (dlopen Rust wrappers / remaining façades / generate) under sibling ADRs without claiming S2-01 `[x]` — **S2-I95:** [vm-gui-dlopen-wrapper rename](stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops nine VM host matches; dlopen/C-ABI remain on-census until later packs.
- **S2-I95:** `gui_dlopen` Rust wrappers ×10 renamed off census (`*_digest`); C-ABI/dlsym strings kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-gui-leaf-rename](stage2-fingerprint-product-sea-gui-leaf-rename.md) · S2-I93
- [stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename](stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename.md) · S2-I95
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

