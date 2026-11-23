# C-1a ManagedHeap 默认降级 census

**Date:** 2026-11-23  
**Scope:** generational / v10–v13 / compact **opt-in**；默认 mark-sweep；用户 stdout `gc:churn:` 需 `SYX_GC_CHURN_STDOUT=1`。≠ 修 weak/immutable · ≠ 重写 promotion。  
**Script:** [`scripts/c1a_heap_default_degrade_census.sh`](../../scripts/c1a_heap_default_degrade_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-0** 薄闸 | **exited** |
| **C-1a** 默认降级 | **exited** |
| Open | C-1b `[x]`；默认下一 **C-1c**；C-1d · C-1a+ 算法修 park |

## Root cause

`ManagedHeap` 各危险层 `unwrap_or(true)`；`collect_generational` + compact 默认组合可 SO；`record_gc_churn_stdout` 无条件写用户 stdout。

## Delivery

1. v9 / v10 / v11 / bridge / v13 / compact → 显式 `=1` 才开  
2. `SYX_GC_CHURN_STDOUT=1` 才写用户 stdout（in-memory lines 仍记）  
3. 永久钉：`c1a_default_degrade_tests`（默认 off · 128 alloc 不 SO）  
4. 测例迁：churn 显式 `SYX_HEAP_COMPACT=1`；v13 断言改为 default off；generational collect host 名对齐  

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| 默认 v9/compact/v13 | **false** | **pass** (`c1a_defaults_off=1`) |
| 默认 128 alloc+collect | 不 SO | **pass** (`c1a_nails=1`) |
| 默认 churn stdout gate | off | **pass** (`c1a_churn_gate=1`) |
| runtime-core / driver c1a nails | green | **pass** (`c1a_nails=1` · `c1a_driver_nails=1`) |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c1a_heap_default_degrade_census_ok` | **1** | **1** · next=C-1b |

## Non-claims

- ≠ 修 weak identity / immutable ObjectRef 写（C-1b/c）
- ≠ 修 promotion / card table / deep roots 真正确性（C-1a+）
- ≠ 关掉 `ManagedHeap::enabled()` mark-sweep 基线
- ≠ C-2 native L006 硬闸 / R-50

## References

- [`c-0a-unknown-sink-census`](c-0a-unknown-sink-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-1a → C-1b
