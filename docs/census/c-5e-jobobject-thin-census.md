# C-5e Windows Job Object thin gate census

**Date:** 2026-11-25  
**Scope:** Windows opt-in Job Object `PROCESS_MEMORY_LIMIT`（复用 `SYX_*_RSS_MB`）；Linux prlimit 不破；macOS fail-closed。**不**宣称 C-5 `[x]`；默认下一 **C-5f**。  
**Script:** [`scripts/c5e_jobobject_thin_census.sh`](../../scripts/c5e_jobobject_thin_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-5a…C-5d** | **exited** |
| **C-5e** Job Object | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5f**；macOS sandbox · 全量 filter · permission-sandbox default-on · tls.server · R-50 park |

## Root cause

`SYX_*_RSS_MB` 在非 Linux 一律 fail-closed；`windows-jobobject-v*` 仅为探针；无 `CreateJobObject*`。

## Delivery

1. `syx-macro::jobobject_windows`（CreateJobObjectW + PROCESS_MEMORY_LIMIT + Assign）  
2. `try_apply_rss_hard_wall_*` 分发 Linux prlimit / Windows Job Object  
3. embed worker + wasmtime child + macro parent assign  
4. evergreen `c5e_*` + unit fail-closed  
5. ADR / env-registry / QuickRef / UsageGuide  
6. SSOT / `02`：C-5e `[x]`；下一 **C-5f**；C-5 `[~]`  
7. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| Job Object API + hooks | **1** | **1** |
| evergreen nails | **pass** | **1** |
| `native=real` | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180/10** | **180/10** |
| simd residual | **≥159** | **159** |
| `c5e_jobobject_thin_census_ok` | **1** | **1** |

## Results (2026-11-25)

```
c5e_api=1
c5e_hooks=1
c5e_neg_pin=1
c5e_native_real_count=16
c5e_tls_server_partial=1
c5e_census_doc=1
c5e_ssot_updated=1
c5e_roadmap_ok=1
c5e_macro_unit=1
c5e_evergreen=1
c5e_c5d_rss_held=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5e_jobobject_thin_census_ok=1
current=C-5e-jobobject-thin
next=C-5f
```

## Non-claims

- ≠ C-5 `[x]` · 全量 Job Object GA · macOS sandbox  
- ≠ permission-sandbox default-on · 真堆计量 · tls.server · R-50

## Soft park

**C-5f** macOS sandbox · 全量 filter · MirOp · CST lossless · CallSuper · C-7c+ · tls.server · R-50
