# C-5a macro/embed threat-model census

**Date:** 2026-11-24  
**Scope:** 假 sandbox 安全承诺降级 + [`macro-embed-threat-model`](../adr/macro-embed-threat-model.md) ADR；`macro_sandbox_mode` → `seccomp-label-unwired`。**不**宣称 C-5 `[x]`；默认下一 **C-5b**。  
**Script:** [`scripts/c5a_threat_model_census.sh`](../../scripts/c5a_threat_model_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4** | **exited** |
| **C-3** | **`[~]`**（C-3a…C-3g `[x]`） |
| **C-5a** threat-model | **exited** |
| **C-5** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-5b**；真 seccomp · ModuleId · tls.server · macOS · R-50 park |

## Root cause

公开面把 macro reexec / embed soft flags / `*SECCOMP*` 标签写成 security isolation 或 hardening GA，超过实现边界。

## Delivery

1. ADR `macro-embed-threat-model.md`（四钉）  
2. CLI / UsageGuide / QuickRef / 报告 / 书 / env-registry 降级  
3. `macro_sandbox_mode` → `seccomp-label-unwired`  
4. SSOT / `02`：C-5a `[x]`；C-5 `[~]`；下一 **C-5b**  
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| threat-model ADR | **1** | **1** (`c5a_adr`) |
| claim scrub probes | **pass** | **1** (`c5a_claim_scrub` · `c5a_cli_help`) |
| mode label honesty | **1** | **1** (`c5a_mode_label` · `c5a_mode_unit`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c5a_threat_model_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
c5a_adr=1
c5a_claim_scrub=1
c5a_mode_label=1
c5a_native_real_count=16
c5a_tls_server_partial=1
c5a_census_doc=1
c5a_ssot_updated=1
c5a_roadmap_ok=1
c5a_mode_unit=1
c5a_cli_help=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c5a_threat_model_census_ok=1
current=C-5a-threat-model
next=C-5b
```

## Non-claims

- ≠ C-5 `[x]` · embed wasm fail-closed（C-5b）
- ≠ 真 seccomp / Job Object / WASI capability / cgroup
- ≠ permission-sandbox default-on · tls.server · macOS · R-50 · 宣称 C-3 `[x]`

## Soft park

C-5b embed fail-closed · ModuleId · full MirOp · exception · tls.server · macOS · R-50 · soft→hard · JIT expand · C-6
