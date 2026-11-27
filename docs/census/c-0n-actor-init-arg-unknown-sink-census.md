# C-0n Actor Init 实参 Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** Actor `Decl::Init` 写 `{Actor}_init` → `function_params`；构造 `Counter(...)` 查该键后复用 C-0f Unknown→地面 sink → **S004**；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]` / Module.Type；默认下一 **跨轴**（C-0a+ prep / C-0e / Module.Type tip）。  
**Script:** [`scripts/c0n_actor_init_arg_unknown_sink_census.sh`](../../scripts/c0n_actor_init_arg_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…m** | **exited** |
| **C-0n** Actor-init sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / Module.Type）；host/extension 方法 GA park |

## Root cause

C-0l 闸住 Class/Struct Init；Actor collect 无 Init 形参表；Call 仅 class/dataclass 查 `{name}_init`——`ActorName(...)` 绕开 sink。

## Delivery

1. `semantic_collect_decls.rs`：Actor Init → `function_params` + labels  
2. `semantic_analyze_call.rs`：`actor_types` 查 `{name}_init` + labels  
3. 钉：`c0n_untyped_actor_init_arg_int_s004` + 字面量/`Any`  
4. SSOT / `02`：C-0n `[x]`；C-0a+ 仍 park  
5. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ Module.Type / C-0e  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| Actor collect C-0n | **1** | **1** |
| Call actor_types key | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Module.Type ctor GA  
