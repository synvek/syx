# ADR: Stage-3 S3-01 positionless helper 降债（vm phase89）+ KPI 闸（S3-I31）

**Status:** Accepted (S3-I31) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I30 vm hosts](stage3-s3-01-positionless-vm-hosts.md), the next densest VM residual is phase89（native + wasm twin）. This window clears both — still **≠** S3-01 `[x]`.

## Decision

1. Reuse [`vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); **forbid** new bare `vm_error(` on IN paths.
2. **IN convert：** [`stdlib_phase89_hosts.rs`](../../crates/syx-vm/src/stdlib_phase89_hosts.rs) **62** + [`wasm_stubs/stdlib_phase89_hosts.rs`](../../crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs) **34** = **96** → `vm_error_at(..., None, Span::DUMMY, ...)`. No user `SourceFile` — no fake span.
3. **Gate KPI：** census `positionless_vm_phase89_bare=0`（两文件合计）；**contract nails `= 0`**; prior vm/driver/miette KPIs retained; **do not** change `host_vm_phase89_bare` (I15 axis).
4. **Ladder next：** **S3-I32** = [`stdlib_phase120_hosts.rs`](../../crates/syx-vm/src/stdlib_phase120_hosts.rs)（~30）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ phase120/lib/http mass this window / ≠ pkg/macro / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `vm_error(` = 0 on phase89 native + wasm stub |
| Census KPI | `positionless_vm_phase89_bare=0` · global ~672 |
| Prior gates | hosts / phase84 / driver = 0 · miette = 1 |
| Contract | `s3_01_span_census_contract` (I31 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Phase89 face gated (native + stub); phase120 → I32.
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
- Prior: [stage3-s3-01-positionless-vm-hosts](stage3-s3-01-positionless-vm-hosts.md) · S3-I30.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-positionless-vm-hosts](stage3-s3-01-positionless-vm-hosts.md) · S3-I30
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
