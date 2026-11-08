# ADR: Stage-3 open — DX & performance (S3-I1)

**Status:** Accepted (S3-I1) · Stage-3 `[~]` · S3-01 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I266 Stage-2 exit](stage2-exit.md), default credibility work shifts to [plan/02 Stage-3](../../plan/02-improvement-roadmap.md): developer experience and performance. The locked first axis is **S3-01** (diagnostics must carry Span). This window **opens** Stage-3 with a diagnostic **Span census baseline** — it does **not** claim S3-01 `[x]`, fix positionless diagnostics, or exit Stage-3.

## Gap: diagnostics vs plan S3-01

| Claim | Today (honest) | plan/02 S3-01 target |
|-------|----------------|----------------------|
| User diags always have Span | ~1010 bare `SyxDiagnostic::error\|warning`; ~9 `.with_span` | Positionless = CI defect |
| miette label/help | labels only if `primary_span` set; no `help()` | label + help |
| IR `span: None` | ~435 in `syx-mir` | **Out of S3-01 scope** (not user diags) |

Baseline: [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md).

## Decision

1. **Open Stage-3** as an active credibility workstream; flip plan Stage-3 `[ ]` → `[~]`.
2. **Open S3-01** as the default construction axis; flip S3-01 `[ ]` → `[~]`.
3. **S3-I1 first slice** — publish diagnostic Span census (+ script); add a thin contract under `syx-driver/tests/` (no diagnostic-source edit this window).
4. **Ladder:** **S3-I2** = CI/`lang` gate against new positionless **user** diagnostics + fix highest-priority bare/`primary_span: None` clusters → residual volume / miette help → dedicated **S3-01 `[x]` claim**.
5. **Non-claims (hard):** ≠ S3-01 `[x]` / ≠ Stage-3 exit / ≠ batch-fill MIR IR spans / ≠ open S3-02…07 construction this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ change language semantics.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Span census | [`docs/stage3-diagnostic-span-census.md`](../stage3-diagnostic-span-census.md) · `scripts/s3_01_span_census.sh` |
| Contract nail | `s3_01_span_census_contract` |
| Plan status | Stage-3 `[~]` · S3-01 `[~]` under this ADR |

## Consequences

