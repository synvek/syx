# ADR: Stage-3 S3-01 host vm phase83 切片 + KPI 闸（S3-I18）

**Status:** Accepted (S3-I18) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I17 phase120 gate](stage3-s3-01-span-host-vm-phase120.md), clear the next **syx-vm** slice ([`stdlib_phase83_hosts.rs`](../../crates/syx-vm/src/stdlib_phase83_hosts.rs), 15 bare) so `host_vm_phase83_bare=0` — still **≠** S3-01 `[x]`; lib / http·os / phase90 deferred.

## Decision

1. **API path:** reuse [`vm_error` / `vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`stdlib_phase83_hosts.rs`](../../crates/syx-vm/src/stdlib_phase83_hosts.rs) only. Assign **R003** to all previously uncoded sites. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_vm_phase83_bare=` counting only `stdlib_phase83_hosts.rs`; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I19** = [`stdlib_phase90_hosts.rs`](../../crates/syx-vm/src/stdlib_phase90_hosts.rs)（~11，`host_vm_phase90_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ lib·http·os·channel·wasm_stubs·phase90 bulk / ≠ rt·cranelift / ≠ new V codes / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| `stdlib_phase83_hosts.rs` bare = 0 | `rg` on file → empty |
| Census KPI | `host_vm_phase83_bare=0` |
| Contract | `s3_01_span_census_contract` (I18 assertions) |
| Helper | [`vm_diag.rs`](../../crates/syx-vm/src/vm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I19 landed** as [stage3-s3-01-span-host-vm-phase-tail](stage3-s3-01-span-host-vm-phase-tail.md)（`host_vm_phase_tail_bare=0`；supersedes single-file phase90）.
- Default next after I19 = **S3-I20 lib.rs 切片** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-vm-phase120](stage3-s3-01-span-host-vm-phase120.md) · S3-I17.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-vm-phase120](stage3-s3-01-span-host-vm-phase120.md) · S3-I17
- [stage3-s3-01-span-host-vm-phase-tail](stage3-s3-01-span-host-vm-phase-tail.md) · S3-I19
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
