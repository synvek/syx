# ADR: Stage-3 S3-01 host llvm 定点 + KPI 闸（S3-I8）

**Status:** Accepted (S3-I8) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I7 driver ops gate](stage3-s3-01-span-host-driver-ops.md), clear the **syx-llvm** residual (30 bare / `lib.rs`) so `host_llvm_bare=0` — still **≠** S3-01 `[x]`; async/codegen/vm deferred.

## Decision

1. **API path:** new [`llvm_error` / `llvm_error_at`](../../crates/syx-llvm/src/llvm_diag.rs) mirroring [`driver_diag.rs`](../../crates/syx-driver/src/driver_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`crates/syx-llvm/src/lib.rs`](../../crates/syx-llvm/src/lib.rs) only (30 sites → `llvm_error(..., "L…")`). No SourceFile pipeline this window → DUMMY span primary.
3. **Gate KPI：** census emits `host_llvm_bare=` counting only `lib.rs`; **contract nails `= 0`**; keep semantic + driver KPIs = 0.
4. **Ladder next：** **S3-I9** = `syx-async` bare 定点（`host_async_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ async·codegen·cranelift·jit·vm·wasm bulk / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| `lib.rs` bare = 0 | `rg` on `crates/syx-llvm/src/lib.rs` → empty |
| Census KPI | `host_llvm_bare=0` |
| Contract | `s3_01_span_census_contract` (I8 assertions) |
| Helper | [`llvm_diag.rs`](../../crates/syx-llvm/src/llvm_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I9 landed** as [stage3-s3-01-span-host-async](stage3-s3-01-span-host-async.md)（`host_async_bare=0`）.
- Default next after I9 = **S3-I10 codegen 小簇** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-driver-ops](stage3-s3-01-span-host-driver-ops.md) · S3-I7.
- Census Current → **S3-I9** (was I8 this ADR).

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-driver-ops](stage3-s3-01-span-host-driver-ops.md) · S3-I7
- [stage3-s3-01-span-host-async](stage3-s3-01-span-host-async.md) · S3-I9
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
