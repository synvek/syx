# ADR: Stage-3 S3-01 host residual 散落批切 + KPI 闸（S3-I23）

**Status:** Accepted (S3-I23) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I22 rt/jit/debug gate](stage3-s3-01-span-host-rt-jit-debug.md), clear remaining **user-facing residual** bare（11 crates / 13 files / ~46）so `host_residual_bare=0` — still **≠** S3-01 `[x]`; miette / claim prep deferred. **OUT** [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs)（API + `cfg(test)` constructor call）.

## Decision

1. **API path:** local helpers per crate (`mir_run_diag` / `mir_diag` / `meta_diag` / `hir_diag` / `cst_diag` / `wasm_emit_diag` / `parser_diag` / `fmt_diag` / `compiler_wasm_diag` / `lint_diag` / `lexer_diag`); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in the 13 residual paths listed in census KPI. Preserve existing codes；uncoded → MIR001 / H001 / W001 / CW001；`mir_lower_ctx` passthrough.
3. **Gate KPI：** census emits `host_residual_bare=` summing the 13 paths; **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I24** = S3-01 claim prep（miette / `with_span` 质量盘点）— still ≠ automatic `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ diagnostics crate / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| 13 residual files bare = 0 | `rg` on each → empty |
| Census KPI | `host_residual_bare=0` |
| Contract | `s3_01_span_census_contract` (I23 assertions) |
| Helpers | 11 crate-local `*_diag.rs` |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- User-facing residual bare debt cleared; workspace total may still show diagnostics `cfg(test)` **1**.
- **S3-I24 landed** as [stage3-s3-01-claim-prep](stage3-s3-01-claim-prep.md)（质量 KPI 基线发布）.
- Default next after I24 = **S3-I25 miette help/label 接线** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-rt-jit-debug](stage3-s3-01-span-host-rt-jit-debug.md) · S3-I22.
- Census Current → **S3-I24**（I23 window closed）.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-rt-jit-debug](stage3-s3-01-span-host-rt-jit-debug.md) · S3-I22
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
