# ADR: Stage-3 S3-01 host driver compile 首簇 + KPI 闸（S3-I6）

**Status:** Accepted (S3-I6) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I5 semantic hub](stage3-s3-01-span-hub.md) (semantic production bare=0), the locked next knife is the **first host cluster** — driver compile/resolve face — still **≠** S3-01 `[x]`; vm/pkg/async bulk deferred.

## Decision

1. **API path:** [`driver_error_at`](../../crates/syx-driver/src/driver_diag.rs) constructs `SyxDiagnostic` without calling `SyxDiagnostic::error(`; optional `with_span`; do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`).
2. **定点修（IN）：** zero bare in exactly five files:
   - [`compile_linked.rs`](../../crates/syx-driver/src/compile_linked.rs)
   - [`build_syx.rs`](../../crates/syx-driver/src/build_syx.rs)
   - [`emit.rs`](../../crates/syx-driver/src/emit.rs)
   - [`check_project.rs`](../../crates/syx-driver/src/check_project.rs)
   - [`module_ref.rs`](../../crates/syx-driver/src/module_ref.rs)
3. **Gate KPI：** census emits `host_driver_compile_bare=` counting only those five files; **contract nails `= 0`**; keep all semantic face KPIs = 0.
4. **Ladder next：** **S3-I7** = driver ops residual（`format_lint`/`test_runner`/`run_exec`/…）定点 KPI — still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ vm·pkg·macro·async·wasm bulk / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Five-file bare = 0 | `rg` on the five files → empty |
| Helper | `driver_diag.rs` · `driver_error_at` |
| Census KPI | `host_driver_compile_bare=0` |
| Contract | `s3_01_span_census_contract` (I6 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I7 landed** as [stage3-s3-01-span-host-driver-ops](stage3-s3-01-span-host-driver-ops.md)（`host_driver_ops_bare=0`；driver src bare=0）.
- Default next = **S3-I8 llvm 定点** — not S3-01 claim.
- Prior: [stage3-s3-01-span-hub](stage3-s3-01-span-hub.md) · S3-I5.
- Census Current → **S3-I7**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-hub](stage3-s3-01-span-hub.md) · S3-I5
- [stage3-s3-01-span-host-driver-ops](stage3-s3-01-span-host-driver-ops.md) · S3-I7
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
