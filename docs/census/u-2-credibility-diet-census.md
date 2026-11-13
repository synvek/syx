# U-2 credibility Stage-3 纯 MD claim 再瘦身 census

**Date:** 2026-11-13  
**Scope:** U-1 / R-48 续刀 — 删 Stage-3 三簇纯 Markdown claim（init_template / lint_ast / vm_hotspot）；≠ 全清 credibility / ≠ D3。  
**Script:** [`scripts/u2_credibility_diet_census.sh`](../../scripts/u2_credibility_diet_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0/U-1 | **收口**（无 R-50） |
| **U-2** | **exited** — 9 Stage-3 纯 MD modules deleted |
| Open | park：JIT Suite-1、`func…async`、simd 159；**U-3** Stage-4 claim diet |

## Root cause

Stage-3 冻结半区仍挂 `7/7` / `S3-I*` 治理自指 census/claim/residual 链，无编译或运行能力钉（[todo-005](../../plan/todo-005.md) D1 尾）。

## Delivery

Deleted **9** Markdown-only modules and unwired `mod` lines in [`credibility/main.rs`](../../crates/syx-driver/tests/credibility/main.rs):

| Cluster | Files removed |
|---------|----------------|
| init_template | `*_census` / `*_claim` / `*_residual_scope` |
| lint_ast | `*_census` / `*_claim` / `*_residual_scope` |
| vm_hotspot | `*_census` / `*_claim` / `*_residual_scope` |

**Retained (mixed):** [`lint_ast_claim_prep_contract.rs`](../../crates/syx-driver/tests/credibility/lint_ast_claim_prep_contract.rs)、[`vm_hotspot_claim_prep_contract.rs`](../../crates/syx-driver/tests/credibility/vm_hotspot_claim_prep_contract.rs)、[`cst_lower_probe_contract.rs`](../../crates/syx-driver/tests/credibility/cst_lower_probe_contract.rs)。

**本窗不伸：** `mono_display_census` / `dx_exit_open`；`package_registry_*` / `lsp_*` / `stdlib_stability_*` / `ecosystem_*`。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| MD-only modules deleted | **≥8** (shipped **9**) | **9** |
| knife `mod` residual in `main.rs` | **0** | **0** |
| prep + `cst_lower` retained | present | **present** |
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
- ≠ delete prep / `cst_lower` mixed nails

## References

- [`u-1-credibility-diet-census`](u-1-credibility-diet-census.md)
- [`r-48-credibility-diet-census`](r-48-credibility-diet-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-2 → U-3 / park
