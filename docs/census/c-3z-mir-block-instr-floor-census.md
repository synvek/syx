# C-3z MIR block-instruction floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_block_instr_floor` → **MIR026**（非空 `blocks` 每块 ≥1 指令；空 `blocks` OK；补 MIR002 空指令块跳过 terminator 洞）；挂钩 fn-name 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3z_mir_block_instr_floor_census.sh`](../../scripts/c3z_mir_block_instr_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3y** | **exited** |
| **C-3z** block-instr floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · `module_names` 空串 · tls.server · R-50 park |

## Root cause

MIR002（C-3b）仅在 `instructions.last()` 存在时查 terminator；空指令块直接跳过 → 无 terminator 仍过 CFG 地板。手搓/`compile_from_mir` 可塞进空指令块。

## Delivery

1. `syx-mir::{verify_mir_block_instr_floor, verify_mir_block_instr_floor_functions}` + unit  
2. 挂钩 unify / codegen（fn-name 之后）  
3. 负向钉：`c3z_mir_block_instr_floor_contract`  
4. SSOT / `02`：C-3z `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 非空 `blocks` ⇒ 每块 `instructions.len() ≥ 1`；空 `blocks` OK  
- ≠ 改写 MIR002 · ≠ succ==term · ≠ dense ids  
- ≠ 改 pilot / 扩 MIR004  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| block-instr-floor API + dual hooks | **1** | **1** |
| MIR026 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
