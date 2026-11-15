# U-11 labeled `repeat` / `break outer` census

**Date:** 2026-11-15  
**Scope:** `RepeatWhile.label` + MIR `loop_stack` push（修 `outer: repeat { break outer }` → MIR001）；≠ CST postfix async；≠ R-50。  
**Script:** [`scripts/u11_repeat_label_census.sh`](../../scripts/u11_repeat_label_census.sh)  
**Hygiene:** 有界探针 only（体一次 `break`）；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-10 | **收口**（无 R-50） |
| **U-11** | **exited** — 标签 `repeat`/`break outer` 绿 |
| Open | park：JIT Suite-1、SyxHost、simd 159；候选 U-12 = CST 例外 ADR / 继续 park |

## Root cause

`parse_repeat_while` 接受标签后丢弃（`let _label = label`）；`Stmt::RepeatWhile` 无 label；MIR 推 `(…, None)`。`while`/`for` 已 `label.clone()`。

## Delivery

1. [`crates/syx-ast/src/lib.rs`](../../crates/syx-ast/src/lib.rs)：`RepeatWhile.label`。
2. [`crates/syx-parser/src/lib.rs`](../../crates/syx-parser/src/lib.rs)：透传 label（`// U-11`）。
3. [`crates/syx-mir/src/mir_lower_stmt.rs`](../../crates/syx-mir/src/mir_lower_stmt.rs)：`push((…, label.clone()))`。
4. 薄钉 [`repeat_label_break.rs`](../../crates/syx-driver/tests/lang/repeat_label_break.rs)。

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `outer: repeat { break outer }` | run | **1** |
| unlabeled `repeat { break }` | run | **1** |
| labeled `while` break | run | **2** |
| lang nail | repeat_label_break | **3/3** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u11_repeat_label_census_ok` | **1** | **1** |

## Non-claims

- ≠ CST / fmt postfix `async`
- ≠ codegen Break 空臂
- ≠ `while case` / 逗号 `if let` / multi-catch
- ≠ `continue outer` 扩面硬钉（同源 label 栈则自然绿）
- ≠ JIT / SyxHost / HTTPClient / D3 / simd wipe / R-50

## References

- [`u-10-while-let-census`](u-10-while-let-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-11 → U-12 / park
