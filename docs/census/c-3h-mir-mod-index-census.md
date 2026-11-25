# C-3h MIR ModuleId thin gate census

**Date:** 2026-11-25  
**Scope:** `ModuleId` + `MirModIndex` → **MIR008**（重名 / 缺名 / stem 撞路径）；`MirModule.module_names`；挂钩 nil-branch **之后**。**不**宣称 C-3 `[x]`；默认下一 **C-5c+**。  
**Script:** [`scripts/c3h_mir_mod_index_census.sh`](../../scripts/c3h_mir_mod_index_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3g** | **exited** |
| **C-3h** ModuleId | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5c+**；全量 MirOp · exception · DefId · Call(ModuleId) IR · tls.server · macOS · R-50 park |

## Root cause

合并 MIR 仅以 `PathBuf`/stem 作身份；无稠密 `ModuleId` 表；同 stem 多路径无 fail-closed。

## Delivery

1. `syx-mir::{ModuleId, MirModIndex, collect_module_names_from_paths}` + unit tests  
2. `MirModule.module_names`（`#[serde(default)]`；空表合法）  
3. 挂钩 unify（populate + build）/ codegen（build；空 OK）  
4. 负向钉：`c3h_mir_mod_index_contract`  
5. SSOT / `02`：C-3h `[x]`；C-3 `[~]`；默认下一 **C-5c+**  
6. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| mod-index API + dual hooks | **1** | **1** (`c3h_mod_api` · `c3h_hooks`) |
| MIR008 neg pin | **pass** | **1** (`c3h_neg_cargo`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3h_mir_mod_index_census_ok` | **1** | **1** |

## Results (2026-11-25)

```
c3h_mod_api=1
c3h_hooks=1
c3h_neg_pin=1
c3h_native_real_count=16
c3h_tls_server_partial=1
c3h_census_doc=1
c3h_ssot_updated=1
c3h_roadmap_ok=1
c3h_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3h_mir_mod_index_census_ok=1
current=C-3h-mir-mod-index
next=C-5c+
```

## Non-claims

- ≠ C-3 `[x]` · DefId · `Call(ModuleId)` IR · 全量 MirOp
- ≠ 改 AST `Module.name=="main"` 当 identity
- ≠ BeginTry/Throw · exception · CST lossless · CallSuper
- ≠ tls.server raise · macOS · R-50 · JIT · 改硬 KPI

## Soft park

全量 MirOp · exception · DefId · Call(ModuleId) IR · CST lossless · CallSuper · C-7c+ · C-5c+（下一）· tls.server · macOS · R-50
