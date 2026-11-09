# ADR: Stage-4 S4-03 open — structured concurrency deepen（S3-I96）

**Status:** Accepted (S3-I96) · Stage-4 `[~]` · S4-03 `[~]` · S4-01/`02` `[x]`  
**Date:** 2026-11-09  
**Context:** After [S3-I95 S4-02 scoped claim](stage4-s4-02-claim.md), default credibility work advances to [plan/02 S4-03](../../plan/02-improvement-roadmap.md): **结构化并发加深**（取消 / TaskGroup / Sendable）. M3 MVP inventory is already thick; this window **opens** S4-03 with a structured-concurrency **census baseline** — it does **not** claim S4-03 `[x]`, change `syx-async`/semantics product behavior, declare `withTaskGroup`/EC-M3 GA, or exit Stage-4.

## Gap: structured concurrency vs plan S4-03

| Claim | Today (honest) | plan/02 S4-03 target |
|-------|----------------|----------------------|
| 取消 | [`CancellationToken`](../../crates/syx-async/src/cancellation.rs)；parent 传播；[`task_tree`](../../crates/syx-driver/tests/concurrency/task_tree.rs) cancel→wait/C004 | 协作取消 / 产品面可验收加深（非从 0） |
| TaskGroup | HostFn `TaskGroupCreate`/`Add`/`Wait`/`Cancel`（[`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) implemented）；书称 M3 风格 ≠ `withTaskGroup` | 结构化组契约加深 / scoped KPI 可宣称 |
| Sendable | [`semantic_concurrency.rs`](../../crates/syx-semantic/src/semantic_concurrency.rs) C003/S019；[`concurrency_surface`](../../crates/syx-driver/tests/concurrency/concurrency_surface.rs)；`@unchecked Sendable` **partial** | 检查面加深 / 诚实边界 |
| GC Nursery | 仓库 `Nursery` ≈ GC nursery — **≠ 并发产品** | **N/A OUT** |
| Stage-4 claim | **无** S4-03 scoped `[x]` / **无** 本轴 census | census → construct → claim |

Baseline: [stage4-structured-concurrency-census](../census/stage4-structured-concurrency-census.md).

## Decision

1. **Open S4-03** as the default construction axis; flip plan S4-03 `[ ]` → `[~]`.
2. Keep **Stage-4 → `[~]`** with checklist **2/5**（S4-01/`02` already claimed；本窗不抬分母）.
3. **S3-I96 first slice** — publish structured-concurrency census (+ script); add a thin contract under `syx-driver/tests/`（**no** `syx-async` / semantics product edit this window）.
4. **Ladder:** **S3-I97** = first construct（**cancel + TaskGroup structured-group KPI hinge** — elevate 现有证据为 machine-checkable Primary，仿 I93）；Sendable/`@unchecked` 深化可 residual → dedicated **S4-03 `[x]` claim**.
5. **Non-claims (hard):** ≠ S4-03 `[x]` / ≠ S4-03 construct（改 `syx-async`/语义行为）/ ≠ Stage-4 exit / ≠ `withTaskGroup` / EC-M3 / TSAN·loom GA / ≠ S4-04…05 施工 / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ 把 GC Nursery 当并发轴 / ≠ language semantics greenfield.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Structured-concurrency census | [`docs/census/stage4-structured-concurrency-census.md`](../census/stage4-structured-concurrency-census.md) · `scripts/s4_03_structured_concurrency_census.sh` |
| Contract nail | `s4_03_structured_concurrency_census_contract` |
| Plan status | Stage-4 `[~]` · S4-03 `[~]` · checklist **2/5** under this ADR |

## Consequences

- S4-03 moves `[ ]` → `[~]`；Stage-4 stays `[~]`（**2/5**）.
- **S3-I97 landed** as [stage4-s4-03-cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md)（cancel/TaskGroup Primary hinge）。
- **S3-I98 landed** as [stage4-s4-03-residual-scope](stage4-s4-03-residual-scope.md)（scoped acceptance）。
- **S3-I99 landed** as [stage4-s4-03-claim](stage4-s4-03-claim.md)（S4-03 scoped `[x]` · Stage-4 **3/5**）。
- Default next = **S3-I100**（S4-04 open）— not Stage-4 exit / not `withTaskGroup` GA.
- Prior: [stage4-s4-02-claim](stage4-s4-02-claim.md) · S3-I95 · [stage4-s4-02-open](stage4-s4-02-open.md) · S3-I92.
- Structured-concurrency census Current → **S3-I99** · Claimed scoped `[x]`。

## References

- [stage4-s4-03-claim](stage4-s4-03-claim.md) · S3-I99
- [stage4-s4-03-residual-scope](stage4-s4-03-residual-scope.md) · S3-I98
- [stage4-s4-03-cancel-taskgroup-hinge](stage4-s4-03-cancel-taskgroup-hinge.md) · S3-I97
- [stage4-s4-02-claim](stage4-s4-02-claim.md) · S3-I95
- [stage4-s4-02-open](stage4-s4-02-open.md) · S3-I92（开窗形态）
- [stage4-structured-concurrency-census](../census/stage4-structured-concurrency-census.md)
- [s4-03-cancel-taskgroup-v0](../census/s4-03-cancel-taskgroup-v0.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S4-03
- [`crates/syx-async`](../../crates/syx-async)
- [`crates/syx-semantic/src/semantic_concurrency.rs`](../../crates/syx-semantic/src/semantic_concurrency.rs)
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §阶段 4
