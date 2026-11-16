# U-26 Ch15/Ch28 S020 MD census

**Date:** 2026-11-16  
**Scope:** Ch15 init-order / Ch28 access-level 假 **S020** 书宣称软化（en+zh）；对齐 appendix-B（S020=MainActor）；≠ 实现诊断；≠ 改标 S060；≠ R-50。  
**Script:** [`scripts/u26_ch15_ch28_s020_md_census.sh`](../../scripts/u26_ch15_ch28_s020_md_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-25 | **收口** |
| **U-26** | **exited** — Ch15/Ch28 S020 书诚实 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、`self.x=`；次选 tuple `.0=` S060 |

## Root cause

树侧无 init-order / access-level 诊断；书仍标 **S020**（appendix-B 已定为 MainActor）。

## Delivery

1. en/zh Ch15 callout + diag 表软化；en/zh Ch28 L80 + diag 表软化
2. 钉 [`ch15_ch28_s020_honesty.rs`](../../crates/syx-driver/tests/lang/ch15_ch28_s020_honesty.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| init-order 无 S020 | check | **1** |
| access-level 无 S020 | check | **1** |
| mutating 仍 S060 | check | **1** |
| await 仍 A002 | check | **1** |
| lang nail | ch15_ch28_s020_honesty | **4/4** |
| books soften | en+zh Ch15/Ch28 | **1** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u26_ch15_ch28_s020_md_census_ok` | **1** | **1** |

## Non-claims

- ≠ 实现 init-order / access-level 诊断
- ≠ 改标 S060 / 新码
- ≠ tuple `.0=` S060 / `self.x=`
- ≠ 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-21-s020-bag-md-census`](u-21-s020-bag-md-census.md)
- [`u-25-let-nested-index-append-s060-census`](u-25-let-nested-index-append-s060-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-26 → park
