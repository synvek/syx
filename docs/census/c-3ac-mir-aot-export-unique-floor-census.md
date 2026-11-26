# C-3ac MIR aot_export_roots uniqueness floor census

**Date:** 2026-11-26  
**Scope:** `verify_mir_aot_export_unique_floor` → **MIR029**（空表 OK；重复 root 名 fail-closed；≠ MIR019 解析）；挂钩 succ-unique 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3ac_mir_aot_export_unique_floor_census.sh`](../../scripts/c3ac_mir_aot_export_unique_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3z + C-3aa…C-3ab** | **exited** |
| **C-3ac** aot-export-unique floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · dense ids · succ==term · 改 pilot · 扩 MIR004 · local_names 空槽 · tls.server · R-50 park |

## Root cause

`lower_module` 已对 `aot_export_roots` dedupe；MIR019 只校名是否可解析，不校表内重复；手搓/`compile_from_mir` 仍可塞重复 root。

## Delivery

1. `syx-mir::verify_mir_aot_export_unique_floor` + unit  
2. 挂钩 unify / codegen（succ-unique 之后；模块级，对齐 C-3s）  
3. 负向钉：`c3ac_mir_aot_export_unique_floor_contract`  
4. SSOT / `02`：C-3ac `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = `aot_export_roots` 名唯一；空表 OK  
- ≠ 改写 MIR019 · ≠ export GA  
- ≠ 改 pilot / 扩 MIR004  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| aot-export-unique-floor API + dual hooks | **1** | **1** |
| MIR029 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
