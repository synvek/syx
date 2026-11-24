# C-7a fuzz-signal honesty census

**Date:** 2026-11-24  
**Scope:** `syx_smoke` 经 `syx-lexer` 真消费；required CI `fuzz-smoke` `-runs=1`；UsageGuide CI met 与 yaml 对齐。**不**宣称 C-7 `[x]`；默认下一 **C-7b**。  
**Script:** [`scripts/c7a_fuzz_honesty_census.sh`](../../scripts/c7a_fuzz_honesty_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环 / 无界 fuzz。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5** | **`[~]`** |
| **C-7a** fuzz honesty | **exited** |
| **C-7** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-7b**；C-7c+ count-gate/scaffold · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

空壳 `syx_smoke` 忽略输入；UsageGuide/ADR 写 PR CI `-runs=1` met，但 `ci.yml` 无 `cargo fuzz`（仅 soft deferred notice）。

## Delivery

1. `fuzz/Cargo.toml` path 依赖 `syx-lexer`；target UTF-8→`scan_all`
2. Required job `fuzz-smoke`：`cargo +nightly fuzz run syx_smoke -- -runs=1`
3. UsageGuide en/zh + SSOT / `02`：C-7a `[x]`；C-7 `[~]`；下一 **C-7b**
4. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| syx_smoke consumes (lexer / no empty ignore-body) | **pass** | **1** (`c7a_source_consume`) |
| `ci.yml` fuzz-smoke + `-runs=1` | **pass** | **1** (`c7a_ci_pin`) |
| bounded local fuzz run (or cargo-fuzz skip note) | **pass** | **1** (`c7a_local_fuzz_run`) |
| UsageGuide CI met ↔ yaml | **pass** | **1** (`c7a_usageguide_aligned`) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c7a_fuzz_honesty_census_ok` | **1** | **1** |
| `c7_exited` | **0** | **0** |

## Results (2026-11-24)

```
c7a_source_consume=1
c7a_ci_pin=1
c7a_usageguide_aligned=1
c7a_local_fuzz_run=1
c7a_native_real_count=16
c7a_tls_server_partial=1
c7a_census_doc=1
c7a_ssot_updated=1
c7a_roadmap_ok=1
c7_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c7a_fuzz_honesty_census_ok=1
current=C-7a
next=C-7b
c7_exited=0
```

## Non-claims

- ≠ C-7 `[x]` · soft `exit 0` 清扫（C-7b）· count-gate/scaffold（C-7c+）
- ≠ 多 fuzz target · 无界 nightly · fuzz GA
- ≠ ModuleId · C-5c+ · tls.server · macOS · R-50 · 宣称 C-3/C-5 `[x]` · 开 C-8

## Soft park

C-7b soft-exit0 · C-7c+ count-gate/scaffold · ModuleId · C-5c+ · tls.server · macOS · R-50
