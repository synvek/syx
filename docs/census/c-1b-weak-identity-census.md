# C-1b weak identity census

**Date:** 2026-11-23  
**Scope:** `weak_load` 存活保 **canonical SharedCell Arc identity**；死后 / `__syx_weak_dead` **仅 Nil**。≠ immutable 写（C-1c）· ≠ 字段 auto-nil 产品糖 · ≠ ManagedHeap 算法修。  
**Script:** [`scripts/c1b_weak_identity_census.sh`](../../scripts/c1b_weak_identity_census.sh)  
**Hygiene:** 有界探针 only；禁无界循环。

## Progress

| Track | Status |
|-------|--------|
| **C-1a** 默认降级 | **exited** |
| **C-1b** weak identity | **exited** |
| Open | C-1c `[x]`；默认下一 **C-1d**；C-0a+/e park |

## Root cause

`Value::weak_load` 在 upgrade 后 `object_ref_from_map(map.clone(), …)` 新建 Arc，导致 `object_ref_identity` / `class_instance_identity` 与源对象不同。

## Delivery

1. `weak_load` 复用 upgrade 的 Arc → `ObjectRef { inner: SharedCell(arc), … }`  
2. dead 标记 / upgrade 失败 → 仅 `Nil`  
3. 永久钉：`c1b_weak_load_preserves_canonical_identity` · `c1b_weak_load_nil_after_mark_object_weak_dead` · drop 后 `weak_load==Nil`  
4. Host `__syx_weak_*` 转发不变（runtime-core 契约即盖 VM）

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| 存活 weak_load identity | == 源 ObjectRef | **pass** (`c1b_load_reuses_arc=1`) |
| drop / mark_dead → weak_load | **Nil** | **pass** (`c1b_nails=1`) |
| runtime-core weak_ref_tests | green | **4/4** |
| `sy,output` | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed; 2 ignored) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |
| `c1b_weak_identity_census_ok` | **1** | **1** · next=C-1c |

## Non-claims

- ≠ `mutable=false` ObjectRef 写拒收（C-1c）
- ≠ 字段级 weak 自动置 nil / suite_memory 全绿
- ≠ 启用 VM `WeakReference` 桩 / 新 MirOp
- ≠ C-1d Host/文档 churn 叙事 · C-2 · R-50

## References

- [`c-1a-heap-default-degrade-census`](c-1a-heap-default-degrade-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · C-1b → C-1c
