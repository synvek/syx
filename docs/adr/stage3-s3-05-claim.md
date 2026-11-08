# ADR: Stage-3 S3-05 plan claim（S3-I77）

**Status:** Accepted (S3-I77) · S3-05 `[x]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I76 residual-scope](stage3-s3-05-residual-scope.md), Hard OUT construct (I73–I74), claim-prep (I75), and scoped acceptance (I76) are landed. Primary fixtures report **L007=0 / L012=0**; L010–L011 / L012 Stmt / full CST lint remain **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with S3-01…04 scoped claim. It does **not** exit Stage-3, open S3-06 construction, or restore hybrid/full-AST as a hard gate.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I72](stage3-s3-05-open.md) · lint-ast census |
| Construct I73–I74 | L007/L012 false-positive · Struct/Extension type-member |
| Claim-prep | [S3-I75](stage3-s3-05-claim-prep.md) · fixtures · criteria draft |
| Residual-scope | [S3-I76](stage3-s3-05-residual-scope.md) · scoped acceptance · deferred hybrid |
| Primary KPI | census `class_demo_*` / `struct_book_point_l007_count` = **0** |
| Contract | `s3_05_claim_contract` · `s3_05_lint_ast_census_contract` |
| Honesty OUT | **≠** Stage-3 exit · **≠** S3-06…07 construct this window · **≠** restore hybrid as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ AST-ize L010–L012 / ≠ full CST lint rewrite / ≠ S2-04 Frontend rewrite / ≠ further false-positive product knives）, **lint AST/CST false-positive** workstream is complete when I73–I76 hinges + I75 scaffolding hold and Primary fixtures stay **L007=0 / L012=0** — **not** 「全规则 AST/CST」 / **not** 「L010–L011 AST-ized」 / **not** 「Stage-3 exit」. Restoring hybrid/full-CST as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S3-05 → `[x]`**.
2. Keep **Stage-3 → `[~]`** until an explicit Stage-3 exit ADR（checklist **5/7 ≈ 71%**）.
3. Update lint-ast census Current=**S3-I77** · **Claimed: scoped `[x]`**；next default = **S3-I78 S3-06 open** — **≠** Stage-3 exit / **≠** open S3-06 construction this window.
4. **Non-claims (hard):** ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-06…07 open or construct this window / ≠ restore L010–L012 hybrid as claim hard gate / ≠ change `syx-lint` rule behavior this window / ≠ reopen S3-04 fib&lt;0.5 / ≠ language semantics.

## Consequences

- S3-05 plan-claimed `[x]`; Stage-3 stays `[~]`（checklist **5/7 ≈ 71%**）.
- **S3-I78 landed** as [stage3-s3-06-open](stage3-s3-06-open.md)（S3-06 open — LSP census）。
- Default next = **S3-I79** construct — not Stage-3 exit.
- Prior: [stage3-s3-05-residual-scope](stage3-s3-05-residual-scope.md) · S3-I76 · open [S3-I72](stage3-s3-05-open.md).
- Lint-ast census Current → **S3-I77** · Claimed scoped `[x]`；LSP-book census Current → **S3-I78**.

## References

- [stage3-s3-06-open](stage3-s3-06-open.md) · S3-I78
- [stage3-s3-05-residual-scope](stage3-s3-05-residual-scope.md) · S3-I76
- [stage3-s3-05-claim-prep](stage3-s3-05-claim-prep.md) · S3-I75
- [stage3-s3-05-open](stage3-s3-05-open.md) · S3-I72
- [stage3-s3-04-claim](stage3-s3-04-claim.md) · S3-I71（claim 窗形态）
- [stage3-lint-ast-census](../stage3-lint-ast-census.md)
- [stage3-lsp-book-census](../stage3-lsp-book-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-05
