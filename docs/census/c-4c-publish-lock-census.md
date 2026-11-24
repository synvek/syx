# C-4c publish requires usable lock census

**Date:** 2026-11-24  
**Scope:** `syx publish` 强制可用 `syx.lock`（缺 → **P014**；有 → `enforce_lock_v16`）；CLI `--dry-run` 同闸。**不**宣称 C-4 `[x]`；默认下一 **C-4d**。  
**Script:** [`scripts/c4c_publish_lock_census.sh`](../../scripts/c4c_publish_lock_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2** | **exited** |
| **C-3** | **`[~]`**（C-3a…C-3g `[x]`） |
| **C-4a / C-4b** | **exited** |
| **C-4c** publish lock | **exited** |
| **C-4** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-4d**；lock v16 裁剪 · path content hash · ModuleId · tls.server · macOS · R-50 park |

## Root cause

`publish_package` 不读 `syx.lock`，可在未 install 情况下发布，破坏「先 install 再 publish」可复现顺序。

## Delivery

1. `require_publish_lock`：缺 lock → **P014**（C-4c）；有 → `enforce_lock_v16`  
2. `publish_package` 入口调用；CLI `syx publish --dry-run` 同闸  
3. integration `c4c_*`；evergreen/registry 夹具补最小 lock  
4. SSOT / `02`：C-4c `[x]`；C-4 `[~]`；默认下一 **C-4d**  
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| require_publish_lock API | **1** | **1** (`c4c_api`) |
| c4c integration | **pass** | **1** (`c4c_integ_cargo` · 4 passed) |
| evergreen registry publish smoke | **pass** | **1** (`c4c_evergreen_smoke` · 3 passed) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c4c_publish_lock_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c4c_api=1
c4c_tests_present=1
c4c_native_real_count=16
c4c_tls_server_partial=1
c4c_census_doc=1
c4c_ssot_updated=1
c4c_roadmap_ok=1
c4c_integ_cargo=1
c4c_evergreen_smoke=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c4c_publish_lock_census_ok=1
current=C-4c-publish-lock
next=C-4d
```

## Non-claims

- ≠ C-4 `[x]` · lock v16 CDN/farm 字段裁剪（C-4d）
- ≠ path 依赖 content hash · manifest↔lock 全量对账
- ≠ tls.server raise · macOS · R-50 · JIT · 改硬 KPI · 宣称 C-3 `[x]`

## Soft park

C-4d lock v16 trim · path content hash · ModuleId · full MirOp · exception · tls.server · macOS · R-50 · soft→hard · JIT expand
