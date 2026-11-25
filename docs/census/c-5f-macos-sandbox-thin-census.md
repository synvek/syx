# C-5f macOS Seatbelt net-deny thin census

**Date:** 2026-11-25  
**Scope:** macOS opt-in Seatbelt net-deny（`sandbox_init` / `sandbox-exec -n no-network`）；in-process embed → null；非 macOS fail-closed。**不**宣称 C-5 `[x]`；默认下一 **C-5g**。  
**Script:** [`scripts/c5f_macos_sandbox_thin_census.sh`](../../scripts/c5f_macos_sandbox_thin_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-5a…C-5e** | **exited** |
| **C-5f** Seatbelt | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5g**；全量 filter · permission-sandbox default-on · tls.server · R-50 park |

## Root cause

`macos-sandbox-*-degraded` 仅为探针；无 `sandbox_init`；RSS/SECCOMP 在 macOS fail-closed。

## Delivery

1. `syx-macro::macos_sandbox`（`sandbox_init(SANDBOX_NAMED, no-network)`）  
2. macro child install；非 macOS → M040  
3. embed in-process → null；wasmtime → `sandbox-exec`  
4. evergreen `c5f_*` + unit  
5. ADR / env-registry / QuickRef / UsageGuide  
6. SSOT / `02`：C-5f `[x]`；下一 **C-5g**；C-5 `[~]`  
7. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| Seatbelt API + hooks | **1** | **1** |
| evergreen nails | **pass** | **1** |
| `native=real` | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180/10** | **180/10** |
| simd residual | **≥159** | **159** |
| `c5f_macos_sandbox_thin_census_ok` | **1** | **1** |

## Results (2026-11-25)

```
c5f_api=1
c5f_hooks=1
c5f_neg_pin=1
c5f_native_real_count=16
c5f_tls_server_partial=1
c5f_census_doc=1
c5f_ssot_updated=1
c5f_roadmap_ok=1
c5f_macro_unit=1
c5f_evergreen=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5f_macos_sandbox_thin_census_ok=1
current=C-5f-macos-sandbox-thin
next=C-5g
```

## Non-claims

- ≠ C-5 `[x]` · App Sandbox GA · 全量 filter · macOS RSS  
- ≠ permission-sandbox default-on · 真堆计量 · tls.server · R-50

## Soft park

**C-5g** 全量 filter · permission-sandbox default-on · MirOp · CST lossless · CallSuper · C-7c+ · tls.server · R-50
