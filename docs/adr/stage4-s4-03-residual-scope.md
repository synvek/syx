# ADR: Stage-4 S4-03 residual-scope（S3-I98）

**Status:** Accepted (S3-I98) · S4-03 `[~]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I97 cancel/TaskGroup hinge](stage4-s4-03-cancel-taskgroup-hinge.md), Primary KPIs are green (`cancel_taskgroup_hinge_helper≥1` · `cancel_token_helper≥1` · `task_group_hostfn_helper≥1` · `task_tree_cancel_test_count≥1`), while Sendable/`@unchecked` deepen, `withTaskGroup`/EC-M3, and TSAN/loom GA remain open. This window revises **S4-03 acceptance semantics** (scoped residual) so a later dedicated claim can flip `[x]` honestly. It does **not** claim S4-03 `[x]`, change `syx-async`/semantics, declare `withTaskGroup` GA, or exit Stage-4.

## Decision

1. **Scoped acceptance (S4-03 stays `[~]` this window):** Under Hard OUT（≠ 改 `syx-async`/语义 / ≠ `withTaskGroup`/EC-M3 / ≠ TSAN·loom GA）, honest S4-03 completion means:
   - **Primary:** I97 hinge holds（[`s4-03-cancel-taskgroup-v0`](../census/s4-03-cancel-taskgroup-v0.md)；cancel + TaskGroup HostFn + task_tree C004）；
   - **KPI:** `cancel_taskgroup_hinge_helper≥1` · `cancel_token_helper≥1` · `task_group_hostfn_helper≥1` · `task_tree_cancel_test_count≥1`；
   - **Scaffolding:** `s4_03_open_helper` · `s4_03_census_helper` · `residual_scope_helper` green.
2. **Deferred / supersede-only:** Sendable/`@unchecked` deepen · `withTaskGroup`/EC-M3 · TSAN/loom GA are **deferred**. Restoring any as a claim hard gate requires a **superseding ADR**. They are **not** blockers for scoped `[x]`. Scope charter: [s4-03-concurrency-residual-scope-v0](../census/s4-03-concurrency-residual-scope-v0.md).
3. **Ladder next:** **S3-I99** = dedicated **S4-03 scoped `[x]` claim** under this acceptance — still ≠ Stage-4 exit.
4. **Non-claims (hard):** ≠ S4-03 `[x]` this window / ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 改 `syx-async`/语义 / ≠ `withTaskGroup`/EC-M3/TSAN·loom GA / ≠ 开 S4-04…05 / ≠ GC Nursery 当并发轴 / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Scope charter | this ADR · `scoped acceptance` / `deferred` / `supersede` |
| Residual scope | [`docs/census/s4-03-concurrency-residual-scope-v0.md`](../census/s4-03-concurrency-residual-scope-v0.md) |
| Census | `residual_scope_helper≥1` · Current=**S3-I98** |
| Plan | S4-03 row stays `[~]` · scoped摘要 |
| Contract | `s4_03_residual_scope_contract` |
| Prior | [I97 cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md) · [I96 open](stage4-s4-03-open.md) |

## Consequences

- Residual-scope hinge landed；S4-03 was `[~]` this window；Stage-4 was `[~]`（**2/5**）.
- **S3-I99 landed** as [stage4-s4-03-claim](stage4-s4-03-claim.md)（S4-03 scoped `[x]` · Stage-4 **3/5**）。
- Default next = **S3-I100**（S4-04 open）— not Stage-4 exit / not S4-04 construct / not `withTaskGroup` GA.
- Prior: [stage4-s4-03-cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md) · S3-I97.
- Structured-concurrency census Current → **S3-I99** · Claimed scoped `[x]`。

## References

- [stage4-s4-03-claim](stage4-s4-03-claim.md) · S3-I99
- [stage4-s4-03-cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md) · S3-I97
- [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96
- [s4-03-concurrency-residual-scope-v0](../census/s4-03-concurrency-residual-scope-v0.md)
- [s4-03-cancel-taskgroup-v0](../census/s4-03-cancel-taskgroup-v0.md)
- [stage4-s4-02-residual-scope](stage4-s4-02-residual-scope.md) · S3-I94（形态模板）
- [stage4-structured-concurrency-census](../census/stage4-structured-concurrency-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-03
- [`crates/syx-async`](../../crates/syx-async)
