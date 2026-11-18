# C-0a Unknown→ground sink census

**Date:** 2026-11-18  
**Scope:** return / 注解 let / Assign 汇点：`Unknown` 且源为无注解形参 Ident 或 Member → 地面具体类型 → **S004**。≠ 关 `(_, Unknown)` · ≠ ErrorType/InferenceVar · ≠ 调用实参。  
**Script:** [`scripts/c0a_unknown_sink_census.sh`](../../scripts/c0a_unknown_sink_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0b/c/d** | **exited** |
| **C-0a** 薄闸 | **exited** — Unknown sink |
| Open | 默认下一 **C-1**；C-0a+ 三分 / C-0e park |

## Root cause

`is_compatible(_, Unknown)` 软放行；无注解形参与缺失 Object 成员产生 `Unknown`，return/let 汇点静默通过。

## Delivery

1. `current_untyped_params`（自由函数 + 方法）
2. `check_unknown_to_ground_sink`（[`semantic_analyze_call.rs`](../../crates/syx-semantic/src/semantic_analyze_call.rs)）
3. 挂 return / 注解 let / Assign
4. **不改** `is_compatible` 软臂
5. Fallout：`parity/generics.sy` → `id<T>(x: T) -> T`（原 `id(x) -> Int` 被汇点闸合法打红）

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `return x` untyped → Int | **S004** | **S004** |
| `o.missing` → Int | **S004** | **S004** |
| Any/Object/`[]`/call arg / Object façade | 仍绿 | **绿** |
| `syx check examples/hello.sy` (std) | **0** | **0** |
| lang nail | type_soundness | **type_soundness** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c0a_unknown_sink_census_ok` | **1** | **1** |

## Non-claims

- ≠ 删除 `(_, Unknown)=>true` / found-Object 软臂
- ≠ 引入 ErrorType / InferenceVar
- ≠ 调用实参 / 下标 / Call / 一跳局部 Unknown
- ≠ C-0e narrowing / C-1 ManagedHeap / R-50

## References

- [`c-0d-mut-ref-arg-census`](c-0d-mut-ref-arg-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-0a → C-1
