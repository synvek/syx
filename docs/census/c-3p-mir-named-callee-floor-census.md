# C-3p MIR named-callee resolve floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_named_callee_floor` → **MIR016**（Call/CallSuper/ActorCall/ActorSend/MakeClosure/TaskSpawn 须 `MirFnIndex` 可解析）；挂钩 fn-lit 地板 **之后**。≠ ConstFnIndex R-33。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3p_mir_named_callee_floor_census.sh`](../../scripts/c3p_mir_named_callee_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3o** | **exited** |
| **C-3p** named-callee floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex fail-closed（R-33） · CallIndirect 语义 · Build 栈高扩 · 完整 DefId · 完整 exception · tls.server · R-50 park |

## Root cause

C-3c 仅建 `MirFnIndex` + 拒重名；命名 callee 缺名迟至 codegen `resolve_fn_index`；unify 无扫描闸。

## Delivery

1. `syx-mir::{verify_mir_named_callee_floor, verify_mir_named_callee_floor_functions}` + unit  
2. 挂钩 unify / codegen（fn-lit 之后；复用已建 MirFnIndex）  
3. 负向钉：`c3p_mir_named_callee_floor_contract`  
4. SSOT / `02`：C-3p `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 命名 callee 六类 op vs MirFnIndex  
- ≠ ConstFnIndex fail-closed（R-33 `-1` 哨兵保留）  
- ≠ CallIndirect / HostFn arity  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| named-callee-floor API + dual hooks | **1** | **1** |
| MIR016 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
