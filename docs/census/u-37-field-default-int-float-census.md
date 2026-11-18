# U-37 field-default Integer→Double promote census

**Date:** 2026-11-18  
**Scope:** 收集期 `Int` + 字段类型 Float/Double → `FieldDefaultLit::Num`；pad 不改；钉用 `x / 2`（I32→`0`，F64→`0.5`）；≠ 继承 pad walk / extension float / R-50。  
**Script:** [`scripts/u37_field_default_int_float_census.sh`](../../scripts/u37_field_default_int_float_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-36 | **收口** |
| **U-37** | **exited** — Integer→Double 字段默认提升 |
| Open | park：JIT、SyxHost、simd 159、闭包 throws 语法、CST、Break、继承字段默认 pad、隐式 super.init |

## Root cause

`field_default_from_expr` 把 Integer 一律收成 `Int` → pad `ConstI32`，忽略声明 `Double`。`print` / `+ 0.5` 不辨种类；`/ 2` 在 I32 上得 `0`。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：`promote_int_default_for_float_field` + struct/class insert
2. 扩钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| struct `Double = 1` → `/2` | run | **0.5** |
| class empty init `Double = 1` → `/2` | run | **0.5** |
| struct `Double = -1` → `/2` | run | **-0.5** |
| U-33…U-36 回归 | run | **1.5 -1.5 -1** |
| lang nail | field_default_literals | **14/14** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u37_field_default_int_float_census_ok` | **1** | **1** |

## Non-claims

- ≠ 继承字段默认 pad 祖先 walk / 真隐式 `CallSuper`
- ≠ extension float / `Group` 剥
- ≠ 闭包 throws 语法 / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-36-field-default-neg-float-census`](u-36-field-default-neg-float-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-37 → park
