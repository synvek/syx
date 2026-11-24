# C-4a Git rev pin + locked git install census

**Date:** 2026-11-24  
**Scope:** Manifest git **强制 `rev`**（缺 → **P043**）；`SYX_INSTALL_LOCKED` 下 Git 从 `syx.lock` 钉死（缺/错 pin → **P014**；不重写 lock）。**不**宣称 C-4 `[x]`；默认下一 **C-4b**。  
**Script:** [`scripts/c4a_git_rev_pin_census.sh`](../../scripts/c4a_git_rev_pin_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2** | **exited** |
| **C-3** | **`[~]`**（C-3a…C-3g `[x]`） |
| **C-4a** git rev + locked git | **exited** |
| **C-4** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-4b**；publish 强锁 · lock v16 裁剪 · path content hash · tls.server · macOS · R-50 park |

## Root cause

`git.rs` 曾 `rev.unwrap_or("main")` — 无 rev 可漂；`SYX_INSTALL_LOCKED` 仅钉 registry，Git 路径不读 lock。

## Delivery

1. `resolve_git_dependency`：禁默认 `main`；缺 rev → **P043**（含依赖名 · `git rev required` · `C-4a`）  
2. `resolve_git_dependency_locked`：lock 缺/非 Git/URL 不一致/HEAD≠pin → **P014**；不按浮动分支重解析  
3. integration：`c4a_git_manifest_missing_rev_rejected` · `c4a_locked_git_reuses_pin` · `c4a_locked_git_rejects_missing_or_wrong_pin`；registry `c04_locked_*` 回归  
4. SSOT / `02`：C-4a `[x]`；C-4 `[~]`；默认下一 **C-4b**  
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| P043 / locked git P014 API | **1** | **1** (`c4a_git_api`) |
| c4a + c04 integration | **pass** | **1** (`c4a_integ_cargo` · 9 passed) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c4a_git_rev_pin_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c4a_git_api=1
c4a_tests_present=1
c4a_native_real_count=16
c4a_tls_server_partial=1
c4a_census_doc=1
c4a_ssot_updated=1
c4a_roadmap_ok=1
c4a_integ_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c4a_git_rev_pin_census_ok=1
current=C-4a-git-rev-pin
next=C-4b
```

## Non-claims

- ≠ C-4 `[x]` · 默认全局 `SYX_INSTALL_LOCKED` · `syx install --locked` 正式旗标（C-4b）
- ≠ `syx publish` 强制 lock · lock v16 CDN/farm 字段裁剪
- ≠ path 依赖 content hash · tls.server raise · macOS · R-50 · JIT · 改硬 KPI
- ≠ 宣称 C-3 `[x]`

## Soft park

C-4b default-locked · publish-requires-lock · lock v16 trim · path content hash · ModuleId · full MirOp · exception · tls.server · macOS · R-50 · soft→hard · JIT expand
