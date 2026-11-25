# C-5d embed/macro RSS hard wall census

**Date:** 2026-11-25  
**Scope:** Linux opt-in `prlimit(RLIMIT_AS)`（embed worker / wasmtime child / macro child）；MEMORY env∪CLI；非 Linux fail-closed。**不**宣称 C-5 `[x]`；默认下一 **C-5e**。  
**Script:** [`scripts/c5d_embed_rss_hard_census.sh`](../../scripts/c5d_embed_rss_hard_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-5a…C-5c-2** | **exited** |
| **C-5d** RSS hard | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5e**；Job Object · 全量 filter · permission-sandbox default-on · tls.server · macOS · R-50 park |

## Root cause

MEMORY=stdout 事后软拒；RSS=Linux `/proc` 事后软查 / macOS None；CLI `memory_mb` 未读；registry 无 `SYX_EMBED_RSS_MB`；wasmtime 无闸。

## Delivery

1. `syx-macro::try_set_as_limit_bytes` / `_for_pid`（`prlimit(RLIMIT_AS)`）  
2. macro child install + 非 Linux → M040  
3. embed worker + wasmtime child pid；非 Linux → null  
4. MEMORY env∪CLI；stdout 事后闸诚实  
5. evergreen `c5d_*` nails  
6. ADR / env-registry / UsageGuide  
7. SSOT / `02`：C-5d `[x]`；下一 **C-5e**；C-5 `[~]`  
8. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| rlimit API + dual hooks | **1** | **1** |
| evergreen nails | **pass** | **1** |
| `native=real` | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180/10** | **180/10** |
| simd residual | **≥159** | **159** |
| `c5d_embed_rss_hard_census_ok` | **1** | **1** |

## Results (2026-11-25)

```
c5d_api=1
c5d_hooks=1
c5d_neg_pin=1
c5d_native_real_count=16
c5d_tls_server_partial=1
c5d_census_doc=1
c5d_ssot_updated=1
c5d_roadmap_ok=1
c5d_macro_unit=1
c5d_evergreen=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5d_embed_rss_hard_census_ok=1
current=C-5d-embed-rss-hard
next=C-5e
```

## Non-claims

- ≠ C-5 `[x]` · Job Object · 全量 syscall GA · 真堆计量  
- ≠ permission-sandbox default-on · tls.server · macOS · R-50

## Soft park

**C-5e** Job Object · 全量 filter · MirOp · CST lossless · CallSuper · C-7c+ · tls.server · macOS · R-50
