# U-6 `catch let` 双接受 census

**Date:** 2026-11-13  
**Scope:** Parser 薄双接受 Swift 形 `catch let e`（≡ `catch e`）；≠ 多 catch / CST / fmt 规范形 / Ch17 fence / R-50。  
**Script:** [`scripts/u6_catch_let_census.sh`](../../scripts/u6_catch_let_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-5 | **收口**（无 R-50） |
| **U-6** | **exited** — `catch let` parse+run 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-7 = grammar 诚实注记 / F1 触发 CST postfix |

## Root cause

`parse_do_catch` 在 `catch` 后只认 `(` / bare ident / bare `{`；`catch let e` 报 P001 Expected `{`。Swift / todo-005 对照为 `catch let err`。

## Delivery

1. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：`parse_do_catch` 在 `Catch` 后可选 `Let`。
2. [`docs/adr/sa-2-01-do-catch.md`](../../docs/adr/sa-2-01-do-catch.md)：Catch forms 增 `catch let name[: Type]`。
3. 薄钉 [`catch_let_dual.rs`](../../crates/syx-driver/tests/lang/catch_let_dual.rs)：`catch let` / typed / paren-less / paren / bare。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `catch let e` | run | **pass** |
| `catch let e: String` | run | **pass** |
| `catch e` / `catch (e)` / bare | run | **pass** |
| lang nail | catch_let_dual | **5/5** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u6_catch_let_census_ok` | **1** | **1** |

## Non-claims

- ≠ 多 `catch` / `catch is T` / `catch let e as T` / `where`
- ≠ CST `take_try_catch_simple` / fmt 规范形双接受（仍发 `catch (e)`）
- ≠ 改 Ch17 书籍 fence / `sy,output` 钉
- ≠ catch 抛值类型检查
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50 / CST postfix async

## References

- [`u-5-credibility-remnant-census`](u-5-credibility-remnant-census.md)
- [`u-4-func-async-census`](u-4-func-async-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-6 → U-7 / park
