# ADR: Stage-3 S3-01 host async 定点 + KPI 闸（S3-I9）

**Status:** Accepted (S3-I9) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I8 llvm gate](stage3-s3-01-span-host-llvm.md), clear the **syx-async** residual (64 bare / 5 files) so `host_async_bare=0` — still **≠** S3-01 `[x]`; wasm/vm/pkg deferred.

## Decision

1. **API path:** new [`async_error` / `async_error_at`](../../crates/syx-async/src/async_diag.rs) mirroring [`llvm_diag.rs`](../../crates/syx-llvm/src/llvm_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in five production files:
   - [`lib.rs`](../../crates/syx-async/src/lib.rs)
   - [`lower.rs`](../../crates/syx-async/src/lower.rs)
   - [`task.rs`](../../crates/syx-async/src/task.rs)
   - [`runtime.rs`](../../crates/syx-async/src/runtime.rs)
   - [`hosts.rs`](../../crates/syx-async/src/hosts.rs)
   No SourceFile pipeline this window → DUMMY span primary. Do **not** edit `dist_transport.rs`.
3. **Gate KPI：** census emits `host_async_bare=` counting `crates/syx-async/src` (excl. tests); **contract nails `= 0`**; keep semantic + driver + llvm KPIs = 0.
4. **Ladder next：** **S3-I10** = `syx-codegen` 小簇定点（~9，`host_codegen_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ wasm·vm·pkg·macro bulk / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ SourceFile 全链路 / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| async src bare = 0 | `rg` on `crates/syx-async/src` excl. tests → empty |
| Census KPI | `host_async_bare=0` |
| Contract | `s3_01_span_census_contract` (I9 assertions) |
| Helper | [`async_diag.rs`](../../crates/syx-async/src/async_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I10 landed** as [stage3-s3-01-span-host-codegen](stage3-s3-01-span-host-codegen.md)（`host_codegen_bare=0`）.
- Default next after I10 = **S3-I11 wasm-run 开窗** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-llvm](stage3-s3-01-span-host-llvm.md) · S3-I8.
- Census Current → **S3-I10** (was I9 this ADR).

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-llvm](stage3-s3-01-span-host-llvm.md) · S3-I8
- [stage3-s3-01-span-host-codegen](stage3-s3-01-span-host-codegen.md) · S3-I10
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
