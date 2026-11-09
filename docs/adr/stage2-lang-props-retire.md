# ADR: Stage-2 lang_props_v* product retire (S2-I107)

**Status:** Accepted (S2-I107)  
**Date:** 2026-11-05  
**Context:** After [S2-I106 C-ABI digest rename](stage2-fingerprint-cabi-dylib-digest-rename.md), census fingerprint/inventory/report axes are **0**. Remaining explicit S2-01 KPI is `syx_driver_src_lang_props_v` **5**. Modules `lang_props_v2…v6` are gate-label const packs consumed only by tests; product cache invalidation already inlines the same `b"lang-props-v*:…"` bytes in `lib.rs`.

## Decision

1. **Authorize deletion** of `crates/syx-driver/src/lang_props_v2.rs` … `lang_props_v6.rs` and their `pub mod` lines.
2. **Do not** change `hasher.update(b"lang-props-v*:…")` invalidation bytes in `lib.rs`.
3. Retarget or remove soft-nail tests that `use syx_driver::lang_props_v*` / `include_str!(…/lang_props_v*.rs)`; keep behavior tests; keep LSP/cache soft-nails that assert against `lib.rs` literals.
4. **Expected census:** `syx_driver_src_lang_props_v` **5 → 0**; fingerprint/inventory/report remain **0**.
5. Later packs (S2-02 soft-nail / S2-01 full勾) under sibling ADRs without claiming S2-01 `[x]` — **S2-I108:** [S2-02 open](stage2-s2-02-open.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c`.

## Consequences

- Product `src` no longer links dedicated lang_props gate modules; invalidation contract unchanged.
- **S2-I108:** S2-02 opened; props fingerprint soft-nail tests deleted (behavior kept).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-cabi-dylib-digest-rename](stage2-fingerprint-cabi-dylib-digest-rename.md) · S2-I106
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

