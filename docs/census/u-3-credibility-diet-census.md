# U-3 credibility Stage-4 纯 MD claim 再瘦身 census

**Date:** 2026-11-13  
**Scope:** U-2 续刀 — 删 Stage-4 九件纯 Markdown claim（stdlib_stability / package_registry+hinge / ecosystem_exit）；≠ 全清 credibility / ≠ D3。  
**Script:** [`scripts/u3_credibility_diet_census.sh`](../../scripts/u3_credibility_diet_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-2 | **收口**（无 R-50） |
| **U-3** | **exited** — 9 Stage-4 纯 MD modules deleted |
| Open | park：JIT Suite-1、`func…async`、simd 159；候选 U-4 |

## Root cause

Stage-4 半区仍挂 S4-01/S4-02 / ecosystem exit 治理自指 census/claim/residual 链，无编译或运行能力钉（[todo-005](../../plan/todo-005.md) D1 尾）。

## Delivery

Deleted **9** Markdown-only modules and unwired `mod` lines in [`credibility/main.rs`](../../crates/syx-driver/tests/credibility/main.rs):

| Cluster | Files removed |
|---------|----------------|
| stdlib_stability (S4-01) | `*_claim` / `*_doctest_census` / `*_residual_scope` |
| package_registry + hinge (S4-02) | `*_census` / `*_claim` / `*_residual_scope` + `file_registry_hinge` |
| ecosystem exit | `ecosystem_exit` / `ecosystem_exit_open` |

**Retained:** prep / `cst_lower`；`lsp_desugar` / `lsp_format`；`stability_ssot`；`prebuilt_release_*` / `parser_*` / `file_split_*`；`r0`…`r13` 产品 census。

**本窗不伸：** `mono_display_census` / `dx_exit_open`；`lsp_book_*` / `lsp_census`；`stdlib_alignment_exit_census`。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| MD-only modules deleted | **≥8** (shipped **9**) | **9** |
| knife `mod` residual in `main.rs` | **0** | **0** |
| credibility smoke | `system_import_smoke` | **ok** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |

## Non-claims

- ≠ delete all credibility contracts / D1 full wipe
- ≠ D3 merge ~24 `s*_census.sh`
- ≠ JIT unpark / `func…async` / SyxHost 全擦 / `Http`→`HTTPClient`
- ≠ 新开 R-50 / simd wipe
- ≠ 填空 `plan/todo-006.md`

## References

- [`u-2-credibility-diet-census`](u-2-credibility-diet-census.md)
- [`u-1-credibility-diet-census`](u-1-credibility-diet-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-3 → U-4 / park
