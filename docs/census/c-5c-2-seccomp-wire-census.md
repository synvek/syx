# C-5c-2 Linux seccomp net-deny wire census

**Date:** 2026-11-25  
**Scope:** Linux opt-in net-deny seccomp-BPF（macro child + embed worker）；非 Linux fail-closed。**不**宣称 C-5 `[x]`；默认下一 **C-5d**。  
**Script:** [`scripts/c5c2_seccomp_wire_census.sh`](../../scripts/c5c2_seccomp_wire_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-5a…C-5c-1** | **exited** |
| **C-5c-2** seccomp | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5d**；Job Object · 全量 filter · permission-sandbox default-on · tls.server · macOS · R-50 park |

## Root cause

`SYX_*_SECCOMP` 仅 probe/`seccomp-label-unwired`；`embed_seccomp_mvp_enabled` 无挂钩；无 BPF 安装。

## Delivery

1. `syx-macro::try_install_seccomp_netdeny`（libc sock_filter；deny socket/connect/bind/listen）  
2. macro child install + mode `seccomp-netdeny-wired`；非 Linux → M040  
3. embed worker install；非 Linux → null  
4. evergreen `c5c2_*` nails  
5. ADR / env-registry / UsageGuide  
6. SSOT / `02`：C-5c-2 `[x]`；下一 **C-5d**；C-5 `[~]`  
7. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| filter API + dual hooks | **1** | **1** |
| evergreen nails | **pass** | **1** |
| `native=real` | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180/10** | **180/10** |
| simd residual | **≥159** | **159** |
| `c5c2_seccomp_wire_census_ok` | **1** | **1** |

## Results (2026-11-25)

```
c5c2_api=1
c5c2_hooks=1
c5c2_neg_pin=1
c5c2_native_real_count=16
c5c2_tls_server_partial=1
c5c2_census_doc=1
c5c2_ssot_updated=1
c5c2_roadmap_ok=1
c5c2_evergreen=1
c5c2_macro_unit=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5c2_seccomp_wire_census_ok=1
current=C-5c-2-seccomp-wire
next=C-5d
```

## Non-claims

- ≠ C-5 `[x]` · Job Object · 全量 syscall GA · RSS 硬杀  
- ≠ permission-sandbox default-on · tls.server · macOS · R-50

## Soft park

**C-5d** RSS · Job Object（C-5c-3）· 全量 MirOp · CST lossless · CallSuper · C-7c+ · tls.server · macOS · R-50
