# ADR: Stage-3 S3-01 miette help/label 接线 + KPI 闸（S3-I25）

**Status:** Accepted (S3-I25) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I24 claim-prep](stage3-s3-01-claim-prep.md), quality KPIs showed `miette_help_impl=0` / `with_span_label_wired=0`. This window wires miette **label + help** on [`SyxDiagnostic`](../../crates/syx-diagnostics/src/lib.rs) — still **≠** S3-01 `[x]`.

## Decision

1. Add `label: Option<String>` and `help: Option<String>` on `SyxDiagnostic`; `error`/`warning` init both to `None`; **do not** change `primary_span: None` defaults.
2. `with_span(..., label)` stores non-empty label; empty → `None`. `labels()` passes `self.label` into `LabeledSpan`.
3. Implement `Diagnostic::help`; add `with_help(self, help)` builder.
4. **Gate KPI：** census `miette_help_impl=1` and `with_span_label_wired=1`; **contract nails `= 1`**; prior bare KPIs remain 0.
5. **Ladder next：** **S3-I26** = positionless helper 首批降债（`positionless_helper_calls`↓；优先有 SourceFile 的 parser/driver 面）— still ≠ S3-01 `[x]`.
6. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ mass `*_error`→`*_error_at` / ≠ change `error`/`warning` `primary_span` default / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| label wired | `with_span` + `labels()` tests |
| help wired | `fn help` + `with_help` test |
| Census KPI | `miette_help_impl=1` · `with_span_label_wired=1` |
| Contract | `s3_01_span_census_contract` (I25 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Claim criterion ③ (miette label/help API) becomes satisfiable; positionless debt unchanged this window (**952**).
- **S3-I26 landed** as [stage3-s3-01-positionless-parser-driver](stage3-s3-01-positionless-parser-driver.md)（`positionless_parser_driver_bare=0`）.
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
- Prior: [stage3-s3-01-claim-prep](stage3-s3-01-claim-prep.md) · S3-I24.
- Census Current → **S3-I40**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-claim-prep](stage3-s3-01-claim-prep.md) · S3-I24
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
