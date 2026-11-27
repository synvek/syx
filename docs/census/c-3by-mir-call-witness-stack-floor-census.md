# C-3by MIR CallWitness stack-height floor census

**Date:** 2026-11-27  
**Scope:** `verify_mir_call_witness_stack_floor` → **MIR077**（pilot∪CallWitness；Δ=(n,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 iterator-next-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3by_mir_call_witness_stack_floor_census.sh`](../../scripts/c3by_mir_call_witness_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…prior** | **exited** |
| **C-3by CallWitness stack floor** | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**；AwaitPoll · BindResult · LoadGlobal 跟踪-only · CallIndirect 完整 arity · CallSuper init · CallClosure 完整 arity · HostFn arity · ConstFnIndex（R-33） · 跨块 φ · DefId · exception · tls.server · R-50 park |

## Root cause

上一刀只闸前序；`CallWitness` 不在 pilot，遇之即弃流。本窗薄闸栈高。

## Delivery

1. `syx-mir::verify_mir_call_witness_stack_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（iterator-next-stack 之后）  
3. 负向钉：`c3by_mir_call_witness_stack_floor_contract`  
4. SSOT / `02`：C-3by `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI

## Honesty

- 界 = pilot∪CallWitness；仅已证高度不足 → MIR077  
- ≠ 改 `is_suite1_pilot_mir_op` · ≠ 扩 MIR004  
- ≠ 跨块 φ

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| call-witness-stack-floor API + dual hooks | **1** | **1** |
| MIR077 call_witness pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
