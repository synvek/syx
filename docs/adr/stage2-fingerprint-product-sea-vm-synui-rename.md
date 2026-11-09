# ADR: Stage-2 fingerprint product-sea VM synui rename (S2-I97)

**Status:** Accepted (S2-I97)  
**Date:** 2026-11-04  
**Context:** After [S2-I96 gui remaining façade rename](stage2-fingerprint-product-sea-gui-remaining-facade-rename.md), seven `fn *_fingerprint` matches remain in synui real+stub sources. Sibling pack: rename Rust symbols only; leave HostFn registration strings `__syx_synui_*` and `HostFnId` enum variants untouched.

## Decision

1. **Authorize rename** of synui Rust fingerprint helpers/hosts to `*_digest` **without deleting** product APIs or changing SYX HostFn names.
2. **S2-I97 pack Δ7** — five unique symbols (real+stub = seven census rows):
   - `view_structural_fingerprint` → `view_structural_digest`
   - `structural_render_fingerprint` → `structural_render_digest`
   - `synui_fingerprint` → `synui_digest` (real + stub)
   - `host_synui_fingerprint` → `host_synui_digest` (real + stub)
   - `host_synui_render_fingerprint` → `host_synui_render_digest` (real + stub macro list)
3. Soft-nail: `hosts.rs` dispatch pointers; `lib.rs` re-export; same-file tests.
4. **Do not rename:** `__syx_synui_fingerprint` / `__syx_synui_render_fingerprint`; `HostFnId::SynuiFingerprint` / `SynuiRenderFingerprint`; `SYNUI_*_FINGERPRINT` consts; C-ABI; `farm_lease_*`; `heap_v100_*`; `lang_props_v*`.
5. **Expected census:** `fingerprint_fn_matches` **74 → 67 (−7)**; report **0**; inventory **0**.
6. Later packs (generate / orphan) under sibling ADRs without claiming S2-01 `[x]` — **S2-I98:** [generate rename](stage2-fingerprint-product-sea-generate-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops seven synui matches; SYX HostFn surface unchanged.
- **S2-I98:** driver generate helpers ×3 renamed off census (`*_digest`); C-ABI/farm_lease/`heap_v100` kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-gui-remaining-facade-rename](stage2-fingerprint-product-sea-gui-remaining-facade-rename.md) · S2-I96
- [stage2-fingerprint-product-sea-generate-rename](stage2-fingerprint-product-sea-generate-rename.md) · S2-I98
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

