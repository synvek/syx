# U-19 `throw` requires `throws` census

**Date:** 2026-11-16  
**Scope:** 非 `throws` 函数内 `throw` → **S028**（`current_function_is_throws`）；书 en/zh 改标（原误标 S020；缺 `try` 误标 P001→**S018**）；Ch06 去掉 `fatalError` 退出宣称；≠ MainActor S020；≠ R-50。  
**Script:** [`scripts/u19_throw_requires_throws_census.sh`](../../scripts/u19_throw_requires_throws_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-18 | **收口** |
| **U-19** | **exited** — S028 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159、逗号 while let、CST、codegen Break、方法/闭包 throws 旗 |

## Root cause

`Stmt::Throw` 只校验抛出类型（String/Error），不查当前函数是否 `throws`；书宣称非 throws → S020，但 S020=MainActor，且非 throws 内 throw 仍 check 绿。

## Delivery

1. [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs)：`current_function_is_throws`
2. [`semantic_analyze_decl.rs`](../../crates/syx-semantic/src/semantic_analyze_decl.rs)：`Decl::Function` 进出置位
3. [`semantic_analyze_stmt.rs`](../../crates/syx-semantic/src/semantic_analyze_stmt.rs)：Throw → **S028**
4. 书 en/zh 改标；薄钉 [`throw_requires_throws.rs`](../../crates/syx-driver/tests/lang/throw_requires_throws.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| 非 throws throw → S028 | check | **1** |
| throws + do/catch | run | **caught** |
| lang nail | throw_requires_throws | **5/5** |
| catch_let dual | still green | **1** |
| books S028 / S018 retarget | en+zh | **1** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u19_throw_requires_throws_census_ok` | **1** | **1** |

## Non-claims

- ≠ 方法 / 闭包 throws 旗深挖（本窗仅 `Decl::Function`）
- ≠ mutating / await 全章 S020 改标
- ≠ 逗号 `while let` / `guard case` / CST / codegen Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-18-guard-else-exit-census`](u-18-guard-else-exit-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-19 → park
