# C-3aa MIR module_names empty-string floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_module_names_floor` → **MIR027**（空表 OK；条目 `""` fail-closed；≠ MIR008 / Call(ModuleId)）；挂钩 block-instr 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3aa_mir_module_names_floor_census.sh`](../../scripts/c3aa_mir_module_names_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z** | **exited** |
| **C-3aa** module-names floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · dup successors · tls.server · R-50 park |

## Root cause

`collect_module_names_from_paths` 已 skip 空 stem，但手搓/`MirModIndex::build` 仍可接受 `module_names` 含 `""` 为毒 ModuleId(0)；无 fail-closed 闸。

## Delivery

1. `syx-mir::verify_mir_module_names_floor` + unit  
2. 挂钩 unify / codegen（block-instr 之后；模块级，对齐 C-3s）  
3. 负向钉：`c3aa_mir_module_names_floor_contract`  
4. SSOT / `02`：C-3aa `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = `module_names` 表项非空；空表 OK  
- ≠ 改 MIR008 去重 · ≠ Call(ModuleId) IR · ≠ dense ModuleId  
- ≠ 改 pilot / 扩 MIR004  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| module-names-floor API + dual hooks | **1** | **1** |
| MIR027 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
