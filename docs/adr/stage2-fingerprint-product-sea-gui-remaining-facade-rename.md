# ADR: Stage-2 fingerprint product-sea gui remaining façade rename (S2-I96)

**Status:** Accepted (S2-I96)  
**Date:** 2026-11-04  
**Context:** After [S2-I95 VM gui_dlopen wrapper rename](stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename.md), six non-leaf `gui_*_fingerprint` façades remain in `syx-gui` `lib.rs`. Sibling pack: rename those façades only; leave all `syx_gui_dylib_*_fingerprint` C-ABI exports and dlsym byte strings untouched.

## Decision

1. **Authorize rename** of six remaining façades from `*_fingerprint` → `*_digest` **without deleting** product APIs or changing return values beyond the symbol name.
2. **S2-I96 pack Δ6** — rename in `crates/syx-gui/src/lib.rs` (defs + same-file test call-sites / assert messages):
   - `gui_ime_preedit_fingerprint` → `gui_ime_preedit_digest`
   - `gui_egui_mvp_fingerprint` → `gui_egui_mvp_digest`
   - `gui_egui_wgpu_fingerprint` → `gui_egui_wgpu_digest`
   - `gui_vertical_rl_fingerprint` → `gui_vertical_rl_digest`
   - `gui_pump_reentrancy_fingerprint` → `gui_pump_reentrancy_digest`
   - `gui_doctor_coverage_fingerprint` → `gui_doctor_coverage_digest`
3. **Do not rename:** `syx_gui_dylib_*_fingerprint` C exports; any `b"syx_gui_dylib_*_fingerprint"` dlsym strings; synui×7; `farm_lease_*`; `heap_v100_*`; `lang_props_v*`.
4. **Expected census:** `fingerprint_fn_matches` **80 → 74 (−6)**; report **0**; inventory **0**.
5. Later packs (synui / generate / orphan) under sibling ADRs without claiming S2-01 `[x]` — **S2-I97:** [vm-synui rename](stage2-fingerprint-product-sea-vm-synui-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops six syx-gui façade matches; C-ABI surface unchanged.
- **S2-I97:** synui Rust symbols ×7 renamed off census (`*_digest`); HostFn `__syx_synui_*` strings kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename](stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename.md) · S2-I95
- [stage2-fingerprint-product-sea-vm-synui-rename](stage2-fingerprint-product-sea-vm-synui-rename.md) · S2-I97
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

