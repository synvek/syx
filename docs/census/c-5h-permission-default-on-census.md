# C-5h permission-sandbox default-on census

**Date:** 2026-11-25  
**Scope:** permission.sandbox **default-on**；`--no-permission-sandbox` / `SYX_PERMISSION_SANDBOX=0` opt-out；无 grant → HostFn **S024**；硬 KPI allow 注入。**不**宣称 C-5 `[x]`；默认下一 **跨轴**。  
**Script:** [`scripts/c5h_permission_default_on_census.sh`](../../scripts/c5h_permission_default_on_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)

## Progress

| Track | Status |
|-------|--------|
| **C-5a…C-5g** | **exited** |
| **C-5h** permission default-on | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **跨轴**（C-3 全量 MirOp / P0 余债）；真全量 syscall GA · tls.server · R-50 park |

## Root cause

`PermissionGrants::default` 用 `env_truthy(SYX_PERMISSION_SANDBOX)` → 默认 false；CLI 仅 `--permission-sandbox` SetTrue；钉锁 default-off。HostFn S024 闸有但不默认开。

## Delivery

1. `env_falsey_opt_out("SYX_PERMISSION_SANDBOX", true)`  
2. CLI `--no-permission-sandbox` 三态 + from_cli  
3. 改写 `default_off` → default-on S024；`c5h_*` opt-out / fs 负向  
4. `.cargo` + `book_pass_rate` allow 注入保硬 KPI  
5. ADR pin3 / env-registry / UsageGuide / QuickRef / capability policy  
6. SSOT / `02`：C-5h `[x]`；C-5 `[~]`；下一 **跨轴**  
7. census + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| default-on API + CLI | **1** | **1** |
| evergreen / security nails | **pass** | **1** |
| `native=real` | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ C-5 `[x]` / OS isolation / macro seccomp  
- ≠ expand S024 to all HostFn  
- ≠ remove opt-out  
- ≠ change hard KPI floors  
