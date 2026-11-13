# U-1 credibility 纯 MD claim 再瘦身 census

**Date:** 2026-11-13  
**Scope:** R-48 续刀 — 再删 Stage-2/3 纯 Markdown claim 契约（≠ 全清 credibility / ≠ D3）。  
**Script:** [`scripts/u1_credibility_diet_census.sh`](../../scripts/u1_credibility_diet_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0 | **收口**（无 R-50） |
| **U-1** | **exited** — 9 纯 MD modules deleted |
| Open | park：JIT Suite-1、`func…async`、simd 159；**U-2** Stage-3 claim diet |

## Root cause

Credibility 仍挂治理自指契约（ADR/`[x]`/census 字符串），无编译或运行能力钉（[todo-005](../../plan/todo-005.md) D1 尾）。R-48 删 8 件后仍有 Stage-2/3 纯 MD 簇。

## Delivery

Deleted **9** Markdown-only modules and unwired `mod` lines in [`credibility/main.rs`](../../crates/syx-driver/tests/credibility/main.rs):

| Cluster | Files removed |
|---------|----------------|
| Stage-2 workspace / version / architecture | `architecture_exit` · `exclude_syx_gui` · `version_census` · `workspace_census` · `workspace_claim` |
| Stage-2 frontend share-tree eval | `frontend_share_tree_evaluation` |
| Stage-3 diagnostic / mono display claims | `diagnostic_span_claim` · `mono_display_claim` |
| Stage-3 exit claim | `dx_exit` |

**Retained (plan-list correction):** [`cst_lower_probe_contract.rs`](../../crates/syx-driver/tests/credibility/cst_lower_probe_contract.rs) — has `Parser::parse` executable nail (≠ pure MD).

Kept: mixed source-shape / executable contracts; `cst_lower_probe`; later Stage claim chains for U-2+.

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| MD-only modules deleted | **≥8** (shipped **9**) | **9** |
| knife `mod` residual in `main.rs` | **0** | **0** |
| `cst_lower_probe` retained | present | **present** |
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
- ≠ delete `cst_lower_probe` (Parser nail)

## References

- [`r-48-credibility-diet-census`](r-48-credibility-diet-census.md)
- [`u-0-filepath-joining-census`](u-0-filepath-joining-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-1 → U-2 / park
