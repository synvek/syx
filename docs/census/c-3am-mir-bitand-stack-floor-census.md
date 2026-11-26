# C-3am MIR BitAnd stack-height floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_bitand_stack_floor` → **MIR039**（pilot∪BitAnd；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / BitOr·Xor·Shl·Shr）；挂钩 make-closure-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3am_mir_bitand_stack_floor_census.sh`](../../scripts/c3am_mir_bitand_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa…C-3al** | **exited** |
| **C-3am** BitAnd stack floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；BitOr/BitXor/Shl/Shr · Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

C-3al 只闸 MakeClosure；`BitAnd { kind }` 不在 pilot，遇之即弃流。VM 二元位运算需两栈元；`kind` 不改栈高。手搓/`compile_from_mir` 可塞 `height < 2` 的 BitAnd。

## Delivery

1. `syx-mir::verify_mir_bitand_stack_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（make-closure 之后；双形态对齐 C-3al）  
3. 负向钉：`c3am_mir_bitand_stack_floor_contract`  
4. SSOT / `02`：C-3am `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = pilot∪BitAnd；仅已证高度下 `height < 2` → MIR039  
- ≠ 改 `is_suite1_pilot_mir_op` · ≠ 扩 MIR004  
- ≠ BitOr/BitXor/Shl/Shr · ≠ 跨块 φ  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| bitand-stack-floor API + dual hooks | **1** | **1** |
| MIR039 bitand pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
