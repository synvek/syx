# C-0i IndexAssign Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** `IndexAssign` Array/Dict 地面 value 右侧 `Unknown`（无注解 Ident / Member）→ 地面具体元素/值 → **S004**；复用 `check_unknown_to_ground_sink`；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**（方法全参 tip / C-0a+ prep / C-0e）。  
**Script:** [`scripts/c0i_index_assign_unknown_sink_census.sh`](../../scripts/c0i_index_assign_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…h** | **exited** |
| **C-0i** IndexAssign sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（方法全参 tip / C-0a+ prep / C-0e）；C-0a+ 三分 · 关软臂 · C-0e · subscript_set / fallback park |

## Root cause

C-0a/g/h 闸住 Assign / CompoundAssign / MemberAssign；`IndexAssign` Array/Dict 已有 assignable，但无 sink，无注解 Ident 经软臂 silent 穿过地面元素类型。

## Delivery

1. `semantic_analyze_expr.rs` IndexAssign：Array + Dict 地面 value 挂 `check_unknown_to_ground_sink`  
2. 钉：`c0i_untyped_index_assign_array_int_s004` + Dict 钉 + 字面量/`Any` 绿钉  
3. SSOT / `02`：C-0i `[x]`；C-0a+ 仍 park  
4. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ subscript_set 早退 / fallback / 方法全参 / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| IndexAssign sink hooks (Array+Dict) | **2** | **2** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ subscript_set / 全方法形参表 GA  
