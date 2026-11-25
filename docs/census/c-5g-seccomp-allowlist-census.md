# C-5g Linux seccomp allowlist thin census

**Date:** 2026-11-25  
**Scope:** Linux opt-in **default-deny** seccomp allowlist（v11 base + minimal runnable supplements）；与 `SYX_*_SECCOMP` 互斥；非 Linux fail-closed。**不**宣称 C-5 `[x]`；默认下一 **C-5h**。  
**Script:** [`scripts/c5g_seccomp_allowlist_census.sh`](../../scripts/c5g_seccomp_allowlist_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)

## Progress

| Track | Status |
|-------|--------|
| **C-5a…C-5f** | **exited** |
| **C-5g** allowlist | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5h**；permission-sandbox default-on · 真全量 syscall GA · tls.server · R-50 park |

## Root cause

现网仅 C-5c-2 **default-allow net-deny**；`embed_v11_seccomp_allowlist` 仅为探针字符串、未接 BPF。

## Delivery

1. `try_install_seccomp_allowlist`（default-deny BPF）  
2. macro/embed hooks + mutex with SECCOMP  
3. evergreen `c5g_*`  
4. ADR / env-registry / UsageGuide / QuickRef  
5. SSOT / `02`：C-5g `[x]`；下一 **C-5h**；C-5 `[~]`  
6. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| allowlist API + hooks | **1** | **1** |
| evergreen nails | **pass** | **1** |
| `native=real` | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180/10** | **180/10** |
| simd residual | **≥159** | **159** |
| `c5g_seccomp_allowlist_census_ok` | **1** | **1** |

## Results (2026-11-25)

```
c5g_api=1
c5g_hooks=1
c5g_neg_pin=1
c5g_native_real_count=16
c5g_tls_server_partial=1
c5g_census_doc=1
c5g_ssot_updated=1
c5g_roadmap_ok=1
c5g_macro_unit=1
c5g_evergreen=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5g_seccomp_allowlist_census_ok=1
current=C-5g-seccomp-allowlist
next=C-5h
```

## Non-claims

- ≠ C-5 `[x]` · full syscall GA · permission-sandbox default-on  
- ≠ allowlist 下全量 macro/embed 负载必绿

## Soft park

**C-5h** permission-sandbox default-on · 真全量 GA · MirOp · CST lossless · CallSuper · C-7c+ · tls.server · R-50
