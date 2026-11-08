# ADR: Stage-3 S3-01 span gate + semantic analyze 主面定点（S3-I2）

**Status:** Accepted (S3-I2) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I1 Stage-3 open + span census](stage3-open.md), the locked next knife is a **CI/`lang` gate** against new positionless user diags on the frontend semantic analyze main face, plus 定点修 of that face — still **≠** S3-01 `[x]`, still ≠ Stage-3 exit, still ≠ vm/host bulk.

## Decision

1. **API path:** attach spans via existing [`error_at`](../../crates/syx-semantic/src/semantic_analyze_call.rs) (`SyxDiagnostic` + `with_span`); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`).
2. **定点修（IN）：** zero bare `SyxDiagnostic::error|warning(` in exactly three files:
   - [`semantic_analyze_expr.rs`](../../crates/syx-semantic/src/semantic_analyze_expr.rs)
   - [`semantic_analyze_stmt.rs`](../../crates/syx-semantic/src/semantic_analyze_stmt.rs)
   - [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)
3. **Gate KPI：** census emits `semantic_analyze_main_bare=` counting only those three files; **contract nails `= 0`**.
4. **Ladder next：** **S3-I3** = `type_member` + `decl` 有 span 的 bare 定点 — still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ full semantic-crate zero / ≠ vm·host bulk / ≠ change diagnostics defaults / ≠ MIR IR span fill / ≠ miette help GA / ≠ open S3-02…07 construction / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Three-file bare = 0 | `rg 'SyxDiagnostic::(error\|warning)\('` on the three analyze files → empty |
| Census KPI | `scripts/s3_01_span_census.sh` → `semantic_analyze_main_bare=0` |
| Contract | `s3_01_span_census_contract` (S3-I2 gate assertions) |
| Plan status | Stage-3 `[~]` · S3-01 `[~]` |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I3 landed** as [stage3-s3-01-span-type-decl](stage3-s3-01-span-type-decl.md)（type_member/decl bare→`error_at` + `semantic_analyze_type_decl_bare=0`）.
- Default next construction window = **S3-I4 concurrency 定点** — not S3-01 claim, not Stage-3 exit.
- Prior: [stage3-open](stage3-open.md) · S3-I1.
- Census Current → **S3-I3**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-type-decl](stage3-s3-01-span-type-decl.md) · S3-I3
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
