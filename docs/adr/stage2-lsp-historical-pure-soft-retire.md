# ADR: Stage-2 S2-02 LSP historical pure include_str soft retire (S2-I121)

**Status:** Accepted (S2-I121) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I120 pure stub + soft_pin retire](stage2-lsp-pure-stub-retire.md), remaining large LSP soft-nail mass is non-stub pure `include_str!(…).contains(…)` tests across historical `lsp_phase*` files (heap/lock/inkwell/farm soft, `*_source_gate`, `*_track_fingerprints_inherited`). Hybrid DI/SynUI `*_stub`×36 (`pN_doc` probes) and `*_lsp_semantic_v*_contract` floors stay. backends wasm column pins deferred.

## Decision

1. **Authorize deletion** of **117** pure soft `#[test]` functions under [`crates/syx-lsp/src/lsp_phase*_real.rs`](../../crates/syx-lsp/src/) that:
   - contain `include_str!(…).contains(…)`; and
   - are **not** `*_contract` / `lsp_semantic*` / `matches("#[test]")` floor gates; and
   - are **not** hybrid (`pN_doc` / `semantic.errors` / LSP runtime).
2. **Relax** floors only:
   - p129 / p130 / p131: **37 → 36**
3. **Do not** change other phase floors; **do not** rename hasher / `cache_v111` historical tags.
4. **Do not** change `driver_gate` floors (`#[test]≥450`, `behavioral≥350`) or `soft_debt_zero_meta_allowable_charter`.
5. **Keep** hybrid stubs ×36; all semantic contracts; props docs_honesty×3; product `*_docs_and_capability` / inventory; `cache_v111`; hasher.
6. **Expected:** S2-01 census KPIs remain **0**; ≈ −117 soft-nail tests.
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- LSP historical pure soft sea closes; follow-on backends column pin retire: [S2-I122](stage2-backends-wasm-column-doc-pin-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-lsp-pure-stub-retire](stage2-lsp-pure-stub-retire.md) · S2-I120
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
