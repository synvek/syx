# ADR: Stage-3 S3-06 open — LSP：书中块零误报（S3-I78）

**Status:** Accepted (S3-I78) · S3-06 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I77 S3-05 claim](stage3-s3-05-claim.md), Stage-3 default work shifts to [plan/02 S3-06](../../plan/02-improvement-roadmap.md): **LSP 书中块零误报**，并补齐重命名 / signature help / 类型提示验收。Honest inventory shows a mature LSP stack (`syx-lsp` · `lsp_ide`) with rename / signatureHelp / hover / diagnostics already wired, while **book-block × LSP diagnostic false positives are not measured** (`book_pass_rate` only gates `syx check`). This window **opens** the workstream with an LSP / book **census baseline** — it does **not** claim S3-06 `[x]`, ship LSP product knives, or exit Stage-3.

## Gap: book LSP vs plan S3-06

| Claim | Today (honest) | plan/02 S3-06 target |
|-------|----------------|----------------------|
| 书中块 LSP 零误报 | `book_pass_rate` = check-only；**无** `book_lsp_*` KPI | 编辑器语义诊断对书块零误报 |
| Rename | `renameProvider` + 词面 `prepare_rename` | 书块可安全 rename |
| Signature help | capabilities + 单测齐 | 书块调用处可用 |
| 类型提示 / hover | 已接线；S3-02 demangle **Claimed** | 书块用户面类型提示可用 |

Baseline: [stage3-lsp-book-census](../stage3-lsp-book-census.md).

## Decision

1. **Open S3-06** as an active Stage-3 workstream; flip plan S3-06 `[ ]` → `[~]`.
2. **S3-I78 first slice** — publish lsp-book census (+ script); add a thin contract under `syx-driver/tests/` (**no** `syx-lsp` / `lsp_ide` behavior edit this window).
3. **Ladder:** **S3-I79** = first construct knife (priority: **书块 LSP 语义诊断误报**轴，由本窗 census 数字驱动) → residual → dedicated **S3-06 `[x]` claim**.
4. **Non-claims (hard):** ≠ S3-06 `[x]` / ≠ 本窗书中块零误报宣称 / ≠ 本窗 `syx-lsp`·`lsp_ide` 产品刀 / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ Wave·F-08 / ≠ open S3-07 construction / ≠ reopen S3-05 lint / S3-04 fib&lt;0.5 / ≠ change language semantics / ≠ expand HostFn surface.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| LSP-book census | [`docs/stage3-lsp-book-census.md`](../stage3-lsp-book-census.md) · `scripts/s3_06_lsp_book_census.sh` |
| Contract nail | `s3_06_lsp_census_contract` |
| Plan status | S3-06 `[~]` under this ADR · Stage-3 `[~]` · **5/7** · S3-01…05 `[x]` |

## Consequences

- S3-06 moved `[ ]` → `[~]` under this open window; later claimed `[x]` by S3-I81.
- **S3-I79 landed** as [stage3-s3-06-s045-lsp-desugar](stage3-s3-06-s045-lsp-desugar.md)（S045 LSP desugar parity）。
- **S3-I80 landed** as [stage3-s3-06-residual-scope](stage3-s3-06-residual-scope.md)（residual-scope）。
- **S3-I81 landed** as [stage3-s3-06-claim](stage3-s3-06-claim.md)（S3-06 `[x]` scoped claim）；Stage-3 stays `[~]`（**6/7**）。
- Default next = **S3-I82**（S3-07 open）— not Stage-3 exit.
- Prior: [stage3-s3-05-claim](stage3-s3-05-claim.md) · S3-I77.
- LSP-book census Current → **S3-I81** · Claimed scoped `[x]`.

## References

- [stage3-s3-06-s045-lsp-desugar](stage3-s3-06-s045-lsp-desugar.md) · S3-I79
- [stage3-s3-05-claim](stage3-s3-05-claim.md) · S3-I77
- [stage3-open](stage3-open.md) · S3-I1
- [stage3-lsp-book-census](../stage3-lsp-book-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-06
- [`crates/syx-lsp/src/lib.rs`](../../crates/syx-lsp/src/lib.rs)
- [`crates/syx-driver/src/lsp_ide.rs`](../../crates/syx-driver/src/lsp_ide.rs)
