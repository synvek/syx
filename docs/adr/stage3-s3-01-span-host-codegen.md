# ADR: Stage-3 S3-01 host codegen 定点 + KPI 闸（S3-I10）

**Status:** Accepted (S3-I10) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I9 async gate](stage3-s3-01-span-host-async.md), clear the **syx-codegen** residual (9 bare / `lib.rs`) so `host_codegen_bare=0` — still **≠** S3-01 `[x]`; wasm/vm deferred.

## Decision

1. **API path:** new [`codegen_error` / `codegen_error_at`](../../crates/syx-codegen/src/codegen_diag.rs) mirroring [`async_diag.rs`](../../crates/syx-async/src/async_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`crates/syx-codegen/src/lib.rs`](../../crates/syx-codegen/src/lib.rs) only (9 sites → `codegen_error(..., code)`). Preserve existing **C020**; assign **G001–G004** to previously uncoded sites. No SourceFile pipeline → DUMMY span primary.
3. **Gate KPI：** census emits `host_codegen_bare=` counting only `lib.rs`; **contract nails `= 0`**; keep semantic + driver + llvm + async KPIs = 0.
4. **Ladder next：** **S3-I11** = `syx-wasm-run` 定点开窗（~76，`host_wasm_run_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ wasm·vm·pkg·macro bulk / ≠ renumber C020 / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ SourceFile into CodeGenerator / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| `lib.rs` bare = 0 | `rg` on `crates/syx-codegen/src/lib.rs` → empty |
| Census KPI | `host_codegen_bare=0` |
| Contract | `s3_01_span_census_contract` (I10 assertions) |
| Helper | [`codegen_diag.rs`](../../crates/syx-codegen/src/codegen_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I11 landed** as [stage3-s3-01-span-host-wasm-run](stage3-s3-01-span-host-wasm-run.md)（`host_wasm_run_bare=0`）.
- Default next after I11 = **S3-I12 pkg 定点** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-async](stage3-s3-01-span-host-async.md) · S3-I9.
- Census Current → **S3-I11** (was I10 this ADR).

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-async](stage3-s3-01-span-host-async.md) · S3-I9
- [stage3-s3-01-span-host-wasm-run](stage3-s3-01-span-host-wasm-run.md) · S3-I11
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
