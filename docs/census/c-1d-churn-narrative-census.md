# C-1d churn 叙事清扫 census

**Date:** 2026-11-23  
**Scope:** 禁「stdout / print `gc:churn:` = GC 成功」叙事；测例/Host/文档标 **Host label echo ≠ GC KPI**。用户 stdout 门闩已由 C-1a 持有。≠ 删 Host · ≠ 重写 GC · ≠ 抬 GcChurn capability。  
**Script:** [`scripts/c1d_churn_narrative_census.sh`](../../scripts/c1d_churn_narrative_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-1a/b/c** | **exited** |
| **C-1d** churn 叙事 | **exited** |
| **C-1 轴** | **exited** |
| Open | C-2a `[x]`；默认下一 **C-2b**；C-0a+/e park |

## Root cause

测例与文档把 `print(__syx_gc_churn_heap)` 的 `gc:churn:{label}` 回显当成 GC 绿；与 C-1a 门闩的 OS telemetry 混淆。

## Delivery

1. quad / stress / driver_gate p98 / relocating：注释与断言改为 Host label echo / in-memory  
2. `layer96::host_gc_churn_heap` 文档：label echo ≠ GC KPI  
3. status-report §5.4 / P0.2 · ga-evidence · implementation-status W158 措辞 scrub  
4. C-1 轴勾完成；默认下一 C-2

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| 测例无「stdout mandatory / GC 成功」conflation | scrubbed | **pass** (`c1d_tests_reframed=1`) |
| Host / managed_heap C-1d 注释 | present | **pass** (`c1d_host_comment=1`) |
| status-report 污染句 | C-1a gated + C-1d echo | **pass** (`c1d_doc_scrub=1`) |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c1d_churn_narrative_census_ok` | **1** | **1** · next=C-2 |

## Non-claims

- ≠ 删除 `__syx_gc_churn_heap` / 改返回 Bool
- ≠ 重写 promotion / card table / deep roots
- ≠ 回滚 C-1a opt-in / 门闩
- ≠ 把 `GcChurnHeap` 标 supported
- ≠ C-2 L006 硬闸 · R-50

## References

- [`c-1c-immutable-write-census`](c-1c-immutable-write-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-1d → C-2
