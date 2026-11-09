# Stage-4 structured concurrency census (S4-03)

**ADR:** [open](adr/stage4-s4-03-open.md) · [cancel-taskgroup-hinge](adr/stage4-s4-03-cancel-taskgroup-hinge.md) · [residual-scope](adr/stage4-s4-03-residual-scope.md) · [**claim**](adr/stage4-s4-03-claim.md) · **Window:** S3-I99 · **Current:** S3-I99 · **Claimed:** scoped `[x]`

**Goal (S4-03 scoped acceptance):** Primary = I97 cancel + TaskGroup + task_tree C004 + I98 residual scaffolding；Sendable / `withTaskGroup` / TSAN = **deferred supersede-only** — **≠** Stage-4 exit / **≠** `syx-async` product knife this window.  
**This census:** S3-I99 scoped claim — Stage-4 **`[~]`** · S4-03 **`[x]`** · checklist **4/5**；下一 = **S3-I100** S4-04 open.

## Command

```bash
bash scripts/s4_03_structured_concurrency_census.sh
```

## Classification (hard)

| Class | Counts as S4-03 debt? | Notes |
|-------|----------------------|-------|
| `CancellationToken` + parent | **Primary** | I97 |
| TaskGroup HostFn ×4 | **Primary** | I97 |
| task_tree cancel→C004 | **Primary** | I97 |
| scoped acceptance charter | **Claimed hinge** | I98 |
| S4-03 scoped `[x]` | **Claimed** | I99 |
| Sendable C003 surface | **Deferred / residual** | ≠ claim blocker |
| `@unchecked Sendable` | **Deferred** | residual |
| GC Nursery | **N/A OUT** | ≠ concurrency product |
| `withTaskGroup` / EC-M3 / TSAN·loom | **Deferred / OUT** | ≠ claim blocker |
| S4-04…05 / Stage-4 exit | **No（OUT）** | |

## KPIs（S3-I99 · Scoped claim · 2026-11-09）

| KPI | Value | Notes |
|-----|------:|-------|
| `s4_03_claim_helper` | **≥1** | I99 claim hinge |
| `residual_scope_helper` | **≥1** | I98 |
| `cancel_taskgroup_hinge_helper` | **≥1** | I97 Primary |
| `s4_03_open_helper` | **≥1** | I96 |
| `s4_03_census_helper` | **≥1** | census anchors |
| `cancel_token_helper` | **≥1** | `CancellationToken` |
| `task_group_hostfn_helper` | **≥1** | TaskGroupCreate/Add/Wait/Cancel |
| `sendable_surface_helper` | **≥1** | inventory（deferred） |
| `task_tree_cancel_test_count` | **≥1** | task_tree + chaos cancel |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S4-03 | **`[x]`** scoped；Primary 绿；Sendable/`withTaskGroup` deferred supersede-only |
| Stage-4 | **`[~]`** · **4/5** |
| Next | **S3-I100** S4-04 open — **landed**（[open ADR](adr/stage4-s4-04-open.md)）；sequence tip = **S3-I101** — ≠ Stage-4 exit |

## Ladder (locked)

1. **S3-I95:** S4-02 scoped `[x]` — landed.
2. **S3-I96:** S4-03 open — landed.
3. **S3-I97:** S4-03 cancel/TaskGroup hinge — landed.
4. **S3-I98:** S4-03 residual-scope — landed.
5. **S3-I99:** S4-03 scoped `[x]` claim — landed.
6. **S3-I100:** S4-04 open — **landed**.
7. **S3-I101:** S4-04 Iterator for-in hinge — next tip。

## Non-claims

- **≠** Stage-4 exit / write `stage4-exit` this window  
- **≠** `syx-async` / semantics 行为改  
- **≠** `withTaskGroup` / EC-M3 / TSAN·loom GA  
- **≠** S4-04…05 施工 / HostFn+1 / F-08 unpark  
- **≠** 把 GC Nursery 当并发轴  

## References

- [stage4-s4-03-claim](adr/stage4-s4-03-claim.md) · S3-I99
- [stage4-s4-03-residual-scope](adr/stage4-s4-03-residual-scope.md) · S3-I98
- [s4-03-concurrency-residual-scope-v0](s4-03-concurrency-residual-scope-v0.md)
- [stage4-s4-03-cancel-taskgroup-hinge](adr/stage4-s4-03-cancel-taskgroup-hinge.md) · S3-I97
- [s4-03-cancel-taskgroup-v0](s4-03-cancel-taskgroup-v0.md)
- [stage4-s4-03-open](adr/stage4-s4-03-open.md) · S3-I96
- [plan/02](../plan/02-improvement-roadmap.md) S4-03
- [`crates/syx-async`](../crates/syx-async)
- [`crates/syx-driver/tests/concurrency`](../crates/syx-driver/tests/concurrency)
