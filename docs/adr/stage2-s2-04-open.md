# ADR: Stage-2 S2-04 open — unified parser (CST→AST) (S2-I143)

**Status:** Accepted (S2-I143) · S2-04 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-06  
**Context:** After [S2-I142 S2-03 claim](stage2-s2-03-claim.md), Stage-2 default work shifts to [plan/02 S2-04](../../plan/02-improvement-roadmap.md): unify the parser so fmt / LSP / lint share one tree and stop dual-maintaining [`syx-cst`](../../crates/syx-cst) grammar alongside [`syx-parser`](../../crates/syx-parser). Today production compile/check/LSP/lint use **AST via `syx-parser`**, while fmt **`--check`** prefers **CST-first** (`format_source_auto`) and fmt **write** still uses AST pretty — two ~6k-LOC hand-written grammar faces. This window **opens** the workstream with a dual-track **census baseline** — it does **not** claim S2-04 `[x]` or delete `syx-cst`.

## Gap: dual track vs plan S2-04

| Claim | Today (honest) | plan/02 S2-04 target |
|-------|----------------|----------------------|
| Single grammar maintenance | **two** crates (`syx-parser` + `syx-cst`) | stop dual-maintaining `syx-cst` syntax |
| Compile / frontend | `Parser::parse` → AST | same tree as tools |
| lint / LSP | AST only (no `syx-cst` dep) | share tree with fmt |
| fmt write (`format_file`) | **AST pretty** (`format_source`) | same tree / lossless where required |
| fmt `--check` | **CST-first** + `CST001`→AST | same path as write (or explicit one SSOT) |
| AST trivia | none | CST→AST (or trivia-bearing AST) |

Baseline numbers and entry points: [stage2-parser-census](../stage2-parser-census.md).

## Decision

1. **Open S2-04** as an active Stage-2 workstream; flip plan S2-04 `[ ]` → `[~]`.
2. **S2-I143 first slice** — document the gap table; publish parser dual-track census (+ script); add a thin behavioral contract under `syx-driver/tests/` (no product `src/` inventory).
3. **Ladder:** **S2-I144** convergence-strategy ([convergence-strategy](stage2-s2-04-convergence-strategy.md)) → migration knives (CST reindent → write/`--check` align → LSP/lint) → retire dual `syx-cst` grammar maintenance → dedicated **S2-04 `[x]`** claim.
4. **Non-claims (hard):** ≠ S2-04 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ delete or freeze `syx-cst` this window / ≠ change `FrontendPipeline` parse path / ≠ land rowan / ≠ open S2-05…07 construction.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Dual-track census | [`docs/stage2-parser-census.md`](../stage2-parser-census.md) · `scripts/s2_04_parser_census.sh` |
| Contract nail | `s2_04_parser_dual_track_baseline` |
| Plan status | S2-04 `[~]` under this ADR |

## Consequences

