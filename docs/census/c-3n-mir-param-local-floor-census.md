# C-3n MIR param/local metadata floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_param_local_floor` → **MIR014**（`param_count ≤ local_count`）；挂钩 CallHost 地板 **之后**。≠ locals 类型/φ。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3n_mir_param_local_floor_census.sh`](../../scripts/c3n_mir_param_local_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3m** | **exited** |
| **C-3n** param/local floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex fail-closed · 完整 DefId · 完整 exception · tls.server · R-50 park |

## Root cause

`MirFunction` 同时携带 `param_count` / `local_count`；MIR011 只校 Load/StoreLocal OOB，不校形参元数据 ⊆ locals；损坏元数据可进入 codegen/VM。

## Delivery

1. `syx-mir::{verify_mir_param_local_floor, verify_mir_param_local_floor_functions}` + unit  
2. 挂钩 unify / codegen（CallHost 之后）  
3. 负向钉：`c3n_mir_param_local_floor_contract`  
4. SSOT / `02`：C-3n `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 元数据 `param_count ≤ local_count`  
- ≠ locals 类型 / φ / join  
- ≠ ConstFnIndex fail-closed / HostFn arity  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| param-local-floor API + dual hooks | **1** | **1** |
| MIR014 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
