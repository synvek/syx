# U-34 field-default unary Neg Int census

**Date:** 2026-11-18  
**Scope:** `var n: Int = -1`（一元 Neg）写入既有 `FieldDefaultLit::Int`；空 init / memberwise 打印 `-1`；≠ Float / 深剥 `-(1)` / R-50。  
**Script:** [`scripts/u34_field_default_unary_census.sh`](../../scripts/u34_field_default_unary_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-33 | **收口** |
| **U-34** | **exited** — unary Neg Int 字段默认 |
| Open | park：JIT、SyxHost、simd 159、闭包 throws 语法、CST、Break、Float 默认 |

## Root cause

Parser 将 `-1` 建成 `Expr::Unary(Neg, Integer)`；U-33 收集器只认 `Expr::Literal` → pad `ConstNil`。发射侧已支持负 `ConstI32`。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：`field_default_from_expr` + 两处收集改调
2. 扩钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| struct `= -1` | run | **-1** |
| class empty init `= -1` | run | **-1** |
| U-33 正字面量回归 | run | **true hi 7** |
| lang nail | field_default_literals | **7/7** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u34_field_default_unary_census_ok` | **1** | **1** |

## Non-claims

- ≠ Float/Double / `ConstNumeric` 截断修
- ≠ 深剥 `-(1)` / `!true` / Call 默认
- ≠ codegen 旁路 / extension 非 Int
- ≠ 隐式 `super.init` / 闭包 throws 语法 / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-33-field-default-literals-census`](u-33-field-default-literals-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-34 → park
