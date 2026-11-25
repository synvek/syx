# C-3l MIR globals OOB floor census

**Date:** 2026-11-25  
**Scope:** `MirModule.global_count` + `verify_mir_global_floor` → **MIR012**（`LoadGlobal`/`StoreGlobal` 相对 `global_count` OOB）；挂钩 local 地板 **之后**。lower 写 count；unify merge max / stdlib 函数片段用 slot-bound 吸收。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3l_mir_global_floor_census.sh`](../../scripts/c3l_mir_global_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3k** | **exited** |
| **C-3l** global floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · global 名表/跨模块 remap · locals 类型/φ · 完整 DefId · 完整 exception · tls.server · R-50 park |

## Root cause

C-3k 闸 locals；`LoadGlobal`/`StoreGlobal` 无模块级界；mir-run StoreGlobal 静默 resize；lower 有 `VarGlobal` 名表却未写入 `MirModule`。

## Delivery

1. `MirModule.global_count` + `verify_mir_global_floor` / `_functions` + `mir_global_slot_bound` / `absorb_mir_global_count` + unit  
2. `lower_module` 写 count；unify merge max + 函数片段 absorb；codegen AST 路径 absorb  
3. 挂钩 unify / codegen（local 之后）  
4. 负向钉：`c3l_mir_global_floor_contract`  
5. SSOT / `02`：C-3l `[x]`；C-3 `[~]`；下一 **跨轴**  
6. census + 硬 KPI  

## Honesty

- ≠ global 名表身份 / 跨模块槽 remap  
- ≠ locals 类型 / φ / Call(ModuleId) IR  
- stdlib 仅函数缓存路径用 `mir_global_slot_bound` 恢复界（未改 cache schema）  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| global-floor API + dual hooks | **1** | **1** |
| MIR012 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / global 名表  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
