# ADR: Stage-3 S3-01 positionless helper 降债（vm hosts）+ KPI 闸（S3-I30）

**Status:** Accepted (S3-I30) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I29 vm phase84](stage3-s3-01-positionless-vm-phase84.md), the largest remaining single-file VM residual is [`hosts.rs`](../../crates/syx-vm/src/hosts.rs) (~70 `vm_error(`). This window clears it — still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); **forbid** new bare `vm_error(` on IN path.
2. **IN convert：** [`hosts.rs`](../../crates/syx-vm/src/hosts.rs) **70** → `vm_error_at(..., None, Span::DUMMY, ...)` in two batches (~35+35). Runtime hosts have **no** user `SourceFile` — no fake span.
3. **Gate KPI：** census `positionless_vm_hosts_bare=0`；**contract nails `= 0`**; prior `positionless_vm_phase84_bare=0` + driver/miette/bare KPIs retained; **do not** change `host_vm_hosts_bare` (I14 `SyxDiagnostic` axis).
4. **Ladder next：** **S3-I31** = [`stdlib_phase89_hosts.rs`](../../crates/syx-vm/src/stdlib_phase89_hosts.rs)（~62）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ phase89/120 mass this window / ≠ pkg/macro / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN face bare | `vm_error(` = 0 on `hosts.rs` |
| Census KPI | `positionless_vm_hosts_bare=0` · global ~768 |
| Prior gates | phase84 / driver positionless = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I30 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Largest VM single-file positionless face gated; phase89 → I31.
- **S3-I31 landed** as [stage3-s3-01-positionless-vm-phase89](stage3-s3-01-positionless-vm-phase89.md)（`positionless_vm_phase89_bare=0`）.
- **S3-I32 landed** as [stage3-s3-01-positionless-vm-phase120](stage3-s3-01-positionless-vm-phase120.md)（phase120/http/os positionless=0）.
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
- Prior: [stage3-s3-01-positionless-vm-phase84](stage3-s3-01-positionless-vm-phase84.md) · S3-I29.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-vm-phase84](stage3-s3-01-positionless-vm-phase84.md) · S3-I29
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
