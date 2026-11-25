# C-7b soft-step fail-closed census

**Date:** 2026-11-24  
**Scope:** soft job 内吞错 `exit 0` → 传播真实 exit；job 仍 `continue-on-error`；白名单三处保留。**不**宣称 C-7 `[x]`；默认下一 **C-7c**。  
**Script:** [`scripts/c7b_soft_exit0_census.sh`](../../scripts/c7b_soft_exit0_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5** | **`[~]`** |
| **C-7a…C-7b** | **exited** |
| **C-7** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-7c**；scaffold/多 fuzz · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

Soft job 虽 `continue-on-error`，step 失败后仍 `exit 0` → Actions UI/注解几乎永不红，软墙无信号。

## Delivery

1. 翻 gui / jit / extended / native host-shake+differential / browser-e2e / windows plugin fail / book rate≠100
2. 白名单：空 plugin · 无 book JSON · domain defer notice
3. SSOT / `02`：C-7b `[x]`；C-7 `[~]`；下一 **C-7c**
4. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| swallow sites fail-closed | **pass** | **1** (`c7b_failclosed_sites`) |
| whitelist three `exit 0` kept | **pass** | **1** (`c7b_whitelist_ok`) |
| soft jobs still `continue-on-error` | **pass** | **1** (count=9) |
| `native=real` count | **16** | **16** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c7b_soft_exit0_census_ok` | **1** | **1** |
| `c7_exited` | **0** | **0** |

## Results (2026-11-24)

```
c7b_continue_on_error_count=9
c7b_failclosed_sites=1
c7b_whitelist_ok=1
c7b_jobs_still_soft=1
c7b_native_real_count=16
c7b_tls_server_partial=1
c7b_census_doc=1
c7b_ssot_updated=1
c7b_roadmap_ok=1
c7_exited=0
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c7b_soft_exit0_census_ok=1
current=C-7b
next=C-7c
c7_exited=0
```

## Non-claims

- ≠ C-7 `[x]` · soft→required · count-gate 三套对齐（C-7c）
- ≠ Farm/CDN 外移 · 多 fuzz · ModuleId · C-5c+ · tls.server · macOS · R-50
- ≠ 宣称 C-3/C-5 `[x]` · 开 C-8

## Soft park

C-7c count-gate · scaffold/多 fuzz · ModuleId · C-5c+ · tls.server · macOS · R-50
