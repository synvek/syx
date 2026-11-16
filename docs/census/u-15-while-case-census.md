# U-15 restore `while case` census

**Date:** 2026-11-16  
**Scope:** 复落 `while case pattern = subject { body }` → `While(true)` + `IfCase` + `else Break`（U-12 曾纸面退出，树缺失）；≠ 逗号 `while let`；≠ `guard let`；≠ R-50。  
**Script:** [`scripts/u15_while_case_census.sh`](../../scripts/u15_while_case_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-11 + U-14 | **树内可证收口** |
| U-12 `while case` | **曾纸面**；本窗以 U-15 复落 |
| U-13 逗号 `if let` | 已由 U-14 复落 |
| **U-15** | **exited** — `while case` 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159、逗号 while let、CST、codegen Break |

## Root cause

`parse_while` 仅特判 `Let`（U-10）；`case` 非 soft keyword → `parse_expr()` → P001。U-12 账面 [x] 但 code/census/nails 曾从树中消失。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_while` Case 臂（`// U-15`）。
2. 薄钉 [`while_case.rs`](../../crates/syx-driver/tests/lang/while_case.rs)。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| bounded match then exit | run | **hit / done** |
| immediate mismatch → skip | run | **done** |
| multi-match then exit | run | **1 2 3 / done** |
| plain `while` | run | **0 1** |
| `if case` | run | **ok** |
| `while let` | run | **0 1** |
| lang nail | while_case | **6/6** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u15_while_case_census_ok` | **1** | **1** |

## Non-claims

- ≠ 逗号 `while let`
- ≠ `guard let` / 链
- ≠ CST / fmt 圆桌 / codegen Break
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50

## References

- [`u-14-if-let-comma-census`](u-14-if-let-comma-census.md)
- [`u-10-while-let-census`](u-10-while-let-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-15 → park
