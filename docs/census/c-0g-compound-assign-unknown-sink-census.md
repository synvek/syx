# C-0g CompoundAssign Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** `CompoundAssign` 右侧 `Unknown`（无注解 Ident / Member）→ 地面具体左值 → **S004**；复用 `check_unknown_to_ground_sink`；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c0g_compound_assign_unknown_sink_census.sh`](../../scripts/c0g_compound_assign_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f** | **exited** |
| **C-0g** CompoundAssign sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / MirOp 厚余片）；C-0a+ 三分 · 关软臂 · C-0e park |

## Root cause

C-0a 闸住 Assign；`CompoundAssign` 仅 `check_assignable` / `promote_numeric`，无注解 Ident 经软臂 silent 穿过。

## Delivery

1. `semantic_analyze_expr.rs` CompoundAssign 挂 `check_unknown_to_ground_sink`  
2. 钉：`c0g_untyped_compound_assign_int_s004` + 字面量/`Any` 绿钉  
3. SSOT / `02`：C-0g `[x]`；C-0a+ 仍 park  
4. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ 方法形参全量 sink / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| CompoundAssign sink hook | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ 全方法形参表 GA  
