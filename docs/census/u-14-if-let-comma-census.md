# U-14 restore comma `if let` census

**Date:** 2026-11-16  
**Scope:** 复落 `if let x = a, let y = b[, cond]` → 嵌套 `IfLet` / 尾部 `If`（U-13 曾纸面退出，树缺失）；else 克隆到各失败臂；≠ 逗号 `while let`；≠ `while case` 复落；≠ R-50。  
**Script:** [`scripts/u14_if_let_comma_census.sh`](../../scripts/u14_if_let_comma_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-11 | **树内可证收口** |
| U-12 `while case` | **仍纸面**（待 U-15 复落） |
| U-13 逗号 `if let` | **曾纸面**；本窗以 U-14 复落 |
| **U-14** | **exited** — 逗号 `if let` 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-15 = 复落 `while case` / 继续 park |

## Root cause

`parse_if_or_if_let` Let 臂单绑定后直接 body；逗号 → P001。U-13 账面 [x] 但 code/census/nails 曾从树中消失。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：Let 臂逗号链折叠（`// U-14`）。
2. 薄钉 [`if_let_comma.rs`](../../crates/syx-driver/tests/lang/if_let_comma.rs)。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| both some | run | **1 2** |
| first nil → else | run | **else** |
| second nil → else | run | **else** |
| trailing cond true | run | **3** |
| trailing cond false → else | run | **else** |
| single `if let` | run | **7** |
| plain `if` | run | **1** |
| lang nail | if_let_comma | **7/7** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u14_if_let_comma_census_ok` | **1** | **1** |

## Non-claims

- ≠ 逗号 `while let`
- ≠ `while case` 复落（U-15）
- ≠ `guard let` 链 / `where` / 省略第二 `let`
- ≠ CST / fmt 圆桌 / 作用域泄漏修
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50

## References

- [`u-11-repeat-label-census`](u-11-repeat-label-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-14 → U-15 / park
