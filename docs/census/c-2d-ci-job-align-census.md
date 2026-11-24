# C-2d ci.job.* surface align census

**Date:** 2026-11-24  
**Scope:** 8× `ci.job.*` `surface`/`notes` 对齐真实 yaml job id；**不**改 yaml / soft / capability `id`。C-2 轴收口；默认下一 **C-3**。  
**Script:** [`scripts/c2d_ci_job_align_census.sh`](../../scripts/c2d_ci_job_align_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-2a…C-2c** | **exited** |
| **C-2d** ci.job surface align | **exited** |
| **C-2** | **`[x]` closed** |
| Open | 默认下一 **C-3**；tls.server · macOS · C-0a+/e · R-37 fossil scripts park |

## Root cause

R-37 化石 surface（`native-strict-main` / `parity-gate` / `jit-tier2-default` / `phase125-*` / `book-pass`）与当前 `ci.yml` job 名脱节；SSOT 已要求 surface 对齐 yaml。

## Delivery

1. 8× `ci.job.*` surface → live yaml ids（`native-linux` · `native-macos` · `extended-suites` · `jit` · `gui-xvfb` · `windows-smoke` · `book-doctest`）  
2. `gen_capabilities.py`  
3. SSOT / `02`：C-2d `[x]`；C-2 `[x]`；默认下一 **C-3**  
4. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `ci.job.*` surface aligned | **8/8** | **8** (`c2d_ci_job_surface_aligned=8`) |
| soft jobs still soft | `continue-on-error: true` | **1** |
| `full-native-linux` | hard (no job soft) | **1** |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2d_ci_job_align_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c2d_ci_job_surface_aligned=8
c2d_soft_jobs_ok=1
c2d_full_native_hard=1
c2d_native_real_count=16
c2d_tls_server_partial=1
c2d_census_doc=1
c2d_ssot_updated=1
c2d_roadmap_ok=1
c2d_no_mirror=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c2d_ci_job_align_census_ok=1
current=C-2d-ci-job-align
next=C-3
```

## Non-claims

- ≠ yaml job rename · soft→hard · capability `id` rename
- ≠ `net.tls.server` raise · macOS required full-native · R-50
- ≠ 修复历史 **R-37 / R-39** fossil census scripts（park）
- ≠ 新增 `ci.job.browser-e2e` orphan 行

## References

- [`c-2c-jit-experimental-label-census`](c-2c-jit-experimental-label-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2d → C-3
