# U-29 nested enum/tuple MemberAssign → S060 census

**Date:** 2026-11-17  
**Scope:** `let` 嵌套路径 enum↔enum / enum→tuple / tuple→enum `MemberAssign` → **S060**；Ident/dataclass→enum 不破；var 绿；≠ `self.x=` / Ch12 MD / R-50。  
**Script:** [`scripts/u29_let_nested_enum_tuple_s060_census.sh`](../../scripts/u29_let_nested_enum_tuple_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-28 | **收口** |
| **U-29** | **exited** — 嵌套 enum/tuple 成员写 S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、`self.x=`；次选 Ch12 MD |

## Root cause

`value_member_assign_immutable_root` 非空路径仅认 dataclass 根跳 → enum/Tuple 中间跳假绿。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：`TypeId` 值路径跳（dataclass / enum / Tuple；class/actor bail）
2. 钉 [`let_nested_enum_tuple_s060.rs`](../../crates/syx-driver/tests/lang/let_nested_enum_tuple_s060.rs)；Ch09 + appendix-B 薄补嵌套

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| let enum→enum `o.inner.payload=` → S060 | check | **1** |
| let enum→tuple `e.t.0=` → S060 | check | **1** |
| let tuple→enum `t.0.payload=` → S060 | check | **1** |
| Ident enum / dataclass→enum | check | **1** |
| var enum→enum | run | **9** |
| lang nail | let_nested_enum_tuple_s060 | **7/7** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u29_let_nested_enum_tuple_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ `self.x=` / Ch12 MD 主刀
- ≠ enum 未知成员/类型闸
- ≠ 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-28-let-enum-member-assign-s060-census`](u-28-let-enum-member-assign-s060-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-29 → park
