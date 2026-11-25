# C-3i MIR try/catch pairing floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_try_floor` → **MIR009**（BeginTry/EndTry 深度；orphan/excess Catch）；挂钩 ModuleId **之后**。Throw 传播（depth-0）不闸。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3i_mir_try_floor_census.sh`](../../scripts/c3i_mir_try_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3h** | **exited** |
| **C-3i** try floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（P0 Unknown 实参 / DefId）；全量 MirOp · 完整 exception · Call(ModuleId) IR · tls.server · R-50 park |

## Root cause

CFG 地板认 `BeginTry` 目标块但不验配对；`EndTry`/`Catch` 失衡与 orphan Catch 可静默过 unify/codegen。

## Delivery

1. `syx-mir::{verify_mir_try_floor, verify_mir_try_floor_functions}` + unit tests  
2. 挂钩 unify / codegen（ModuleId 之后）  
3. 负向钉：`c3i_mir_try_floor_contract`  
4. SSOT / `02`：C-3i `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- Catch 在 lower 中位于 `EndTry` **之后**（catch pad）→ Catch 不以 try 深度闸，而以 orphan/excess 闸  
- Unary `?` 无 Catch → 允许 BeginTry/EndTry 无 Catch  
- `throws` 内传播 Throw（无本地 try）→ **不** MIR009  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| try-floor API + dual hooks | **1** | **1** |
| MIR009 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / DefId / 支配·φ / 完整 exception  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 闸 depth-0 Throw（传播合法）  
