# C-3q MIR Build stack-height floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_build_stack_floor` → **MIR017**（Array/Dict/SetBuild 栈 underflow）；挂钩 named-callee 地板 **之后**。≠ 改 `is_suite1_pilot_mir_op`；≠ MIR004 扩义。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3q_mir_build_stack_floor_census.sh`](../../scripts/c3q_mir_build_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3p** | **exited** |
| **C-3q** Build stack floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · 完整 DefId · 完整 exception · tls.server · R-50 park |

## Root cause

C-3d 遇非 pilot（含 Build）abandon；VM/mir-run 对 Array/Dict/SetBuild 用 `unwrap_or(Nil)` 静默垫栈；无 MIR fail-closed 闸。

## Delivery

1. `syx-mir::{verify_mir_build_stack_floor, verify_mir_build_stack_floor_functions}` + unit  
2. 挂钩 unify / codegen（named-callee 之后）  
3. 负向钉：`c3q_mir_build_stack_floor_contract`  
4. SSOT / `02`：C-3q `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = pilot ∪ ArrayBuild/DictBuild/SetBuild 单流栈高；**仅 Build underflow → MIR017**  
- ≠ 改 `is_suite1_pilot_mir_op` / MIR004 语义  
- CallHost / 非 trackable / 非 Build underflow → abandon（非 entry StoreLocal 等）  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| build-stack-floor API + dual hooks | **1** | **1** |
| MIR017 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
