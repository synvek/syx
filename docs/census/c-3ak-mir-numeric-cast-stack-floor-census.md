# C-3ak MIR NumericCast stack-height floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_numeric_cast_stack_floor` → **MIR037**（pilot∪NumericCast；仅已证空栈 NumericCast；≠ 改 pilot / 扩 MIR004 / MakeClosure）；挂钩 bitnot-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3ak_mir_numeric_cast_stack_floor_census.sh`](../../scripts/c3ak_mir_numeric_cast_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa…C-3aj** | **exited** |
| **C-3ak** NumericCast stack floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；MakeClosure 栈 · Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

C-3aj 只闸 BitNot；`NumericCast { from, to }` 不在 pilot，遇之即弃流。VM NumericCast 需栈顶；`from`/`to` 不改栈高。手搓/`compile_from_mir` 可塞空栈 NumericCast。

## Delivery

1. `syx-mir::verify_mir_numeric_cast_stack_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（bitnot-stack 之后；双形态对齐 C-3aj）  
3. 负向钉：`c3ak_mir_numeric_cast_stack_floor_contract`  
4. SSOT / `02`：C-3ak `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = pilot∪NumericCast；仅已证高度下空栈 NumericCast → MIR037  
- ≠ 改 `is_suite1_pilot_mir_op` · ≠ 扩 MIR004  
- ≠ MakeClosure 栈 · ≠ 跨块 φ  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| numeric-cast-stack-floor API + dual hooks | **1** | **1** |
| MIR037 numeric-cast pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
