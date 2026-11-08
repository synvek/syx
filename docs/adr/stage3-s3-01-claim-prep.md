# ADR: Stage-3 S3-01 claim-prep 盘点 + 质量 KPI 闸（S3-I24）

**Status:** Accepted (S3-I24) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S3-I23 residual gate](stage3-s3-01-span-host-residual.md), user-facing bare `SyxDiagnostic::error|warning(` debt is cleared. Remaining claim debt is **Span quality** + **miette label/help**. This window **publishes claim criteria draft + quality KPI baselines** — still **≠** S3-01 `[x]`.

## Claim criteria draft（本窗只盘点，不宣称满足）

1. 用户面 bare `SyxDiagnostic::error|warning(` = 0（excl. diagnostics `cfg(test)`）— **已满足**（I23）。
2. 有 `SourceFile` 的用户路径不得长期走 DUMMY / 便利 `*_error(`（后续窗闸 `positionless_helper_calls`）。
3. miette：`labels()` 使用真实 label 文本；`Diagnostic::help()` 可返回。
4. 独立 **S3-01 `[x]` claim** 窗（≠ I24 / ≠ I25 alone）。

## Decision

1. **Publish** claim-prep quality KPIs via census script（基线记录，**不**钉 =0）：`positionless_helper_calls`、`miette_help_impl`、`with_span_label_wired`；保留 `with_span_calls` / `primary_span_none_literal`。
2. **Do not** change [`syx-diagnostics`](../../crates/syx-diagnostics/src/lib.rs) defaults（`primary_span: None` on `error`/`warning`）；**do not** implement `help()` or wire `_label` this window.
3. **Gate：** contract nails KPI **names published** + Current=I24；prior bare KPIs remain 0.
4. **Ladder next：** **S3-I25** = wire `with_span` label + implement `Diagnostic::help`（`miette_help_impl=1` / `with_span_label_wired=1`）— still ≠ S3-01 `[x]`.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ mass `*_error`→`*_error_at` / ≠ change diagnostics defaults / ≠ MIR IR / ≠ open S3-02…07 / ≠ HostFn+1 / ≠ F-08.

## Baseline（I23 后 / I24 发布）

| KPI | Baseline | Notes |
|-----|----------:|-------|
| user bare (`host_*` + residual) | **0** | gated |
| `with_span_calls` | ~51 | mostly helper defs |
| `primary_span_none_literal` | ~48 | helper ctor literals |
| `positionless_helper_calls` | **952** | production `*_error(` excl `*_diag.rs` |
| `miette_help_impl` | **0** | no `fn help` in diagnostics |
| `with_span_label_wired` | **0** | `with_span(..., _label)` discarded |

## Evidence

| Check | Evidence |
|-------|----------|
| Claim criteria draft | this ADR |
| Census / script | `positionless_helper_calls` / `miette_help_impl` / `with_span_label_wired` echoed |
| Contract | `s3_01_span_census_contract` (I24 assertions) |

## Consequences

- S3-01 stays `[~]`; Stage-3 stays `[~]`.
- Claim-readiness becomes measurable; debt volume unchanged this window.
- **S3-I25 landed** as [stage3-s3-01-miette-help-label](stage3-s3-01-miette-help-label.md)（`miette_help_impl=1` / `with_span_label_wired=1`）.
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
- **S3-I40 landed** as [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md)（filtered `positionless_helper_calls=0`；claim ② 对齐）.
- **S3-I41 landed** as [stage3-s3-01-claim](stage3-s3-01-claim.md)（S3-01 `[x]` scoped claim）.
- Default next after I41 = **S3-I42 S3-02 open** — not Stage-3 exit.
- Prior: [stage3-s3-01-span-host-residual](stage3-s3-01-span-host-residual.md) · S3-I23.
- Census Current → **S3-I41**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-01-span-host-residual](stage3-s3-01-span-host-residual.md) · S3-I23
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-01
