# ADR: Stage-2 fingerprint_const VM synui rename (S2-I103)

**Status:** Accepted (S2-I103)  
**Date:** 2026-11-04  
**Context:** After [S2-I102 gui leaf const rename](stage2-fingerprint-const-gui-leaf-rename.md), remaining `fingerprint_const` mass is dominated by VM `SYNUI_*_FINGERPRINT` (real+stub+re-export). [S2-I97](stage2-fingerprint-product-sea-vm-synui-rename.md) already renamed HostFn **implementations** to `*_digest` while keeping these consts. Sibling pack: rename const identifiers only.

## Decision

1. **Authorize rename** of all `SYNUI_*_FINGERPRINT` const identifiers → `SYNUI_*_DIGEST` **without** changing `&str` literal values or HostFn registration strings.
2. **S2-I103 pack** — `synui_hosts.rs`, `synui_hosts_stub.rs`, `lib.rs` re-exports, soft-nail `synui_surface.rs`.
3. **Do not rename:** `__syx_synui_fingerprint` / `__syx_synui_render_fingerprint`; `HostFnId::SynuiFingerprint` / `SynuiRenderFingerprint`; `SELECT_*_FINGERPRINT` (deferred); C-ABI; `heap_v100_*`; `lang_props_v*`.
4. **Expected census:** `fingerprint_const_matches` **201 → 55 (−146)** (confirm with script); `fingerprint_fn` **20**; report **0**; inventory **0**.
5. Later packs (SELECT const / heap digest / C-ABI ABI) under sibling ADRs without claiming S2-01 `[x]` — **S2-I104:** [fingerprint-const select rename](stage2-fingerprint-const-select-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c`.

## Consequences

- Census drops synui const identifier matches; HostFn SYX surface unchanged.
- **S2-I104:** SELECT `*_FINGERPRINT` consts renamed to `*_DIGEST`; const axis expected 0; C-ABI/heap fn floor kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-const-gui-leaf-rename](stage2-fingerprint-const-gui-leaf-rename.md) · S2-I102
- [stage2-fingerprint-const-select-rename](stage2-fingerprint-const-select-rename.md) · S2-I104
- [stage2-fingerprint-product-sea-vm-synui-rename](stage2-fingerprint-product-sea-vm-synui-rename.md) · S2-I97
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
