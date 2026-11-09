# ADR: Stage-2 S2-04 plan claim (S2-I164)

**Status:** Accepted (S2-I164) · S2-04 `[x]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I163 dual-grammar freeze charter](stage2-s2-04-dual-grammar-freeze-charter.md), the S2-04 construction ladder opened in [S2-I143](stage2-s2-04-open.md) meets the [I159](stage2-s2-04-frontend-share-tree-evaluation.md) Decision 7 acceptance checklist: documented testable parse-strategy SSOT, census dual-grammar KPI freeze, and lint Module-source honesty. This dedicated window flips the plan checkbox only — aligned with [S2-I142 S2-03 claim](stage2-s2-03-claim.md). It does **not** exit Stage-2, migrate `FrontendPipeline` onto CST, or delete `syx-cst`.

## Evidence

| Axis (I159 checklist) | Evidence |
|-----------------------|----------|
| Documented parse strategy SSOT | [I159](stage2-s2-04-frontend-share-tree-evaluation.md) · [I163 Decision 2](stage2-s2-04-dual-grammar-freeze-charter.md) · [census Entry map](../census/stage2-parser-census.md) |
| KPI freeze / stop expanding | [I163](stage2-s2-04-dual-grammar-freeze-charter.md) · `s2_04_dual_grammar_freeze_contract` · freeze baseline `combined_grammar_loc` **12381** |
| Lint Module honesty (lower \| Parser) | [I161](stage2-s2-04-lint-lower-path.md) · `s2_04_lint_lower_path_contract` · `lint_source_auto` → `lower_cst_subset_probe` |
| Tools CST-first | [I151–I153](stage2-s2-04-fmt-write-migration.md) · `s2_04_fmt_strategy_contract` · `s2_04_lint_cst_first_contract` · `s2_04_lsp_format_contract` |
| Lower probe | [I160](stage2-s2-04-cst-lower-probe.md) / [I162](stage2-s2-04-cst-lower-probe-expand.md) · `s2_04_cst_lower_probe_contract` |
| Compile still Parser (honest interim) | `FrontendPipeline` → `Parser::parse` · `syx-frontend` **no** `syx-cst` |
| Honesty OUT | **≠** runtime one-tree compile · **≠** dual-grammar retired · **≠** Stage-2 exit |

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S2-04 → `[x]`**.
2. Keep **Stage-2 → `[~]`** until S2-05…S2-07 acceptance (or explicit Stage-2 exit ADR).
3. **Scoped `[x]` meaning:** interim dual-track SSOT (tools = CST-first + lower/auto; compile = `Parser` / `FrontendPipeline` until migrate) + freeze KPI + lint Module-source honesty — **not** 「compile and tools already share one runtime parse product」.
4. Update [stage2-s2-04-open](stage2-s2-04-open.md) / [stage2-open](stage2-open.md) / [dual-grammar-freeze-charter](stage2-s2-04-dual-grammar-freeze-charter.md) Consequences; next default = **S2-05 open** or Frontend migrate (separate window).
5. **Non-claims (hard):** ≠ Stage-2 exit / ≠ Frontend already on CST / ≠ delete `syx-cst` / ≠ claim dual-grammar **retired** (only frozen) / ≠ LSP lint diagnostics shipped / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ open or complete S2-05…07 this window.

## Consequences

- S2-04 plan-claimed; Stage-2 stays `[~]` (S2-01…04 `[x]`; S2-05…07 open).
- Default next = **S2-05** open (file-split) or Frontend CST migrate ADR — **not** Stage-2 exit.
- **S2-I165:** [s2-05-open](stage2-s2-05-open.md) landed — S2-05 → `[~]` (census); next construction = **S2-I166 `syx-mir` first extract**（≠ S2-05 `[x]` / ≠ Stage-2 exit）.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-04-open](stage2-s2-04-open.md) · S2-I143
- [stage2-s2-04-frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159
- [stage2-s2-04-dual-grammar-freeze-charter](stage2-s2-04-dual-grammar-freeze-charter.md) · S2-I163
- [stage2-s2-03-claim](stage2-s2-03-claim.md) · S2-I142 (claim pattern)
- [stage2-parser-census](../census/stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
