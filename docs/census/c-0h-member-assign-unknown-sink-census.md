# C-0h MemberAssign Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** `MemberAssign` 右侧 `Unknown`（无注解 Ident / Member）→ 地面具体字段 → **S004**；`infer_member_type` + assignable + `check_unknown_to_ground_sink`；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**（IndexAssign tip / C-0a+ prep / C-0e）。  
**Script:** [`scripts/c0h_member_assign_unknown_sink_census.sh`](../../scripts/c0h_member_assign_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f + C-0g** | **exited** |
| **C-0h** MemberAssign sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（IndexAssign tip / C-0a+ prep / C-0e）；C-0a+ 三分 · 关软臂 · C-0e · 方法全参 sink park |

## Root cause

C-0a/C-0g 闸住 Assign / CompoundAssign；`MemberAssign` 仅 `analyze_expr(value)`，无 `check_assignable` / sink，无注解 Ident 经软臂 silent 穿过。

## Delivery

1. `semantic_analyze_expr.rs` MemberAssign：`infer_member_type` → assignable + `check_unknown_to_ground_sink`  
2. 钉：`c0h_untyped_member_assign_int_s004` + 字面量/`Any` 绿钉  
3. SSOT / `02`：C-0h `[x]`；C-0a+ 仍 park  
4. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ IndexAssign sink / 方法形参全量 sink / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| MemberAssign sink hook | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ IndexAssign / 全方法形参表 GA  
