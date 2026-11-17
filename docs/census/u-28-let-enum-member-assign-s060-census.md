# U-28 `let` enum MemberAssign → S060 census

**Date:** 2026-11-17  
**Scope:** `enum_cases` 根与 dataclass→enum 跳 `MemberAssign` → **S060**；var 绿；tuple/struct S060 不破；≠ `self.x=` / Ch12 MD / R-50。  
**Script:** [`scripts/u28_let_enum_member_assign_s060_census.sh`](../../scripts/u28_let_enum_member_assign_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-27 | **收口** |
| **U-28** | **exited** — let enum 关联写 S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、`self.x=`；次选 Ch12 MD |

## Root cause

`value_member_assign_immutable_root` 仅认 dataclass/Tuple → Named enum → `None`。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：`enum_cases` 根 + dataclass→enum 跳
2. 钉 [`let_enum_member_assign_s060.rs`](../../crates/syx-driver/tests/lang/let_enum_member_assign_s060.rs)；Ch09 + appendix-B 薄补

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| let `e.payload=` → S060 | check | **1** |
| let nested `w.e.payload=` → S060 | check | **1** |
| var enum | run | **9** |
| tuple / struct S060 | check | **1** |
| lang nail | let_enum_member_assign_s060 | **6/6** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u28_let_enum_member_assign_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ `self.x=` / Ch12 MD 主刀
- ≠ enum 未知成员/类型闸
- ≠ 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-27-let-tuple-member-assign-s060-census`](u-27-let-tuple-member-assign-s060-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-28 → park
