# U-24 nested `let` member assign → S060 census

**Date:** 2026-11-16  
**Scope:** 值类型 `Member` 链 `MemberAssign` 对不可变根 Ident → **S060**；经 class/actor 跳放行；Ident 级 U-23 不破；≠ Ch15/Ch28 MD；≠ R-50。  
**Script:** [`scripts/u24_let_nested_member_assign_s060_census.sh`](../../scripts/u24_let_nested_member_assign_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-23 | **收口** |
| **U-24** | **exited** — 嵌套值类型成员写 S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、Ch15/Ch28 S020 |

## Root cause

U-23 仅 Ident；`o.inner`（`Expr::Member`）未走根 → 嵌套值类型字段写假绿。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：`value_member_assign_immutable_root` + 扩展 `check_member_assign_on_immutable`
2. 钉 [`let_nested_member_assign_s060.rs`](../../crates/syx-driver/tests/lang/let_nested_member_assign_s060.rs)；Ch10 + appendix-B 薄补

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| let nested `o.inner.x=` → S060 | check | **1** |
| let struct→class `w.cell.x=` | run | **2** |
| var nested | run | **2** |
| Ident U-23 S060 | check | **1** |
| lang nail | let_nested_member_assign_s060 | **6/6** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u24_let_nested_member_assign_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ 嵌套 IndexAssign / append 主刀
- ≠ enum / `self.x=` mutating
- ≠ Ch15/Ch28 S020 书袋
- ≠ 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-23-let-member-assign-s060-census`](u-23-let-member-assign-s060-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-24 → park
