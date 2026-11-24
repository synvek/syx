# C-3d MIR stack-height floor census

**Date:** 2026-11-24  
**Scope:** `verify_mir_stack_floor` → **MIR004**（suite1/pilot 子集）；挂钩 `build_unified_mir` + `compile_from_mir_with_source`（CFG + FnIndex **之后**）；CallHost/非 pilot 放弃该流；Return 空栈 = mir-run Nil pad。**不**宣称 C-3 `[x]`；默认下一 **C-3e**。  
**Script:** [`scripts/c3d_mir_stack_floor_census.sh`](../../scripts/c3d_mir_stack_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3c** | **exited** |
| **C-3d** stack-height floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-3e**；ModuleId · 全量 MirOp verifier · tls.server · macOS · R-50 park |

## Root cause

mir-run 对缺参用 `unwrap_or(Nil)` 垫栈；无 MIR 值栈有界高度闸；Wasm `stack_effect` ≠ MIR 值栈。

## Delivery

1. `syx-mir::{verify_mir_stack_floor, verify_mir_stack_floor_functions}` + unit tests  
2. 挂钩 unify / codegen（FnIndex 之后；codegen 验去重后函数）  
3. Pilot Δ（mir-run SSOT；Print Δ=0）；CallHost/非 pilot 放弃流；Return 空 = Nil pad；仅验 **entry block**（无跨块 join）  
4. 负向钉：`c3d_mir_stack_floor_contract`  
5. SSOT / `02`：C-3d `[x]`；C-3 `[~]`；默认下一 **C-3e**  
6. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| stack floor API + dual hooks | **1** | **1** (`c3d_stack_api` · `c3d_hooks`) |
| MIR004 neg pin | **pass** | **1** (`c3d_neg_cargo`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3d_mir_stack_floor_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c3d_stack_api=1
c3d_hooks=1
c3d_neg_pin=1
c3d_native_real_count=16
c3d_tls_server_partial=1
c3d_census_doc=1
c3d_ssot_updated=1
c3d_roadmap_ok=1
c3d_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3d_mir_stack_floor_census_ok=1
current=C-3d-mir-stack-floor
next=C-3e
```

## Non-claims

- ≠ C-3 `[x]` · ModuleId 图式 · `Call(FuncId)` IR 迁移
- ≠ 跨块 join φ · 全量 MirOp type/支配/exception verifier
- ≠ 去 hybrid AST · AOT compact 重写
- ≠ `net.tls.server` raise · macOS required full-native · R-50 · soft→hard
- ≠ JIT 扩面 · 改硬 KPI
- ≠ mir-run/VM Print 语义强对齐（本窗钉 MIR 表）

## Soft park

ModuleId · full MirOp verifier · tls.server · macOS full-native · R-50 · soft→hard · JIT expand
