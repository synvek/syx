# C-0s subscript get/set index Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** Extension collect 为 `__subscript_get/set` 写 mangled `function_params`；`Expr::Index` get + `IndexAssign` set 对 **首参** 补 `check_unknown_to_ground_sink` → **S004**；value 仍 C-0k；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**（C-0a+ prep / C-0e / enum case payload / Module.Type prep）。  
**Script:** [`scripts/c0s_subscript_index_unknown_sink_census.sh`](../../scripts/c0s_subscript_index_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…r** | **exited** |
| **C-0s** subscript-index sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / enum / Module.Type） |

## Root cause

C-0k/m 闸住 subscript **value**；get/set 臂对 index 仅 `analyze_expr`。class-body 已有 mangled params（C-0j）；extension 缺 `function_params`——`b[i]` 假绿。

## Delivery

1. `semantic_collect_decls.rs`：extension `__subscript_*` → `function_params`  
2. `semantic_analyze_expr.rs`：get + set index sink（首参）  
3. 钉：`c0s_*` get/set S004 + literal/Any；回归 c0k/m/r  
4. SSOT / `02`：C-0s `[x]`；C-0a+ 仍 park  
5. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ enum case payload / Module.Type / C-0e / 关软臂  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| C-0s collect | **1** | **1** |
| C-0s analyze hooks (×2) | **2** | **2** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Module.Type / enum case payload GA  
