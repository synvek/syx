# U-30 non-`mutating` self write → S060 census

**Date:** 2026-11-17  
**Scope:** struct/enum 方法体无 `mutating` 时写 `self` / 隐式字段 → **S060**；class/init/`mutating` 绿；`let`+mutating 调用仍 S060；≠ 逗号 while let / 字段默认 nil / R-50。  
**Script:** [`scripts/u30_mutating_self_assign_s060_census.sh`](../../scripts/u30_mutating_self_assign_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-29 | **收口** |
| **U-30** | **exited** — 非 mutating 写 self → S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、字段默认 nil |

## Root cause

`self` 在 type-member 分析中一律 `mutable: true`；`resolve_assign_target` 隐式字段写不看 `mutating` → Ch12 L106 假绿。

## Delivery

1. [`semantic_analyzer.rs`](../../crates/syx-semantic/src/semantic_analyzer.rs)：`current_function_is_mutating`
2. [`semantic_analyze_type_member.rs`](../../crates/syx-semantic/src/semantic_analyze_type_member.rs)：`self_mutable_for_method`；Init/setter 仍可写
3. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：enclosing 字段闸 + 不可变 `self=` → S060
4. 钉 [`mutating_self_assign_s060.rs`](../../crates/syx-driver/tests/lang/mutating_self_assign_s060.rs)；Ch12 + appendix-B 薄补

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| non-mutating `self.x=` → S060 | check | **1** |
| non-mutating implicit field → S060 | check | **1** |
| mutating / class run | run | **1** |
| `let` + mutating call | check | **1** |
| lang nail | mutating_self_assign_s060 | **7/7** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u30_mutating_self_assign_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ 逗号 `while let` / 字段默认 nil
- ≠ 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-29-let-nested-enum-tuple-s060-census`](u-29-let-nested-enum-tuple-s060-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-30 → park
