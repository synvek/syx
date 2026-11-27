# C-0m class-body subscript_set Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** Class/Struct 体内 `__subscript_get/set` 登记 `subscript_*` 表 → 复用 C-0k IndexAssign Unknown→地面 value → **S004**；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**（C-0a+ prep / C-0e / Actor·Module.Type ctor）。  
**Script:** [`scripts/c0m_class_body_subscript_unknown_sink_census.sh`](../../scripts/c0m_class_body_subscript_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…l** | **exited** |
| **C-0m** class-body subscript | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / Actor·Module.Type）；host/extension 方法 GA park |

## Root cause

C-0k 仅 extension collect 写 `subscript_set_types`；Class/Struct 体内 desugared `__subscript_*` 未登记 → `has_subscript_set` 假 → IndexAssign 假绿。

## Delivery

1. `semantic_collect_decls.rs`：Class + Struct Function 镜像 extension 登记  
2. 钉：`c0m_untyped_class_subscript_set_int_s004` + 字面量/`Any`（类体内 desugared）  
3. SSOT / `02`：C-0m `[x]`；C-0a+ 仍 park  
4. census + 硬 KPI（探针可用表面 `subscript` + `syx check`）  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ ErrorType / InferenceVar 三分  
- ≠ 改 C-0k analyze / C-0e / Actor  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| Class+Struct C-0m collect | **2** | **2** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Actor / Module.Type ctor GA  
