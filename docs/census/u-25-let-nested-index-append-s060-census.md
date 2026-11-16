# U-25 nested `let` IndexAssign / append → S060 census

**Date:** 2026-11-16  
**Scope:** 值类型 `Member` 链上 `IndexAssign` + mutating 集合调用 → **S060**；class/actor 跳放行；Ident U-22 / U-24 MemberAssign 不破；≠ Ch15/Ch28 MD；≠ R-50。  
**Script:** [`scripts/u25_let_nested_index_append_s060_census.sh`](../../scripts/u25_let_nested_index_append_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-24 | **收口** |
| **U-25** | **exited** — 嵌套集合下标/append S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、Ch15/Ch28 S020、`self.x=` |

## Root cause

`mutating_receiver_binding` Ident-only；`s.arr`（`Expr::Member`）下标/append 假绿。U-24 helper 终点须 dataclass，不能直接用于 Array 叶。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：`immutable_value_mut_root` + IndexAssign / mutating call 接线
2. 钉 [`let_nested_index_append_s060.rs`](../../crates/syx-driver/tests/lang/let_nested_index_append_s060.rs)；Ch05 + appendix-B 薄补

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| let nested `s.arr[i]=` → S060 | check | **1** |
| let nested `s.arr.append` → S060 | check | **1** |
| let nested Dict `[k]=` → S060 | check | **1** |
| let class Array append | run | **2** |
| var nested Array | check no S060 | **1** |
| Ident / U-24 S060 | check | **1** |
| lang nail | let_nested_index_append_s060 | **8/8** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u25_let_nested_index_append_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ Ch15/Ch28 S020 书袋
- ≠ `self.x=` / self mutable
- ≠ enum / 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-24-let-nested-member-assign-s060-census`](u-24-let-nested-member-assign-s060-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-25 → park
