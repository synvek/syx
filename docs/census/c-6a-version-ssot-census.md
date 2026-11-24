# C-6a version/capability SSOT census

**Date:** 2026-11-24  
**Scope:** Cargo `[workspace.package].version` = 公开 semver SSOT；对齐 banner / plan/01 / capability `syx_version`；credibility+R-49 跟 `V`。**不**宣称 C-6 `[x]`；默认下一 **C-6b**。  
**Script:** [`scripts/c6a_version_ssot_census.sh`](../../scripts/c6a_version_ssot_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4** | **exited** |
| **C-3 / C-5** | **`[~]`** |
| **C-6a** version SSOT | **exited** |
| **C-6** | **`[~]`**（≠ `[x]`） |
| Open | 默认下一 **C-6b**；gen 从 Cargo 读 · ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

CLI/Cargo 已是 0.4.0，但 docs banner / capability meta / plan/01 / 旧 census 反钉 0.2.x–0.3.0，多源漂移。

## Delivery

1. Banner + plan/01 + 三 capability `syx_version` → `V` + regen  
2. credibility / R-49 / S2-07 探针跟 `V`  
3. SSOT / `02`：C-6a `[x]`；C-6 `[~]`；下一 **C-6b**  
4. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| workspace SSOT parse | **1** | **1** (`workspace_semver=0.4.0`) |
| surface align probes | **pass** | **1** (`c6a_surfaces_aligned` · gen_check · CLI) |
| credibility nails | **pass** | **1** (`c6a_credibility`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c6a_version_ssot_census_ok` | **1** | **1** |

## Results (2026-11-24)

```
workspace_semver=0.4.0
c6a_ssot_parse=1
c6a_surfaces_aligned=1
c6a_credibility_clean=1
c6a_gen_check=1
c6a_native_real_count=16
c6a_tls_server_partial=1
c6a_census_doc=1
c6a_ssot_updated=1
c6a_roadmap_ok=1
c6a_credibility=1
c6a_s207=1
c6a_cli_matches_ssot=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c6a_version_ssot_census_ok=1
current=C-6a-version-ssot
next=C-6b
```

## Non-claims

- ≠ C-6 `[x]` · gen 脚本自动读 Cargo（C-6b）
- ≠ 全站/archive 化石清扫 · `syx-gui` 0.1.3 · 删 `phase`
- ≠ ModuleId · C-5c+ · tls.server · macOS · R-50 · 宣称 C-3/C-5 `[x]`

## Soft park

C-6b gen-from-Cargo · ModuleId · C-5c+ timeout/seccomp · tls.server · macOS · R-50
