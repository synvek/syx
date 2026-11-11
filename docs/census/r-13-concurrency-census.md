# R-13 Concurrency census (post-SA residual)

**Status:** R-13 exit hinge met (R-13-01 + R-13-02 `[x]`) · next = **R-14+ residual**  
**Date:** 2026-11-10  
**Script:** [`scripts/r13_concurrency_census.sh`](../../scripts/r13_concurrency_census.sh)

## Exit hinge (R-13)

| ID | Claim | Evidence |
|----|-------|----------|
| R-13-01 | Concurrency public task surface off Object | KPI below; task bags → `Dictionary`, any-value slots → `Sendable` |
| R-13-02 | Hard gate + credibility | concurrency test suite + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r12_hinge_still_green` | **1** | Rollup of [`r12_text_testing_census.sh`](../../scripts/r12_text_testing_census.sh) |
| `concurrency_public_object_residual` | **0** | Concurrency methods ≠ Object |
| `concurrency_task_handle_dictionary` | **1** | detach / join / cancel / taskGroup* use `Dictionary` |
| `concurrency_any_value_sendable` | **1** | awaitValue / mainActorRun / taskLocal* use `Sendable` |
| `object_sig_residual_count` | **≤198** (ceiling) | R-13 absolute retired in R-27; was 213 (−15) at exit |
| `object_sig_residual_files` | **≤14** (ceiling) | absolute owned by newest window (R-27 → 186/11) |

### Concurrency slice (R-13-01)

Two honest spellings, chosen by runtime shape rather than uniformity:

| API | Was | Now | Runtime shape |
|-----|-----|-----|---------------|
| `detach(_:)` / `detach(_:args:)` | `-> Object` | `-> Dictionary` | `{__task_id, __cancelled}` bag |
| `join(_:)` | `Object` → `Object` | `Dictionary` → `Dictionary` | `{__done, __value}` bag |
| `cancel(_:)` | `task: Object` | `task: Dictionary` | task bag |
| `taskGroupCreate()` | `-> Object` | `-> Dictionary` | `{__group_id}` bag |
| `taskGroupAdd/Wait/Cancel` | `Object` | `Dictionary` | group/task bags |
| `awaitValue(_:)` | `Object` → `Object` | `Dictionary` → `Sendable` | pushes user value |
| `mainActorRun(_:)` | `-> Object` | `-> Sendable` | pushes user value |
| `taskLocalSet(_:_:)` | `value: Object` | `value: Sendable` | stores any value |
| `taskLocalGet(_:)` | `-> Object` | `-> Sendable` | any value |
| `detach(_:args:)` args | `Array<Object>` | `Array<Sendable>` | argument array |

Source: [`packages/core/concurrency.sy`](../../packages/core/concurrency.sy).

**Why not one spelling for all twelve:** task handles and groups are runtime bags
(`crates/syx-async/src/task.rs` `to_value` / `to_value_with_id`), so `Sendable` would hide the
`{__task_id}` structure and contradict the R-8/R-10/R-11 `Dictionary` precedent. Conversely
`awaitValue` / `mainActorRun` / `taskLocal*` carry arbitrary user values, so `Dictionary` would lie.

**Census scope change:** the inventory count now excludes `//` comment lines and additionally
matches `Array<Object>`. Baseline moves from 217/17 (raw regex) to 213/15 (code lines); R-13
takes it to 198/14. The four doc-comment hits previously counted in
[`packages/net/mod.sy`](../../packages/net/mod.sy) and
[`packages/networking/mod.sy`](../../packages/networking/mod.sy) were reworded, and
[`scripts/r12_text_testing_census.sh`](../../scripts/r12_text_testing_census.sh) now keeps a
monotonic (non-increasing) guard instead of pinning absolute values.

**Scoped honesty:** SyxHost keeps `task_join(task: String) -> Object` and the rest of the
`task_*` host surface; registry return types are unchanged. Public spelling is typed.
≠ `TaskHandle` / `TaskGroup` nominal type GA; ≠ `Task<T>` generic; ≠ simd wipe;
≠ collections `Element`/`Key`.

## How to run

```bash
bash scripts/r13_concurrency_census.sh
```

## Next

**R-15** — 测试基础设施债收口（`syx-driver --lib` 58/0、count_gate 死路径、空洞断言）。下一 = **R-16+**；仍开：wasm emit W001 全量拒绝、`data class` 合成缺失。
