# ADR: Stage-3 S3-01 host driver ops 收口 + KPI 闸（S3-I7）

**Status:** Accepted (S3-I7) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I6 driver compile gate](stage3-s3-01-span-host-driver-compile.md), clear the remaining **driver ops** residual (40 bare / 9 files) so `syx-driver/src` production bare = 0 — still **≠** S3-01 `[x]`; llvm/async/vm deferred.

## Decision

1. **API path:** reuse [`driver_error_at`](../../crates/syx-driver/src/driver_diag.rs) / `driver_error`; do **not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults (`primary_span: None` on `error`/`warning`).
2. **定点修（IN）：** zero bare in exactly nine files:
   - [`format_lint.rs`](../../crates/syx-driver/src/format_lint.rs)
   - [`test_runner.rs`](../../crates/syx-driver/src/test_runner.rs)
   - [`native_build.rs`](../../crates/syx-driver/src/native_build.rs)
   - [`run_exec.rs`](../../crates/syx-driver/src/run_exec.rs)
   - [`debug_repl.rs`](../../crates/syx-driver/src/debug_repl.rs)
   - [`doc_project.rs`](../../crates/syx-driver/src/doc_project.rs)
   - [`driver_options.rs`](../../crates/syx-driver/src/driver_options.rs)
   - [`run_wasm.rs`](../../crates/syx-driver/src/run_wasm.rs)
   - [`cranelift_stubs.rs`](../../crates/syx-driver/src/cranelift_stubs.rs)
3. **Gate KPI：** census emits `host_driver_ops_bare=` counting only those nine files; **contract nails `= 0`**; keep `host_driver_compile_bare=0` + semantic face KPIs = 0. After this window, **`syx-driver/src` production bare = 0**.
4. **Ladder next：** **S3-I8** = `syx-llvm` bare 定点（`host_llvm_bare=0`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ vm·pkg·macro·async·wasm bulk / ≠ miette help / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Nine-file bare = 0 | `rg` on the nine files → empty |
| Driver src bare = 0 | `rg` on `crates/syx-driver/src` excl. tests → empty |
| Census KPI | `host_driver_ops_bare=0` |
| Contract | `s3_01_span_census_contract` (I7 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I8 landed** as [stage3-s3-01-span-host-llvm](stage3-s3-01-span-host-llvm.md)（`host_llvm_bare=0`）.
- Default next after I8 = **S3-I9 async 定点** — not S3-01 claim.
- Prior: [stage3-s3-01-span-host-driver-compile](stage3-s3-01-span-host-driver-compile.md) · S3-I6.
- Census Current → **S3-I8** (was I7 this ADR).

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-driver-compile](stage3-s3-01-span-host-driver-compile.md) · S3-I6
- [stage3-s3-01-span-host-llvm](stage3-s3-01-span-host-llvm.md) · S3-I8
- [stage3-diagnostic-span-census](../census/stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
