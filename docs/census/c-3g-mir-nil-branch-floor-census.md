# C-3g MIR JumpIfNil condition tag census

**Date:** 2026-11-24  
**Scope:** `verify_mir_nil_branch_floor` → **MIR007**（JumpIfNil 条件仅 `Nil|Unknown`；Nil pop / Unknown 留栈）；挂钩 branch floor **之后**。**不**宣称 C-3 `[x]`；默认下一 **C-4**。  
**Script:** [`scripts/c3g_mir_nil_branch_floor_census.sh`](../../scripts/c3g_mir_nil_branch_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3f** | **exited** |
| **C-3g** JumpIfNil tag | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-4**；ModuleId · 全量 MirOp · exception · tls.server · macOS · R-50 park |

## Root cause

JumpIfNil 非 suite1 pilot，C-3f branch floor 遇之放弃流；`if let` / `??` 条件无编译期粗 tag 闸。

## Delivery

1. `syx-mir::{verify_mir_nil_branch_floor, verify_mir_nil_branch_floor_functions}` + unit tests  
2. 挂钩 unify / codegen（branch 之后）  
3. mir-run 对齐：Nil pop；Unknown 留栈  
4. 负向钉：`c3g_mir_nil_branch_floor_contract`  
5. SSOT / `02`：C-3g `[x]`；C-3 `[~]`；默认下一 **C-4**  
6. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| nil-branch API + dual hooks | **1** | **1** (`c3g_nil_api` · `c3g_hooks`) |
| MIR007 neg pin | **pass** | **1** (`c3g_neg_cargo`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3g_mir_nil_branch_floor_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c3g_nil_api=1
c3g_hooks=1
c3g_neg_pin=1
c3g_native_real_count=16
c3g_tls_server_partial=1
c3g_census_doc=1
c3g_ssot_updated=1
c3g_roadmap_ok=1
c3g_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3g_mir_nil_branch_floor_census_ok=1
current=C-3g-mir-nil-branch-floor
next=C-4
```

## Non-claims

- ≠ C-3 `[x]` · ModuleId · `Call(FuncId)` IR
- ≠ BeginTry/Throw 配对 · 支配 · 跨块 φ
- ≠ 改 suite1 pilot 谓词 · mir-run JumpIfNil 语义
- ≠ tls.server raise · macOS · R-50 · JIT · 改硬 KPI

## Soft park

ModuleId · full MirOp verifier · exception · tls.server · macOS full-native · R-50 · soft→hard · JIT expand
