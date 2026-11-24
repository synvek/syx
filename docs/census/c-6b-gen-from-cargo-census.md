# C-6b gen-from-Cargo census

**Date:** 2026-11-24  
**Scope:** `gen_capabilities.py` / `gen_ir_capabilities.py` 从 Cargo 注入 `syx_version`；TOML 就地改写；`--check` TOML≠`V` fail-closed；去掉 IR `"117.0"` 缺省。**宣称 C-6 `[x]`**；默认下一 **C-7**。  
**Script:** [`scripts/c6b_gen_from_cargo_census.sh`](../../scripts/c6b_gen_from_cargo_census.sh)  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Hygiene:** 有界探针 only；禁无界循环；负向 TOML 漂移用备份/恢复，勿污染树。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1 / C-2 / C-4 / C-6** | **exited** |
| **C-3 / C-5** | **`[~]`** |
| **C-6a** version SSOT | **exited** |
| **C-6b** gen-from-Cargo | **exited** |
| Open | 默认下一 **C-7**；ModuleId · C-5c+ · tls.server · macOS · R-50 park |

## Root cause

C-6a 钉了公开面到 Cargo `V`，但 gen 只透传 TOML；`--check` 不验 `V`；IR gen 缺省可落到化石 `"117.0"`。改 Cargo 后 TOML/生成物可再漂。

## Delivery

1. Shared [`scripts/syx_cargo_ssot.py`](../../scripts/syx_cargo_ssot.py)：解析 `[workspace.package].version`；读/改写 TOML `syx_version`
2. 两 gen：注入 `V`；非 check 时改写 stale TOML；`--check` TOML≠`V` → fail；去 `"117.0"`
3. 负向：伪造 TOML 旧版 → `--check` 非零（备份恢复）
4. SSOT / `02`：C-6b `[x]`；**C-6 `[x]`**；下一 **C-7**
5. census + 硬 KPI 持有

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| gen source contract (Cargo parse / no 117.0) | **pass** | **1** (`c6b_source_contract`) |
| `--check` green | **pass** | **1** (`c6b_gen_check`) |
| negative TOML drift `--check` | **fail (non-zero)** | **1** (`c6b_negative_check`) |
| `native=real` count | **16** | **16** |
| `net.tls.server` | **partial** | **1** |
| `sy,output` | **12/12** | **12/12** (en=6 zh=6) |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c6b_gen_from_cargo_census_ok` | **1** | **1** |
| `c6_exited` | **1** | **1** |

## Results (2026-11-24)

```
workspace_semver=0.4.0
c6b_source_contract=1
c6b_toml_aligned=1
c6b_gen_check=1
c6b_negative_check=1
c6b_post_restore_check=1
c6b_native_real_count=16
c6b_tls_server_partial=1
c6b_census_doc=1
c6b_ssot_updated=1
c6b_roadmap_ok=1
c6_exited=1
output_run_ok=12
output_run_total=12
browser_failed=0
wasm_passed=46
wasm_failed=0
object_sig_residual_count=180
object_sig_residual_files=10
simd_object_residual=159
c6b_gen_from_cargo_census_ok=1
current=C-6b
next=C-7
c6_exited=1
```

## Non-claims

- ≠ 全站/archive 化石清扫 · `syx-gui` · 删 `phase` · `manifest.json` 自动写
- ≠ ModuleId · C-5c+ timeout/seccomp · tls.server · macOS · R-50 · JIT
- ≠ 宣称 C-3/C-5 `[x]` · 开 C-7 实现（仅登记下一）

## Soft park

ModuleId · C-5c+ timeout/seccomp · tls.server · macOS · R-50 · C-7 implement
