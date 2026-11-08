# ADR: Stage-3 S3-01 host vm hosts 切片 + KPI 闸（S3-I14）

**Status:** Accepted (S3-I14) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I13 macro gate](stage3-s3-01-span-host-macro.md), clear the first **syx-vm** slice ([`hosts.rs`](../../crates/syx-vm/src/hosts.rs), 70 bare) so `host_vm_hosts_bare=0` — still **≠** S3-01 `[x]`; phase hosts deferred.

## Decision

1. **API path:** new [`vm_error` / `vm_error_at`](../../crates/syx-vm/src/vm_diag.rs) mirroring [`macro_diag.rs`](../../crates/syx-macro/src/macro_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`hosts.rs`](../../crates/syx-vm/src/hosts.rs) only. Preserve **R001–R012 / U001 / V081**; assign **V082+** to previously uncoded sites. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_vm_hosts_bare=` counting only `hosts.rs`; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I15** = [`stdlib_phase89_hosts.rs`](../../crates/syx-vm/src/stdlib_phase89_hosts.rs)（~62，`host_vm_phase89_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ phase·http·os·channel·wasm_stubs bulk / ≠ rt·cranelift / ≠ renumber R/U/V081 / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| `hosts.rs` bare = 0 | `rg` on `crates/syx-vm/src/hosts.rs` → empty |
| Census KPI | `host_vm_hosts_bare=0` |
| Contract | `s3_01_span_census_contract` (I14 assertions) |
| Helper | [`vm_diag.rs`](../../crates/syx-vm/src/vm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I15 landed** as [stage3-s3-01-span-host-vm-phase89](stage3-s3-01-span-host-vm-phase89.md)（`host_vm_phase89_bare=0`）.
- Default next after I15 = **S3-I16 phase84 切片** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-macro](stage3-s3-01-span-host-macro.md) · S3-I13.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-macro](stage3-s3-01-span-host-macro.md) · S3-I13
- [stage3-s3-01-span-host-vm-phase89](stage3-s3-01-span-host-vm-phase89.md) · S3-I15
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
