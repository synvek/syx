# U-21 S020 bag MD census (mutating→S060 / await→A002)

**Date:** 2026-11-16  
**Scope:** 书 en/zh 改标 — mutating→**S060**、非 async 内 await→**A002**；appendix-B 收窄 S020；≠ 闭包 throws；≠ 逗号 while let；≠ R-50。  
**Script:** [`scripts/u21_s020_bag_md_census.sh`](../../scripts/u21_s020_bag_md_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-20 | **收口** |
| **U-21** | **exited** — S020 袋书诚实 |
| Open | park：JIT Suite-1、SyxHost、simd 159、逗号 while let、CST、codegen Break、闭包 throws、Ch15/Ch28 S020 |

## Root cause

树侧 mutating=S060、await-outside-async=A002 已久；书仍误标 S020（MainActor 袋）。

## Delivery

1. en/zh Ch01/07/10/12/18 + ref03 + appendix-B 改标
2. 薄钉 [`s020_bag_honesty.rs`](../../crates/syx-driver/tests/lang/s020_bag_honesty.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| mutating → S060 | check | **1** |
| await → A002 | check | **1** |
| lang nail | s020_bag_honesty | **3/3** |
| books retarget | en+zh bag | **1** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u21_s020_bag_md_census_ok` | **1** | **1** |

## Non-claims

- ≠ 闭包 throws
- ≠ 逗号 `while let` / `guard case` / CST / codegen Break
- ≠ Ch15 init-order / Ch28 access-level S020 深挖
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-20-method-throws-s028-census`](u-20-method-throws-s028-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-21 → park
