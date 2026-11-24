# C-4e path content hash census

**Date:** 2026-11-24  
**Scope:** path 依赖 directory content hash（`PackageSource::Path.checksum`）；Locked 缺/错 → **P014**。清单收齐 → **C-4 `[x]`**；默认下一 **C-5**。  
**Script:** [`scripts/c4e_path_content_hash_census.sh`](../../scripts/c4e_path_content_hash_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2** | **exited** |
| **C-3** | **`[~]`**（C-3a…C-3g `[x]`） |
| **C-4a…C-4d** | **exited** |
| **C-4e** path content hash | **exited** |
| **C-4** | **`[x]`** |
| Open | 默认下一 **C-5**；ModuleId · tls.server · macOS · R-50 park |

## Root cause

Path 依赖 lock 仅记 `version`+`path`；Locked install 活解析、不钉内容——与 registry checksum / git rev 不对称，目录可变而 lock 无感。

## Delivery

1. `PackageSource::Path.checksum`（`serde(default)`；`sha256:…`）  
2. `path_tree_checksum`：字典序文件 walk；排除 `.git` / `target` / `.DS_Store`  
3. Unlocked 写 hash；Locked 缺/错/漂移 → **P014**（C-4e）  
4. `c4e_*` 三钉；SSOT / `02`：C-4e `[x]`；**C-4 `[x]`**；下一 **C-5**  
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| path checksum API | **1** | **1** (`c4e_api`) |
| c4e integration (3) | **pass** | **1** (`c4e_integ_cargo` · 3 passed) |
| path evergreen smoke | **pass** | **1** (`c4e_evergreen_smoke` · 3 passed) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c4e_path_content_hash_census_ok` | **1** | **1** |
| `c4_exited` | **1** | **1** |

## Results (2026-11-24)

```
c4e_api=1
c4e_tests_present=1
c4e_native_real_count=16
c4e_tls_server_partial=1
c4e_census_doc=1
c4e_ssot_updated=1
c4e_roadmap_ok=1
c4_exited=1
c4e_integ_cargo=1
c4e_evergreen_smoke=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c4e_path_content_hash_census_ok=1
current=C-4e-path-content-hash
next=C-5
c4_exited=1
```

## Non-claims

- ≠ manifest↔lock 全量对账 · 删 `SyxLockfile` infra 字段 · 削弱 `enforce_lock_*`
- ≠ 复杂 ignore / `.syxignore` · 顶层 `path_sha256` map · lock schema major
- ≠ tls.server raise · macOS · R-50 · JIT · 宣称 C-3 `[x]` · 开 C-5 实现

## Soft park

C-5 macro/embed · ModuleId · full MirOp · exception · tls.server · macOS · R-50 · soft→hard · JIT expand
