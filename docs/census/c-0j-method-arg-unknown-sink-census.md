# C-0j 方法实参 Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** 用户 Struct/Class 方法 mangled `function_params` + Member 调用右侧 `Unknown`（无注解 Ident）→ 地面具体形参 → **S004**；复用 `check_unknown_to_ground_sink`；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]` / host·extension 全方法 GA；默认下一 **跨轴**（C-0a+ prep / C-0e / subscript_set tip）。  
**Script:** [`scripts/c0j_method_arg_unknown_sink_census.sh`](../../scripts/c0j_method_arg_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…i** | **exited** |
| **C-0j** method-arg sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / subscript_set）；host/extension MethodId · Init 形参表 park |

## Root cause

C-0f 闸住自由函数 + collection 单参；Struct/Class 方法仅写 `function_param_labels`，Member `else` 只 `analyze_expr`，无注解 Ident 经软臂 silent 穿过地面形参。

## Delivery

1. `semantic_collect_decls.rs`：Struct/Class 非泛型方法写 mangled `function_params`  
2. `semantic_analyze_call.rs` Member `else`：查 `{Type}_{method}` + C-0f 同形 sink/assignable  
3. 钉：`c0j_untyped_method_arg_int_s004` + 字面量/`Any` 绿钉  
4. SSOT / `02`：C-0j `[x]`；C-0a+ 仍 park  
5. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ host/stdlib MethodId / extension / Init / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| collect function_params (Struct+Class) | **2** | **2** |
| Member C-0j sink hook | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ 全方法 / host / extension GA  
