# ADR: Stage-3 S3-01 positionless helper 降债（driver emit/test/native）+ KPI 闸（S3-I28）

**Status:** Accepted (S3-I28) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I27 driver compile](stage3-s3-01-positionless-driver-compile.md), residual `driver_error(` remained on emit/test/native faces. This window clears those plus `run_wasm` / `cranelift_stubs` — still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`driver_error_at`](../../crates/syx-driver/src/driver_diag.rs); **forbid** new bare `driver_error(` on IN paths.
2. **IN convert：** [`emit.rs`](../../crates/syx-driver/src/emit.rs) (4) + [`test_runner.rs`](../../crates/syx-driver/src/test_runner.rs) (9) + [`native_build.rs`](../../crates/syx-driver/src/native_build.rs) (9) + [`run_wasm.rs`](../../crates/syx-driver/src/run_wasm.rs) (2) + [`cranelift_stubs.rs`](../../crates/syx-driver/src/cranelift_stubs.rs) (1) = **25**. Sites are IO/toolchain failures → `None` + `Span::DUMMY` (no fake user span).
3. **Gate KPI：** census `positionless_driver_emit_test_native_bare=0`；**contract nails `= 0`**; prior parser_driver / compile / bare / miette KPIs retained.
4. **Ladder next：** **S3-I29** = 全局 positionless 最大非-driver `*_error(` 簇（如 `syx-vm`）首批降债 — still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ VM/pkg mass this window / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `driver_error(` = 0 on IN five files |
| Census KPI | `positionless_driver_emit_test_native_bare=0` · global ~871 |
| Prior gates | parser_driver / compile = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I28 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Driver production-face `driver_error(` cleared (diag wrappers excluded).
- **S3-I29 landed** as [stage3-s3-01-positionless-vm-phase84](stage3-s3-01-positionless-vm-phase84.md)（`positionless_vm_phase84_bare=0`）.
- **S3-I30 landed** as [stage3-s3-01-positionless-vm-hosts](stage3-s3-01-positionless-vm-hosts.md)（`positionless_vm_hosts_bare=0`）.
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
- Default next after I41 = **S3-I42 S3-02 open** — not Stage-3 exit. (S3-I41 claim landed.)
- Prior: [stage3-s3-01-positionless-driver-compile](stage3-s3-01-positionless-driver-compile.md) · S3-I27.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-driver-compile](stage3-s3-01-positionless-driver-compile.md) · S3-I27
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
