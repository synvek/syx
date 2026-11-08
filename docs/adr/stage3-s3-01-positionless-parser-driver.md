# ADR: Stage-3 S3-01 positionless helper 首批降债（parser/driver）+ KPI 闸（S3-I26）

**Status:** Accepted (S3-I26) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I25 miette help/label](stage3-s3-01-miette-help-label.md), claim criterion ③ is wired but positionless helper volume remained ~952. This window converts the first **parser + driver faces that already have `SourceFile` (or clear IO-before-SF sites via `*_error_at(None, DUMMY)`)** — still **≠** S3-01 `[x]`.

## Decision

1. Reuse existing [`parser_error_at`](../../crates/syx-parser/src/parser_diag.rs) / [`driver_error_at`](../../crates/syx-driver/src/driver_diag.rs); **forbid** new bare `parser_error(` / `driver_error(` on IN paths.
2. **IN convert：** [`parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs) (4) + driver `format_lint` / `check_project` / `run_exec` / `debug_repl` / `module_ref` / `driver_options` / `doc_project`. With SF → `Some` + real or whole-file `Span::new(0, len)`; IO/path before `SourceFile::new` → `None` + `Span::DUMMY` (no fake span).
3. **Gate KPI：** census `positionless_parser_driver_bare=0`（上列路径上 `parser_error(` / `driver_error(` 合计；**不含** `_error_at`）；**contract nails `= 0`**; prior bare KPIs remain 0; miette KPIs remain 1.
4. **Ladder next：** **S3-I27** = driver `build_syx` + `compile_linked` 可 span 残面批切 — still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ VM/pkg/wasm-run/macro/async / ≠ mass remaining positionless / ≠ change diagnostics `primary_span` defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| IN faces bare | `parser_error(` / `driver_error(` = 0 on IN paths |
| Census KPI | `positionless_parser_driver_bare=0` · global `positionless_helper_calls` ~924 |
| Miette retained | `miette_help_impl=1` · `with_span_label_wired=1` |
| Contract | `s3_01_span_census_contract` (I26 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Claim criterion ② volume drops on parser/driver first batch; residual compile faces → I27.
- **S3-I27 landed** as [stage3-s3-01-positionless-driver-compile](stage3-s3-01-positionless-driver-compile.md)（`positionless_driver_compile_bare=0`）.
- **S3-I28 landed** as [stage3-s3-01-positionless-driver-emit-test-native](stage3-s3-01-positionless-driver-emit-test-native.md)（`positionless_driver_emit_test_native_bare=0`）.
- **S3-I29 landed** as [stage3-s3-01-positionless-vm-phase84](stage3-s3-01-positionless-vm-phase84.md)（`positionless_vm_phase84_bare=0`）.
- **S3-I30 landed** as [stage3-s3-01-positionless-vm-hosts](stage3-s3-01-positionless-vm-hosts.md)（`positionless_vm_hosts_bare=0`）.
- **S3-I31 landed** as [stage3-s3-01-positionless-vm-phase89](stage3-s3-01-positionless-vm-phase89.md)（`positionless_vm_phase89_bare=0`）.
- **S3-I32 landed** as [stage3-s3-01-positionless-vm-phase120](stage3-s3-01-positionless-vm-phase120.md)（phase120/http/os positionless=0）.
- **S3-I33 landed** as [stage3-s3-01-positionless-vm-lib](stage3-s3-01-positionless-vm-lib.md)（lib/phase83/channel/tail positionless=0）.
- **S3-I34 landed** as [stage3-s3-01-positionless-vm-mid](stage3-s3-01-positionless-vm-mid.md)（mid + wasm-run positionless=0）.
- **S3-I35 landed** as [stage3-s3-01-positionless-pkg](stage3-s3-01-positionless-pkg.md)（`positionless_pkg_bare=0`）.
- **S3-I36 landed** as [stage3-s3-01-positionless-macro](stage3-s3-01-positionless-macro.md)（`positionless_macro_bare=0`）.
- **S3-I37 landed** as [stage3-s3-01-positionless-async-llvm](stage3-s3-01-positionless-async-llvm.md)（`positionless_async_bare=0` · `positionless_llvm_bare=0`）.
- **S3-I38 landed** as [stage3-s3-01-positionless-rt-jit-debug](stage3-s3-01-positionless-rt-jit-debug.md)（rt/jit/debug/codegen positionless=0）.
- **S3-I39 landed** as [stage3-s3-01-positionless-residual](stage3-s3-01-positionless-residual.md)（十 residual KPI=0）.
- **S3-I40 landed** as [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md)（`positionless_helper_calls=0` filtered）.
- Default next after I41 = **S3-I42 S3-02 open** — not Stage-3 exit. (S3-I41 claim landed.)
- Prior: [stage3-s3-01-miette-help-label](stage3-s3-01-miette-help-label.md) · S3-I25.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-miette-help-label](stage3-s3-01-miette-help-label.md) · S3-I25
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
