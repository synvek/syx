# U-5 credibility 余 4 纯 MD 清扫 census

**Date:** 2026-11-13  
**Scope:** 清扫 U-1…U-3 后余下的 4 件纯 Markdown claim（dx_exit_open + lsp_book 三件）；≠ D3 / ≠ 删 desugar。  
**Script:** [`scripts/u5_credibility_remnant_census.sh`](../../scripts/u5_credibility_remnant_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-4 | **收口**（无 R-50） |
| **U-5** | **exited** — 4 remnant 纯 MD deleted |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-6（`catch let`） |

## Root cause

同尺 diet（≥8）之后仍挂 4 件纯 MD 治理自指契约；roadmap 已点名为本窗候选。

## Delivery

Deleted **4** Markdown-only modules and unwired `mod` lines in [`credibility/main.rs`](../../crates/syx-driver/tests/credibility/main.rs):

| File |
|------|
| `dx_exit_open_contract` |
| `lsp_book_claim_contract` |
| `lsp_book_residual_scope_contract` |
| `lsp_census_contract` |

**Retained:** `lsp_desugar` / `lsp_format`；`mono_display_census`；`stdlib_alignment_exit_census`；`r0`…`r13`。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| MD-only modules deleted | **4** | **4** |
| knife `mod` residual | **0** | **0** |
| `lsp_desugar` retained | present + green | **ok** |
| credibility smoke | `system_import_smoke` | **ok** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u5_credibility_remnant_census_ok` | **1** | **1** |

## Non-claims

- ≠ D3 merge `s*_census.sh` / ≠ 删 `lsp_desugar`
- ≠ JIT / SyxHost / HTTPClient / simd wipe / R-50
- ≠ `catch let` / CST postfix async / Array.first
- ≠ 填空 `plan/todo-006.md`

## References

- [`u-4-func-async-census`](u-4-func-async-census.md)
- [`u-3-credibility-diet-census`](u-3-credibility-diet-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-5 → U-6 / park
