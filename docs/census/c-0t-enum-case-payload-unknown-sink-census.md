# C-0t Enum case payload Unknown→ground sink census

**Date:** 2026-11-27  
**Scope:** `Decl::Enum` / FinalClass variants：有字段 case 写 `function_params` + `ParamLabel::Same` labels，键 **`{Enum}_{case}`** 与 bare **`case`**（仅当 bare 未占用）；复用既有 Ident/Member Call sink → **S004**；**未**改 Call / 关软臂。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**（C-0a+ prep / C-0e / Module.Type prep）。  
**Script:** [`scripts/c0t_enum_case_payload_unknown_sink_census.sh`](../../scripts/c0t_enum_case_payload_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d + C-0f…s** | **exited** |
| **C-0t** enum case payload sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-0a+ prep / C-0e / Module.Type prep） |

## Root cause

Enum associated-value 构造 `E.wrap(n: x)` / bare `wrap(n: x)` 走 Call，但 collect 只写 `enum_case_field_types`、从不写 `function_params`——Ident/Member 已有 C-0f/C-0j 循环却查空假绿。

## Delivery

1. `semantic_collect_decls.rs`：`register_enum_case_payload_params`（toplevel + nested Enum/FinalClass）  
2. Call **不改**；软臂保留  
3. 钉：`c0t_*` Member S004 + literal/Any；回归 c0o/c0s  
4. SSOT / `02`：C-0t `[x]`；C-0a+ 仍 park  
5. census + 硬 KPI  

## Honesty

- **不**关 `is_compatible(_, Unknown)`  
- ≠ Module.Type ctor / 跨 enum 同名 case 全量消解 / C-0e / 关软臂  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| C-0t collect helper | **1** | **1** |
| enum_case_field_types sites synced | **4** | **4** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3/C-5 `[x]`  
- ≠ Module.Type GA  
