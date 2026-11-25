# C-0f call-arg Unknown→ground sink census

**Date:** 2026-11-25  
**Scope:** 调用实参 `Unknown`（无注解 Ident / Member）→ 地面具体形参 → **S004**；复用 `check_unknown_to_ground_sink`；**未**关 `(_, Unknown)`。**不**宣称 C-0a+ / C-3/C-5 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c0f_call_arg_unknown_sink_census.sh`](../../scripts/c0f_call_arg_unknown_sink_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0a…d** | **exited** |
| **C-0f** call-arg sink | **exited** |
| **C-0** | **`[x]`** 薄闸（≠ C-0a+） |
| Open | 默认下一 **跨轴**（C-3 DefId / P0 余债）；C-0a+ 三分 · C-0e · DefId · 全量 MirOp park |

## Root cause

C-0a 闸住 return/let/assign；`analyze_call` 对地面形参仍靠 `is_compatible(_, Unknown)` 软放行；绿钉 `c0a_untyped_arg_to_int_param_still_ok` 锁洞。

## Delivery

1. 自由函数参数循环 + collection 元素路径挂 `check_unknown_to_ground_sink`  
2. 翻钉 → `c0f_untyped_arg_to_int_param_s004`；保 `Any`/字面量  
3. SSOT / `02`：C-0f `[x]`；C-0a+ 仍 park（去「调用实参」条）  
4. census + 硬 KPI  

## Honesty

- 形参自身为 `Unknown` 时仍跳过 assign-check（`id(_ x)` parity）  
- **不**关 `is_compatible(_, Unknown)`  
- Member 方法多数未走 `function_params` 全量 assign（本窗不扩）  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| call-arg sink hook | **1** | **1** |
| lang nails | **pass** | **1** |
| soft arms retained | **1** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-0a+ 三分 / 关软臂 / C-0e  
- ≠ C-3 DefId / C-3·C-5 `[x]`  
- ≠ 全方法形参表 GA  
