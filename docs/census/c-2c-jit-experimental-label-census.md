# C-2c JIT experimental label census

**Date:** 2026-11-24  
**Scope:** 公开面 JIT Stub /「default on」→ **experimental-real-subset · default off · non_default_kpi**。≠ JIT 扩面 / ≠ 硬 KPI / ≠ `native=real` 变更（仍 **16**）。默认下一 **C-2d**。  
**Script:** [`scripts/c2c_jit_experimental_label_census.sh`](../../scripts/c2c_jit_experimental_label_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2b-13** | **exited** |
| **C-2c** JIT label scrub | **exited** |
| Open | 默认下一 **C-2d**；tls.server · macOS · C-0a+/e park |

## Root cause

SSOT / capability 已标 JIT experimental，但 FeaturePriority §23 仍写 Stub，env-registry 写 `SYX_JIT_TIER2` default on，与 cap「default off」冲突。

## Delivery

1. FeaturePriority EN/ZH §23 → experimental-real-subset  
2. env-registry EN/ZH → default off  
3. AnalysisReport JIT Stub 行 scrub  
4. SSOT / `02`：C-2c `[x]`；默认下一 **C-2d**  
5. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| FeaturePriority §23 | experimental · no product Stub | **1** (`c2c_featurepriority_ok=1`) |
| env-registry `SYX_JIT_TIER2` | default off · 无 default on | **1** (`c2c_env_registry_ok=1`) |
| AnalysisReport | 无 Hot-path counter stub | **1** (`c2c_analysis_report_ok=1`) |
| `runtime.jit.tier2` | `experimental-real-subset` | **1** (`c2c_jit_cap_ok=1`) |
| CI job `jit` | `continue-on-error: true` | **1** (`c2c_jit_ci_soft=1`) |
| `native=real` count | **16** (unchanged) | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2c_jit_experimental_label_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c2c_featurepriority_ok=1
c2c_env_registry_ok=1
c2c_analysis_report_ok=1
c2c_jit_cap_ok=1
c2c_jit_ci_soft=1
c2c_native_real_count=16
c2c_tls_server_partial=1
c2c_census_doc=1
c2c_ssot_updated=1
c2c_roadmap_ok=1
c2c_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2c_jit_experimental_label_census_ok=1
current=C-2c-jit-experimental-label
next=C-2d
```

## Non-claims

- ≠ JIT Suite-1 扩面 / 默认开 JIT / 取消 soft CI
- ≠ `net.tls.server` 升格 · macOS required full-native · C-2d job rename · R-50
- ≠ 抬高或降低产品 `native=real` 计数

## References

- [`c-2b-13-tls-client-raise-census`](c-2b-13-tls-client-raise-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2c → C-2d
