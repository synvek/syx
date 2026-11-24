# C-3f MIR JumpIfFalse condition tag census

**Date:** 2026-11-24  
**Scope:** `verify_mir_branch_floor` → **MIR006**（JumpIfFalse 条件仅 `Bool|Unknown`）；比较/逻辑作 Bool 生产者；挂钩 type floor **之后**。**不**宣称 C-3 `[x]`；默认下一 **C-3g**。  
**Script:** [`scripts/c3f_mir_branch_floor_census.sh`](../../scripts/c3f_mir_branch_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3e** | **exited** |
| **C-3f** JumpIfFalse tag | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-3g**；ModuleId · 全量 MirOp verifier · tls.server · macOS · R-50 park |

## Root cause

JumpIfFalse 条件无编译期粗 tag 闸；mir-run `as_bool` 宽化掩盖 concrete I32/Str/Nil；比较 op 非 suite1 pilot 会导致上游 type floor 放弃流。

## Delivery

1. `syx-mir::{verify_mir_branch_floor, verify_mir_branch_floor_functions}` + unit tests  
2. 挂钩 unify / codegen（type 之后）  
3. Cmp/logic Bool 生产者扩展（≠ 改 suite1 谓词）  
4. 负向钉：`c3f_mir_branch_floor_contract`  
5. SSOT / `02`：C-3f `[x]`；C-3 `[~]`；默认下一 **C-3g**  
6. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| branch floor API + dual hooks | **1** | **1** (`c3f_branch_api` · `c3f_hooks`) |
| MIR006 neg pin | **pass** | **1** (`c3f_neg_cargo`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3f_mir_branch_floor_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c3f_branch_api=1
c3f_hooks=1
c3f_neg_pin=1
c3f_native_real_count=16
c3f_tls_server_partial=1
c3f_census_doc=1
c3f_ssot_updated=1
c3f_roadmap_ok=1
c3f_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3f_mir_branch_floor_census_ok=1
current=C-3f-mir-branch-floor
next=C-3g
```

## Non-claims

- ≠ C-3 `[x]` · ModuleId · `Call(FuncId)` IR
- ≠ JumpIfNil / 支配 / exception 配对 / 跨块 φ
- ≠ 改 `is_suite1_pilot_mir_op` 全集 · mir-run `as_bool`
- ≠ tls.server raise · macOS · R-50 · JIT · 改硬 KPI

## Soft park

ModuleId · full MirOp verifier · tls.server · macOS full-native · R-50 · soft→hard · JIT expand
