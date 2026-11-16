# U-16 comma `guard let` census

**Date:** 2026-11-16  
**Scope:** `guard let x = a, let y = b[, cond] else { E }` → `Stmt::Seq` of `GuardLet` + optional trailing `If`（书宣称对齐）；else 克隆到各失败臂；≠ 裸 `guard cond`；≠ 逗号 `while let`；≠ R-50。  
**Script:** [`scripts/u16_guard_let_comma_census.sh`](../../scripts/u16_guard_let_comma_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-15 | **收口**（U-12/U-13 纸面已复落） |
| **U-16** | **exited** — 逗号 `guard let` 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159、逗号 while let、CST、codegen Break |

## Root cause

`parse_guard_let` 单绑定后强制 `else`；逗号 → P001。书 Ch06 表行宣称 `guard let x = opt, cond else` Implemented。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_guard_let` 逗号链 → Seq（`// U-16`）。
2. 薄钉 [`guard_let_comma.rs`](../../crates/syx-driver/tests/lang/guard_let_comma.rs)。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| both some continues | run | **1 2** |
| first nil → else | run | **else** |
| second nil → else | run | **else** |
| trailing cond true | run | **3** |
| trailing cond false → else | run | **else** |
| single `guard let` | run | **7** |
| `if let` comma | run | **1 2** |
| lang nail | guard_let_comma | **7/7** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u16_guard_let_comma_census_ok` | **1** | **1** |

## Non-claims

- ≠ 裸 `guard cond`（无 let）
- ≠ 省略第二 `let`
- ≠ 逗号 `while let`
- ≠ `for (k,v)` / CST / codegen Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-15-while-case-census`](u-15-while-case-census.md)
- [`u-14-if-let-comma-census`](u-14-if-let-comma-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-16 → park
