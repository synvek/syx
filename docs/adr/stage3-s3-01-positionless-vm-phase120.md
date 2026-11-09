# ADR: Stage-3 S3-01 positionless helper 降债（vm phase120 + http/os 加速）+ KPI 闸（S3-I32）

**Status:** Accepted (S3-I32) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I31 vm phase89](stage3-s3-01-positionless-vm-phase89.md), the locked next face is phase120（~30, no wasm twin）. This window clears it and **accelerates** densest neighbors http(+stub)+os — still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); **forbid** new bare `vm_error(` on IN paths.
2. **IN convert（accelerate pack）：** [`stdlib_phase120_hosts.rs`](../../crates/syx-vm/src/stdlib_phase120_hosts.rs) **30** + [`http.rs`](../../crates/syx-vm/src/http.rs) **21** + [`wasm_stubs/http.rs`](../../crates/syx-vm/src/wasm_stubs/http.rs) **22** + [`os_hosts.rs`](../../crates/syx-vm/src/os_hosts.rs) **18** = **91** → `vm_error_at(..., None, Span::DUMMY, ...)`. No user `SourceFile` — no fake span.
3. **Gate KPIs：** census `positionless_vm_phase120_bare=0` · `positionless_vm_http_bare=0`（两 http 文件）· `positionless_vm_os_bare=0`；**contract nails each `= 0`**; prior vm/driver/miette KPIs retained; **do not** change `host_vm_phase120_bare` (I17 axis) or prior `host_*` http/os axes.
4. **Ladder next：** **S3-I33** = [`lib.rs`](../../crates/syx-vm/src/lib.rs)（~28）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ lib.rs / phase83/channel mass this window / ≠ pkg/macro / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `vm_error(` = 0 on phase120 + http(+stub) + os |
| Census KPIs | three gates = 0 · global ~581 |
| Prior gates | phase89 / hosts / phase84 / driver = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I32 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Phase120 + http/os faces gated; lib → I33.
- **S3-I33 landed** as [stage3-s3-01-positionless-vm-lib](stage3-s3-01-positionless-vm-lib.md)（lib/phase83/channel/tail positionless=0）.
- **S3-I34 landed** as [stage3-s3-01-positionless-vm-mid](stage3-s3-01-positionless-vm-mid.md)（mid + wasm-run positionless=0）.
- **S3-I35 landed** as [stage3-s3-01-positionless-pkg](stage3-s3-01-positionless-pkg.md)（`positionless_pkg_bare=0`）.
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
- Prior: [stage3-s3-01-positionless-vm-phase89](stage3-s3-01-positionless-vm-phase89.md) · S3-I31.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-vm-phase89](stage3-s3-01-positionless-vm-phase89.md) · S3-I31
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
