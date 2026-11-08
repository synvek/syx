# ADR: Stage-3 S3-01 semantic hub 定点 + KPI 闸（S3-I5）

**Status:** Accepted (S3-I5) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I4 concurrency gate](stage3-s3-01-span-concurrency.md), the locked next knife clears the remaining **semantic hub** residual (analyzer / type_checker / types / infer_host / import_resolver) — still **≠** S3-01 `[x]`, still ≠ Stage-3 exit; vm/host deferred to S3-I6.

## Decision

1. **API path:** attach spans via existing [`error_at`](../../crates/syx-semantic/src/semantic_analyze_call.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`).
2. **定点修（IN）：** zero bare `SyxDiagnostic::error|warning(` in exactly five files:
   - [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs)
   - [`type_checker.rs`](../../crates/syx-semantic/src/type_checker.rs)
   - [`semantic_types.rs`](../../crates/syx-semantic/src/semantic_types.rs)
   - [`semantic_infer_host.rs`](../../crates/syx-semantic/src/semantic_infer_host.rs)
   - [`import_resolver.rs`](../../crates/syx-semantic/src/import_resolver.rs)
3. **Structured errors：** `Scope::define` → `ScopeError::Duplicate`；`AssignError` / `ImportAccessError` mapped at call face with `error_at`; S012 uses `type_decl_spans` from collect.
4. **Gate KPI：** census emits `semantic_hub_bare=` counting only those five files; **contract nails `= 0`**; keep prior face KPIs = 0. After this window, **syx-semantic production src bare = 0**.
5. **Ladder next：** **S3-I6** = first vm/driver host bare cluster census + 定点 KPI — still ≠ S3-01 `[x]`.
6. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ vm·host bulk / ≠ miette help GA / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Hub bare = 0 | `rg` on five hub files → empty |
| Semantic crate bare = 0 | `rg` on `crates/syx-semantic/src` excl. tests → empty |
| Prior gates | main / type_decl / concurrency still 0 |
| Census KPI | `semantic_hub_bare=0` |
| Contract | `s3_01_span_census_contract` (I5 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I6 landed** as [stage3-s3-01-span-host-driver-compile](stage3-s3-01-span-host-driver-compile.md)（`host_driver_compile_bare=0`）.
- Default next construction window = **S3-I7 driver ops 定点** — not S3-01 claim, not Stage-3 exit.
- Prior: [stage3-s3-01-span-concurrency](stage3-s3-01-span-concurrency.md) · S3-I4.
- Census Current → **S3-I6**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-concurrency](stage3-s3-01-span-concurrency.md) · S3-I4
- [stage3-s3-01-span-host-driver-compile](stage3-s3-01-span-host-driver-compile.md) · S3-I6
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
