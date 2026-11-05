# ADR: Stage-2 fingerprint_const gui leaf rename (S2-I102)

**Status:** Accepted (S2-I102)  
**Date:** 2026-11-04  
**Context:** After [S2-I101 farm_lease digest rename](stage2-fingerprint-farm-lease-digest-rename.md), `fingerprint_fn` remains at 20 (C-ABI×19 + heap×1). C-ABI cannot drop without ABI rewrite. Open the deferred **`fingerprint_const`** axis: rename syx-gui leaf-module `*_FINGERPRINT` const identifiers to `*_DIGEST` without changing string literal values.

## Decision

1. **Authorize rename** of syx-gui leaf `pub const *_FINGERPRINT` identifiers → `*_DIGEST` **without** changing `&str` literal values or hasher inputs.
2. **S2-I102 pack** — files: `shape.rs`, `egui_backend.rs`, `wgpu_present.rs`, `skia_layout.rs`, `skia_glyph.rs`, `emoji.rs`, plus `lib.rs` call-sites.
3. **Do not rename:** `syx_gui_dylib_*_fingerprint` C-ABI / dlsym; `heap_v100_*_fingerprint`; VM `SYNUI_*_FINGERPRINT`; parser `SELECT_*_FINGERPRINT`; `lang_props_v*`; TOML field `fingerprint`.
4. **Expected census:** `fingerprint_const_matches` **241 → 201 (−40)**; `fingerprint_fn` **20**; report **0**; inventory **0**.
5. Later packs (remaining const sea / heap digest / C-ABI ABI strategy) under sibling ADRs without claiming S2-01 `[x]` — **S2-I103:** [fingerprint-const vm-synui rename](stage2-fingerprint-const-vm-synui-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c`.

## Consequences

- Census drops gui-leaf const identifier matches; C-ABI/fn floor unchanged.
- **S2-I103:** VM `SYNUI_*_FINGERPRINT` consts renamed to `*_DIGEST`; HostFn strings kept; SELECT const deferred.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-farm-lease-digest-rename](stage2-fingerprint-farm-lease-digest-rename.md) · S2-I101
- [stage2-fingerprint-const-vm-synui-rename](stage2-fingerprint-const-vm-synui-rename.md) · S2-I103
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

