# U-40 extension self.init non-Int census

**Date:** 2026-11-18  
**Scope:** `extension_init_literal_fields` → `FieldDefaultLit` via `field_default_from_expr`；shell 发 Bool/Str/Num（Num `dest:Some`）；unify 认 ConstBool/Str；保 Integer S1-I7；≠ CallSuper / Ch15 MD / R-50。  
**Script:** [`scripts/u40_extension_init_non_int_census.sh`](../../scripts/u40_extension_init_non_int_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-39 | **收口** |
| **U-40** | **exited** — extension self.init 非 Int |
| Open | park：JIT、SyxHost、simd 159、闭包 throws、CST、Break、真 CallSuper、Ch15 MD |

## Root cause

Collector Integer-only → non-Int `self.init` 整调用 `None`；shell 只发 PLATFORM_INT；unify 不认 Bool/Str shell。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：collector + register
2. [`mir_lower_type_members.rs`](../../crates/syx-mir/src/mir_lower_type_members.rs)：shell 四臂
3. [`mir_unify.rs`](../../crates/syx-driver/src/mir_unify.rs)：Bool/Str 谓词
4. 扩钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| extension Double | run | **1.5 / 0** |
| extension Bool/String | run | **true / ok** |
| Integer S1-I7 | run | **0 / 0** |
| Dog inherited | run | **anon / lab** |
| lang nail | field_default_literals | **19/19** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u40_extension_init_non_int_census_ok` | **1** | **1** |

## Non-claims

- ≠ 真隐式 `CallSuper` / Ch15 MD 改写
- ≠ `((7))` / `--1` / `!true` / Call 默认
- ≠ 部分成功 / 非空 init 参数 / extension Int→Double promote 表
- ≠ 闭包 throws / CST / Break / JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-39-field-default-group-census`](u-39-field-default-group-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-40 → park
