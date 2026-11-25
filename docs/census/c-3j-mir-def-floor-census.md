# C-3j MIR DefId thin floor census

**Date:** 2026-11-25  
**Scope:** 可选 `MirFunction.owner_module` + `verify_mir_def_floor` → **MIR010**（OOB owner；同 `(owner, name)` 冲突）；挂钩 try 地板 **之后**。全 `None` / 空表通过。stdlib merge 保持 `None`。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3j_mir_def_floor_census.sh`](../../scripts/c3j_mir_def_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3i** | **exited** |
| **C-3j** Def floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp / P0 C-0a+/C-0e）；Call(ModuleId) IR · 完整 DefId 图 · 完整 exception · tls.server · R-50 park |

## Root cause

C-3c `MirFnIndex` 与 C-3h `MirModIndex` 分表、互不关联；`MirFunction` 无 per-module 归属 → 无法 fail-close OOB / 同 module 重名 def。

## Delivery

1. `MirFunction.owner_module: Option<u32>` + `syx-mir::{MirDefIndex, verify_mir_def_floor, verify_mir_def_floor_functions}` + unit  
2. unify 用户 dep/入口 populate；stdlib 保持 None  
3. 挂钩 unify / codegen（try 之后）  
4. 负向钉：`c3j_mir_def_floor_contract`  
5. SSOT / `02`：C-3j `[x]`；C-3 `[~]`；下一 **跨轴**  
6. census + 硬 KPI  

## Honesty

- ≠ 全 DefId 图 / `Call(FuncId|ModuleId)` IR / 跨 crate 符号  
- 全 `None`（stdlib / 旧路径 / 单文件）→ 通过  
- 同名跨不同 owner → 本地板允许（name 唯一仍由 C-3a/MIR003）  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| def-floor API + dual hooks | **1** | **1** |
| MIR010 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / 完整 DefId / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
