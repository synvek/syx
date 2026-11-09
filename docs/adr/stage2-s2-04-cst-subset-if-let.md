# ADR: Stage-2 S2-04 CST subset if-let-simple (S2-I154)

**Status:** Accepted (S2-I154) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I153 lint CST-first](stage2-s2-04-lint-cst-first.md), tool-surface parse strategy (fmt + lint) is aligned, but many Production Scope shapes still hit `CST001` and fall back to AST — notably `if let Ident = …` because [`take_if_simple`](../../crates/syx-cst/src/lib.rs) rejected `Let` alongside `Case`. W720 `if_let_mvp/` fixtures + W877 if-simple skeleton make this a single-window grammar knife. This window deepens CST only — it does **not** claim Frontend co-tree, retire dual grammar, or flip S2-04 `[x]`.

## Decision

1. **Add** `take_if_let_simple`: `if let Ident = <simple|call-simple>` then braced / braceless body, optional `else` / `else if` (same shape as W877 if-simple).
2. **Wire** into `take_simple_stmt` / `take_while_body_stmt` body chains; `else if` arms may recurse into if-let-simple or if-simple.
3. **`take_if_simple`:** reject **only** `if case` (no longer reject `if let`).
4. **Validate messages:** admit if-let-simple; keep `if case` out of subset.
5. **Flip** unit nail `subset_rejects_if_let` → roundtrip Ok; keep `subset_rejects_if_case` CST001.
6. **Non-claims (hard):** ≠ if case / ≠ guard-let / ≠ FrontendPipeline / ≠ delete `syx-cst` / ≠ lint-on-CST / ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open S2-05…07.

## Evidence

| Check | Evidence |
|-------|----------|
| Grammar knife | `take_if_let_simple` in `syx-cst` |
| Fixtures | `if_let_mvp` basic / with_else / two_ifs / call_rhs → `parse_cst_subset` Ok |
| `if case` | still CST001 |
| Contract | `s2_04_cst_subset_if_let_contract` |

## Consequences

- S2-04 stays `[~]`; Stage-2 stays `[~]`.
- `format_source_auto` / `lint_source_auto` CST-success path covers simple `if let` (no forced AST fallback for those shapes).
- `mixed.sy` (range in body) may still CST001 — honest AST fallback retained.
- **S2-I155 landed:** [cst-subset-guard-let](stage2-s2-04-cst-subset-guard-let.md) — `take_guard_let_simple` (**≠** if case / ≠ S2-04 `[x]`).
- Next default = more CST deepen (if-case / range) or Frontend share tree → retire dual `syx-cst` → dedicated **S2-04 `[x]`** claim — **not** Stage-2 exit.
- Prior: [lint-cst-first](stage2-s2-04-lint-cst-first.md) · S2-I153.

## References

- [stage2-s2-04-lint-cst-first](stage2-s2-04-lint-cst-first.md) · S2-I153
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
- W877 / ADR-1221 (if-simple); W720 `if_let_mvp/`