- S2-04 moves `[ ]` → `[~]`; Stage-2 stays `[~]`.
- **S2-I144:** [convergence-strategy](stage2-s2-04-convergence-strategy.md) — fmt-first; CST reindent prerequisite; write stays AST.
- **S2-I145:** [cst-reindent-func-body](stage2-s2-04-cst-reindent-func-body.md) — opt-in FunctionDecl reindent first knife (**≠** GA).
- **S2-I146:** [cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md) — nested relative depth (**≠** GA).
- **S2-I147:** [cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) — FunctionDecl 2→4 normalize (**≠** GA).
- **S2-I148:** [cst-reindent-class-stored](stage2-s2-04-cst-reindent-class-stored.md) — class stored-member track (**≠** GA).
- **S2-I149:** [cst-reindent-env-gate](stage2-s2-04-cst-reindent-env-gate.md) — `SYX_FMT_CST_REINDENT` (**≠** GA).
- **S2-I150:** [cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) — `format_source_cst` → `format_cst_reindent` (**≠** write 迁入).
- **S2-I151:** [fmt-write-migration](stage2-s2-04-fmt-write-migration.md) — `format_file` → `format_dry_run` / auto (**≠** S2-04 `[x]`).
- **S2-I152:** [lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) — LSP format → dry_run/auto (**≠** lint 共树 / ≠ S2-04 `[x]`).
- **S2-I153:** [lint-cst-first](stage2-s2-04-lint-cst-first.md) — `lint_source_auto` (**≠** lint-on-CST / ≠ S2-04 `[x]`).
- **S2-I154:** [cst-subset-if-let](stage2-s2-04-cst-subset-if-let.md) — `take_if_let_simple` (**≠** if case / ≠ Frontend / ≠ S2-04 `[x]`).
- **S2-I155:** [cst-subset-guard-let](stage2-s2-04-cst-subset-guard-let.md) — `take_guard_let_simple` (**≠** if case / ≠ Frontend / ≠ S2-04 `[x]`).
- **S2-I156:** [cst-subset-if-case](stage2-s2-04-cst-subset-if-case.md) — `take_if_case_simple` (**≠** KeyPath / ≠ Frontend / ≠ S2-04 `[x]`).
- **S2-I157:** [cst-subset-range-in-call](stage2-s2-04-cst-subset-range-in-call.md) — `take_integer_range_simple` (**≠** RangeDyn / ≠ Frontend / ≠ S2-04 `[x]`).
- **S2-I158:** [cst-subset-optional-chain](stage2-s2-04-cst-subset-optional-chain.md) — `take_optional_chain_simple` (**≠** force-unwrap / ≠ Frontend / ≠ S2-04 `[x]`).
- **S2-I159:** [frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) — second axis = CST→AST lower (**≠** FrontendPipeline impl / ≠ S2-04 `[x]`).
- **S2-I160:** [cst-lower-probe](stage2-s2-04-cst-lower-probe.md) — `lower_cst_subset_probe` + Parser Module parity (**≠** FrontendPipeline / ≠ S2-04 `[x]`).
- **S2-I161:** [lint-lower-path](stage2-s2-04-lint-lower-path.md) — `lint_source_auto` → lower; LWR001→Parser (**≠** FrontendPipeline / ≠ expand probe / ≠ S2-04 `[x]`).
- **S2-I162:** [cst-lower-probe-expand](stage2-s2-04-cst-lower-probe-expand.md) — W791–797 let/var/return (**≠** while/typed / ≠ FrontendPipeline / ≠ S2-04 `[x]`).
- **S2-I163:** [dual-grammar-freeze-charter](stage2-s2-04-dual-grammar-freeze-charter.md) — stop default `take_*` expand; KPI freeze (**≠** S2-04 `[x]` / ≠ Frontend migrate).
- **S2-I164:** [s2-04-claim](stage2-s2-04-claim.md) — plan **S2-04 `[x]`** scoped (**≠** Stage-2 exit / ≠ Frontend already on CST).
- **S2-I165:** [s2-05-open](stage2-s2-05-open.md) — S2-05 → `[~]` census open; next = **S2-I166 `syx-mir` first extract** (**≠** S2-05 `[x]` / ≠ Stage-2 exit).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-03-claim](stage2-s2-03-claim.md) · S2-I142
- [stage2-s2-04-convergence-strategy](stage2-s2-04-convergence-strategy.md) · S2-I144
- [stage2-s2-04-cst-reindent-func-body](stage2-s2-04-cst-reindent-func-body.md) · S2-I145
- [stage2-s2-04-cst-reindent-func-nested](stage2-s2-04-cst-reindent-func-nested.md) · S2-I146
- [stage2-s2-04-cst-reindent-func-2to4](stage2-s2-04-cst-reindent-func-2to4.md) · S2-I147
- [stage2-s2-04-cst-reindent-class-stored](stage2-s2-04-cst-reindent-class-stored.md) · S2-I148
- [stage2-s2-04-cst-reindent-env-gate](stage2-s2-04-cst-reindent-env-gate.md) · S2-I149
- [stage2-s2-04-cst-reindent-ga](stage2-s2-04-cst-reindent-ga.md) · S2-I150
- [stage2-s2-04-fmt-write-migration](stage2-s2-04-fmt-write-migration.md) · S2-I151
- [stage2-s2-04-lsp-document-formatting](stage2-s2-04-lsp-document-formatting.md) · S2-I152
- [stage2-s2-04-lint-cst-first](stage2-s2-04-lint-cst-first.md) · S2-I153
- [stage2-s2-04-cst-subset-if-let](stage2-s2-04-cst-subset-if-let.md) · S2-I154
- [stage2-s2-04-cst-subset-guard-let](stage2-s2-04-cst-subset-guard-let.md) · S2-I155
- [stage2-s2-04-cst-subset-if-case](stage2-s2-04-cst-subset-if-case.md) · S2-I156
- [stage2-s2-04-cst-subset-range-in-call](stage2-s2-04-cst-subset-range-in-call.md) · S2-I157
- [stage2-s2-04-cst-subset-optional-chain](stage2-s2-04-cst-subset-optional-chain.md) · S2-I158
- [stage2-s2-04-frontend-share-tree-evaluation](stage2-s2-04-frontend-share-tree-evaluation.md) · S2-I159
- [stage2-s2-04-cst-lower-probe](stage2-s2-04-cst-lower-probe.md) · S2-I160
- [stage2-s2-04-lint-lower-path](stage2-s2-04-lint-lower-path.md) · S2-I161
- [stage2-s2-04-cst-lower-probe-expand](stage2-s2-04-cst-lower-probe-expand.md) · S2-I162
- [stage2-s2-04-dual-grammar-freeze-charter](stage2-s2-04-dual-grammar-freeze-charter.md) · S2-I163
- [stage2-s2-04-claim](stage2-s2-04-claim.md) · S2-I164
- [stage2-parser-census](../stage2-parser-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-04
- Analysis: [`plan/ProjectAnalysisAndImprovementPlan.zh-CN.md`](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §2.4 / §4.2.3
