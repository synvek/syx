# C-3b MIR CFG / terminator floor census

**Date:** 2026-11-24  
**Scope:** `verify_mir_cfg_floor` → **MIR002** fail-closed（entry / OOB jump·successor / missing terminator）；挂钩 `build_unified_mir` + `compile_from_mir_with_source`。**不**宣称 C-3 `[x]`；默认下一 **C-3c**。  
**Script:** [`scripts/c3b_mir_cfg_floor_census.sh`](../../scripts/c3b_mir_cfg_floor_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-3a** | **exited** |
| **C-3b** CFG/terminator floor | **exited** |
| **C-3** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-3c**；全量 MirOp verifier · tls.server · macOS · R-50 park |

## Root cause

统一 MIR / 直喂 codegen 路径此前无 CFG 有界闸；非法 entry、越界 Jump、非空块缺 terminator 可落到后端。LLVM/opt 仅消费或裁剪，不 fail-closed。

## Delivery

1. `syx-mir::verify_mir_cfg_floor` + unit tests  
2. 挂钩 `build_unified_mir` 与 `compile_from_mir_with_source`  
3. 负向钉：`c3b_mir_cfg_floor_contract`  
4. SSOT / `02`：C-3b `[x]`；C-3 `[~]`；默认下一 **C-3c**  
5. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| verify API + dual hooks | **1** | **1** (`c3b_verify_api` · `c3b_hooks`) |
| MIR002 neg pin | **pass** | **1** (`c3b_neg_cargo`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c3b_mir_cfg_floor_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c3b_verify_api=1
c3b_hooks=1
c3b_neg_pin=1
c3b_native_real_count=16
c3b_tls_server_partial=1
c3b_census_doc=1
c3b_ssot_updated=1
c3b_roadmap_ok=1
c3b_neg_cargo=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c3b_mir_cfg_floor_census_ok=1
current=C-3b-mir-cfg-floor
next=C-3c
```

## Non-claims

- ≠ C-3 `[x]` · DefId · 全量 MirOp stack/type/支配/exception verifier
- ≠ 去 hybrid AST · 改后端指令语义
- ≠ `net.tls.server` raise · macOS required full-native · R-50 · soft→hard
- ≠ JIT 扩面 · 改硬 KPI

## References

- [`c-3a-mir-quality-demote-census`](c-3a-mir-quality-demote-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-3b → C-3c