- Stage-3 and S3-01 move `[ ]` → `[~]`.
- **S3-I2 landed** as [stage3-s3-01-span-gate-semantic](stage3-s3-01-span-gate-semantic.md)（三主面 bare→`error_at` + `semantic_analyze_main_bare=0` 闸）.
- **S3-I3 landed** as [stage3-s3-01-span-type-decl](stage3-s3-01-span-type-decl.md)（type_member/decl bare→`error_at` + `semantic_analyze_type_decl_bare=0`）.
- **S3-I4 landed** as [stage3-s3-01-span-concurrency](stage3-s3-01-span-concurrency.md)（concurrency bare→`error_at` + `semantic_concurrency_bare=0`）.
- **S3-I5 landed** as [stage3-s3-01-span-hub](stage3-s3-01-span-hub.md)（hub 五文件 + `semantic_hub_bare=0`；semantic 生产源 bare=0）.
- **S3-I6 landed** as [stage3-s3-01-span-host-driver-compile](stage3-s3-01-span-host-driver-compile.md)（`host_driver_compile_bare=0` + `driver_error_at`）.
- **S3-I7 landed** as [stage3-s3-01-span-host-driver-ops](stage3-s3-01-span-host-driver-ops.md)（`host_driver_ops_bare=0`；driver src bare=0）.
- **S3-I8 landed** as [stage3-s3-01-span-host-llvm](stage3-s3-01-span-host-llvm.md)（`host_llvm_bare=0` + `llvm_error`）.
- **S3-I9 landed** as [stage3-s3-01-span-host-async](stage3-s3-01-span-host-async.md)（`host_async_bare=0` + `async_error`）.
- **S3-I10 landed** as [stage3-s3-01-span-host-codegen](stage3-s3-01-span-host-codegen.md)（`host_codegen_bare=0` + `codegen_error`）.
- **S3-I11 landed** as [stage3-s3-01-span-host-wasm-run](stage3-s3-01-span-host-wasm-run.md)（`host_wasm_run_bare=0` + `wasm_run_error`）.
- **S3-I12 landed** as [stage3-s3-01-span-host-pkg](stage3-s3-01-span-host-pkg.md)（`host_pkg_bare=0` + `pkg_error`）.
- **S3-I13 landed** as [stage3-s3-01-span-host-macro](stage3-s3-01-span-host-macro.md)（`host_macro_bare=0` + `macro_error`）.
- **S3-I14 landed** as [stage3-s3-01-span-host-vm-hosts](stage3-s3-01-span-host-vm-hosts.md)（`host_vm_hosts_bare=0` + `vm_error`）.
- **S3-I15 landed** as [stage3-s3-01-span-host-vm-phase89](stage3-s3-01-span-host-vm-phase89.md)（`host_vm_phase89_bare=0` + `vm_error`）.
- **S3-I16 landed** as [stage3-s3-01-span-host-vm-phase84](stage3-s3-01-span-host-vm-phase84.md)（`host_vm_phase84_bare=0` + `vm_error`）.
- **S3-I17 landed** as [stage3-s3-01-span-host-vm-phase120](stage3-s3-01-span-host-vm-phase120.md)（`host_vm_phase120_bare=0` + `vm_error`）.
- **S3-I18 landed** as [stage3-s3-01-span-host-vm-phase83](stage3-s3-01-span-host-vm-phase83.md)（`host_vm_phase83_bare=0` + `vm_error`）.
- **S3-I19 landed** as [stage3-s3-01-span-host-vm-phase-tail](stage3-s3-01-span-host-vm-phase-tail.md)（`host_vm_phase_tail_bare=0` + `vm_error`；6 phase files）.
- **S3-I20 landed** as [stage3-s3-01-span-host-vm-mid](stage3-s3-01-span-host-vm-mid.md)（`host_vm_mid_bare=0` + `vm_error`；10 mid files）.
- **S3-I21 landed** as [stage3-s3-01-span-host-vm-wasm-stubs](stage3-s3-01-span-host-vm-wasm-stubs.md)（`host_vm_wasm_stubs_bare=0` + `vm_error`）.
- **S3-I22 landed** as [stage3-s3-01-span-host-rt-jit-debug](stage3-s3-01-span-host-rt-jit-debug.md)（`host_rt_jit_debug_bare=0` + `rt_error`/`jit_error`/`debug_error`）.
- **S3-I23 landed** as [stage3-s3-01-span-host-residual](stage3-s3-01-span-host-residual.md)（`host_residual_bare=0` + 11 local helpers）.
- **S3-I24 landed** as [stage3-s3-01-claim-prep](stage3-s3-01-claim-prep.md)（claim 判据草案 + 质量 KPI 基线）.
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
- **S3-I40 landed** as [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md)（`positionless_helper_calls=0` filtered）.
- **S3-I41 landed** as [stage3-s3-01-claim](stage3-s3-01-claim.md)（S3-01 `[x]` scoped claim；Stage-3 `[~]`）.
- **S3-I42 landed** as [stage3-s3-02-open](stage3-s3-02-open.md)（S3-02 `[~]` open + mono-display census）.
- **S3-I43 landed** as [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md)（semantic demangle hinge）.
- **S3-I44 landed** as [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md)（debug stack user-facing names）.
- **S3-I45 landed** as [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md)（runtime `__type` user-facing）.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- Default next construction window = **S3-03 open** — not Stage-3 exit.
- Prior: [stage2-exit](stage2-exit.md) · S2-I266.

## References

