# ADR: Stage-3 S3-06 plan claim（S3-I81）

**Status:** Accepted (S3-I81) · S3-06 `[x]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I80 residual-scope](stage3-s3-06-residual-scope.md), Hard OUT construct (I79 S045 desugar), open (I78), and scoped acceptance (I80) are landed. Primary KPIs report **`s045_lsp_desugar_nail_count=0`** and **`book_lsp_semantic_mismatch_count=0`** on sample fixtures; rename / signature / hover book axes / full fence remain **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with S3-01…05 scoped claim. It does **not** exit Stage-3, open S3-07 construction, or restore full-fence/zero-FP literal as a hard gate.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I78](stage3-s3-06-open.md) · lsp-book census |
| Construct I79 | S045 LSP desugar parity |
| Residual-scope | [S3-I80](stage3-s3-06-residual-scope.md) · scoped acceptance · deferred rename/sig/fence |
| Primary KPI | `s045_lsp_desugar_nail_count=0` · `book_lsp_semantic_mismatch_count=0` |
| Contract | `s3_06_claim_contract` · `s3_06_lsp_census_contract` |
| Honesty OUT | **≠** Stage-3 exit · **≠** S3-07 construct this window · **≠** restore full fence as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ rename·signature·hover product knives / ≠ full fence LSP sweep / ≠ further false-positive product knives beyond I79）, **LSP 书中块语义误报** workstream is complete when I79 hinge + I80 scaffolding hold and Primary sample fixtures stay green — **not** 「全书/全 fence 零误报」 / **not** 「rename·sig·hover 产品验收齐」 / **not** 「Stage-3 exit」。Restoring full-fence or plan-literal zero-FP as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S3-06 → `[x]`**.
2. Keep **Stage-3 → `[~]`** until an explicit Stage-3 exit ADR（checklist later advanced to **7/7** under S3-I85；exit still separate）.
3. Update lsp-book census Current=**S3-I81** · **Claimed: scoped `[x]`**；next default = **S3-I82 S3-07 open** — **≠** Stage-3 exit / **≠** open S3-07 construction this window.
4. **Non-claims (hard):** ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-07 open or construct this window / ≠ 全书零误报字面宣称 / ≠ rename·sig·hover 产品刀 / ≠ full fence / ≠ change `lsp_ide`/`syx-lsp` this window / ≠ reopen S3-05 lint / S3-04 fib&lt;0.5 / ≠ language semantics.

## Consequences

- S3-06 plan-claimed `[x]`; Stage-3 stays `[~]`（checklist **7/7** after S3-I85）。
- **S3-I82 landed** as [stage3-s3-07-open](stage3-s3-07-open.md)（S3-07 open）。
- **S3-I83 landed** as [stage3-s3-07-default-vm](stage3-s3-07-default-vm.md)（default-vm construct）。
- **S3-I84 landed** as [stage3-s3-07-residual-scope](stage3-s3-07-residual-scope.md)（residual-scope）。
- **S3-I85 landed** as [stage3-s3-07-claim](stage3-s3-07-claim.md)（S3-07 `[x]` scoped claim）。
- Default next = **S3-I86**（Stage-3 exit open）— not Stage-3 exit itself.
- Prior: [stage3-s3-06-residual-scope](stage3-s3-06-residual-scope.md) · S3-I80 · open [S3-I78](stage3-s3-06-open.md).
- LSP-book census Current → **S3-I81** · Claimed scoped `[x]`；init-template census Current → **S3-I85**.

## References

- [stage3-s3-06-residual-scope](stage3-s3-06-residual-scope.md) · S3-I80
- [stage3-s3-06-s045-lsp-desugar](stage3-s3-06-s045-lsp-desugar.md) · S3-I79
- [stage3-s3-06-open](stage3-s3-06-open.md) · S3-I78
- [stage3-s3-05-claim](stage3-s3-05-claim.md) · S3-I77（claim 窗形态）
- [stage3-lsp-book-census](../stage3-lsp-book-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-06
