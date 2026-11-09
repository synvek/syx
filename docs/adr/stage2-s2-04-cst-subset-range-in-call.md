# ADR: Stage-2 S2-04 CST subset range-in-call-simple (S2-I157)

**Status:** Accepted (S2-I157) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I156 if-case-simple](stage2-s2-04-cst-subset-if-case.md), control-flow deepen covers if-let / guard-let / if-case, but `print(1...3)` / `print(1..<3)` still hit `CST001` because [`is_simple_value`](../../crates/syx-cst/src/lib.rs) consumes only one token and leaves `DotDotDot`/`DotDotLess` unmatched. W717/W718 range MVP fixtures and three control-flow `mixed.sy` files share this debt. This window adds integer range as a **call/print/return arg** form only — it does **not** claim RangeDyn, optional-chain, Frontend co-tree, or S2-04 `[x]`.

## Decision

1. **Add** `take_integer_range_simple`: `Integer` (`...` | `..<`) `Integer` (closed / half-open).
2. **Wire** before bare-simple branches in `take_call_arg`, `take_print_simple_call`, and `take_return_simple`. **Do not** extend `is_simple_value` (3-token composite).
3. **Tests:** roundtrip Ok on closed/half-open literal shapes; unlock `closed_range_mvp` literal/call_arg/two_ranges/mixed; `half_open_range_mvp` literal/call_arg/two_ranges; and `if_let_mvp|guard_let_mvp|if_case_mvp/mixed.sy`.
4. **Non-claims (hard):** ≠ RangeDyn / ≠ optional-chain (`x?.y`) / ≠ if-case KeyPath / ≠ FrontendPipeline / ≠ delete `syx-cst` / ≠ lint-on-CST / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Grammar knife | `take_integer_range_simple` in `syx-cst` |
| Range MVP | `closed_range_mvp` + `half_open_range_mvp` (sans half_open mixed) |
| Control-flow mixed | three `*/mixed.sy` with `print(1...3)` → Ok |
| Contract | `s2_04_cst_subset_range_in_call_contract` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- `format_source_auto` / `lint_source_auto` CST-success path covers integer range in print/call/return args.
- **S2-I158 landed:** [cst-subset-optional-chain](stage2-s2-04-cst-subset-optional-chain.md) — unlocks `half_open_range_mvp/mixed.sy` (**≠** force-unwrap / ≠ S2-04 `[x]`).
- Next default = Frontend share tree **evaluation ADR** or more deepen → retire dual `syx-cst` → dedicated **S2-04 `[x]`** claim — **not** Stage-2 exit.
- Prior: [cst-subset-if-case](stage2-s2-04-cst-subset-if-case.md) · S2-I156.

## References

- [stage2-s2-04-cst-subset-if-case](stage2-s2-04-cst-subset-if-case.md) · S2-I156
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
- W717 `closed_range_mvp/` · W718 `half_open_range_mvp/`
