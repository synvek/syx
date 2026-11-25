# C-3x MIR Pop stack-height floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_pop_stack_floor` → **MIR024**（pilot∪Pop；仅 Pop underflow；其余 abandon；≠ 改 pilot）；挂钩 succ-term 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3x_mir_pop_stack_floor_census.sh`](../../scripts/c3x_mir_pop_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3w** | **exited** |
| **C-3x** pop-stack floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型·φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

`Pop` 不在 `is_suite1_pilot`；MIR004/MIR017 遇之 abandon；VM 空栈 Pop 静默；unify/codegen 无 fail-closed 闸。

## Delivery

1. `syx-mir::{verify_mir_pop_stack_floor, verify_mir_pop_stack_floor_functions}` + unit  
2. 挂钩 unify / codegen（succ-term 之后）  
3. 负向钉：`c3x_mir_pop_stack_floor_contract`  
4. SSOT / `02`：C-3x `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 仅 Pop underflow → MIR024；非 Pop underflow / 非 trackable → abandon  
- 非 entry 块在本块未见 push 时的 Pop underflow → abandon（unary `try` Jump→after 后 Stmt::Expr Pop；incoming height 未知；≠ 跨块 φ）  
- ≠ 改 `is_suite1_pilot_mir_op`  
- ≠ 扩 MIR004  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| pop-stack-floor API + dual hooks | **1** | **1** |
| MIR024 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
