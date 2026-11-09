# ADR: Stage-3 S3-01 host vm phase-tail 批切 + KPI 闸（S3-I19）

**Status:** Accepted (S3-I19) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I18 phase83 gate](stage3-s3-01-span-host-vm-phase83.md), accelerate by clearing **all remaining non-zero** `stdlib_phase*_hosts` in one window（6 files / 35 bare）so `host_vm_phase_tail_bare=0` — supersedes the prior single-file phase90 next; still **≠** S3-01 `[x]`; lib / http·os deferred.

## Decision

1. **API path:** reuse [`vm_error` / `vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in these six files only: `stdlib_phase90_hosts.rs` · `phase91` · `phase92` · `phase95` · `phase86` · `phase87`. Preserve **V081** / **R010**; assign **R003** to previously uncoded sites. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_vm_phase_tail_bare=` summing the six paths; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I20** = [`lib.rs`](../../crates/syx-vm/src/lib.rs)（~28，`host_vm_lib_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ lib·http·os·channel·wasm_stubs bulk / ≠ rt·cranelift / ≠ renumber V081/R010 / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| six phase-tail files bare = 0 | `rg` on each → empty |
| Census KPI | `host_vm_phase_tail_bare=0` |
| Contract | `s3_01_span_census_contract` (I19 assertions) |
| Helper | [`vm_diag.rs`](../../crates/syx-vm/src/vm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I20 landed** as [stage3-s3-01-span-host-vm-mid](stage3-s3-01-span-host-vm-mid.md)（`host_vm_mid_bare=0`；supersedes single-file lib）.
- Default next after I20 = **S3-I21 wasm_stubs 批切** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-vm-phase83](stage3-s3-01-span-host-vm-phase83.md) · S3-I18.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-vm-phase83](stage3-s3-01-span-host-vm-phase83.md) · S3-I18
- [stage3-s3-01-span-host-vm-mid](stage3-s3-01-span-host-vm-mid.md) · S3-I20
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
