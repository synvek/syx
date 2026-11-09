# ADR: Stage-3 S3-01 host pkg 定点 + KPI 闸（S3-I12）

**Status:** Accepted (S3-I12) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I11 wasm-run gate](stage3-s3-01-span-host-wasm-run.md), clear the **syx-pkg** residual (87 bare / 11 files) so `host_pkg_bare=0` — still **≠** S3-01 `[x]`; macro/vm deferred.

## Decision

1. **API path:** new [`pkg_error` / `pkg_error_at`](../../crates/syx-pkg/src/pkg_diag.rs) mirroring [`wasm_run_diag.rs`](../../crates/syx-wasm-run/src/wasm_run_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare across eleven production files under [`crates/syx-pkg/src`](../../crates/syx-pkg/src). Preserve existing **P001–P014 / R001 / S412–S423**; assign **P020+** to previously uncoded sites. Do **not** renumber codes or split S422. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_pkg_bare=` counting `crates/syx-pkg/src` (excl. tests); **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I13** = `syx-macro` 定点（~98，`host_macro_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ macro·vm bulk / ≠ renumber P/R/S / ≠ TOML span / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| pkg src bare = 0 | `rg` on `crates/syx-pkg/src` excl. tests → empty |
| Census KPI | `host_pkg_bare=0` |
| Contract | `s3_01_span_census_contract` (I12 assertions) |
| Helper | [`pkg_diag.rs`](../../crates/syx-pkg/src/pkg_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I13 landed** as [stage3-s3-01-span-host-macro](stage3-s3-01-span-host-macro.md)（`host_macro_bare=0`）.
- Default next after I13 = **S3-I14 vm hosts 切片** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-wasm-run](stage3-s3-01-span-host-wasm-run.md) · S3-I11.
- Census Current → **S3-I13** (was I12 this ADR).

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-wasm-run](stage3-s3-01-span-host-wasm-run.md) · S3-I11
- [stage3-s3-01-span-host-macro](stage3-s3-01-span-host-macro.md) · S3-I13
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
