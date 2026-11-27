# C-0k subscript_set IndexAssign Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** extension `has_subscript_set` IndexAssign：`subscript_return_type` 地面 value + 右侧 `Unknown`（无注解 Ident）→ **S004**；复用 `check_unknown_to_ground_sink`；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]` / class 体内 subscript / Init；默认下一 **跨轴**（Init tip / C-0a+ prep / C-0e）。  
**Script:** [`scripts/c0k_subscript_set_unknown_sink_census.sh`](../../scripts/c0k_subscript_set_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…j** | **exited** |
| **C-0k** subscript_set sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（Init tip / C-0a+ prep / C-0e）；class 体内 subscript · index 形参 park |

## Root cause

C-0i 闸住 Array/Dict IndexAssign；`has_subscript_set` 早退仅 `analyze_expr`，extension 已有 `subscript_return_type` 却未挂 sink。

## Delivery

1. `semantic_analyze_expr.rs` IndexAssign：`has_subscript_set` 臂挂 assignable + `check_unknown_to_ground_sink`  
2. 钉：`c0k_untyped_subscript_set_int_s004` + 字面量/`Any`（desugared accessors；helpers 无 frontend desugar）  
3. SSOT / `02`：C-0k `[x]`；C-0a+ 仍 park  
4. census + 硬 KPI（探针用表面 `subscript` + `syx check`）  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ Init / class 体内 subscript / index 形参 / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| subscript_set sink hook | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Init GA / class 体内 subscript sink  
