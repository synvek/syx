# ADR: Stage-3 S3-01 positionless helper 降债（driver compile）+ KPI 闸（S3-I27）

**Status:** Accepted (S3-I27) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I26 parser/driver first batch](stage3-s3-01-positionless-parser-driver.md), residual positionless on driver compile faces remained. This window converts **`build_syx` + `compile_linked`** — still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`driver_error_at`](../../crates/syx-driver/src/driver_diag.rs); **forbid** new bare `driver_error(` on IN paths.
2. **IN convert：** [`build_syx.rs`](../../crates/syx-driver/src/build_syx.rs) (17) + [`compile_linked.rs`](../../crates/syx-driver/src/compile_linked.rs) (11). With SF/Decl span → `Some` + real span（`diagnose_build_syx` S401；`resolve_module_with_span` M001/M003/M010）；IO/路径 before SF → `None` + `Span::DUMMY`.
3. **Gate KPI：** census `positionless_driver_compile_bare=0`；**contract nails `= 0`**; prior `positionless_parser_driver_bare=0` + bare/miette KPIs retained.
4. **Ladder next：** **S3-I28** = driver `emit` + `test_runner` + `native_build` positionless 残面 — still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ emit·test_runner·native_build this window / ≠ VM/pkg / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `driver_error(` = 0 on IN paths |
| Census KPI | `positionless_driver_compile_bare=0` · global `positionless_helper_calls` ~896 |
| Prior gates | `positionless_parser_driver_bare=0` · miette KPIs = 1 |
| Contract | `s3_01_span_census_contract` (I27 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Claim criterion ② drops another ~28 sites; emit/test/native → I28.
- **S3-I28 landed** as [stage3-s3-01-positionless-driver-emit-test-native](stage3-s3-01-positionless-driver-emit-test-native.md)（`positionless_driver_emit_test_native_bare=0`）.
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

- **S3-I41 landed** as [stage3-s3-01-claim](stage3-s3-01-claim.md)（S3-01 `[x]` scoped claim）.

- **S3-I42 landed** as [stage3-s3-02-open](stage3-s3-02-open.md)（S3-02 `[~]` open）.

- **S3-I43 landed** as [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md).
- **S3-I44 landed** as [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md)（debug stack display）.
- **S3-I45 landed** as [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md)（runtime `__type` 用户面）.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- Default next after I47 = **S3-03 open** — not Stage-3 exit.
- Prior: [stage3-s3-01-positionless-parser-driver](stage3-s3-01-positionless-parser-driver.md) · S3-I26.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-parser-driver](stage3-s3-01-positionless-parser-driver.md) · S3-I26
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
