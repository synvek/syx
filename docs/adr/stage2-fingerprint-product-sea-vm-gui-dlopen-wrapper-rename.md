# ADR: Stage-2 fingerprint product-sea VM gui_dlopen wrapper rename (S2-I95)

**Status:** Accepted (S2-I95)  
**Date:** 2026-11-04  
**Context:** After [S2-I94 VM stdlib_phase97 host rename](stage2-fingerprint-product-sea-vm-stdlib97-host-rename.md), hosts call `gui_dlopen::dlopen_*_fingerprint` Rust wrappers that remain on-census. Sibling pack: rename the **Rust wrapper surface** only; leave dlsym `b"syx_gui_dylib_*_fingerprint"` byte strings and all C-ABI exports untouched.

## Decision

1. **Authorize rename** of nine `dlopen_*_fingerprint` wrappers plus `read_fingerprint` to `*_digest` **without deleting** product APIs or changing return values beyond the symbol name.
2. **S2-I95 pack Δ10** — rename in `crates/syx-vm/src/gui_dlopen.rs` and soft-nail `stdlib_phase97_hosts.rs` call sites:
   - `dlopen_ttf_fingerprint` → `dlopen_ttf_digest`
   - `dlopen_ime_preedit_fingerprint` → `dlopen_ime_preedit_digest`
   - `dlopen_doctor_coverage_fingerprint` → `dlopen_doctor_coverage_digest`
   - `dlopen_harfbuzz_fingerprint` → `dlopen_harfbuzz_digest`
   - `dlopen_font_fallback_fingerprint` → `dlopen_font_fallback_digest`
   - `dlopen_wgpu_composite_fingerprint` → `dlopen_wgpu_composite_digest`
   - `dlopen_emoji_fingerprint` → `dlopen_emoji_digest`
   - `dlopen_vertical_rl_fingerprint` → `dlopen_vertical_rl_digest`
   - `dlopen_skia_glyph_fingerprint` → `dlopen_skia_glyph_digest`
   - `read_fingerprint` → `read_digest`
3. **Do not rename:** `syx_gui_dylib_*_fingerprint` C exports; any `b"syx_gui_dylib_*_fingerprint"` dlsym strings; syx-gui remaining façades; `farm_lease_*`; `heap_v100_*`; `lang_props_v*`.
4. **Expected census:** `fingerprint_fn_matches` **90 → 80 (−10)**; report **0**; inventory **0**.
5. Later packs (remaining façades / synui / generate) under sibling ADRs without claiming S2-01 `[x]` — **S2-I96:** [gui-remaining-facade rename](stage2-fingerprint-product-sea-gui-remaining-facade-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops ten VM dlopen-wrapper matches; C-ABI surface unchanged.
- **S2-I96:** syx-gui remaining façades ×6 renamed off census (`*_digest`); C-ABI/dlsym strings kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-vm-stdlib97-host-rename](stage2-fingerprint-product-sea-vm-stdlib97-host-rename.md) · S2-I94
- [stage2-fingerprint-product-sea-gui-remaining-facade-rename](stage2-fingerprint-product-sea-gui-remaining-facade-rename.md) · S2-I96
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

