# ADR: Stage-4 S4-03 cancel/TaskGroup hinge（S3-I97）

**Status:** Accepted (S3-I97) · S4-03 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I96 S4-03 open](stage4-s4-03-open.md), census showed cancel / TaskGroup / Sendable inventory green while Stage-4 still lacked a **Primary construct hinge**. Existing `CancellationToken`, TaskGroup HostFn×4, and task_tree/chaos cancel→C004 nails already pass. This window elevates that evidence into a **machine-checkable hinge** (charter + census KPIs + contract). It does **not** claim S4-03 `[x]`, change `syx-async`/semantics, declare `withTaskGroup` GA, or exit Stage-4.

## Decision

1. **Publish** charter [`docs/census/s4-03-cancel-taskgroup-v0.md`](../census/s4-03-cancel-taskgroup-v0.md)（Primary = cancel + TaskGroup；Sendable/`withTaskGroup` deferred）。
2. **Nail Primary KPIs** on existing evidence（no product edit）：
   - `cancel_token_helper≥1` · `CancellationToken`
   - `task_group_hostfn_helper≥1` · TaskGroupCreate/Add/Wait/Cancel
   - `task_tree_cancel_test_count≥1` · `m3_a01_task_tree_cancel` + `m3_c04_chaos_task_group_cancel`
   - `cancel_taskgroup_hinge_helper≥1` · this hinge ADR/charter
3. **Ladder next:** **S3-I98** = S4-03 residual-scope — still ≠ S4-03 `[x]`.
4. **Non-claims (hard):** ≠ S4-03 `[x]` / ≠ Stage-4 exit / ≠ 改 `syx-async`/语义 / ≠ `withTaskGroup` / EC-M3 / TSAN·loom GA / ≠ Sendable Primary this window / ≠ S4-04…05 / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ GC Nursery 当并发轴 / ≠ language semantics greenfield.

## Evidence

| Check | Evidence |
|-------|----------|
| Charter | [`docs/census/s4-03-cancel-taskgroup-v0.md`](../census/s4-03-cancel-taskgroup-v0.md) |
| Census KPI | `cancel_taskgroup_hinge_helper≥1` · cancel_token / TaskGroup HostFn / task_tree counts |
| Contract | `s4_03_cancel_taskgroup_hinge_contract` |
| Prior | [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96 |

## Consequences

- Cancel/TaskGroup Primary hinge landed；S4-03 was `[~]` this window；Stage-4 was `[~]`（**2/5**）.
- **S3-I98 landed** as [stage4-s4-03-residual-scope](stage4-s4-03-residual-scope.md)（scoped acceptance · deferred Sendable/`withTaskGroup`）。
- **S3-I99 landed** as [stage4-s4-03-claim](stage4-s4-03-claim.md)（S4-03 scoped `[x]` · Stage-4 **3/5**）。
- Default next = **S3-I100**（S4-04 open）— not Stage-4 exit / not `withTaskGroup` GA.
- Prior: [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96.
- Structured-concurrency census Current → **S3-I99** · Claimed scoped `[x]`。

## References

- [stage4-s4-03-claim](stage4-s4-03-claim.md) · S3-I99
- [stage4-s4-03-residual-scope](stage4-s4-03-residual-scope.md) · S3-I98
- [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96
- [s4-03-cancel-taskgroup-v0](../census/s4-03-cancel-taskgroup-v0.md)
- [s4-03-concurrency-residual-scope-v0](../census/s4-03-concurrency-residual-scope-v0.md)
- [stage4-structured-concurrency-census](../census/stage4-structured-concurrency-census.md)
- [stage4-s4-02-file-registry-hinge](stage4-s4-02-file-registry-hinge.md) · S3-I93（hinge 形态）
- [plan/02](../../plan/02-improvement-roadmap.md) S4-03
- [`crates/syx-async`](../../crates/syx-async)
