# C-3s MIR aot_export_roots name-resolve floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_aot_export_floor` → **MIR019**（空 OK；非空名须 ∈ `functions[].name`）；挂钩 local-debug 地板 **之后**。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3s_mir_aot_export_floor_census.sh`](../../scripts/c3s_mir_aot_export_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3r** | **exited** |
| **C-3s** aot-export floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型·φ · ConstFnIndex（R-33） · CallIndirect · 跨块 φ · Dup/MakeClosure 栈 · 完整 DefId · 完整 exception · tls.server · R-50 · export GA park |

## Root cause

`aot_export_roots` 驱动 AOT multi-root shake；未知 root 今日在 `reachable_fn_names_from_roots` **静默丢弃**；unify/codegen 无 fail-closed 闸。

## Delivery

1. `syx-mir::verify_mir_aot_export_floor` + unit  
2. 挂钩 unify / codegen（local-debug 之后）  
3. 负向钉：`c3s_mir_aot_export_floor_contract`  
4. SSOT / `02`：C-3s `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = 非空 root 名 ∈ `functions[].name`；空 = main-only OK  
- ≠ 改 AOT shake 静默语义  
- ≠ export GA / auto-discovery  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| aot-export-floor API + dual hooks | **1** | **1** |
| MIR019 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
