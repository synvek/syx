# C-0l Init 实参 Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** Class/Struct 显式 Init + Struct memberwise 写 `{Type}_init` → `function_params`；构造 `Foo(...)` 查该键后复用 C-0f Unknown→地面 sink → **S004**；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]` / Actor / Module.Type；默认下一 **跨轴**（C-0a+ prep / C-0e / class-body subscript）。  
**Script:** [`scripts/c0l_init_arg_unknown_sink_census.sh`](../../scripts/c0l_init_arg_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…k** | **exited** |
| **C-0l** Init-arg sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / class-body subscript）；Actor/Module.Type · DataClass 扩面 park |

## Root cause

C-0f/j 闸住自由函数与用户方法；Init 仅有 `function_param_labels[{Type}_init]`，Call 却用 `function_params.get(Foo)`——构造调用绕开 sink。

## Delivery

1. `semantic_collect_decls.rs`：Class/Struct Init + memberwise 写 `function_params`  
2. `semantic_analyze_call.rs`：class/dataclass 查 `{name}_init`  
3. 钉：`c0l_untyped_init_arg_int_s004` + 字面量/`Any` 绿钉  
4. SSOT / `02`：C-0l `[x]`；C-0a+ 仍 park  
5. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ Actor / Module.Type / DataClass 扩面 / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| collect C-0l sites | **3** | **3** |
| Call `{Type}_init` key | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Actor / Module.Type ctor GA  
