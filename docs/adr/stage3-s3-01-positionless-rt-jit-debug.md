# ADR: Stage-3 S3-01 positionless helper 降债（rt + jit + debug + codegen）+ KPI 闸（S3-I38）

**Status:** Accepted (S3-I38) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I37 async+llvm](stage3-s3-01-positionless-async-llvm.md), the locked next face is [`syx-rt`](../../crates/syx-rt)（~23）。单打偏轻，本窗加速打包 cranelift-jit + debug + codegen（共 **72**）— still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`rt_error_at`](../../crates/syx-rt/src/rt_diag.rs)、[`jit_error_at`](../../crates/syx-cranelift-jit/src/jit_diag.rs)、[`debug_error_at`](../../crates/syx-debug/src/debug_diag.rs)、[`codegen_error_at`](../../crates/syx-codegen/src/codegen_diag.rs)；IN 改用 `_at`；**forbid** new bare `rt_error(` / `jit_error(` / `debug_error(` / `codegen_error(` on IN paths.
2. **IN convert：** 5 files **72** → `*_error_at(..., None, Span::DUMMY, ...)`：`bridge` · `abi` · jit `lib` · debug `lib` · codegen `lib`。No user `SourceFile` — no fake span.
3. **Gate KPIs：** census `positionless_rt_bare=0` · `positionless_jit_bare=0` · `positionless_debug_bare=0` · `positionless_codegen_bare=0`；**contract nails all = 0**；prior async/llvm/… KPIs retained；**do not** change `host_rt_jit_debug_bare` / `host_codegen_bare`（I22/I10 轴）.
4. **Ladder next：** **S3-I39** = residual MIR/meta 面（`mir_error` / `mir_run_error` / `meta_error` 等）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ mir/meta mass this window / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR semantics / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | rt/jit/debug/codegen bare = 0 on IN files |
| Census KPI | four KPIs = 0 · global ~72 |
| Prior gates | async/llvm/macro/pkg/… = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I38 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- rt/jit/debug/codegen positionless faces gated; residual MIR/meta → I39.
- **S3-I39 landed** as [stage3-s3-01-positionless-residual](stage3-s3-01-positionless-residual.md)（十 residual KPI=0）.
- **S3-I40 landed** as [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md)（`positionless_helper_calls=0` filtered）.
- Default next after I41 = **S3-I42 S3-02 open** — not Stage-3 exit. (S3-I41 claim landed.)
- Prior: [stage3-s3-01-positionless-async-llvm](stage3-s3-01-positionless-async-llvm.md) · S3-I37.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-async-llvm](stage3-s3-01-positionless-async-llvm.md) · S3-I37
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
