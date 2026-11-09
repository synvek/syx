# ADR: Stage-2 S2-04 CST subset guard-let-simple (S2-I155)

**Status:** Accepted (S2-I155) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I154 if-let-simple](stage2-s2-04-cst-subset-if-let.md), CST subset covers simple `if let` but W721 `guard let` still hits `CST001` and forces AST fallback on fmt/lint auto paths. AST [`parse_guard_let`](../../crates/syx-parser/src/lib.rs) already defines `guard let Ident = expr else { … }` plus optional trailing success `{ … }`. This window deepens CST only — it does **not** claim if-case patterns, Frontend co-tree, retire dual grammar, or flip S2-04 `[x]`.

## Decision

1. **Add** `take_guard_let_simple`: `guard let Ident = <simple|call-simple> else { stmts }` + optional trailing success `{ stmts }` (matches AST optional body).
2. **Wire** into `take_simple_stmt` / body chains so nested while/if bodies may hold guard-let-simple.
3. **Validate messages:** admit guard-let-simple; keep `if case` and range literals out of subset.
4. **Unit + contract:** roundtrip Ok on `guard_let_mvp` basic / call_rhs / two_guards / multi_else; `if case` still CST001.
5. **Non-claims (hard):** ≠ if case / ≠ guard-case / ≠ FrontendPipeline / ≠ delete `syx-cst` / ≠ lint-on-CST / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ range literal / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Grammar knife | `take_guard_let_simple` in `syx-cst` |
| Fixtures | `guard_let_mvp` basic / call_rhs / two_guards / multi_else → `parse_cst_subset` Ok |
| KeyPath `if case` | still CST001（dot-variant landed in I156） |
| Contract | `s2_04_cst_subset_guard_let_contract` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- `format_source_auto` / `lint_source_auto` CST-success path covers simple `guard let` (no forced AST fallback for those shapes).
- `mixed.sy` (range in success body) may still CST001 — honest AST fallback retained.
- **S2-I156 landed:** [cst-subset-if-case](stage2-s2-04-cst-subset-if-case.md) — `take_if_case_simple` (**≠** KeyPath / ≠ S2-04 `[x]`).
- Next default = range-in-call or Frontend share tree → retire dual `syx-cst` → dedicated **S2-04 `[x]`** claim — **not** Stage-2 exit.
- Prior: [cst-subset-if-let](stage2-s2-04-cst-subset-if-let.md) · S2-I154.

## References

- [stage2-s2-04-cst-subset-if-let](stage2-s2-04-cst-subset-if-let.md) · S2-I154
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
- W721 `guard_let_mvp/`
