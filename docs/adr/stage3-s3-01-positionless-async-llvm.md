# ADR: Stage-3 S3-01 positionless helper 降债（syx-async + syx-llvm）+ KPI 闸（S3-I37）

**Status:** Accepted (S3-I37) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I36 syx-macro](stage3-s3-01-positionless-macro.md), the locked next face is [`syx-async`](../../crates/syx-async)（~64）。单打偏轻，本窗加速打包 [`syx-llvm`](../../crates/syx-llvm)（~30）— still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`async_error_at`](../../crates/syx-async/src/async_diag.rs)、[`llvm_error_at`](../../crates/syx-llvm/src/llvm_diag.rs)；async lib re-exports `_at`；llvm `lib.rs` uses `_at`；**forbid** new bare `async_error(` / `llvm_error(` on IN paths.
2. **IN convert：** async 5 files **64** + llvm `lib.rs` **30** → `*_error_at(..., None, Span::DUMMY, ...)`。No user `SourceFile` — no fake span.
3. **Gate KPIs：** census `positionless_async_bare=0` + `positionless_llvm_bare=0`；**contract nails both = 0**；prior macro/pkg/vm/driver/miette KPIs retained；**do not** change `host_async_bare` / `host_llvm_bare`（I9/I8 轴）.
4. **Ladder next：** **S3-I38** = [`syx-rt`](../../crates/syx-rt)（~23 `rt_error(`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ rt/jit/debug mass this window / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `async_error(` = 0 on 5 files · `llvm_error(` = 0 on `lib.rs` |
| Census KPI | `positionless_async_bare=0` · `positionless_llvm_bare=0` · global ~144 |
| Prior gates | macro / pkg / mid / wasm-run / vm… = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I37 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- async + llvm positionless faces gated; rt → I38.
- **S3-I38 landed** as [stage3-s3-01-positionless-rt-jit-debug](stage3-s3-01-positionless-rt-jit-debug.md)（rt/jit/debug/codegen positionless=0）.
- **S3-I39 landed** as [stage3-s3-01-positionless-residual](stage3-s3-01-positionless-residual.md)（十 residual KPI=0）.
- **S3-I40 landed** as [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md)（`positionless_helper_calls=0` filtered）.

- **S3-I41 landed** as [stage3-s3-01-claim](stage3-s3-01-claim.md)（S3-01 `[x]` scoped claim）.

- **S3-I42 landed** as [stage3-s3-02-open](stage3-s3-02-open.md)（S3-02 `[~]` open）.

- **S3-I43 landed** as [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md).
- **S3-I44 landed** as [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md)（debug stack display）.
- **S3-I45 landed** as [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md)（runtime `__type` 用户面）.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- Default next after I47 = **S3-03 open** — not Stage-3 exit.
- Prior: [stage3-s3-01-positionless-macro](stage3-s3-01-positionless-macro.md) · S3-I36.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-macro](stage3-s3-01-positionless-macro.md) · S3-I36
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
