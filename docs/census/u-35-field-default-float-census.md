# U-35 field-default Double literals census

**Date:** 2026-11-18  
**Scope:** `Literal::Double` → `FieldDefaultLit::Num`；两处 pad `ConstNumeric` + `dest: Some(fresh_reg)`；≠ 一元 `-1.5` / extension float / R-50。  
**Script:** [`scripts/u35_field_default_float_census.sh`](../../scripts/u35_field_default_float_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-34 | **收口** |
| **U-35** | **exited** — Double 字段默认字面量 |
| Open | park：JIT、SyxHost、simd 159、闭包 throws 语法、CST、Break、一元 `-1.5`、隐式 super.init |

## Root cause

收集器不收 `Literal::Double` → pad `ConstNil`。`ConstNumeric` 若 `dest: None` 会在 mir-run 截断 bits。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：`Num { kind, bits }` + Double 收集
2. [`mir_lower_expr_call_ident.rs`](../../crates/syx-mir/src/mir_lower_expr_call_ident.rs) / [`mir_lower_class_init.rs`](../../crates/syx-mir/src/mir_lower_class_init.rs)：Num → `ConstNumeric` + `dest: Some`
3. 扩钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| struct `Double = 1.5` | run | **1.5** |
| class empty init `Double = 1.5` | run | **1.5** |
| U-33/U-34 回归 | run | **-1 true hi** |
| lang nail | field_default_literals | **9/9** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u35_field_default_float_census_ok` | **1** | **1** |

## Non-claims

- ≠ 一元 `-1.5` / `var x: Double = 1`（Integer）
- ≠ extension float / 全局 mir-run bits 截断修
- ≠ 隐式 `super.init` / 闭包 throws 语法 / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-34-field-default-unary-census`](u-34-field-default-unary-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-35 → park
