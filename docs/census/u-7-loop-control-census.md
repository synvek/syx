# U-7 MIR 循环控制 census

**Date:** 2026-11-13  
**Scope:** MIR 补齐 `repeat-while` / `break` / `continue`（修静默错答）；≠ codegen 字节码 Break；≠ if-let trailing；≠ R-50。  
**Script:** [`scripts/u7_loop_control_census.sh`](../../scripts/u7_loop_control_census.sh)

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-6 | **收口**（无 R-50） |
| **U-7** | **exited** — break/continue/repeat MIR run 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-8 = `if let` trailing-closure |

## Root cause

`mir_lower_stmt` 对 `Break` / `Continue` / `RepeatWhile` 落入 `_ => {}`；无 loop 目标栈 → `break` 忽略（`5`≠`2`）、`repeat` 不降级（`0`≠`3`）。

## Delivery

1. [`mir_lower_ctx.rs`](../../crates/syx-mir/src/mir_lower_ctx.rs)：`loop_stack` + `resolve_loop_target`。
2. [`mir_lower_stmt.rs`](../../crates/syx-mir/src/mir_lower_stmt.rs)：`While`/`ForIn` push/pop；`RepeatWhile`/`Break`/`Continue` 臂。
3. 薄钉 [`loop_control_mir.rs`](../../crates/syx-driver/tests/lang/loop_control_mir.rs)：break→`2`、continue→`8`、repeat→`3`。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `while`+`break` | run → `2` | **2** |
| `for`+`continue` | run → `8` | **8** |
| `repeat-while` | run → `3` | **3** |
| lang nail | loop_control_mir | **3/3** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u7_loop_control_census_ok` | **1** | **1** |

## Non-claims

- ≠ codegen 字节码 `Break`/`Continue` 空臂
- ≠ labeled `break outer` 完备验收（实现可解析，非本窗硬钉）
- ≠ `if let` trailing-closure / 分号 / 参考书 grammar
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50 / CST postfix

## References

- [`u-6-catch-let-census`](u-6-catch-let-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-7 → U-8 / park
