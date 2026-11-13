# U-4 `func…async` 双接受 census

**Date:** 2026-11-13  
**Scope:** Parser 薄双接受 Swift 形 `func f() async`（保留 `async func`）；≠ 全书改写 / ≠ CST / ≠ R-50。  
**Script:** [`scripts/u4_func_async_census.sh`](../../scripts/u4_func_async_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-3 | **收口**（无 R-50） |
| **U-4** | **exited** — postfix `async` parse+run 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；**U-5** remnant 4 MD cleanup |

## Root cause

仅前缀 `async func` 置 `is_async`；`)` 后遇 `async` 报 P001 Expected `{`。Swift / todo-005 对照为 `func fetch() async -> T`。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_function_with_name_opts` 在 `)` 后可选 `Async` → `is_async = true`。
2. 薄钉 [`func_async_dual.rs`](../../crates/syx-driver/tests/lang/func_async_dual.rs)：postfix / postfix+arrow / prefix。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| postfix `func() async` | check+run | **pass** |
| postfix `func() async -> T` | run | **pass** |
| prefix `async func` | run | **pass** |
| lang nail | 3/3 | **pass** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |

## Non-claims

- ≠ 改 Ch07/Ch18 书籍 fence / `sy,output` 钉
- ≠ CST `take_func_shell` / fmt 双接受
- ≠ 删 credibility 余 4 纯 MD
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50

## References

- [`u-3-credibility-diet-census`](u-3-credibility-diet-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-4 → U-5 / park
