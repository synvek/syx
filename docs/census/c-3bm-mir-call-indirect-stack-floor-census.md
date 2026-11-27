# C-3bm MIR CallIndirect stack-height floor census

**Date:** 2026-11-27  
**Scope:** `verify_mir_call_indirect_stack_floor` → **MIR065**（pilot∪CallIndirect；Δ=(n+1,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 arr_sorted-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3bm_mir_call_indirect_stack_floor_census.sh`](../../scripts/c3bm_mir_call_indirect_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa…C-3bl** | **exited** |
| **C-3bm CallIndirect stack floor** | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect 完整 arity · TaskJoin · JumpIfNil · 跨块 φ · DefId · exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

C-3bl 只闸 ArrSorted；`CallIndirect` 不在 pilot，遇之即弃流。本窗薄闸栈高。手搓可塞已证高度不足的 CallIndirect。

## Delivery

1. `syx-mir::verify_mir_call_indirect_stack_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（arr_sorted-stack 之后）  
3. 负向钉：`c3bm_mir_call_indirect_stack_floor_contract`  
4. SSOT / `02`：C-3bm `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI

## Honesty

- 界 = pilot∪CallIndirect；仅已证高度下不足 → MIR065  
- ≠ 改 `is_suite1_pilot_mir_op` · ≠ 扩 MIR004  
- ≠ 跨块 φ · ≠ CallIndirect 完整 arity

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| call_indirect-stack-floor API + dual hooks | **1** | **1** |
| MIR065 call_indirect pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
