# C-8f CST residual note census

**Date:** 2026-11-25  
**Scope:** UsageGuide en+zh 钉 refuse-lossy **≠** format lossless GA。**不**改 formatter；**不**宣称 C-8 `[x]`；默认下一 **C-3h**。  
**Script:** [`scripts/c8f_cst_residual_note_census.sh`](../../scripts/c8f_cst_residual_note_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5 / C-7** | **`[~]`** |
| **C-8a…e** | **exited** |
| **C-8f** CST residual note | **exited** |
| **C-8** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-3h**；CST lossless 实现 · 真 CallSuper · 去 parser legacy · C-7c+ · 全量 MirOp · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

UsageGuide 已写 CST-first / F003 / `--allow-lossy`，但未显式 **≠ lossless GA**，易被读成全语言 trivia GA。

## Delivery

1. UsageGuide en+zh Honesty (C-8f)  
2. FormatPolicy 注释一行；credibility 钉扩  
3. SSOT / `02`：C-8f `[x]`；C-8 `[~]`；下一 **C-3h**  
4. 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| UsageGuide en+zh Honesty ≠ lossless GA | **pass** | **1** |
| FormatPolicy C-8f note | **pass** | **1** |
| credibility nails | **pass** | **1** (2/2) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c8f_cst_residual_note_census_ok` | **1** | **1** |
| `c8_exited` | **0** | **0** |

## Results (2026-11-25)

```
c8f_usageguide_en_ok=1
c8f_usageguide_zh_ok=1
c8f_fmt_policy_note=1
c8f_honesty_nails=1
c8f_native_real_count=16
c8f_tls_server_partial=1
c8f_census_doc=1
c8f_ssot_updated=1
c8f_roadmap_ok=1
c8_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c8f_cst_residual_note_census_ok=1
current=C-8f
next=C-3h
c8_exited=0
```

## Non-claims

- ≠ C-8 `[x]` · CST 子集扩面 · format lossless GA 实现
- ≠ 真 CallSuper · 去 parser legacy · ModuleId 实现（C-3h）· C-7c+ · C-5c+ · tls.server · macOS · R-50

## Soft park

CST lossless 实现 · 真 CallSuper · 去 parser legacy · C-7c+ · 全量 MirOp · exception · C-5c+ · tls.server · macOS · R-50
