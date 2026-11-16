# U-20 method `throws` flag census (S028)

**Date:** 2026-11-16  
**Scope:** 类型成员 `Decl::Function` / `Decl::Init` 置 `current_function_is_throws`；修复 U-19 后 throws 方法内 `throw` **假红 S028**；≠ 闭包 throws；≠ S020 袋 MD；≠ R-50。  
**Script:** [`scripts/u20_method_throws_s028_census.sh`](../../scripts/u20_method_throws_s028_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-19 | **收口** |
| **U-20** | **exited** — method/Init throws 旗绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159、逗号 while let、CST、codegen Break、S020 袋 MD |

## Root cause

U-19 仅在顶层 `Decl::Function` 置旗；[`analyze_type_member`](../../crates/syx-semantic/src/semantic_analyze_type_member.rs) 丢弃 `throws_type` / `throws` → throws 方法内 throw 恒走 S028 假红。

## Delivery

1. [`semantic_analyze_type_member.rs`](../../crates/syx-semantic/src/semantic_analyze_type_member.rs)：Function / Init 进出置 `current_function_is_throws`
2. 薄钉 [`method_throws_s028.rs`](../../crates/syx-driver/tests/lang/method_throws_s028.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| throws method throw | check | **1** |
| non-throws method → S028 | check | **1** |
| throws method do/catch | run | **caught** |
| top-level throws | check | **1** |
| init throws | check | **1** |
| lang nail | method_throws_s028 | **5/5** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u20_method_throws_s028_census_ok` | **1** | **1** |

## Non-claims

- ≠ 闭包 throws 语法 / 旗
- ≠ mutating / await 全章 S020→S060/A002
- ≠ 逗号 `while let` / `guard case` / CST / codegen Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-19-throw-requires-throws-census`](u-19-throw-requires-throws-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-20 → park
