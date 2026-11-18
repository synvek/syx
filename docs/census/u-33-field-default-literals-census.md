# U-33 field-default Bool/String/Int literals census

**Date:** 2026-11-17  
**Scope:** MIR `FieldDefaultLit`（Int/Bool/Str）空 init / memberwise 不再 `ConstNil`；≠ Float / 嵌套构造 / codegen 旁路 / R-50。  
**Script:** [`scripts/u33_field_default_literals_census.sh`](../../scripts/u33_field_default_literals_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-32 | **收口** |
| **U-33** | **exited** — Bool/String/Int 字段默认字面量 |
| Open | park：JIT、SyxHost、simd 159、闭包 throws 语法、CST、Break、Float 默认 |

## Root cause

`dataclass_field_default_i64` 只收 `Literal::Integer`；Bool/String 缺省 → `ConstNil`。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：`FieldDefaultLit` + `dataclass_field_defaults`
2. [`mir_lower_expr_call_ident.rs`](../../crates/syx-mir/src/mir_lower_expr_call_ident.rs) / [`mir_lower_class_init.rs`](../../crates/syx-mir/src/mir_lower_class_init.rs)：pad 发 `ConstI32`/`ConstBool`/`ConstStr`
3. 薄钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| struct Bool/String/Int | run | **true hi 7** |
| class empty init defaults | run | **anon false 7** |
| explicit override | run | **3** |
| missing default → nil | run | **nil** |
| Counter Int 回归 | run | **1** |
| lang nail | field_default_literals | **5/5** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u33_field_default_literals_census_ok` | **1** | **1** |

## Non-claims

- ≠ Float/Double / 插值 / Call 默认 / unary `-1`
- ≠ codegen `i32` 旁路图
- ≠ extension 非 Int `self.init`
- ≠ 闭包 throws 语法 / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-32-closure-throw-s028-census`](u-32-closure-throw-s028-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-33 → park
