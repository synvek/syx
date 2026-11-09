# ADR: Stage-3 S3-01 host macro 定点 + KPI 闸（S3-I13）

**Status:** Accepted (S3-I13) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I12 pkg gate](stage3-s3-01-span-host-pkg.md), clear the **syx-macro** residual (79 bare / 12 `.rs` + orphan dump text) so `host_macro_bare=0` — still **≠** S3-01 `[x]`; vm deferred.

## Decision

1. **API path:** new [`macro_error` / `macro_error_at`](../../crates/syx-macro/src/macro_diag.rs) mirroring [`pkg_diag.rs`](../../crates/syx-pkg/src/pkg_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in twelve production `.rs` files under [`crates/syx-macro/src`](../../crates/syx-macro/src). Preserve **E001–E016 / W001**; assign **M020+** to previously uncoded sites. Delete unused `builtin_body.txt` / `user.rs.raw`.
3. **Gate KPI：** census emits `host_macro_bare=` counting `crates/syx-macro/src --glob '*.rs'`; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I14** = `syx-vm` 首切片 `hosts.rs`（~70，`host_vm_hosts_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ vm·rt·cranelift bulk / ≠ renumber E/W / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| macro `.rs` bare = 0 | `rg` on `crates/syx-macro/src --glob '*.rs'` → empty |
| Census KPI | `host_macro_bare=0` |
| Contract | `s3_01_span_census_contract` (I13 assertions) |
| Helper | [`macro_diag.rs`](../../crates/syx-macro/src/macro_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I14 landed** as [stage3-s3-01-span-host-vm-hosts](stage3-s3-01-span-host-vm-hosts.md)（`host_vm_hosts_bare=0`）.
- Default next after I14 = **S3-I15 phase89 切片** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-pkg](stage3-s3-01-span-host-pkg.md) · S3-I12.
- Census Current → **S3-I14** (was I13 this ADR).

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-pkg](stage3-s3-01-span-host-pkg.md) · S3-I12
- [stage3-s3-01-span-host-vm-hosts](stage3-s3-01-span-host-vm-hosts.md) · S3-I14
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
