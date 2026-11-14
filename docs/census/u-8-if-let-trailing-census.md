# U-8 `if let` / `if case` trailing-closure census

**Date:** 2026-11-14  
**Scope:** Parser 关 `if let` RHS / `if case` subject 的 trailing-closure（修调用吞 `{` → P001）；≠ `while let`；≠ R-50。  
**Script:** [`scripts/u8_if_let_trailing_census.sh`](../../scripts/u8_if_let_trailing_census.sh)  
**Hygiene:** 有界探针 only；禁无界 `while`。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-7 | **收口**（无 R-50） |
| **U-8** | **exited** — `if let`/`if case` + 调用 RHS 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-9 = 分号 / codegen Break |

## Root cause

`parse_if_or_if_let` 中 plain `if` 已关 `allow_trailing_closure`；`if let` / `if case` 未关 → `f()` 后 `{` 被当成 trailing closure。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`if let` RHS / `if case` subject 外包关旗（`// U-8`）。
2. 薄钉 [`if_let_trailing.rs`](../../crates/syx-driver/tests/lang/if_let_trailing.rs)：调用 / paren / plain `if`。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `if let v = f()` | run | **7** |
| `if case 7 = f()` | run | **ok** |
| plain `if f()` | run | **1** |
| lang nail | if_let_trailing | **4/4** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u8_if_let_trailing_census_ok` | **1** | **1** |

## Non-claims

- ≠ `while let` AST
- ≠ 逗号链 `if let x = a, let y = b`
- ≠ codegen Break / 分号 / 参考书 grammar
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50 / CST postfix

## References

- [`u-7-loop-control-census`](u-7-loop-control-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-8 → U-9 / park
