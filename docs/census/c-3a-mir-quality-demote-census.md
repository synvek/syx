# C-3a MIR quality heuristic demote census

**Date:** 2026-11-24  
**Scope:** 禁止 `syx-codegen` 对同名 MIR body 静默 quality 选胜 → **MIR001** fail-closed；同名 overload 上游 arity 唯一化（`__a{N}`）。**不**宣称 C-3 `[x]`；默认下一 **C-3b**。  
**Script:** [`scripts/c3a_mir_quality_demote_census.sh`](../../scripts/c3a_mir_quality_demote_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2** | **`[x]` closed** |
| **C-3a** quality fail-closed | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-3b**；DefId / 全量 verifier · tls.server · macOS · R-50 park |

## Root cause

`compile_from_mir_with_source` / hybrid `best_mir` 曾用 `mir_function_quality` 对同名 body **静默高分覆盖**，掩盖 overload 名碰撞（如 `SyxHost.task_detach` 1/2 参）。

## Delivery

1. Codegen：`unique_mir_functions_by_name` — 同 fingerprint 保首次；冲突 → MIR001  
2. MIR：同类型同 staticness overload → `{Type}_{name}__a{arity}`；call 侧解析；stdlib 预广告 callable  
3. 负向钉：`c3a_mir_quality_demote_contract`  
4. SSOT / `02`：C-3a `[x]`；C-3 `[~]`；默认下一 **C-3b**  
5. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| no silent quality pick in select loops | **1** | **1** (`c3a_no_silent_quality_pick`) |
| MIR001 neg pin | **pass** | **1** (`c3a_neg_cargo`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3a_mir_quality_demote_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c3a_no_silent_quality_pick=1
c3a_neg_pin=1
c3a_arity_uniquify=1
c3a_native_real_count=16
c3a_tls_server_partial=1
c3a_census_doc=1
c3a_ssot_updated=1
c3a_roadmap_ok=1
c3a_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3a_mir_quality_demote_census_ok=1
current=C-3a-mir-quality-demote
next=C-3b
```

## Non-claims

- ≠ C-3 `[x]` · DefId · 全量 MirOp verifier · 去 hybrid AST
- ≠ `net.tls.server` raise · macOS required full-native · R-50 · soft→hard
- ≠ JIT 扩面 · 改硬 KPI

## References

- [`c-2d-ci-job-align-census`](c-2d-ci-job-align-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-3a → C-3b
