# C-3ae MIR local_names empty-string-slot floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_local_names_empty_floor` → **MIR031**（空向量 OK；非空禁 `""`；≠ MIR018 / locals 类型语义）；挂钩 local-types-empty 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3ae_mir_local_names_empty_floor_census.sh`](../../scripts/c3ae_mir_local_names_empty_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa…C-3ad** | **exited** |
| **C-3ae** local-names-empty floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure/ToStr 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

C-3r / MIR018 只校非空 `local_names` 长度 == `local_count`；不禁槽内 `""`。lower 填 `local_{i}`；AOT 可整向量 clear。手搓/`compile_from_mir` 仍可塞空串槽。

## Delivery

1. `syx-mir::verify_mir_local_names_empty_floor` / `_functions` + unit  
2. 挂钩 unify / codegen（local-types-empty 之后；双形态对齐 C-3r/C-3ad）  
3. 负向钉：`c3ae_mir_local_names_empty_floor_contract`  
4. SSOT / `02`：C-3ae `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 非空 `local_names` 禁 `""`；空向量 OK  
- ≠ 改写 MIR018 · ≠ locals 类型/φ 语义  
- ≠ Dup/MakeClosure/ToStr 栈 · ≠ 改 pilot / 扩 MIR004  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| local-names-empty-floor API + dual hooks | **1** | **1** |
| MIR031 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
