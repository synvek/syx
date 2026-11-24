# C-4b locked-by-default install census

**Date:** 2026-11-24  
**Scope:** 有 `syx.lock` 时 `install` **默认锁定**；`InstallMode::{Auto,Locked,Unlocked}`；CLI `--locked`；`update`/`add` → Unlocked。**不**宣称 C-4 `[x]`；默认下一 **C-4c**。  
**Script:** [`scripts/c4b_locked_default_census.sh`](../../scripts/c4b_locked_default_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2** | **exited** |
| **C-3** | **`[~]`**（C-3a…C-3g `[x]`） |
| **C-4a** | **exited** |
| **C-4b** locked default | **exited** |
| **C-4** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-4c**；publish 强锁 · lock v16 裁剪 · path content hash · ModuleId · tls.server · macOS · R-50 park |

## Root cause

有 `syx.lock` 时默认 `install` 仍重解析并 `save_lockfile`；锁定仅靠 `SYX_INSTALL_LOCKED` env，CLI 无 `--locked`，`update` ≡ `install`。

## Delivery

1. `InstallMode::{Auto,Locked,Unlocked}` + thread-local 注入 `install_locked()`  
2. Auto：有 lock → 锁定不重写；无 lock → 写 lock；env `LOCKED`/`UNLOCKED` 兼容  
3. `update_dependencies` / `add_dependency` → Unlocked；CLI `syx install --locked`  
4. integration：`c4b_*` + c04/c4a 回归  
5. SSOT / `02`：C-4b `[x]`；C-4 `[~]`；默认下一 **C-4c**  
6. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| InstallMode + default-lock API | **1** | **1** (`c4b_api`) |
| c4b + c04 + c4a integration | **pass** | **1** (`c4b_integ_cargo` · 13 passed) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c4b_locked_default_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c4b_api=1
c4b_tests_present=1
c4b_native_real_count=16
c4b_tls_server_partial=1
c4b_census_doc=1
c4b_ssot_updated=1
c4b_roadmap_ok=1
c4b_integ_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c4b_locked_default_census_ok=1
current=C-4b-locked-default
next=C-4c
```

## Non-claims

- ≠ C-4 `[x]` · `syx publish` 强制 lock（C-4c）
- ≠ lock v16 CDN/farm 字段裁剪 · path content hash
- ≠ 删除 `SYX_INSTALL_LOCKED` 兼容
- ≠ tls.server raise · macOS · R-50 · JIT · 改硬 KPI · 宣称 C-3 `[x]`

## Soft park

C-4c publish-requires-lock · lock v16 trim · path content hash · ModuleId · full MirOp · exception · tls.server · macOS · R-50 · soft→hard · JIT expand
