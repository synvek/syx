# C-3aq MIR Shr stack-height floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_shr_stack_floor` → **MIR043**（pilot∪Shr；Δ=(2,1)；仅已证高度不足；≠ 改 pilot / 扩 MIR004 / Mod·Eq）；挂钩 shl-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3aq_mir_shr_stack_floor_census.sh`](../../scripts/c3aq_mir_shr_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa…C-3ap** | **exited** |
| **C-3aq** Shr stack floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Mod/Eq 等 (2,1) · Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

C-3ap 只闸 Shl；`Shr { kind }` 不在 pilot，遇之即弃流。VM 右移需两栈元；`kind` 不改栈高。手搓/`compile_from_mir` 可塞 `height < 2` 的 Shr。

## Delivery

1. `syx-mir::verify_mir_shr_stack_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（shl-stack 之后；双形态对齐 C-3ap）  
3. 负向钉：`c3aq_mir_shr_stack_floor_contract`  
4. SSOT / `02`：C-3aq `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = pilot∪Shr；仅已证高度下 `height < 2` → MIR043  
- ≠ 改 `is_suite1_pilot_mir_op` · ≠ 扩 MIR004  
- ≠ Mod/Eq · ≠ 跨块 φ  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| shr-stack-floor API + dual hooks | **1** | **1** |
| MIR043 shr pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
