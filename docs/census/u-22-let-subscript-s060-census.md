# U-22 `let` subscript assign → S060 census

**Date:** 2026-11-16  
**Scope:** `IndexAssign` 对不可变 Ident → **S060**；与 `append` S060 同尺；hello2 `var d`；≠ Ch15/Ch28 MD；≠ R-50。  
**Script:** [`scripts/u22_let_subscript_s060_census.sh`](../../scripts/u22_let_subscript_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-21 | **收口** |
| **U-22** | **exited** — let 下标写 S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、Ch15/Ch28 S020 |

## Root cause

`append` 已闸 S060；`IndexAssign` 未查 base `mutable` → `let` 集合下标写假绿。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：`check_index_assign_on_immutable`
2. [`semantic_analyze_expr.rs`](../../crates/syx-semantic/src/semantic_analyze_expr.rs)：`IndexAssign` 调用
3. 钉 [`let_subscript_s060.rs`](../../crates/syx-driver/tests/lang/let_subscript_s060.rs)；[`hello2.sy`](../../examples/hello2.sy) → `var d`

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| let Array `[i]=` → S060 | check | **1** |
| let Dict `[k]=` → S060 | check | **1** |
| var subscript | run | **9** |
| append S060 | check | **1** |
| lang nail | let_subscript_s060 | **5/5** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u22_let_subscript_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ MemberAssign 全闸 / `self.field[i]=`
- ≠ Ch15/Ch28 S020 书袋
- ≠ 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-21-s020-bag-md-census`](u-21-s020-bag-md-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-22 → park
