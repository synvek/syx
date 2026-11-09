# ADR: Stage-3 S3-01 host vm phase89 切片 + KPI 闸（S3-I15）

**Status:** Accepted (S3-I15) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I14 vm-hosts gate](stage3-s3-01-span-host-vm-hosts.md), clear the next **syx-vm** slice ([`stdlib_phase89_hosts.rs`](../../crates/syx-vm/src/stdlib_phase89_hosts.rs), 62 bare) so `host_vm_phase89_bare=0` — still **≠** S3-01 `[x]`; wasm_stubs / other phase hosts deferred.

## Decision

1. **API path:** reuse [`vm_error` / `vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`stdlib_phase89_hosts.rs`](../../crates/syx-vm/src/stdlib_phase89_hosts.rs) only. Preserve **R003**; assign **R003** to previously uncoded sites. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_vm_phase89_bare=` counting only `stdlib_phase89_hosts.rs`; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I16** = [`stdlib_phase84_hosts.rs`](../../crates/syx-vm/src/stdlib_phase84_hosts.rs)（~33，`host_vm_phase84_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ wasm_stubs·phase84·http·os·channel bulk / ≠ rt·cranelift / ≠ renumber R/U/V081 / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| `stdlib_phase89_hosts.rs` bare = 0 | `rg` on file → empty |
| Census KPI | `host_vm_phase89_bare=0` |
| Contract | `s3_01_span_census_contract` (I15 assertions) |
| Helper | [`vm_diag.rs`](../../crates/syx-vm/src/vm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I16 landed** as [stage3-s3-01-span-host-vm-phase84](stage3-s3-01-span-host-vm-phase84.md)（`host_vm_phase84_bare=0`）.
- Default next after I16 = **S3-I17 phase120 切片** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-vm-hosts](stage3-s3-01-span-host-vm-hosts.md) · S3-I14.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-vm-hosts](stage3-s3-01-span-host-vm-hosts.md) · S3-I14
- [stage3-s3-01-span-host-vm-phase84](stage3-s3-01-span-host-vm-phase84.md) · S3-I16
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
