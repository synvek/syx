# ADR: Stage-3 S3-01 type_member/decl 定点 + KPI 闸（S3-I3）

**Status:** Accepted (S3-I3) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I2 main-face gate](stage3-s3-01-span-gate-semantic.md), the locked next knife is **type_member + decl** bare→`error_at` with a two-file KPI gate — still **≠** S3-01 `[x]`, still ≠ Stage-3 exit, still ≠ vm/host / concurrency bulk.

## Decision

1. **API path:** attach spans via existing [`error_at`](../../crates/syx-semantic/src/semantic_analyze_call.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`).
2. **定点修（IN）：** zero bare `SyxDiagnostic::error|warning(` in exactly two files:
   - [`semantic_analyze_type_member.rs`](../../crates/syx-semantic/src/semantic_analyze_type_member.rs)
   - [`semantic_analyze_decl.rs`](../../crates/syx-semantic/src/semantic_analyze_decl.rs)
3. **Cross-file（allowed）：** `expr`/`call`/`stmt` may only pass `span` into helpers (`validate_type_expr`, `validate_*_hashable`, `validate_generic_instantiation*`, `check_actor_field_access`, `check_equatable_binary`); **must not** add new bare constructors (`semantic_analyze_main_bare` stays **0**).
4. **Gate KPI：** census emits `semantic_analyze_type_decl_bare=` counting only those two files; **contract nails `= 0`**; keep `semantic_analyze_main_bare=0`.
5. **Ladder next：** **S3-I4** = concurrency（+ optional analyzer/type_checker residual）定点 — still ≠ S3-01 `[x]`.
6. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ full semantic-crate zero / ≠ concurrency+vm·host bulk / ≠ change diagnostics defaults / ≠ MIR IR span fill / ≠ miette help GA / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Two-file bare = 0 | `rg 'SyxDiagnostic::(error\|warning)\('` on type_member + decl → empty |
| Main face still 0 | `semantic_analyze_main_bare=0` |
| Census KPI | `scripts/s3_01_span_census.sh` → `semantic_analyze_type_decl_bare=0` |
| Contract | `s3_01_span_census_contract` (I3 assertions) |
| Plan status | Stage-3 `[~]` · S3-01 `[~]` |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I4 landed** as [stage3-s3-01-span-concurrency](stage3-s3-01-span-concurrency.md)（concurrency bare→`error_at` + `semantic_concurrency_bare=0`）.
- Default next construction window = **S3-I5 semantic hub 定点** — not S3-01 claim, not Stage-3 exit.
- Prior: [stage3-s3-01-span-gate-semantic](stage3-s3-01-span-gate-semantic.md) · S3-I2.
- Census Current → **S3-I4**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-gate-semantic](stage3-s3-01-span-gate-semantic.md) · S3-I2
- [stage3-s3-01-span-concurrency](stage3-s3-01-span-concurrency.md) · S3-I4
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
