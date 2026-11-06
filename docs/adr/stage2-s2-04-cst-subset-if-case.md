# ADR: Stage-2 S2-04 CST subset if-case-simple (S2-I156)

**Status:** Accepted (S2-I156) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I155 guard-let-simple](stage2-s2-04-cst-subset-guard-let.md), CST subset covers `if let` and `guard let`, but W723 `if case .variant…` still hits `CST001` because [`take_if_simple`](../../crates/syx-cst/src/lib.rs) rejects `Case`. AST already parses `.Ident[(let Ident…)]? = subject`. This window deepens CST with a **narrow** dot-variant pattern only — it does **not** claim KeyPath / range / Frontend co-tree / S2-04 `[x]`.

## Decision

1. **Add** `take_if_case_variant_pattern`: `.Ident` [`(` `let` Ident (`,` `let` Ident)* `)`]? (no labels, no KeyPath, no tuple, no `case let`).
2. **Add** `take_if_case_simple`: `if case` + pattern + `=` + `<simple|call-simple>` + then/else (same body shape as if-let-simple); `else if` may recurse into if-case / if-let / if-simple.
3. **Wire** into `take_simple_stmt` (beside if-let, before if-simple).
4. **Validate messages:** admit if-case-simple; keep KeyPath / range / non-dot patterns out.
5. **Tests:** roundtrip Ok on `if_case_mvp` basic / no_binding / with_else / two_cases; KeyPath `if case \Foo.bar` still CST001.
6. **Non-claims (hard):** ≠ range / ≠ KeyPath / ≠ tuple / ≠ case-let / ≠ labeled associated / ≠ if_case_literal|range|is / ≠ FrontendPipeline / ≠ delete `syx-cst` / ≠ lint-on-CST / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Grammar knife | `take_if_case_simple` + `take_if_case_variant_pattern` in `syx-cst` |
| Fixtures | `if_case_mvp` basic / no_binding / with_else / two_cases → `parse_cst_subset` Ok |
| KeyPath | still CST001 |
| Contract | `s2_04_cst_subset_if_case_contract` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- `format_source_auto` / `lint_source_auto` CST-success path covers simple `if case .variant…`.
- **S2-I157 landed:** [cst-subset-range-in-call](stage2-s2-04-cst-subset-range-in-call.md) — `take_integer_range_simple` unlocks `mixed.sy` family (**≠** RangeDyn / ≠ S2-04 `[x]`).
- Next default = Frontend share tree evaluation or more deepen → retire dual `syx-cst` → dedicated **S2-04 `[x]`** claim — **not** Stage-2 exit.
- Prior: [cst-subset-guard-let](stage2-s2-04-cst-subset-guard-let.md) · S2-I155.

## References

- [stage2-s2-04-cst-subset-guard-let](stage2-s2-04-cst-subset-guard-let.md) · S2-I155
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-parser-census](../stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
- W723 `if_case_mvp/`
