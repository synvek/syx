# C-3v MIR flat↔blocks instruction-count floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_flat_blocks_floor` → **MIR022**（任一侧空 OK；双非空须 `sum(block.instr)==instructions.len()`）；挂钩 block-id 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3v_mir_flat_blocks_floor_census.sh`](../../scripts/c3v_mir_flat_blocks_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3u** | **exited** |
| **C-3v** flat-blocks floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型·φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · 指令/跳转 PC 对齐 · tls.server · R-50 park |

## Root cause

Phase-98 双表示：`blocks` 与 flat `instructions` 可同时非空；lower/`resync` 对齐条数，手写/损坏 MIR 可不等；unify/codegen 无 fail-closed 闸。

## Delivery

1. `syx-mir::{verify_mir_flat_blocks_floor, verify_mir_flat_blocks_floor_functions}` + unit  
2. 挂钩 unify / codegen（block-id 之后）  
3. 负向钉：`c3v_mir_flat_blocks_floor_contract`  
4. SSOT / `02`：C-3v `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 任一侧空 OK；双非空 ⇒ 条数相等  
- ≠ 指令内容 / 跳转 PC 语义一致  
- ≠ 改 flatten / resync  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| flat-blocks-floor API + dual hooks | **1** | **1** |
| MIR022 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
