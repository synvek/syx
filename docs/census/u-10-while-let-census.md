# U-10 `while let` census

**Date:** 2026-11-15  
**Scope:** `parse_while` 接受 `while let Ident = expr`，解糖为 `while true { if let … else { break } }`；≠ `while case`；≠ R-50。  
**Script:** [`scripts/u10_while_let_census.sh`](../../scripts/u10_while_let_census.sh)  
**Hygiene:** 有界探针 only（Optional→nil）；禁无界 `while`。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-9 | **收口**（无 R-50） |
| **U-10** | **exited** — 有界 `while let` 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-11 = CST postfix async / 继续 park |

## Root cause

`parse_while` 在 `while` 后直接 `parse_expr()`；遇 `let` → P001。无 `Stmt::WhileLet`。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_while` Let 分支解糖（`// U-10`）。
2. 薄钉 [`while_let.rs`](../../crates/syx-driver/tests/lang/while_let.rs)：有界 Optional / 立即 nil / plain while / if let。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `while let` 0..2 | run | **0 1 2** |
| `while let` nil | run | **done** |
| plain `while` | run | **0 1 2** |
| `if let` | run | **7** |
| lang nail | while_let | **4/4** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u10_while_let_census_ok` | **1** | **1** |

## Non-claims

- ≠ `while case`
- ≠ 逗号链 `while let x = a, let y = b`
- ≠ pattern `while let .foo(x)`
- ≠ codegen Break 空臂
- ≠ CST / fmt postfix `async`
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50

## References

- [`u-9-semicolon-census`](u-9-semicolon-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-10 → U-11 / park
