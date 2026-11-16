# U-17 bare `guard` census

**Date:** 2026-11-16  
**Scope:** `guard cond else { E }` → `Stmt::If(cond, then=[], else=E)`（布尔早退）；`guard let` / 逗号链不变；≠ S020 接线；≠ 逗号 `while let`；≠ R-50。  
**Script:** [`scripts/u17_guard_bare_census.sh`](../../scripts/u17_guard_bare_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-16 | **收口** |
| **U-17** | **exited** — 裸 `guard` 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159、逗号 while let、S020 else-exit、CST、codegen Break |

## Root cause

`TokenKind::Guard` 仅走 `parse_guard_let` → 强制 `Let`；裸 `guard true else` → P001。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_guard` 分派（`// U-17`）。
2. 薄钉 [`guard_bare.rs`](../../crates/syx-driver/tests/lang/guard_bare.rs)。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `guard true` continues | run | **1** |
| `guard false` → else | run | **else** |
| expr cond continues | run | **3** |
| single `guard let` | run | **7** |
| comma `guard let` | run | **1 2** |
| lang nail | guard_bare | **5/5** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u17_guard_bare_census_ok` | **1** | **1** |

## Non-claims

- ≠ S020 else-must-exit 接线
- ≠ 逗号 `while let` / `guard case`
- ≠ `for (k,v)` / CST / codegen Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-16-guard-let-comma-census`](u-16-guard-let-comma-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-17 → park
