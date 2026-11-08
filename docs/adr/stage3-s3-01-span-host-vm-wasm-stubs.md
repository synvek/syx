# ADR: Stage-3 S3-01 host vm wasm_stubs 批切 + KPI 闸（S3-I21）

**Status:** Accepted (S3-I21) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I20 mid gate](stage3-s3-01-span-host-vm-mid.md), clear remaining **syx-vm wasm_stubs**（2 files / 56 bare, all **V081**）so `host_vm_wasm_stubs_bare=0` — still **≠** S3-01 `[x]`; rt / cranelift / debug deferred.

## Decision

1. **API path:** reuse [`vm_error` / `vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`wasm_stubs/stdlib_phase89_hosts.rs`](../../crates/syx-vm/src/wasm_stubs/stdlib_phase89_hosts.rs) and [`wasm_stubs/http.rs`](../../crates/syx-vm/src/wasm_stubs/http.rs). Preserve **V081**. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_vm_wasm_stubs_bare=` summing the two paths; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I22** = `syx-rt` + `syx-cranelift-jit` + `syx-debug` 批（~63，`host_rt_jit_debug_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ rt·cranelift·debug / ≠ renumber V081 / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| two wasm_stubs files bare = 0 | `rg` on each → empty |
| Census KPI | `host_vm_wasm_stubs_bare=0` |
| Contract | `s3_01_span_census_contract` (I21 assertions) |
| Helper | [`vm_diag.rs`](../../crates/syx-vm/src/vm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **syx-vm tree bare → 0** after this window.
- **S3-I22 landed** as [stage3-s3-01-span-host-rt-jit-debug](stage3-s3-01-span-host-rt-jit-debug.md)（`host_rt_jit_debug_bare=0`）.
- Default next after I22 = **S3-I23 residual 散落批切** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-vm-mid](stage3-s3-01-span-host-vm-mid.md) · S3-I20.
- Census Current → **S3-I22**（I21 window closed）.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-vm-mid](stage3-s3-01-span-host-vm-mid.md) · S3-I20
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
