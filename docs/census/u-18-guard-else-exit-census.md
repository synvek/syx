# U-18 `guard let` else-must-exit census

**Date:** 2026-11-16  
**Scope:** `guard let` else 未退出 → **S027**（`block_definitely_exits`）；书 en/zh 改标（原误标 S020）；≠ MainActor S020；≠ 裸 `guard` If；≠ `fatalError`/无限循环当退出；≠ R-50。  
**Script:** [`scripts/u18_guard_else_exit_census.sh`](../../scripts/u18_guard_else_exit_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-17 | **收口** |
| **U-18** | **exited** — S027 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159、逗号 while let、CST、codegen Break |

## Root cause

`GuardLet` 语义只 `analyze_block(else)`；书宣称未退出→S020，但 S020=MainActor，且非退出 else 仍 check/run 绿。

## Delivery

1. [`semantic_helpers.rs`](../../crates/syx-semantic/src/semantic_helpers.rs)：`block_definitely_exits`
2. [`semantic_analyze_stmt.rs`](../../crates/syx-semantic/src/semantic_analyze_stmt.rs)：GuardLet → **S027**
3. 书 en/zh 改标；薄钉 [`guard_else_exit.rs`](../../crates/syx-driver/tests/lang/guard_else_exit.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| print-only else → S027 | check | **1** |
| empty else → S027 | check | **1** |
| return else binds | run | **7** |
| bare guard empty else ≠ S027 | check | **1** |
| lang nail | guard_else_exit | **9/9** |
| books S027 retarget | en+zh | **1** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u18_guard_else_exit_census_ok` | **1** | **1** |

## Non-claims

- ≠ 裸 `guard`（U-17 If）退出检查
- ≠ `fatalError` / 无界 `while true` 当退出
- ≠ 逗号 `while let` / `guard case` / CST / codegen Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-17-guard-bare-census`](u-17-guard-bare-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-18 → park
