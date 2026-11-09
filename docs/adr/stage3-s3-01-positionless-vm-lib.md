# ADR: Stage-3 S3-01 positionless helper 降债（vm lib + phase83/channel/tail 加速）+ KPI 闸（S3-I33）

**Status:** Accepted (S3-I33) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I32 phase120+http/os](stage3-s3-01-positionless-vm-phase120.md), the locked next face is [`lib.rs`](../../crates/syx-vm/src/lib.rs)（~28）. This window clears it and **accelerates** densest neighbors phase83+channel+phase_tail — still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); **forbid** new bare `vm_error(` on IN paths.
2. **IN convert（accelerate pack）：** [`lib.rs`](../../crates/syx-vm/src/lib.rs) **28** + [`stdlib_phase83_hosts.rs`](../../crates/syx-vm/src/stdlib_phase83_hosts.rs) **15** + [`channel_hosts.rs`](../../crates/syx-vm/src/channel_hosts.rs) **12** + phase_tail 六文件（90/91/92/95/86/87）**35** = **90** → `vm_error_at(..., None, Span::DUMMY, ...)`. No user `SourceFile` — no fake span（含解释器路径）.
3. **Gate KPIs：** census `positionless_vm_lib_bare=0` · `positionless_vm_phase83_bare=0` · `positionless_vm_channel_bare=0` · `positionless_vm_phase_tail_bare=0`；**contract nails each `= 0`**; prior vm/driver/miette KPIs retained; **do not** change `host_vm_mid_bare` / `host_vm_phase_tail_bare` / `host_vm_phase83_bare` (SyxDiagnostic axes).
4. **Ladder next：** **S3-I34** = VM mid residual（`script_hosts`+`async_iter_hosts`+`gui_*`+`jit`+`synui` ≈14）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ script/async/gui/jit mass this window / ≠ pkg/macro / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `vm_error(` = 0 on lib + phase83 + channel + phase_tail |
| Census KPIs | four gates = 0 · global ~491 |
| Prior gates | phase120/http/os / phase89 / driver = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I33 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Lib + phase83/channel/tail faces gated; mid residual → I34.
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
- Prior: [stage3-s3-01-positionless-vm-phase120](stage3-s3-01-positionless-vm-phase120.md) · S3-I32.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-vm-phase120](stage3-s3-01-positionless-vm-phase120.md) · S3-I32
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
