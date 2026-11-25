# C-3r MIR local debug metadata floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_local_debug_floor` → **MIR018**（非空 `local_names`/`local_types` 须 `len == local_count`）；挂钩 build-stack 地板 **之后**。空向量 OK（AOT strip）。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3r_mir_local_debug_floor_census.sh`](../../scripts/c3r_mir_local_debug_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3q** | **exited** |
| **C-3r** local-debug floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型·φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · tls.server · R-50 park |

## Root cause

`local_names`/`local_types` 为 DAP debug 元数据；损坏长度可进入 unify/codegen；C-3n/k 不校 debug 向量；无 fail-closed 闸。

## Delivery

1. `syx-mir::{verify_mir_local_debug_floor, verify_mir_local_debug_floor_functions}` + unit  
2. 挂钩 unify / codegen（build-stack 之后）  
3. 负向钉：`c3r_mir_local_debug_floor_contract`  
4. SSOT / `02`：C-3r `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 非空 debug 向量 `len == local_count`；空 = AOT/生产 strip OK  
- ≠ locals 类型语义 / φ  
- ≠ 改 AOT strip 行为  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| local-debug-floor API + dual hooks | **1** | **1** |
| MIR018 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
