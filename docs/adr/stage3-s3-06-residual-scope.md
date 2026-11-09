# ADR: Stage-3 S3-06 residual-scope（S3-I80）

**Status:** Accepted (S3-I80) · S3-06 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I79 S045 LSP desugar](stage3-s3-06-s045-lsp-desugar.md), the Primary desugar hinge and sample book fixture semantic alignment are green, while plan/open still literally aim at 「书中块零误报」plus rename / signature help / 类型提示. This window revises **S3-06 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S3-06 `[x]`, ship rename/sig/hover knives, full fence LSP sweep, or Stage-3 exit.

## Decision

1. **Scoped acceptance (S3-06 stays `[~]` this window):** Under Hard OUT（≠ rename·signature·hover product knives / ≠ full fence LSP sweep / ≠ further false-positive product knives beyond I79 / ≠ change `diagnostics_for_source`）, honest S3-06 completion means:
   - **Primary:** I79 desugar hinge holds (`should_desugar_top_level_stmts` in `analyze_document_for_lsp`); `s045_lsp_desugar_nail_count=0`;
   - **Sample fixtures:** ≥3 book-named fixtures → `book_lsp_semantic_mismatch_count=0`;
   - **Scaffolding:** census helpers green (`stage3_lsp_census_helper` · `s045_lsp_desugar_helper` · `analyze_document_helper`).
2. **Deferred / supersede-only:** rename honesty · signature book 验收 · hover 书块产品轴 · **full fence** LSP sweep · plan 字面「全书零误报」are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`.
3. **Honesty:** S3-02 demangle hover remains Claimed on the mono-display axis; S3-06 scoped claim does **not** re-require hover product work.
4. **Ladder next:** **S3-I81** = dedicated **S3-06 scoped `[x]` claim** under this acceptance — still ≠ Stage-3 exit / ≠ S3-07 this window.
5. **Non-claims (hard):** ≠ S3-06 `[x]` this window / ≠ 书中块零误报宣称 / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-07 / ≠ rename·signature·hover 产品刀 / ≠ full fence LSP sweep / ≠ reopen S3-05 lint / S3-04 fib&lt;0.5 / ≠ change `lsp_ide` / `syx-lsp` behavior this window / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Census | `residual_scope_helper≥1` · Current=**S3-I80** |
| Plan | S3-06 row stays `[~]` · scoped摘要 |
| Contract | `s3_06_residual_scope_contract` |
| Prior | [I79 S045 desugar](stage3-s3-06-s045-lsp-desugar.md) · [I78 open](stage3-s3-06-open.md) |

## Consequences

- Residual-scope hinge landed；S3-06 was `[~]` under this window.
- **S3-I81 landed** as [stage3-s3-06-claim](stage3-s3-06-claim.md)（S3-06 `[x]` scoped claim）。
- Default next = **S3-I82**（S3-07 open）— not Stage-3 exit.
- Prior: [stage3-s3-06-s045-lsp-desugar](stage3-s3-06-s045-lsp-desugar.md) · S3-I79.
- Census Current → **S3-I81** · Claimed scoped `[x]`.

## References

- [stage3-s3-06-claim](stage3-s3-06-claim.md) · S3-I81
- [stage3-s3-06-s045-lsp-desugar](stage3-s3-06-s045-lsp-desugar.md) · S3-I79
- [stage3-s3-06-open](stage3-s3-06-open.md) · S3-I78
- [stage3-s3-05-residual-scope](stage3-s3-05-residual-scope.md) · S3-I76（形态模板）
- [stage3-lsp-book-census](../census/stage3-lsp-book-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-06
