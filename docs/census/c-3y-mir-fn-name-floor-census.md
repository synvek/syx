# C-3y MIR function-name floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_fn_name_floor` → **MIR025**（`functions[].name` 非空；空模块 OK；≠ `source_name`）；挂钩 pop-stack 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3y_mir_fn_name_floor_census.sh`](../../scripts/c3y_mir_fn_name_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3x** | **exited** |
| **C-3y** fn-name floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · tls.server · R-50 park |

## Root cause

`MirFunction.name` 可被手搓/坏序列化为 `""`；C-3s 只校 aot_export_roots 对已有名的解析，不校名本身非空；空名可进入 codegen/索引路径。

## Delivery

1. `syx-mir::{verify_mir_fn_name_floor, verify_mir_fn_name_floor_functions}` + unit  
2. 挂钩 unify / codegen（pop-stack 之后）  
3. 负向钉：`c3y_mir_fn_name_floor_contract`  
4. SSOT / `02`：C-3y `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 每个 `functions[i].name` 非空；空模块 OK  
- ≠ `FunctionUnit.source_name`  
- ≠ Dup/MakeClosure 栈 / 改 pilot / 扩 MIR004  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| fn-name-floor API + dual hooks | **1** | **1** |
| MIR025 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
