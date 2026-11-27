# C-0q Index read index/key Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** `Expr::Index` Array index + Dict key 在 `check_assignable_with_literal` 之后补 `check_unknown_to_ground_sink` → **S004**；**未**关 `(_, Unknown)`；**不**改 String / subscript_get / IndexAssign / Call / collect。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**（C-0a+ prep / C-0e / enum case payload / Module.Type prep / subscript index）。  
**Script:** [`scripts/c0q_index_read_index_key_unknown_sink_census.sh`](../../scripts/c0q_index_read_index_key_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…p** | **exited** |
| **C-0q** Index-read index/key sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / enum case payload / Module.Type / subscript index） |

## Root cause

C-0p 闸住 IndexAssign 写路径 index/key；`Expr::Index` 读路径同形仅有 assignable，无 sink——`a[i]` / `d[k]` 假绿。

## Delivery

1. `semantic_analyze_expr.rs`：Index Array/Dict 各一处 C-0q sink（`found_*` clone）  
2. 钉：`c0q_untyped_array_index_read_int_s004` + `c0q_untyped_dict_key_read_string_s004` + 字面量/`Any`  
3. SSOT / `02`：C-0q `[x]`；C-0a+ 仍 park  
4. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ String index / subscript get·set index / enum payload / Module.Type / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| C-0q hooks (×2) | **2** | **2** |
| C-0p retained | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Module.Type / enum case payload GA  
