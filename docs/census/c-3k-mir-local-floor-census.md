# C-3k MIR locals OOB floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_local_floor` → **MIR011**（`LoadLocal`/`StoreLocal` 相对 `local_count` OOB）；挂钩 Def 地板 **之后**。合法槽 / 空函数通过。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3k_mir_local_floor_census.sh`](../../scripts/c3k_mir_local_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3j** | **exited** |
| **C-3k** local floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) IR · locals 类型/φ · LoadGlobal · 完整 DefId · 完整 exception · tls.server · R-50 park |

## Root cause

stack/type 地板只认高度与粗 tag；`LoadLocal(i)`/`StoreLocal(i)` 相对 `local_count` 无 OOB 闸，坏 MIR 可静默过 unify/codegen。

## Delivery

1. `syx-mir::{verify_mir_local_floor, verify_mir_local_floor_functions}` + unit  
2. 挂钩 unify / codegen（Def 之后）  
3. 负向钉：`c3k_mir_local_floor_contract`  
4. SSOT / `02`：C-3k `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- ≠ locals 槽类型 / 跨块 φ / `LoadGlobal` 表  
- ≠ 全量 MirOp / Call(ModuleId) IR / 完整 DefId  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| local-floor API + dual hooks | **1** | **1** |
| MIR011 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / locals 类型 · φ  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
