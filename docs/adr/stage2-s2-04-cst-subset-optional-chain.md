# ADR: Stage-2 S2-04 CST subset optional-chain-simple (S2-I158)

**Status:** Accepted (S2-I158) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I157 range-in-call](stage2-s2-04-cst-subset-range-in-call.md), integer ranges are in the CST print/call/return arg surface, but `print(x?.y)` still hits `CST001` — nailed by `half_open_range_mvp/mixed.sy` and W716 `optional_chain_mvp/`. AST already parses `Ident ? . Ident` as `OptionalChain`. This window adds a **single-segment** optional-chain arg form only — it does **not** claim multi-hop chains, force-unwrap, Frontend co-tree, or S2-04 `[x]`.

## Decision

1. **Add** `take_optional_chain_simple`: `Ident` `?` `.` `Ident` (exactly one hop).
2. **Wire** before bare-simple branches in `take_call_arg`, `take_print_simple_call`, and `take_return_simple` (same layer as integer-range).
3. **Tests:** roundtrip Ok on `optional_chain_mvp` ident / call_arg / two_chains (+ nested if same shape); unlock `half_open_range_mvp/mixed.sy`; flip I157 CST001 nail for that file.
4. **Non-claims (hard):** ≠ multi-hop `a?.b?.c` / ≠ chain-then-call / ≠ force-unwrap `!` / ≠ FrontendPipeline / ≠ delete `syx-cst` / ≠ lint-on-CST / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07 / ≠ switch.

## Evidence

| Check | Evidence |
|-------|----------|
| Grammar knife | `take_optional_chain_simple` in `syx-cst` |
| W716 | `optional_chain_mvp` ident / call_arg / two_chains |
| I157 hook | `half_open_range_mvp/mixed.sy` → Ok |
| Contract | `s2_04_cst_subset_optional_chain_contract` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- `format_source_auto` / `lint_source_auto` CST-success path covers single-hop `?.`.
- `optional_chain_mvp/mixed.sy` (`print(1!)`) may still CST001 — honest AST fallback retained.
- **S2-I159 landed:** [frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) — second axis = CST→AST lower (**≠** FrontendPipeline impl / ≠ S2-04 `[x]`).
- **S2-I160 landed:** [cst-lower-probe](stage2-s2-04-cst-lower-probe.md) — `lower_cst_subset_probe` (**≠** FrontendPipeline / ≠ S2-04 `[x]`).
- Next default = expand probe / lint→lower → retire dual grammar → dedicated **S2-04 `[x]`** claim — **not** Stage-2 exit.
- Prior: [cst-subset-range-in-call](stage2-s2-04-cst-subset-range-in-call.md) · S2-I157.

## References

- [stage2-s2-04-cst-subset-range-in-call](stage2-s2-04-cst-subset-range-in-call.md) · S2-I157
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
- W716 `optional_chain_mvp/`
