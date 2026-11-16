# U-23 `let` member assign → S060 census

**Date:** 2026-11-16  
**Scope:** `MemberAssign` 对不可变 **struct** Ident → **S060**；`class`/`actor` `let` 字段写仍合法；与 append / IndexAssign 同尺；≠ Ch15/Ch28 MD；≠ R-50。  
**Script:** [`scripts/u23_let_member_assign_s060_census.sh`](../../scripts/u23_let_member_assign_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-22 | **收口** |
| **U-23** | **exited** — let struct 成员写 S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、Ch15/Ch28 S020 |

## Root cause

`IndexAssign` / `append` 已闸 S060；`MemberAssign` 未查 base `mutable` + `dataclass_names` → `let` struct 字段写假绿。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：`check_member_assign_on_immutable`
2. [`semantic_analyze_expr.rs`](../../crates/syx-semantic/src/semantic_analyze_expr.rs)：`MemberAssign` 调用
3. 钉 [`let_member_assign_s060.rs`](../../crates/syx-driver/tests/lang/let_member_assign_s060.rs)；Ch10 + appendix-B 薄补

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| let struct `p.x=` → S060 | check | **1** |
| let class `b.n=` | run | **9** |
| var struct | run | **3** |
| IndexAssign / append S060 | check | **1** |
| lang nail | let_member_assign_s060 | **7/7** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u23_let_member_assign_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ enum MemberAssign 全闸 / `self.x=` 深挖
- ≠ Ch15/Ch28 S020 书袋
- ≠ 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-22-let-subscript-s060-census`](u-22-let-subscript-s060-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-23 → park
