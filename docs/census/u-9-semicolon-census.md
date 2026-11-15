# U-9 statement semicolon tolerance census

**Date:** 2026-11-15  
**Scope:** `parse_block_stmts` 吞可选 `;`（修 `if { }; next` / `let x = 1; …` → P001）；≠ codegen Break；≠ R-50。  
**Script:** [`scripts/u9_semicolon_census.sh`](../../scripts/u9_semicolon_census.sh)  
**Hygiene:** 有界探针 only；禁无界 `while`。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-8 | **收口**（无 R-50） |
| **U-9** | **exited** — 块内语句后可选 `;` 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-10 = codegen Break 死臂 / 继续 park |

## Root cause

`parse_block_stmts` 解析完一条语句后不吃 `;`；仅表达式语句臂可选吞一个 `;`。`if`/`let` 等语句后遇 `;` 被当成下一条语句起点 → P001。class/actor 成员循环已有 `while match Semicolon` 先例。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_block_stmts` 每条语句前吞 0..n 个 `;`（`// U-9`）。
2. 薄钉 [`stmt_semicolon.rs`](../../crates/syx-driver/tests/lang/stmt_semicolon.rs)：`if { };` / `;;` / 无 `;` / `let …;`。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `if true { }; print(1)` | run | **1** |
| `if true { } ;; print(2)` | run | **2** |
| `let x = 1; print(x)` | run | **1** |
| no-semicolon `if` | run | **1** |
| lang nail | stmt_semicolon | **4/4** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u9_semicolon_census_ok` | **1** | **1** |

## Non-claims

- ≠ codegen Break 空臂
- ≠ `while let` AST
- ≠ CST / fmt postfix `async`
- ≠ 逗号链 `if let`
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50

## References

- [`u-8-if-let-trailing-census`](u-8-if-let-trailing-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-9 → U-10 / park
