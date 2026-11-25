# C-3u MIR MirBlock.id uniqueness floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_block_id_floor` → **MIR021**（空 blocks OK；重复 `MirBlock.id` fail-closed）；挂钩 init-pc 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3u_mir_block_id_floor_census.sh`](../../scripts/c3u_mir_block_id_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3t** | **exited** |
| **C-3u** block-id floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型·φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · flat↔blocks 条数 · dense ids · tls.server · R-50 park |

## Root cause

C-3b `IndexSet` **静默去重**重复 `block.id`；flatten/LLVM 按 id 覆盖；unify/codegen 无 fail-closed 闸。

## Delivery

1. `syx-mir::{verify_mir_block_id_floor, verify_mir_block_id_floor_functions}` + unit  
2. 挂钩 unify / codegen（init-pc 之后）  
3. 负向钉：`c3u_mir_block_id_floor_contract`  
4. SSOT / `02`：C-3u `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 空 blocks OK；否则 `IndexSet(ids).len() == blocks.len()`  
- ≠ dense `0..n-1`  
- ≠ 改 C-3b / flatten / LLVM  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| block-id-floor API + dual hooks | **1** | **1** |
| MIR021 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
