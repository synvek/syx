# U-38 inherited field-default pad census

**Date:** 2026-11-18  
**Scope:** `lower_class_init` pad 经 `lookup_field_default` 沿 `class_superclasses` 查祖先字面量；复制进子类 dict；≠ 真隐式 `CallSuper` / memberwise arity / R-50。  
**Script:** [`scripts/u38_inherited_field_default_census.sh`](../../scripts/u38_inherited_field_default_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-37 | **收口** |
| **U-38** | **exited** — 继承字段默认 pad 祖先 walk |
| Open | park：JIT、SyxHost、simd 159、闭包 throws 语法、CST、Break、Group 剥、真隐式 CallSuper |

## Root cause

Defaults keyed by declaring type `(Animal, name)`；pad looked up `(Dog, name)` → miss → `ConstNil`。

## Delivery

1. [`mir_module_index.rs`](../../crates/syx-mir/src/mir_module_index.rs)：`lookup_field_default`
2. [`mir_lower_class_init.rs`](../../crates/syx-mir/src/mir_lower_class_init.rs)：pad 改调 helper
3. 扩钉 [`field_default_literals.rs`](../../crates/syx-driver/tests/lang/field_default_literals.rs)

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `Dog(breed:"lab").name` / `.breed` | run | **anon / lab** |
| U-33…U-37 回归 | run | **1.5 -1 true** |
| lang nail | field_default_literals | **15/15** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u38_inherited_field_default_census_ok` | **1** | **1** |

## Non-claims

- ≠ 真隐式 `CallSuper` / 父 init 体副作用
- ≠ 无 init memberwise arity / actor 继承
- ≠ Group 剥 / extension float
- ≠ 闭包 throws 语法 / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50
- ≠ Ch15 书宣称已改写为本窗交付

## References

- [`u-37-field-default-int-float-census`](u-37-field-default-int-float-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-38 → park
