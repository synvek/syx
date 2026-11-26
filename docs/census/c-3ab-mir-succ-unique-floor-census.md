# C-3ab MIR successors uniqueness floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_succ_unique_floor` → **MIR028**（块内 `successors` 无重复目标；空 OK；≠ succ==term）；挂钩 module-names 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3ab_mir_succ_unique_floor_census.sh`](../../scripts/c3ab_mir_succ_unique_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa** | **exited** |
| **C-3ab** succ-unique floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · local_names 空槽 · tls.server · R-50 park |

## Root cause

`LowerCtx::link` 已 `contains` 去重，合法 lower 不产重复边；手搓/`compile_from_mir` 仍可 `vec![n,n]`；MIR023 只校 Jump* ⊇，不禁列表内重复。

## Delivery

1. `syx-mir::{verify_mir_succ_unique_floor, verify_mir_succ_unique_floor_functions}` + unit  
2. 挂钩 unify / codegen（module-names 之后）  
3. 负向钉：`c3ab_mir_succ_unique_floor_contract`  
4. SSOT / `02`：C-3ab `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 块内 `successors` 目标唯一；空 blocks / 空 successors OK  
- ≠ succ==term · ≠ 改 MIR023  
- ≠ 改 pilot / 扩 MIR004  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| succ-unique-floor API + dual hooks | **1** | **1** |
| MIR028 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
