# S4-03 cancel / TaskGroup v0（S3-I97）

**ADR:** [cancel-taskgroup-hinge](../adr/stage4-s4-03-cancel-taskgroup-hinge.md) · **Window:** S3-I97  
**Purpose:** Charter for Stage-4 **Primary** acceptance of S4-03「结构化并发加深」— elevate existing cancel + TaskGroup evidence to a credibility hinge.  
**≠** change [`syx-async`](../../crates/syx-async) / semantics · **≠** `withTaskGroup` GA · **≠** Sendable Primary this window.

## Primary（in scope for later scoped `[x]`）

| Axis | Meaning | Evidence anchors |
|------|---------|------------------|
| CancellationToken | Parent-propagating cancel token | [`cancellation.rs`](../../crates/syx-async/src/cancellation.rs) |
| TaskGroup HostFn | Create / Add / Wait / Cancel | [`host_fn_capabilities.toml`](../../packages/internal/host_fn_capabilities.toml) |
| Cancel → C004 nails | Group cancel wait / chaos interrupt | [`task_tree.rs`](../../crates/syx-driver/tests/concurrency/task_tree.rs)；[`chaos_sampling.rs`](../../crates/syx-driver/tests/concurrency/chaos_sampling.rs) |

## Deferred / residual / supersede-only（not blockers for Primary）

| Axis | Notes |
|------|-------|
| Sendable / `@unchecked` deepen | Inventory green；residual-scope |
| `withTaskGroup` / EC-M3 | OUT until superseding ADR |
| TSAN / loom GA | Product concurrency stress GA OUT |
| GC Nursery | **N/A** — ≠ concurrency product |

## Out（Hard）

- S4-03 `[x]` this window · Stage-4 exit · S4-04…05 construction  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- Language semantics greenfield  

## Ladder

- **S3-I97:** this charter + hinge ADR + census Primary KPIs  
- **S3-I98:** residual-scope（scoped acceptance vs deferred Sendable/`withTaskGroup`）— **landed**（[residual ADR](../adr/stage4-s4-03-residual-scope.md)）  
- **S3-I99:** dedicated S4-03 scoped `[x]` claim（next tip）  
