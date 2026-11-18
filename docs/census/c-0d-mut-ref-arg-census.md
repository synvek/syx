# C-0d `&mut` bare-value arg census

**Date:** 2026-11-18  
**Scope:** 非泛型自由函数实参：形参 `MutReference` 且 AST 非 `Borrow{mutable:true}`（剥 Group）→ **S004**。≠ 删 `is_compatible` 第 63 行 · 不改 `deref_type` · 共享 `&` 裸值 · 方法/init。  
**Script:** [`scripts/c0d_mut_ref_arg_census.sh`](../../scripts/c0d_mut_ref_arg_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0b** / **C-0c** | **exited** |
| **C-0d** | **exited** — bare value → `&mut` S004 |
| Open | 默认下一 **C-0a**；C-0e park |

## Root cause

`is_compatible(MutReference(T), T)` 为真；Ident 经 `deref_type` 剥引用后 assign-check 放行裸值。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：AST 闸（`peel_expr_groups` / `is_explicit_mut_borrow`）
2. 钉 [`type_soundness.rs`](../../crates/syx-driver/tests/lang/type_soundness.rs)
3. **不改** `type_checker` MutReference 宽松臂、`deref_type`

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `bump(n)` | **S004** | **S004** |
| `bump(&mut n)` / `bump((&mut n))` | check **0** | **0** |
| `id(n)` for `&Int` | check **0** | **0** |
| lang nail | type_soundness | **17/17** |
| exclusive_borrow | 不回归 | **2/2** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c0d_mut_ref_arg_census_ok` | **1** | **1** |

## Non-claims

- ≠ 删 `is_compatible` `(MutReference, found)` 臂 / 改 `deref_type`
- ≠ 方法 / init / 泛型 / host `&mut` 传参
- ≠ 收紧共享 `& T` 裸值
- ≠ C-0a Unknown 三分 / C-0e narrowing / R-50

## References

- [`c-0c-call-arity-census`](c-0c-call-arity-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-0d → C-0a
