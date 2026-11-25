# C-3o MIR fn-index literal OOB floor census

**Date:** 2026-11-25  
**Scope:** `verify_mir_fn_lit_floor` → **MIR015**（`CallWitness` / `AwaitSuspend.resume_fn` vs `functions.len()`）；挂钩 param-local 地板 **之后**。≠ ConstFnIndex R-33；≠ CallIndirect arity。**不**宣称 C-3 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c3o_mir_fn_lit_floor_census.sh`](../../scripts/c3o_mir_fn_lit_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3n** | **exited** |
| **C-3o** fn-lit floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（全量 MirOp 余片 / P0 C-0a+/C-0e）；Call(ModuleId) · HostFn arity · trim remap 验 · global 名表 · locals 类型/φ · ConstFnIndex fail-closed（R-33） · CallIndirect 语义 · 完整 DefId · 完整 exception · tls.server · R-50 park |

## Root cause

`CallWitness` / `AwaitSuspend.resume_fn` 为 fn-index 字面量（`mir_aot` remap）；无 MIR fail-closed 槽界；VM 对 CallWitness OOB 仅迟至 R003。

## Delivery

1. `syx-mir::{verify_mir_fn_lit_floor, verify_mir_fn_lit_floor_functions}` + unit  
2. 挂钩 unify / codegen（param-local 之后；codegen 传入 `mir.functions.len()`）  
3. 负向钉：`c3o_mir_fn_lit_floor_contract`  
4. SSOT / `02`：C-3o `[x]`；C-3 `[~]`；下一 **跨轴**  
5. census + 硬 KPI  

## Honesty

- 界 = `CallWitness(a,b)` / `AwaitSuspend.resume_fn` `< functions.len()`  
- ≠ ConstFnIndex fail-closed（R-33 `-1` 哨兵保留）  
- ≠ CallIndirect（lower 里 arity）  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| fn-lit-floor API + dual hooks | **1** | **1** |
| MIR015 neg pin | **pass** | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-3 `[x]` / 全量 MirOp / Call(ModuleId) IR  
- ≠ C-5 `[x]` / 真全量 syscall GA  
- ≠ 关 `is_compatible(_, Unknown)` / C-0a+  
