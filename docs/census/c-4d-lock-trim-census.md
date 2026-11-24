# C-4d lock v16 infra trim census

**Date:** 2026-11-24  
**Scope:** `graph_to_lockfile` 默认停写 CDN/farm/billing/marketplace env 快照与硬编码 CDN regions；**保留** schema + `enforce_lock_*`。**不**宣称 C-4 `[x]`；默认下一 **C-4e**。  
**Script:** [`scripts/c4d_lock_trim_census.sh`](../../scripts/c4d_lock_trim_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2** | **exited** |
| **C-3** | **`[~]`**（C-3a…C-3g `[x]`） |
| **C-4a…C-4c** | **exited** |
| **C-4d** lock trim | **exited** |
| **C-4** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-4e**；path content hash · ModuleId · tls.server · macOS · R-50 park |

## Root cause

默认 `install`/`update` 把进程 env 的 farm/CDN/billing 及硬编码 phase8 CDN regions 烙进 `syx.lock`，膨胀且把 infra 快照误当依赖可复现契约。

## Delivery

1. `graph_to_lockfile`：仅写依赖核心字段；infra → `Default` 空  
2. `registry_cdn_hard`：改钉 env CDN 行为，不要求 lock 含 `artifact_cdn_base`  
3. `c4d_graph_to_lockfile_omits_infra_env`  
4. SSOT / `02`：C-4d `[x]`；C-4 `[~]`；默认下一 **C-4e**  
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| graph_to_lockfile trim API | **1** | **1** (`c4d_api`) |
| c4d + CDN behavior tests | **pass** | **1** (`c4d_integ_cargo` · 1 passed; `c4d_evergreen_smoke` · 3 passed) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c4d_lock_trim_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c4d_api=1
c4d_tests_present=1
c4d_native_real_count=16
c4d_tls_server_partial=1
c4d_census_doc=1
c4d_ssot_updated=1
c4d_roadmap_ok=1
c4d_integ_cargo=1
c4d_evergreen_smoke=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c4d_lock_trim_census_ok=1
current=C-4d-lock-trim
next=C-4e
```

## Non-claims

- ≠ C-4 `[x]` · path 依赖 content hash（C-4e）
- ≠ 删除 `SyxLockfile` 字段 · 削弱 `enforce_lock_v12…v16`
- ≠ tls.server raise · macOS · R-50 · JIT · 改硬 KPI · 宣称 C-3 `[x]`

## Soft park

C-4e path content hash · ModuleId · full MirOp · exception · tls.server · macOS · R-50 · soft→hard · JIT expand · C-5
