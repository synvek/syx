# U-39 field-default Group peel census

**Date:** 2026-11-18  
**Scope:** `field_default_from_expr` 入口一层 `Expr::Group` 剥；Neg 单次递归 + `negate_int_or_num`；≠ `((7))` / `--1` / extension float / R-50。  
**Script:** [`scripts/u39_field_default_group_census.sh`](../../scripts/u39_field_default_group_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-38 | **收口** |
| **U-39** | **exited** — Group 一层剥 |
| Open | park：JIT、SyxHost、simd 159、闭包 throws、CST、Break、extension float、真 CallSuper、Ch15 MD |

## Root cause

括号建成 `Expr::Group`；收集器只认 `Literal` / `Neg(Literal)` → pad `ConstNil`。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：入口 Group 剥 + Neg 递归取负
2. 扩钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| struct Group / 深剥 / `((7))` | run | **7 1.5 -1 -1.5 -1 nil** |
| class `(7)` + `-(1.5)` | run | **7 / -1.5** |
| Dog inherited | run | **anon / lab** |
| lang nail | field_default_literals | **17/17** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u39_field_default_group_census_ok` | **1** | **1** |

## Non-claims

- ≠ `((7))` 多层剥 / `--1` / `!true`
- ≠ extension float / 真隐式 `CallSuper` / Ch15 MD 改写
- ≠ 闭包 throws 语法 / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-38-inherited-field-default-census`](u-38-inherited-field-default-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-39 → park
