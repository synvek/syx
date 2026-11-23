# C-2a Backend/CI matrix SSOT census

**Date:** 2026-11-23  
**Scope:** 钉死 **default_hard** = VM + MIR + Wasm **46/46** + browser **0**；native L006 / soft jobs / JIT = **soft · experimental · 非默认 KPI**。≠ C-2b full-native 硬红 · ≠ 批量降 `native=real` · ≠ 修 mirror-VM。  
**SSOT:** [`docs/gates/backend-ci-matrix.md`](../gates/backend-ci-matrix.md)  
**Script:** [`scripts/c2a_backend_ci_matrix_census.sh`](../../scripts/c2a_backend_ci_matrix_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0 / C-1** | **exited** |
| **C-2a** matrix SSOT | **exited** |
| Open | 默认下一 **C-2b**；C-2c/d · C-0a+/e park |

## Root cause

文档/capability 常把 soft native（L006 skip、`continue-on-error`、mirror-VM）读成「native 支持」；默认保护面未书面分栏。

## Delivery

1. SSOT `docs/gates/backend-ci-matrix.md`：default_hard / soft_non_default / experimental  
2. 硬否句：L006 skip ≠ native supported  
3. `01`/`02`/`README` 指向 SSOT；C-2a `[x]` → next C-2b  
4. census 机读 SSOT + `ci.yml` soft job 登记 + 硬 KPI

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| SSOT default_hard 声明 | VM+MIR+Wasm46/46+browser0 | **pass** (`c2a_default_hard_named=1`) |
| SSOT native/L006/JIT | soft / experimental / 非默认 KPI | **pass** (`c2a_ssot=1`) |
| 禁「L006 skip = native supported」 | absent in SSOT | **pass** (`c2a_no_claim_l006_is_native_green=1`) |
| `ci.yml` soft jobs listed | native-linux/macos · jit · browser-e2e | **pass** (`c2a_ci_soft_native_listed=1`) |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c2a_backend_ci_matrix_census_ok` | **1** | **1** · next=C-2b |

## Non-claims

- ≠ CI required full-native / 禁 soft_native exit 0（C-2b）
- ≠ 删 `run_native_strict_or_mirror_vm`（C-2b）
- ≠ 批量 demote `native=real`（C-2b）
- ≠ C-2c JIT 文案全扫 · C-2d job 名全对齐 · R-50

## References

- [`c-1d-churn-narrative-census`](c-1d-churn-narrative-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-2a → C-2b
