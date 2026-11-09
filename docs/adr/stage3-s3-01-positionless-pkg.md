# ADR: Stage-3 S3-01 positionless helper 降债（syx-pkg）+ KPI 闸（S3-I35）

**Status:** Accepted (S3-I35) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I34 vm mid+wasm-run](stage3-s3-01-positionless-vm-mid.md), the locked next face is [`syx-pkg`](../../crates/syx-pkg)（~87 `pkg_error(`）. This window clears the whole crate — still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`pkg_error_at`](../../crates/syx-pkg/src/pkg_diag.rs)；lib re-exports `_at`；**forbid** new bare `pkg_error(` on IN paths.
2. **IN convert：** 11 files **87** → `pkg_error_at(..., None, Span::DUMMY, ...)`：`registry` · `sources/registry` · `lock_enforce` · `sources/git` · `manifest` · `lockfile` · `lib` · `sources/path` · `installer` · `package` · `resolver`. No user `SourceFile` — no fake span.
3. **Gate KPI：** census `positionless_pkg_bare=0`；**contract nails `= 0`**; prior vm/driver/miette KPIs retained; **do not** change `host_pkg_bare` (I12 axis).
4. **Ladder next：** **S3-I36** = [`syx-macro`](../../crates/syx-macro)（~76 `macro_error(`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ macro/async/llvm mass this window / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `pkg_error(` = 0 on 11 files |
| Census KPI | `positionless_pkg_bare=0` · global ~314 |
| Prior gates | mid / wasm-run / vm… = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I35 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- syx-pkg positionless face gated; macro → I36.
- **S3-I36 landed** as [stage3-s3-01-positionless-macro](stage3-s3-01-positionless-macro.md)（`positionless_macro_bare=0`）.
- **S3-I37 landed** as [stage3-s3-01-positionless-async-llvm](stage3-s3-01-positionless-async-llvm.md)（`positionless_async_bare=0` · `positionless_llvm_bare=0`）.
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
- Prior: [stage3-s3-01-positionless-vm-mid](stage3-s3-01-positionless-vm-mid.md) · S3-I34.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-vm-mid](stage3-s3-01-positionless-vm-mid.md) · S3-I34
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
