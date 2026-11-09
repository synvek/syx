# ADR: Stage-4 S4-03 plan claim（S3-I99）

**Status:** Accepted (S3-I99) · S4-03 `[x]` · Stage-4 `[~]`  
**Date:** 2026-11-09  
**Context:** After [S3-I98 residual-scope](stage4-s4-03-residual-scope.md), Hard OUT construct ([I97 cancel/TaskGroup hinge](stage4-s4-03-cancel-taskgroup-hinge.md)), open ([I96](stage4-s4-03-open.md)), and scoped acceptance (I98) are landed. Primary KPIs report **`cancel_taskgroup_hinge_helper≥1`**, **`cancel_token_helper≥1`**, **`task_group_hostfn_helper≥1`**, **`task_tree_cancel_test_count≥1`**, **`residual_scope_helper≥1`**, with honest deferred axes（Sendable/`@unchecked` · `withTaskGroup`/EC-M3 · TSAN/loom GA）remaining **deferred supersede-only**. This dedicated window flips the plan checkbox only — aligned with [S3-I95 S4-02 claim](stage4-s4-02-claim.md). It does **not** exit Stage-4, change `syx-async`/semantics, or restore deferred axes as hard gates.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I96](stage4-s4-03-open.md) · [census](../stage4-structured-concurrency-census.md) |
| Construct I97 | [`s4-03-cancel-taskgroup-v0`](../s4-03-cancel-taskgroup-v0.md) · cancel + TaskGroup KPIs |
| Residual-scope | [S3-I98](stage4-s4-03-residual-scope.md) · scoped acceptance · [scope-v0](../s4-03-concurrency-residual-scope-v0.md) |
| Primary KPI | `cancel_taskgroup_hinge_helper≥1` · cancel_token≥1 · TaskGroup HostFn≥1 · task_tree≥1 · `residual_scope_helper≥1` · `s4_03_claim_helper≥1` |
| Contract | `s4_03_claim_contract` · `s4_03_structured_concurrency_census_contract` |
| Honesty OUT | **≠** Stage-4 exit · **≠** write `stage4-exit` this window · **≠** restore deferred as gate |

## Scoped `[x]` meaning

Under Hard OUT（≠ 改 `syx-async`/语义 / ≠ `withTaskGroup`/EC-M3 / ≠ TSAN·loom GA）, **结构化并发加深** workstream is complete when I97 hinge + I98 scaffolding hold and Primary KPIs stay green — **not** 「Sendable 已产品加深」 / **not** 「`withTaskGroup` 已 GA」 / **not** 「TSAN/loom 已产品化」 / **not** 「Stage-4 exit」。Restoring deferred axes as a claim gate requires a **superseding ADR**.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S4-03 → `[x]`**.
2. Keep **Stage-4 → `[~]`** until an explicit Stage-4 exit ADR（checklist **4/5** items claimed；exit still separate）。
3. Update structured-concurrency census Current=**S3-I99** · **Claimed: scoped `[x]`**；next default = **S3-I100 S4-04 open** — **≠** Stage-4 exit / **≠** S4-04 construct this window.
4. **Non-claims (hard):** ≠ Stage-4 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ write `stage4-exit` this window / ≠ 改 `syx-async`/语义 / ≠ `withTaskGroup`/EC-M3/TSAN·loom GA / ≠ S4-04…05 施工 / ≠ GC Nursery 当并发轴 / ≠ language semantics.

## Consequences

- S4-03 plan-claimed `[x]`; Stage-4 stays `[~]`（checklist **4/5**）.
- **S3-I100 landed** as [stage4-s4-04-open](stage4-s4-04-open.md)（S4-04 `[~]` open · Stage-4 **4/5**）。
- Default next = **S3-I101**（S4-04 custom-Iterator for-in hinge）— not S4-04 `[x]` / not Stage-4 exit / not `makeIterator` GA.
- Prior: [stage4-s4-03-residual-scope](stage4-s4-03-residual-scope.md) · S3-I98 · construct [S3-I97](stage4-s4-03-cancel-taskgroup-hinge.md) · open [S3-I96](stage4-s4-03-open.md).
- Structured-concurrency census Current → **S3-I99** · Claimed scoped `[x]`；sequence census Current → **S3-I100**.

## References

- [stage4-s4-04-open](stage4-s4-04-open.md) · S3-I100
- [stage4-s4-03-residual-scope](stage4-s4-03-residual-scope.md) · S3-I98

- [stage4-s4-03-cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md) · S3-I97
- [stage4-s4-03-open](stage4-s4-03-open.md) · S3-I96
- [s4-03-concurrency-residual-scope-v0](../s4-03-concurrency-residual-scope-v0.md)
- [s4-03-cancel-taskgroup-v0](../s4-03-cancel-taskgroup-v0.md)
- [stage4-s4-02-claim](stage4-s4-02-claim.md) · S3-I95（claim 窗形态）
- [stage4-structured-concurrency-census](../stage4-structured-concurrency-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-03
- [`crates/syx-async`](../../crates/syx-async)
