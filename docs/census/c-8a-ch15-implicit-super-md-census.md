# C-8a Ch15 implicit-super MD honesty census

**Date:** 2026-11-25  
**Scope:** Ch15 en+zh：省略 `super.init` = U-38 字段垫 **≠ CallSuper**；真隐式 CallSuper **未实现**。**不**宣称 C-8 `[x]`；默认下一 **C-8b**。  
**Script:** [`scripts/c8a_ch15_implicit_super_md_census.sh`](../../scripts/c8a_ch15_implicit_super_md_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5 / C-7** | **`[~]`** |
| **C-8a** Ch15 MD | **exited** |
| **C-8** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-8b**；C-8c fork 清单 · 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

Ch15 宣称「compiler inserts / 编译器自动插入」隐式 `super.init`；实现是 U-38 继承字段默认垫，真隐式 `CallSuper` 已 park。

## Delivery

1. en+zh Ch15 机制叙事改写 + Swift 差异表  
2. lang 薄钉 `c8a_ch15_implicit_super_md_honesty`  
3. SSOT / `02`：C-8a `[x]`；C-8 `[~]`；下一 **C-8b**  
4. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| en+zh no insert claim | **pass** | **1** (`c8a_md_en_ok` / `c8a_md_zh_ok`) |
| en+zh ≠ CallSuper + not implemented | **pass** | **1** |
| lang honesty nails | **pass** | **1** (2/2) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c8a_ch15_md_census_ok` | **1** | **1** |
| `c8_exited` | **0** | **0** |

## Results (2026-11-25)

```
c8a_md_en_ok=1
c8a_md_zh_ok=1
c8a_honesty_nails=1
c8a_native_real_count=16
c8a_tls_server_partial=1
c8a_census_doc=1
c8a_ssot_updated=1
c8a_roadmap_ok=1
c8_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c8a_ch15_md_census_ok=1
current=C-8a
next=C-8b
c8_exited=0
```

## Non-claims

- ≠ C-8 `[x]` · 实现隐式 CallSuper · 改 MIR/VM
- ≠ formatter `--allow-lossy`（C-8b）· book/parser 全表（C-8c）
- ≠ C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50
- ≠ 宣称「省略 super.init 不可用」（U-38 行为保留）

## Soft park

C-8b refuse-lossy · C-8c fork list · 真 CallSuper · C-7c+ · ModuleId · C-5c+ · tls.server · macOS · R-50
