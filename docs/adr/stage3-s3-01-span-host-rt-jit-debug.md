# ADR: Stage-3 S3-01 host rt/jit/debug 批切 + KPI 闸（S3-I22）

**Status:** Accepted (S3-I22) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I21 wasm_stubs gate](stage3-s3-01-span-host-vm-wasm-stubs.md), clear **syx-rt + syx-cranelift-jit + syx-debug**（4 files / ~63 bare）so `host_rt_jit_debug_bare=0` — still **≠** S3-01 `[x]`; residual scatter deferred.

## Decision

1. **API path:** local helpers [`rt_diag`](../../crates/syx-rt/src/rt_diag.rs) / [`jit_diag`](../../crates/syx-cranelift-jit/src/jit_diag.rs) / [`debug_diag`](../../crates/syx-debug/src/debug_diag.rs) (`*_error` / `*_error_at`); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`bridge.rs`](../../crates/syx-rt/src/bridge.rs) + [`abi.rs`](../../crates/syx-rt/src/abi.rs), [`syx-cranelift-jit/src/lib.rs`](../../crates/syx-cranelift-jit/src/lib.rs), [`syx-debug/src/lib.rs`](../../crates/syx-debug/src/lib.rs). Preserve **C004 / C006 / C009**; uncoded rt → **R010**; jit → **J001**; debug → **D001**. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_rt_jit_debug_bare=` summing the four paths; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I23** = residual scatter batch（mir-run/mir/meta/hir/… ≈47，`host_residual_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ residual crates / ≠ renumber C00x / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| four target files bare = 0 | `rg` on each → empty |
| Census KPI | `host_rt_jit_debug_bare=0` |
| Contract | `s3_01_span_census_contract` (I22 assertions) |
| Helpers | `rt_diag` / `jit_diag` / `debug_diag` |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I23 landed** as [stage3-s3-01-span-host-residual](stage3-s3-01-span-host-residual.md)（`host_residual_bare=0`）.
- Default next after I23 = **S3-I24 claim prep** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-vm-wasm-stubs](stage3-s3-01-span-host-vm-wasm-stubs.md) · S3-I21.
- Census Current → **S3-I23**（I22 window closed）.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-vm-wasm-stubs](stage3-s3-01-span-host-vm-wasm-stubs.md) · S3-I21
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
