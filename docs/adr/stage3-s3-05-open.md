# ADR: Stage-3 S3-05 open — lint 基于 AST/CST（S3-I72）

**Status:** Accepted (S3-I72) · S3-05 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I71 S3-04 claim](stage3-s3-04-claim.md), Stage-3 default work shifts to [plan/02 S3-05](../../plan/02-improvement-roadmap.md): lint on AST/CST so **L007/L012-class false positives** are eliminated. Honest inventory shows hybrid rules — L001–L009 on AST `Module`, L010–L012 via `lint_source_text` line heuristics; [`examples/class_demo.sy`](../../examples/class_demo.sy) still emits L007 (class field as `VarGlobal`) and multiple L012 (`print(` inside methods). This window **opens** the workstream with a lint / false-positive **census baseline** — it does **not** claim S3-05 `[x]`, ship lint rule product cuts, or exit Stage-3.

## Gap: L007/L012 vs plan S3-05

| Claim | Today (honest) | plan/02 S3-05 target |
|-------|----------------|----------------------|
| Lint based on AST/CST | L001–L009 AST; **L010–L012** text (`lint_source_text`) | Rules driven by AST/CST structure |
| L007 false positives | Class member `var` without init → L007 via recursive `lint_decl` | No L007 on class fields |
| L012 false positives | Any trimmed line starting `print(` → L012 | No L012 for nested/method `print` (or Stmt-level rule) |
| Canonical fixture | `class_demo`: **L007≥1**, **L012≥1** (open baseline) | Fixture green on those codes |

Baseline: [stage3-lint-ast-census](../census/stage3-lint-ast-census.md).

## Decision

1. **Open S3-05** as an active Stage-3 workstream; flip plan S3-05 `[ ]` → `[~]`.
2. **S3-I72 first slice** — publish lint-ast census (+ script); add a thin contract under `syx-driver/tests/` (**no** `syx-lint` rule behavior edit this window).
3. **Ladder:** **S3-I73** = first construct knife (priority: **L007 class-field context** and/or **L012 non-top-level `print`**) → residual AST-ize of text rules → dedicated **S3-05 `[x]` claim**.
4. **Non-claims (hard):** ≠ S3-05 `[x]` / ≠ fix L007/L012 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ Wave·F-08 / ≠ open S3-06…07 construction / ≠ reopen S3-04 fib&lt;0.5 / VM·JIT product knife / ≠ change S2-04 dual-track Frontend / ≠ change language semantics / ≠ expand HostFn surface.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Lint-ast census | [`docs/census/stage3-lint-ast-census.md`](../census/stage3-lint-ast-census.md) · `scripts/s3_05_lint_ast_census.sh` |
| Contract nail | `s3_05_lint_ast_census_contract` |
| Plan status | S3-05 `[~]` under this ADR · Stage-3 `[~]` · **4/7** · S3-01…04 `[x]` |

## Consequences

- S3-05 moves `[ ]` → `[~]`; Stage-3 stays `[~]`（checklist **4/7 ≈ 57%**）; S3-01…04 stay `[x]`.
- **S3-I73 landed** as [stage3-s3-05-l007-l012-false-positive](stage3-s3-05-l007-l012-false-positive.md)（Class/Actor skip L007 · L012 column-0）。
- **S3-I74 landed** as [stage3-s3-05-struct-extension-type-member](stage3-s3-05-struct-extension-type-member.md)（Struct/Extension type-member）。
- **S3-I75 landed** as [stage3-s3-05-claim-prep](stage3-s3-05-claim-prep.md)（claim-prep）。
- **S3-I76 landed** as [stage3-s3-05-residual-scope](stage3-s3-05-residual-scope.md)（residual-scope）。
- **S3-I77 landed** as [stage3-s3-05-claim](stage3-s3-05-claim.md)（S3-05 `[x]` scoped claim）。
- Default next = **S3-I78**（S3-06 open — LSP）— not Stage-3 exit.
- Prior: [stage3-s3-04-claim](stage3-s3-04-claim.md) · S3-I71.
- Lint-ast census Current → **S3-I77** · Claimed scoped `[x]`.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-04-claim](stage3-s3-04-claim.md) · S3-I71
- [stage3-lint-ast-census](../census/stage3-lint-ast-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-05
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) L007/L012
