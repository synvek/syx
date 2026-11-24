# C-5b embed wasm fail-closed census

**Date:** 2026-11-24  
**Scope:** `SYX_EMBED_WASM_SANDBOX=1` 时 wasm 失败/不可用 → `syx_embed_v2_run` **null**（禁静默 in-process）。**不**宣称 C-5 `[x]`；默认下一 **C-6**。  
**Script:** [`scripts/c5b_embed_wasm_fail_closed_census.sh`](../../scripts/c5b_embed_wasm_fail_closed_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4** | **exited** |
| **C-3** | **`[~]`**（C-3a…C-3g `[x]`） |
| **C-5a** | **exited** |
| **C-5b** embed wasm fail-closed | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-6**；C-5c+ timeout/seccomp · ModuleId · tls.server · macOS · R-50 park |

## Root cause

`SYX_EMBED_WASM_SANDBOX=1` 时 wasmtime 不可用或 `embed_wasmtime_real_run`→`None` 后仍静默落到 `compile_and_run_embedded`，与 opt-in sandbox 意图矛盾。

## Delivery

1. `syx_embed_v2_run`：enabled 且未成功 → null  
2. `embed_wasm_inprocess_fallback_allowed` = `!embed_wasm_sandbox_enabled()`  
3. `c5b_*` 两钉；ADR pin2 + env-registry  
4. SSOT / `02`：C-5b `[x]`；C-5 `[~]`；下一 **C-6**  
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| fail-closed contract | **1** | **1** (`c5b_api`) |
| c5b evergreen (2) | **pass** | **1** (`c5b_evergreen_smoke` · 2 passed) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c5b_embed_wasm_fail_closed_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c5b_api=1
c5b_tests_present=1
c5b_native_real_count=16
c5b_tls_server_partial=1
c5b_census_doc=1
c5b_ssot_updated=1
c5b_roadmap_ok=1
c5b_evergreen_smoke=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5b_embed_wasm_fail_closed_census_ok=1
current=C-5b-embed-wasm-fail-closed
next=C-6
```

## Non-claims

- ≠ C-5 `[x]` · timeout 接线 · 真 seccomp/WASI
- ≠ 改 `--embed-sandbox` 语义 · permission-sandbox default-on
- ≠ tls.server · macOS · R-50 · 宣称 C-3 `[x]` · 开 C-6 实现

## Soft park

C-6 version SSOT · C-5c+ timeout/seccomp · ModuleId · full MirOp · exception · tls.server · macOS · R-50
