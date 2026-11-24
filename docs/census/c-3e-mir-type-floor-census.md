# C-3e MIR type-tag floor census

**Date:** 2026-11-24  
**Scope:** `verify_mir_type_floor` → **MIR005**（suite1/pilot 粗 tag）；挂钩 `build_unified_mir` + `compile_from_mir_with_source`（stack floor **之后**）；Add 拼接合法；Nil/非数值 Sub·Mul·Div 拒。**不**宣称 C-3 `[x]`；默认下一 **C-3f**。  
**Script:** [`scripts/c3e_mir_type_floor_census.sh`](../../scripts/c3e_mir_type_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3d** | **exited** |
| **C-3e** type-tag floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-3f**；ModuleId · 全量 MirOp verifier · tls.server · macOS · R-50 park |

## Root cause

MIR 值栈无粗种类闸；mir-run 对 Nil 数值 op fail-closed（R003），对 Add 字符串拼接合法；编译期曾可放过 category 错配。

## Delivery

1. `syx-mir::{verify_mir_type_floor, verify_mir_type_floor_functions}` + unit tests  
2. 挂钩 unify / codegen（stack 之后；codegen 验去重后函数）  
3. Tag 规则对齐 mir-run；CallHost/非 pilot/非入口块放弃  
4. 负向钉：`c3e_mir_type_floor_contract`  
5. SSOT / `02`：C-3e `[x]`；C-3 `[~]`；默认下一 **C-3f**  
6. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| type floor API + dual hooks | **1** | **1** (`c3e_type_api` · `c3e_hooks`) |
| MIR005 neg pin | **pass** | **1** (`c3e_neg_cargo`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3e_mir_type_floor_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c3e_type_api=1
c3e_hooks=1
c3e_neg_pin=1
c3e_native_real_count=16
c3e_tls_server_partial=1
c3e_census_doc=1
c3e_ssot_updated=1
c3e_roadmap_ok=1
c3e_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3e_mir_type_floor_census_ok=1
current=C-3e-mir-type-floor
next=C-3f
```

## Non-claims

- ≠ C-3 `[x]` · ModuleId 图式 · `Call(FuncId)` IR 迁移
- ≠ locals 槽位类型 · 跨块 join φ · 全量 MirOp type/支配/exception verifier
- ≠ 去 hybrid AST · AOT compact 重写
- ≠ `net.tls.server` raise · macOS required full-native · R-50 · soft→hard
- ≠ JIT 扩面 · 改硬 KPI
- ≠ 改 mir-run Add 拼接 / Nil R003 语义

## Soft park

ModuleId · full MirOp verifier · tls.server · macOS full-native · R-50 · soft→hard · JIT expand
