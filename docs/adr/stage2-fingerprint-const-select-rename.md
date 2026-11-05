# ADR: Stage-2 fingerprint_const SELECT rename (S2-I104)

**Status:** Accepted (S2-I104)  
**Date:** 2026-11-05  
**Context:** After [S2-I103 VM synui const rename](stage2-fingerprint-const-vm-synui-rename.md), remaining `fingerprint_const` mass is **entirely** the SELECT cluster (`SELECT_*_FINGERPRINT` in parser + `channel_hosts` + soft-nail). Sibling pack: rename const identifiers only to clear the const axis to 0.

## Decision

1. **Authorize rename** of all `SELECT_*_FINGERPRINT` const identifiers → `SELECT_*_DIGEST` **without** changing `&str` literal values.
2. **S2-I104 pack** — `syx-parser/src/lib.rs`, `syx-vm/src/channel_hosts.rs`, soft-nail `select_deadline_strict.rs` + `lsp_phase109`…`116_real.rs`.
3. **Do not rename:** `syx_gui_dylib_*_fingerprint` C-ABI; `heap_v100_*_fingerprint` (deferred); `__syx_synui_*` / `HostFnId`; `lang_props_v*`; `PHASE*_WASM_FINGERPRINTS` (plural; outside census `\b`).
4. **Expected census:** `fingerprint_const_matches` **55 → 0 (−55)** (confirm with script); `fingerprint_fn` **20**; report **0**; inventory **0**.
5. Later packs (heap digest / C-ABI ABI) under sibling ADRs without claiming S2-01 `[x]` — **S2-I105:** [fingerprint heap-tip digest rename](stage2-fingerprint-heap-tip-digest-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c` — even after const axis reaches 0.

## Consequences

- Census drops SELECT const identifier matches; const axis cleared; fn floor (C-ABI×19 + heap×1) unchanged.
- **S2-I105:** heap tip `*_fingerprint` → `*_digest`; expected fn **20→19**; C-ABI floor kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-const-vm-synui-rename](stage2-fingerprint-const-vm-synui-rename.md) · S2-I103
- [stage2-fingerprint-const-gui-leaf-rename](stage2-fingerprint-const-gui-leaf-rename.md) · S2-I102
- [stage2-fingerprint-heap-tip-digest-rename](stage2-fingerprint-heap-tip-digest-rename.md) · S2-I105
- [stage2-fingerprint-product-sea-select-channel-rename](stage2-fingerprint-product-sea-select-channel-rename.md) · S2-I90
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

