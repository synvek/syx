# C-3c MirFnIndex / FuncId census

**Date:** 2026-11-24  
**Scope:** `FuncId` + `MirFnIndex::build/require` → **MIR003**；挂钩 `build_unified_mir` + `compile_from_mir_with_source`（在 CFG floor 之后）；Call 缺名 fail-closed。**不**宣称 C-3 `[x]`；默认下一 **C-3d**。  
**Script:** [`scripts/c3c_mir_fn_index_census.sh`](../../scripts/c3c_mir_fn_index_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a…C-3b** | **exited** |
| **C-3c** FuncId / MirFnIndex | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-3d**；ModuleId · 全量 MirOp verifier · tls.server · macOS · R-50 park |

## Root cause

Call 身份仅为 `SmolStr` 线性查找；缺名曾可静默绑 fn 0；无稠密 name→id 表。

## Delivery

1. `syx-mir::{FuncId, MirFnIndex}` + unit tests  
2. 挂钩 unify / codegen（verify 之后 build）  
3. `resolve_fn_index` → MIR003（Call 路径）；ConstFnIndex 保留 R-33 `-1` 哨兵（≠ fn 0）  
4. 负向钉：`c3c_mir_fn_index_contract`  
5. SSOT / `02`：C-3c `[x]`；C-3 `[~]`；默认下一 **C-3d**  
6. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| FuncId + MirFnIndex + dual hooks | **1** | **1** (`c3c_fn_index_api` · `c3c_hooks`) |
| MIR003 neg pin | **pass** | **1** (`c3c_neg_cargo`) |
| `resolve_fn_index` no fn-0 silent | **1** | **1** (`c3c_resolve_no_fn0`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3c_mir_fn_index_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c3c_fn_index_api=1
c3c_hooks=1
c3c_resolve_no_fn0=1
c3c_neg_pin=1
c3c_native_real_count=16
c3c_tls_server_partial=1
c3c_census_doc=1
c3c_ssot_updated=1
c3c_roadmap_ok=1
c3c_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3c_mir_fn_index_census_ok=1
current=C-3c-mir-fn-index
next=C-3d
```

## Non-claims

- ≠ C-3 `[x]` · ModuleId 图式 · `Call(FuncId)` IR 迁移
- ≠ 全量 MirOp stack/type/支配/exception verifier
- ≠ 去 hybrid AST · AOT compact 重写
- ≠ `net.tls.server` raise · macOS required full-native · R-50 · soft→hard
- ≠ JIT 扩面 · 改硬 KPI

## References

- [`c-3b-mir-cfg-floor-census`](c-3b-mir-cfg-floor-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-3c → C-3d
