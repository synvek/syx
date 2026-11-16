# U-27 `let` tuple MemberAssign → S060 census

**Date:** 2026-11-16  
**Scope:** `TypeId::Tuple` 根与 dataclass→Tuple 末跳 `MemberAssign` → **S060**；var 绿；Ident struct S060 不破；≠ `self.x=` / enum / R-50。  
**Script:** [`scripts/u27_let_tuple_member_assign_s060_census.sh`](../../scripts/u27_let_tuple_member_assign_s060_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| 编号账本 SA-0…R-49 + U-0…U-26 | **收口** |
| **U-27** | **exited** — let tuple 成员写 S060 |
| Open | park：JIT、SyxHost、simd 159、逗号 while let、CST、Break、闭包 throws、`self.x=` |

## Root cause

`value_member_assign_immutable_root` 仅认 Named dataclass → Tuple 根/字段直接 `None`。

## Delivery

1. [`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)：Tuple 根 + dataclass→Tuple 末跳
2. 钉 [`let_tuple_member_assign_s060.rs`](../../crates/syx-driver/tests/lang/let_tuple_member_assign_s060.rs)；Ch02 + appendix-B 薄补

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| let `t.0=` → S060 | check | **1** |
| let `t.x=` → S060 | check | **1** |
| let nested `b.pair.0=` → S060 | check | **1** |
| var `t.0=` | run | **3** |
| Ident struct S060 | check | **1** |
| lang nail | let_tuple_member_assign_s060 | **6/6** |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `u27_let_tuple_member_assign_s060_census_ok` | **1** | **1** |

## Non-claims

- ≠ `self.x=` / enum MemberAssign
- ≠ class 持有 tuple `.0=` codegen
- ≠ 逗号 `while let` / 闭包 throws / CST / Break
- ≠ JIT / SyxHost / D3 / simd wipe / R-50

## References

- [`u-26-ch15-ch28-s020-md-census`](u-26-ch15-ch28-s020-md-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · U-27 → park