- [stage2-exit](stage2-exit.md) · S2-I266
- [stage3-diagnostic-span-census](../stage3-diagnostic-span-census.md)
- [stage3-s3-01-span-gate-semantic](stage3-s3-01-span-gate-semantic.md) · S3-I2
- [stage3-s3-01-span-type-decl](stage3-s3-01-span-type-decl.md) · S3-I3
- [stage3-s3-01-span-concurrency](stage3-s3-01-span-concurrency.md) · S3-I4
- [stage3-s3-01-span-hub](stage3-s3-01-span-hub.md) · S3-I5
- [stage3-s3-01-span-host-driver-compile](stage3-s3-01-span-host-driver-compile.md) · S3-I6
- [stage3-s3-01-span-host-driver-ops](stage3-s3-01-span-host-driver-ops.md) · S3-I7
- [stage3-s3-01-span-host-llvm](stage3-s3-01-span-host-llvm.md) · S3-I8
- [stage3-s3-01-span-host-async](stage3-s3-01-span-host-async.md) · S3-I9
- [stage3-s3-01-span-host-codegen](stage3-s3-01-span-host-codegen.md) · S3-I10
- [stage3-s3-01-span-host-wasm-run](stage3-s3-01-span-host-wasm-run.md) · S3-I11
- [stage3-s3-01-span-host-pkg](stage3-s3-01-span-host-pkg.md) · S3-I12
- [stage3-s3-01-span-host-macro](stage3-s3-01-span-host-macro.md) · S3-I13
- [stage3-s3-01-span-host-vm-hosts](stage3-s3-01-span-host-vm-hosts.md) · S3-I14
- [stage3-s3-01-span-host-vm-phase89](stage3-s3-01-span-host-vm-phase89.md) · S3-I15
- [stage3-s3-01-span-host-vm-phase84](stage3-s3-01-span-host-vm-phase84.md) · S3-I16
- [stage3-s3-01-span-host-vm-phase120](stage3-s3-01-span-host-vm-phase120.md) · S3-I17
- [stage3-s3-01-span-host-vm-phase83](stage3-s3-01-span-host-vm-phase83.md) · S3-I18
- [stage3-s3-01-span-host-vm-phase-tail](stage3-s3-01-span-host-vm-phase-tail.md) · S3-I19
- [stage3-s3-01-span-host-vm-mid](stage3-s3-01-span-host-vm-mid.md) · S3-I20
- [stage3-s3-01-span-host-vm-wasm-stubs](stage3-s3-01-span-host-vm-wasm-stubs.md) · S3-I21
- [stage3-s3-01-span-host-rt-jit-debug](stage3-s3-01-span-host-rt-jit-debug.md) · S3-I22
- [stage3-s3-01-span-host-residual](stage3-s3-01-span-host-residual.md) · S3-I23
- [stage3-s3-01-claim-prep](stage3-s3-01-claim-prep.md) · S3-I24
- [stage3-s3-01-miette-help-label](stage3-s3-01-miette-help-label.md) · S3-I25
- [stage3-s3-01-positionless-parser-driver](stage3-s3-01-positionless-parser-driver.md) · S3-I26
- [stage3-s3-01-positionless-driver-compile](stage3-s3-01-positionless-driver-compile.md) · S3-I27
- [stage3-s3-01-positionless-driver-emit-test-native](stage3-s3-01-positionless-driver-emit-test-native.md) · S3-I28
- [stage3-s3-01-positionless-vm-phase84](stage3-s3-01-positionless-vm-phase84.md) · S3-I29
- [stage3-s3-01-positionless-vm-hosts](stage3-s3-01-positionless-vm-hosts.md) · S3-I30
- [stage3-s3-01-positionless-vm-phase89](stage3-s3-01-positionless-vm-phase89.md) · S3-I31
- [stage3-s3-01-positionless-vm-phase120](stage3-s3-01-positionless-vm-phase120.md) · S3-I32
- [stage3-s3-01-positionless-vm-lib](stage3-s3-01-positionless-vm-lib.md) · S3-I33
- [stage3-s3-01-positionless-vm-mid](stage3-s3-01-positionless-vm-mid.md) · S3-I34
- [stage3-s3-01-positionless-pkg](stage3-s3-01-positionless-pkg.md) · S3-I35
- [stage3-s3-01-positionless-macro](stage3-s3-01-positionless-macro.md) · S3-I36
- [stage3-s3-01-positionless-async-llvm](stage3-s3-01-positionless-async-llvm.md) · S3-I37
- [stage3-s3-01-positionless-rt-jit-debug](stage3-s3-01-positionless-rt-jit-debug.md) · S3-I38
- [stage3-s3-01-positionless-residual](stage3-s3-01-positionless-residual.md) · S3-I39
- [stage3-s3-01-noise-claim-prep](stage3-s3-01-noise-claim-prep.md) · S3-I40
- [plan/02](../../plan/02-improvement-roadmap.md) Stage-3 / S3-01
