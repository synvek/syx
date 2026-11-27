# C-0o DataClass memberwise Init Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** `Decl::DataClass` collect 写 `{Type}_init` → labels + `function_params`；Call 已有 `dataclass_names` 查该键 → 复用 C-0f Unknown→地面 sink → **S004**；**未**改 Call / **未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]` / Module.Type；默认下一 **跨轴**（C-0a+ prep / C-0e / IndexAssign index tip / Module.Type prep）。  
**Script:** [`scripts/c0o_dataclass_memberwise_unknown_sink_census.sh`](../../scripts/c0o_dataclass_memberwise_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…n** | **exited** |
| **C-0o** DataClass-memberwise sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / IndexAssign index / Module.Type prep）；host/extension 方法 GA park |

## Root cause

Struct memberwise（C-0l）已写 `{Type}_init`；`Decl::DataClass` 仅 `dataclass_fields`；Call 对 `dataclass_names` 查 `{name}_init` 却查空——`P(v)` 假绿。

## Delivery

1. `semantic_collect_decls.rs`：DataClass → `function_param_labels` + `function_params`（字段序；`ParamLabel::Omitted` 保 `P(3,4)` 表面；≠ Struct Same）  
2. Call：**不改**（已支持 `dataclass_names` + `{name}_init`）  
3. 钉：`c0o_untyped_dataclass_init_arg_int_s004` + 字面量/`Any`  
4. SSOT / `02`：C-0o `[x]`；C-0a+ 仍 park  
5. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ Module.Type / C-0e / IndexAssign index  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| DataClass collect C-0o | **1** | **1** |
| Call dataclass_names key | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Module.Type ctor GA  
