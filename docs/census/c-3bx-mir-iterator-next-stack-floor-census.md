# C-3bx MIR IteratorNext stack-height floor census

**Date:** 2026-11-27  
**Scope:** `verify_mir_iterator_next_stack_floor` → **MIR076**（pilot∪IteratorNext；Δ=(n,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004）；挂钩 actor-call-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3bx_mir_iterator_next_stack_floor_census.sh`](../../scripts/c3bx_mir_iterator_next_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…prior** | **exited** |
| **C-3bx IteratorNext stack floor** | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**；AwaitPoll · BindResult · CallSuper · CallClosure/CallWitness arity · CallIndirect 完整 arity · HostFn arity · ConstFnIndex（R-33） · 跨块 φ · DefId · exception · tls.server · R-50 park |

## Root cause

上一刀只闸前序；`IteratorNext` 不在 pilot，遇之即弃流。本窗薄闸栈高。

## Delivery

1. `syx-mir::verify_mir_iterator_next_stack_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（actor-call-stack 之后）  
3. 负向钉：`c3bx_mir_iterator_next_stack_floor_contract`  
4. SSOT / `02`：C-3bx `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI

## Honesty

- 界 = pilot∪IteratorNext；仅已证高度不足 → MIR076  
- ≠ 改 `is_suite1_pilot_mir_op` · ≠ 扩 MIR004  
- ≠ 跨块 φ

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| iterator-next-stack-floor API + dual hooks | **1** | **1** |
| MIR076 iterator_next pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
