# U-32 closure throw → S028 census

**Date:** 2026-11-17  
**Scope:** 闭包体内清 `current_function_is_throws`，闭包 `throw` 一律 **S028**（堵住 throws 父函数下假绿→R007）；≠ 闭包 `throws` 语法 / 字段默认 / R-50。  
**Script:** [`scripts/u32_closure_throw_s028_census.sh`](../../scripts/u32_closure_throw_s028_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-31 | **收口** |
| **U-32** | **exited** — 闭包 throw → S028 |
| Open | park：JIT、SyxHost、simd 159、字段默认字面量、闭包 throws 语法、CST、Break |

## Root cause

`Expr::Closure` 分析体时继承父函数 `current_function_is_throws`；父为 `throws` 时闭包内 `throw` check 放行，run 崩为 R007。

## Delivery

1. [`semantic_analyze_expr.rs`](../../crates/syx-semantic/src/semantic_analyze_expr.rs)：`Expr::Closure` 进出清/恢复旗（`// U-32`）。
2. 薄钉 [`closure_throw_s028.rs`](../../crates/syx-driver/tests/lang/closure_throw_s028.rs)。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| throws 父 + 闭包 throw → S028 | check | **1** |
| 非 throws 父 + 闭包 throw → S028 | check | **1** |
| 顶层 throws 直接 throw | run | **caught** |
| 方法 throws 回归 | run | **caught** |
| lang nail | closure_throw_s028 | **5/5** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u32_closure_throw_s028_census_ok` | **1** | **1** |

## Non-claims

- ≠ 闭包 `throws` 语法 / `Task { throw }`
- ≠ 字段默认 Bool/String 字面量
- ≠ CST / codegen Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-31-while-let-comma-census`](u-31-while-let-comma-census.md)
- [`u-19-throw-requires-throws-census`](u-19-throw-requires-throws-census.md)
- [`u-20-method-throws-s028-census`](u-20-method-throws-s028-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-32 → park
