# U-36 field-default unary Neg Double census

**Date:** 2026-11-18  
**Scope:** `Unary Neg` + `Literal::Double` → 既有 `FieldDefaultLit::Num`（`bits` 取负）；pad 不改；≠ Integer→Double / extension float / R-50。  
**Script:** [`scripts/u36_field_default_neg_float_census.sh`](../../scripts/u36_field_default_neg_float_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-35 | **收口** |
| **U-36** | **exited** — unary Neg Double 字段默认 |
| Open | park：JIT、SyxHost、simd 159、闭包 throws 语法、CST、Break、Integer→Double、隐式 super.init |

## Root cause

`field_default_from_expr` Neg 臂只收 Integer；`-1.5` 是 `Neg(Double)` → `None` → pad `ConstNil`。正 `1.5` 与 Num emit（`dest: Some`）已由 U-35 就绪。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：Neg + Double → `Num { bits: float_literal_bits(kind, -*value) }`
2. 扩钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| struct `Double = -1.5` | run | **-1.5** |
| class empty init `Double = -1.5` | run | **-1.5** |
| U-33…U-35 回归 | run | **1.5 -1 true hi** |
| lang nail | field_default_literals | **11/11** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u36_field_default_neg_float_census_ok` | **1** | **1** |

## Non-claims

- ≠ Integer→Double / 深剥 `-(1.5)` / Call 默认
- ≠ extension float / 全局 mir-run bits 截断修
- ≠ 隐式 `super.init` / 闭包 throws 语法 / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-35-field-default-float-census`](u-35-field-default-float-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-36 → park
