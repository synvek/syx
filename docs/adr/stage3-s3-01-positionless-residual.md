# ADR: Stage-3 S3-01 positionless helper 降债（residual MIR/meta+尾面）+ KPI 闸（S3-I39）

**Status:** Accepted (S3-I39) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I38 rt+jit+debug+codegen](stage3-s3-01-positionless-rt-jit-debug.md), remaining **real** positionless helpers total **42** across mir/mir-run/meta/hir/cst/wasm-emit/fmt/compiler-wasm/lexer/lint. This window clears them all — still **≠** S3-01 `[x]`.

## Decision

1. Reuse each crate’s existing `*_error_at`；IN 改用 `_at`；**forbid** new bare corresponding `*_error(` on IN paths.
2. **IN convert：** 12 files **42** → `*_error_at(..., None, Span::DUMMY, ...)` except **lexer**：`lexer_error(...).with_span(src, span, …)` → `lexer_error_at(msg, Some(src), span, code)`（保留真实 span）。
3. **Gate KPIs：** census nails `positionless_mir_bare` · `positionless_mir_run_bare` · `positionless_meta_bare` · `positionless_hir_bare` · `positionless_cst_bare` · `positionless_wasm_emit_bare` · `positionless_fmt_bare` · `positionless_compiler_wasm_bare` · `positionless_lexer_bare` · `positionless_lint_bare` all **=0**；prior gates retained；**do not** change `host_residual_bare`.
4. **Ladder next：** **S3-I40** = census 噪声收紧 + claim-prep 对齐 — still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ noise-as-convert / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR `span: None` semantics / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | 12 files bare = 0 |
| Census KPI | ten residual KPIs = 0 · global ~30 noise |
| Prior gates | rt/jit/…/async/llvm/… = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I39 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- All remaining real positionless helper faces gated; noise/claim-prep → I40.
- **S3-I40 landed** as [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md)（`positionless_helper_calls=0` filtered）.
- Default next after I41 = **S3-I42 S3-02 open** — not Stage-3 exit. (S3-I41 claim landed.)
- Prior: [stage3-s3-01-positionless-rt-jit-debug](stage3-s3-01-positionless-rt-jit-debug.md) · S3-I38.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-rt-jit-debug](stage3-s3-01-positionless-rt-jit-debug.md) · S3-I38
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
