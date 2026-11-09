# ADR: Stage-2 S2-02 LSP hybrid fingerprint stub soft retire (S2-I123)

**Status:** Accepted (S2-I123) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I122 backends column pin retire](stage2-backends-wasm-column-doc-pin-retire.md), remaining LSP soft-nail mass in the former “hybrid×36” KEEP set ([S2-I120](stage2-lsp-pure-stub-retire.md)) splits into (a) include_str fingerprint + trivial smoke stubs and (b) pure `pN_doc` DI probes / SynUI v13 deferred policy stubs. This window retires (a) only.

## Decision

1. **Revise** S2-I120 KEEP for hybrid stubs: no longer keep all 36; keep only pure DI `pN_doc`×6 + `*_synui_deferred_no_v13_stub`×4.
2. **Authorize deletion** of **26** fingerprint hybrid `*_stub` tests under [`lsp_phase124`–`135_real.rs`](../../crates/syx-lsp/src/) that contain `include_str!(self).contains(…)` (attr/SynUI surface pins + optional trivial smoke).
3. **Relax** floors:
   - p128–p131: **36 → 35**
   - p132–p133: **37 → 36**
   - p134–p135: **36 → 35**
4. **Do not** change p124–p127 floors; **do not** rename hasher / `cache_v111` tags.
5. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
6. **Keep** DI×6 + v13 deferred×4; all `*_contract`; props docs_honesty×3; product caps/inventory; `cache_v111`; hasher.
7. **Expected:** S2-01 census KPIs remain **0**; ≈ −26 soft-nail tests.
8. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Hybrid fingerprint soft subclass closes; residual LSP stubs = KEEP×10.
- Follow-on platform fingerprint/surface soft retire: [S2-I124](stage2-platform-fingerprint-surface-soft-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-lsp-pure-stub-retire](stage2-lsp-pure-stub-retire.md) · S2-I120
- [stage2-backends-wasm-column-doc-pin-retire](stage2-backends-wasm-column-doc-pin-retire.md) · S2-I122
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
