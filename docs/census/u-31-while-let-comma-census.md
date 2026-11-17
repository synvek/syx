# U-31 comma `while let` census

**Date:** 2026-11-17  
**Scope:** `while let x = a, let y = b[, cond] { body }` → nested `IfLet` / trailing `If` + `else { break }` inside `while true`；对齐 U-14/U-16；≠ 字段默认 nil / 闭包 throws / R-50。  
**Script:** [`scripts/u31_while_let_comma_census.sh`](../../scripts/u31_while_let_comma_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-30 | **收口** |
| **U-31** | **exited** — 逗号 `while let` 绿 |
| Open | park：JIT、SyxHost、simd 159、字段默认 nil、闭包 throws、CST、Break |

## Root cause

`parse_while` Let 臂单绑定后直接 body；逗号 → P001。U-10 census 已显式 non-claim 逗号链；`if let` / `guard let` 逗号链已由 U-14/U-16 落地。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_while` Let 臂逗号环 + 嵌套 IfLet/If + break（`// U-10 / U-31`）。
2. 薄钉 [`while_let_comma.rs`](../../crates/syx-driver/tests/lang/while_let_comma.rs)；Ch06 en+zh 表薄补。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| both some | run | **1 2** |
| first nil → skip | run | **done** |
| second nil → skip | run | **done** |
| trailing cond true | run | **3** |
| trailing cond false → skip | run | **done** |
| single `while let` | run | **0** |
| `if let` comma | run | **1 2** |
| lang nail | while_let_comma | **7/7** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u31_while_let_comma_census_ok` | **1** | **1** |

## Non-claims

- ≠ 字段默认 nil / 闭包 throws
- ≠ 省略第二 `let` / `where` / pattern `while let .foo`
- ≠ `while case` 逗号链 / CST / codegen Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-30-mutating-self-assign-s060-census`](u-30-mutating-self-assign-s060-census.md)
- [`u-14-if-let-comma-census`](u-14-if-let-comma-census.md)
- [`u-16-guard-let-comma-census`](u-16-guard-let-comma-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-31 → park
