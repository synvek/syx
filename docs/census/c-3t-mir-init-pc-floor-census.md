# C-3t MIR init_pc bounds floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_init_pc_floor` → **MIR020**（双 0 OK；否则 `init_body_pc ≤ init_wrap_pc ≤ instructions.len()`）；挂钩 aot-export 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3t_mir_init_pc_floor_census.sh`](../../scripts/c3t_mir_init_pc_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3s** | **exited** |
| **C-3t** init-pc floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型·φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · block.id 唯一 · tls.server · R-50 park |

## Root cause

`init_body_pc`/`init_wrap_pc` 驱动 VM super-init 切片；损坏有界可进入 unify/codegen；无 fail-closed 闸。

## Delivery

1. `syx-mir::{verify_mir_init_pc_floor, verify_mir_init_pc_floor_functions}` + unit  
2. 挂钩 unify / codegen（aot-export 之后）  
3. 负向钉：`c3t_mir_init_pc_floor_contract`  
4. SSOT / `02`：C-3t `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 双 0 OK；否则 `body ≤ wrap ≤ instructions.len()`  
- ≠ 要求 `_init` 名后缀  
- ≠ 改 CallSuper / VM 切片语义  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| init-pc-floor API + dual hooks | **1** | **1** |
| MIR020 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
