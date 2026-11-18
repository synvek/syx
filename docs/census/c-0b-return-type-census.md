# C-0b return type assignability census

**Date:** 2026-11-18  
**Scope:** `Stmt::Return` 对声明返回类型做 assignability（**S004**）；方法体设 mangled `current_function`（`Type_method`）。≠ arity / `&mut` / Unknown 三分 / missing-return CFG / C-0e narrowing。  
**Script:** [`scripts/c0b_return_type_census.sh`](../../scripts/c0b_return_type_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| SA / R / U 编号账本 | **收口** |
| **C-0b** | **exited** — wrong return → S004 |
| Open | 默认下一 **C-0c**；序 C-0b→c→d→a；C-0e park |

## Root cause

`Stmt::Return` 只分析表达式与 lifetime，不读 `function_return_types`；方法路径未设 `current_function`，无法对上 `Type_method` 返回表。

## Delivery

1. [`semantic_analyze_stmt.rs`](../../crates/syx-semantic/src/semantic_analyze_stmt.rs)：return → `check_assignable_with_literal`（S004）；裸 `return` 对非 Void 声明亦拒
2. [`semantic_analyze_type_member.rs`](../../crates/syx-semantic/src/semantic_analyze_type_member.rs)：方法体 `current_function = Type_method`
3. 钉 [`type_soundness.rs`](../../crates/syx-driver/tests/lang/type_soundness.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `return "wrong"` → Int | check **S004** | **S004** |
| `return 1` → Int | check **0** | **0** |
| method wrong return | **S004** | **S004** |
| lang nail | type_soundness | **5/5** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c0b_return_type_census_ok` | **1** | **1** |

## Non-claims

- ≠ C-0c arity / C-0d `&mut` 值传参 / C-0a Unknown 三分
- ≠ C-0e integer narrowing / 书 S008 对齐
- ≠ 全覆盖 missing-return 控制流（仅裸 `return` 非 Void）
- ≠ MIR verifier / JIT / SyxHost / CallSuper / R-50

## References

- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-0b → C-0c
- [`plan/syx-status-final-report.md`](../../plan/syx-status-final-report.md) §4.1 / P0.1
