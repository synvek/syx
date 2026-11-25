# C-3w MIR successor⊇terminator jump-target floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_succ_term_floor` → **MIR023**（terminator `Jump*` 目标须 ∈ `successors`；`Return`/`Throw` 多余边 OK）；挂钩 flat-blocks 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3w_mir_succ_term_floor_census.sh`](../../scripts/c3w_mir_succ_term_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3v** | **exited** |
| **C-3w** succ-term floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型·φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · Pop 栈 · 指令/PC 对齐 · tls.server · R-50 park |

## Root cause

C-3b 仅闸 OOB；terminator `Jump*` 目标可不在 `successors`；unify/codegen 无 fail-closed 闸。

## Delivery

1. `syx-mir::{verify_mir_succ_term_floor, verify_mir_succ_term_floor_functions}` + unit  
2. 挂钩 unify / codegen（flat-blocks 之后）  
3. 负向钉：`c3w_mir_succ_term_floor_contract`  
4. SSOT / `02`：C-3w `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = Jump/JumpIfFalse/JumpIfNil 目标 ∈ successors；Return/Throw 不要求空 successors  
- ≠ succ == term 集合相等  
- ≠ mid-block BeginTry 目标 ∈ 本块 successors  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| succ-term-floor API + dual hooks | **1** | **1** |
| MIR023 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
