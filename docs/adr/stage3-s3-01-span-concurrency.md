# ADR: Stage-3 S3-01 concurrency 定点 + KPI 闸（S3-I4）

**Status:** Accepted (S3-I4) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I3 type_member/decl gate](stage3-s3-01-span-type-decl.md), the locked next knife is **concurrency** bare→`error_at` with a single-file KPI gate — still **≠** S3-01 `[x]`, still ≠ Stage-3 exit; hub residual (analyzer/type_checker/…) deferred to S3-I5.

## Decision

1. **API path:** attach spans via existing [`error_at`](../../crates/syx-semantic/src/semantic_analyze_call.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`).
2. **定点修（IN）：** zero bare `SyxDiagnostic::error|warning(` in [`semantic_concurrency.rs`](../../crates/syx-semantic/src/semantic_concurrency.rs) only.
3. **Detach anchor：** [`module_first_task_detach_span`](../../crates/syx-semantic/src/semantic_helpers.rs) for module-level C003; Call/Ident/Member use local `span`.
4. **Gate KPI：** census emits `semantic_concurrency_bare=` counting only that file; **contract nails `= 0`**; keep `semantic_analyze_main_bare=0` + `semantic_analyze_type_decl_bare=0`.
5. **Ladder next：** **S3-I5** = semantic hub 残留 9 定点（`semantic_hub_bare=0`）— still ≠ S3-01 `[x]`.
6. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ hub 9 this window / ≠ vm·host bulk / ≠ change diagnostics defaults / ≠ MIR IR / ≠ miette help GA / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Concurrency bare = 0 | `rg` on `semantic_concurrency.rs` → empty |
| Prior gates | `main_bare=0` · `type_decl_bare=0` |
| Census KPI | `scripts/s3_01_span_census.sh` → `semantic_concurrency_bare=0` |
| Contract | `s3_01_span_census_contract` (I4 assertions) |
| Plan status | Stage-3 `[~]` · S3-01 `[~]` |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I5 landed** as [stage3-s3-01-span-hub](stage3-s3-01-span-hub.md)（hub 五文件 bare→0 + `semantic_hub_bare=0`）.
- Default next construction window = **S3-I6 host 首簇** — not S3-01 claim, not Stage-3 exit.
- Prior: [stage3-s3-01-span-type-decl](stage3-s3-01-span-type-decl.md) · S3-I3.
- Census Current → **S3-I5**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-gate-semantic](stage3-s3-01-span-gate-semantic.md) · S3-I2
- [stage3-s3-01-span-type-decl](stage3-s3-01-span-type-decl.md) · S3-I3
- [stage3-s3-01-span-hub](stage3-s3-01-span-hub.md) · S3-I5
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
