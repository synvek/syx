# C-0r String Index Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** `Expr::Index` `TypeId::String`：期望 `platform_int` → assignable → `check_unknown_to_ground_sink` → **S004**；返回仍 `Character`；**未**关 `(_, Unknown)`；**不**改 Array/Dict/subscript / IndexAssign / Call。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**（C-0a+ prep / C-0e / enum case payload / Module.Type prep / subscript index）。  
**Script:** [`scripts/c0r_string_index_unknown_sink_census.sh`](../../scripts/c0r_string_index_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…q** | **exited** |
| **C-0r** String-index sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / enum / Module.Type / subscript index） |

## Root cause

C-0q 闸住 Array/Dict Index 读；String 臂仅 `analyze_expr(index)`——无注解 `s[i]` 假绿。

## Delivery

1. `semantic_analyze_expr.rs`：String Index → C-0r sink  
2. 钉：`c0r_untyped_string_index_int_s004` + 字面量/`Any`；回归 c0q_  
3. SSOT / `02`：C-0r `[x]`；C-0a+ 仍 park  
4. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ String IndexAssign / subscript index / enum / Module.Type / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| C-0r hook | **1** | **1** |
| C-0q retained | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Module.Type / enum case payload / subscript index GA  
