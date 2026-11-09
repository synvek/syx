# ADR: Stage-3 S3-01 host wasm-run 定点 + KPI 闸（S3-I11）

**Status:** Accepted (S3-I11) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I10 codegen gate](stage3-s3-01-span-host-codegen.md), clear the **syx-wasm-run** residual (76 bare / 2 files) so `host_wasm_run_bare=0` — still **≠** S3-01 `[x]`; vm/pkg deferred.

## Decision

1. **API path:** new [`wasm_run_error` / `wasm_run_error_at`](../../crates/syx-wasm-run/src/wasm_run_diag.rs) mirroring [`codegen_diag.rs`](../../crates/syx-codegen/src/codegen_diag.rs); do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`). **禁止**裸 `SyxDiagnostic::error(`.
2. **定点修（IN）：** zero bare in [`lib.rs`](../../crates/syx-wasm-run/src/lib.rs) + [`bridge.rs`](../../crates/syx-wasm-run/src/bridge.rs). bridge codes align async **C…**; lib uses **W500+**. Do **not** change `host_fail` VM-diag passthrough or VM host registration. No SourceFile → DUMMY.
3. **Gate KPI：** census emits `host_wasm_run_bare=` counting `crates/syx-wasm-run/src` (excl. tests); **contract nails `= 0`**; keep prior KPIs = 0.
4. **Ladder next：** **S3-I12** = `syx-pkg` 定点（~87，`host_pkg_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ vm·pkg·macro bulk / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ SourceFile 全链 / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| wasm-run src bare = 0 | `rg` on `crates/syx-wasm-run/src` excl. tests → empty |
| Census KPI | `host_wasm_run_bare=0` |
| Contract | `s3_01_span_census_contract` (I11 assertions) |
| Helper | [`wasm_run_diag.rs`](../../crates/syx-wasm-run/src/wasm_run_diag.rs) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I12 landed** as [stage3-s3-01-span-host-pkg](stage3-s3-01-span-host-pkg.md)（`host_pkg_bare=0`）.
- Default next after I12 = **S3-I13 macro 定点** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-codegen](stage3-s3-01-span-host-codegen.md) · S3-I10.
- Census Current → **S3-I12** (was I11 this ADR).

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-codegen](stage3-s3-01-span-host-codegen.md) · S3-I10
- [stage3-s3-01-span-host-pkg](stage3-s3-01-span-host-pkg.md) · S3-I12
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
