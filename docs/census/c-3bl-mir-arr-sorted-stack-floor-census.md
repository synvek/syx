# C-3bl MIR ArrSorted stack-height floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_arr_sorted_stack_floor` → **MIR064**（pilot∪ArrSorted；Δ=(1,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 store-global-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3bl_mir_arr_sorted_stack_floor_census.sh`](../../scripts/c3bl_mir_arr_sorted_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa…C-3bk** | **exited** |
| **C-3bl ArrSorted stack floor** | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · HostFn arity · trim remap · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · DefId · exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

C-3bk 只闸 StoreGlobal；`ArrSorted` 不在 pilot，遇之即弃流。VM 排序需一栈元并推回。手搓可塞已证空栈 ArrSorted。

## Delivery

1. `syx-mir::verify_mir_arr_sorted_stack_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（store-global-stack 之后）  
3. 负向钉：`c3bl_mir_arr_sorted_stack_floor_contract`  
4. SSOT / `02`：C-3bl `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI

## Honesty

- 界 = pilot∪ArrSorted；仅已证高度下 `height < 1` → MIR064  
- ≠ 改 `is_suite1_pilot_mir_op` · ≠ 扩 MIR004  
- ≠ 跨块 φ

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| arr-sorted-stack-floor API + dual hooks | **1** | **1** |
| MIR064 arr_sorted pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
