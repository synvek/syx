# ADR: Stage-3 S3-01 positionless helper 降债（vm mid + wasm-run 加速）+ KPI 闸（S3-I34）

**Status:** Accepted (S3-I34) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I33 vm lib+tail](stage3-s3-01-positionless-vm-lib.md), the locked next face is VM mid residual（~14）. This window clears it and **accelerates** densest tidy non-vm face `syx-wasm-run`（76）— still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`vm_error_at`](../../crates/syx-vm/src/vm_diag.rs) and [`wasm_run_error_at`](../../crates/syx-wasm-run/src/wasm_run_diag.rs)（lib re-exports `_at`）；**forbid** new bare `vm_error(` / `wasm_run_error(` on IN paths.
2. **IN convert（accelerate pack）：** mid 六文件 **14**（`script_hosts` / `async_iter_hosts` / `gui_invoke` / `gui_dlopen` / `synui_hosts_stub` / `jit`）+ [`lib.rs`](../../crates/syx-wasm-run/src/lib.rs) **54** + [`bridge.rs`](../../crates/syx-wasm-run/src/bridge.rs) **22** = **90** → `*_error_at(..., None, Span::DUMMY, ...)`. No user `SourceFile` — no fake span.
3. **Gate KPIs：** census `positionless_vm_mid_bare=0` · `positionless_wasm_run_bare=0`；**contract nails each `= 0`**; prior vm/driver/miette KPIs retained; **do not** change `host_vm_mid_bare` / `host_wasm_run_bare` (SyxDiagnostic axes).
4. **Ladder next：** **S3-I35** = [`syx-pkg`](../../crates/syx-pkg)（~88 `pkg_error(`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ pkg/macro/async mass this window / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | mid `vm_error(` = 0 · wasm-run `wasm_run_error(` = 0 |
| Census KPIs | two gates = 0 · global ~401 |
| Prior gates | lib/phase83/channel/tail / phase120… = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I34 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- syx-vm production `vm_error(` call sites gated to 0; wasm-run face gated; pkg → I35.
- **S3-I35 landed** as [stage3-s3-01-positionless-pkg](stage3-s3-01-positionless-pkg.md)（`positionless_pkg_bare=0`）.
- **S3-I36 landed** as [stage3-s3-01-positionless-macro](stage3-s3-01-positionless-macro.md)（`positionless_macro_bare=0`）.
- **S3-I37 landed** as [stage3-s3-01-positionless-async-llvm](stage3-s3-01-positionless-async-llvm.md)（`positionless_async_bare=0` · `positionless_llvm_bare=0`）.
- **S3-I38 landed** as [stage3-s3-01-positionless-rt-jit-debug](stage3-s3-01-positionless-rt-jit-debug.md)（rt/jit/debug/codegen positionless=0）.
- **S3-I39 landed** as [stage3-s3-01-positionless-residual](stage3-s3-01-positionless-residual.md)（十 residual KPI=0）.
- **S3-I40 landed** as [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md)（`positionless_helper_calls=0` filtered）.
- Default next after I41 = **S3-I42 S3-02 open** — not Stage-3 exit. (S3-I41 claim landed.)
- Prior: [stage3-s3-01-positionless-vm-lib](stage3-s3-01-positionless-vm-lib.md) · S3-I33.
- Census Current → **S3-I40**（I34 window closed）.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-vm-lib](stage3-s3-01-positionless-vm-lib.md) · S3-I33
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
