# ADR: Stage-3 S3-01 host vm phase120 切片 + KPI 闸（S3-I17）

**Status:** Accepted (S3-I17) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I16 phase84 gate](stage3-s3-01-span-host-vm-phase84.md), clear the next **syx-vm** slice ([`stdlib_phase120_hosts.rs`](../../crates/syx-vm/src/stdlib_phase120_hosts.rs), 30 bare) so `host_vm_phase120_bare=0` — still **≠** S3-01 `[x]`; lib / http·os / phase83 deferred.

## Decision

1. **API path:** reuse [`vm_error` / `vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`stdlib_phase120_hosts.rs`](../../crates/syx-vm/src/stdlib_phase120_hosts.rs) only. Assign **R003** to all previously uncoded sites. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_vm_phase120_bare=` counting only `stdlib_phase120_hosts.rs`; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I18** = [`stdlib_phase83_hosts.rs`](../../crates/syx-vm/src/stdlib_phase83_hosts.rs)（~15，`host_vm_phase83_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ lib·http·os·channel·wasm_stubs·phase83 bulk / ≠ rt·cranelift / ≠ new V codes / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| `stdlib_phase120_hosts.rs` bare = 0 | `rg` on file → empty |
| Census KPI | `host_vm_phase120_bare=0` |
| Contract | `s3_01_span_census_contract` (I17 assertions) |
| Helper | [`vm_diag.rs`](../../crates/syx-vm/src/vm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I18 landed** as [stage3-s3-01-span-host-vm-phase83](stage3-s3-01-span-host-vm-phase83.md)（`host_vm_phase83_bare=0`）.
- Default next after I18 = **S3-I19 phase90 切片** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-vm-phase84](stage3-s3-01-span-host-vm-phase84.md) · S3-I16.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-vm-phase84](stage3-s3-01-span-host-vm-phase84.md) · S3-I16
- [stage3-s3-01-span-host-vm-phase83](stage3-s3-01-span-host-vm-phase83.md) · S3-I18
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
