# ADR: Stage-2 fingerprint_fn gui C-ABI/dlsym digest rename (S2-I106)

**Status:** Accepted (S2-I106)  
**Date:** 2026-11-05  
**Context:** After [S2-I105 heap tip digest rename](stage2-fingerprint-heap-tip-digest-rename.md), `fingerprint_fn` is **19**, all `syx_gui_dylib_*_fingerprint` C exports. Prior slices deferred this floor. In-tree sole dlsym consumer is `gui_dlopen.rs` (9 of 19 symbols). Authorize coordinated C export + dlsym rename as intentional monorepo ABI break (no dual-symbol / no `export_name` keep-old).

## Decision

1. **Authorize rename** of all nineteen `syx_gui_dylib_*_fingerprint` C exports → `syx_gui_dylib_*_digest` **without** changing `c"…"` return literals.
2. **S2-I106 pack Δ19** — `crates/syx-gui/src/dylib_api.rs` + `crates/syx-vm/src/gui_dlopen.rs` (9× `b"…_fingerprint"` dlsym strings + matching struct fields).
3. **Do not:** keep old export names via `export_name` / dual symbols; rename HostFn `__syx_synui_*_fingerprint` / `HostFnId`; touch `lang_props_v*` (deferred); fold tip body.
4. **Expected census:** `fingerprint_fn_matches` **19 → 0 (−19)**; `fingerprint_const` **0**; report **0**; inventory **0**.
5. Later packs (`lang_props_v*` / S2-02 soft-nail) under sibling ADRs without claiming S2-01 `[x]` — **S2-I107:** [lang_props retire](stage2-lang-props-retire.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c` — even after fn axis reaches 0.

## Consequences

- Census `fingerprint_fn` clears; out-of-tree loaders of old C symbol names break (none in-repo).
- **S2-I107:** product `lang_props_v2…v6` deleted; hasher `b"lang-props-v*:…"` kept; expected lang_props KPI **5→0**.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-heap-tip-digest-rename](stage2-fingerprint-heap-tip-digest-rename.md) · S2-I105
- [stage2-lang-props-retire](stage2-lang-props-retire.md) · S2-I107
- [stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename](stage2-fingerprint-product-sea-vm-gui-dlopen-wrapper-rename.md) · S2-I95
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

