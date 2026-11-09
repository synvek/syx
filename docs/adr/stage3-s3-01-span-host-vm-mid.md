# ADR: Stage-3 S3-01 host vm mid 批切 + KPI 闸（S3-I20）

**Status:** Accepted (S3-I20) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I19 phase-tail gate](stage3-s3-01-span-host-vm-phase-tail.md), accelerate by clearing **all remaining non-`wasm_stubs` production files** in one window（10 files / ~93 bare）so `host_vm_mid_bare=0` — supersedes the prior single-file lib.rs next; still **≠** S3-01 `[x]`; wasm_stubs / rt deferred.

## Decision

1. **API path:** reuse [`vm_error` / `vm_error_at`](../../crates/syx-vm/src/vm_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in: `lib.rs` · `http.rs` · `os_hosts.rs` · `channel_hosts.rs` · `script_hosts.rs` · `async_iter_hosts.rs` · `gui_invoke.rs` · `gui_dlopen.rs` · `synui_hosts_stub.rs` · `jit.rs`. Preserve existing **R/U/C/A/V** codes; assign **R003** to previously uncoded sites. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_vm_mid_bare=` summing the ten paths; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I21** = `crates/syx-vm/src/wasm_stubs/**`（~56，`host_vm_wasm_stubs_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ wasm_stubs / ≠ rt·cranelift / ≠ renumber existing codes / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| ten mid files bare = 0 | `rg` on each → empty |
| Census KPI | `host_vm_mid_bare=0` |
| Contract | `s3_01_span_census_contract` (I20 assertions) |
| Helper | [`vm_diag.rs`](../../crates/syx-vm/src/vm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I21 landed** as [stage3-s3-01-span-host-vm-wasm-stubs](stage3-s3-01-span-host-vm-wasm-stubs.md)（`host_vm_wasm_stubs_bare=0`）.
- Default next after I21 = **S3-I22 rt/cranelift/debug 批切** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-vm-phase-tail](stage3-s3-01-span-host-vm-phase-tail.md) · S3-I19.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-vm-phase-tail](stage3-s3-01-span-host-vm-phase-tail.md) · S3-I19
- [stage3-s3-01-span-host-vm-wasm-stubs](stage3-s3-01-span-host-vm-wasm-stubs.md) · S3-I21
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
