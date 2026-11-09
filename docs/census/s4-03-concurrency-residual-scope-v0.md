# S4-03 concurrency residual-scope v0（S3-I98）

**ADR:** [residual-scope](../adr/stage4-s4-03-residual-scope.md) · **Window:** S3-I98  
**Purpose:** Bound scoped acceptance vs deferred axes for S4-03「结构化并发加深」— documentation only.  
**≠** change [`syx-async`](../../crates/syx-async) / semantics · **≠** `withTaskGroup` GA · **≠** TSAN/loom GA.

## Primary（already landed — claim-ready）

| Item | Status |
|------|--------|
| CancellationToken + TaskGroup HostFn | [I97](../adr/stage4-s4-03-cancel-taskgroup-hinge.md) · [v0](s4-03-cancel-taskgroup-v0.md) |
| task_tree / chaos cancel→C004 | census Primary greens |
| Open inventory | [I96](../adr/stage4-s4-03-open.md) |

## Deferred / supersede-only（not blockers for scoped `[x]`）

| Axis | Meaning |
|------|---------|
| Sendable / `@unchecked` deepen | Check-surface honesty beyond inventory |
| `withTaskGroup` / EC-M3 | Language sugar / EC product claim |
| TSAN / loom GA | Concurrency stress product GA |

Restoring any as a **hard claim gate** requires a **superseding ADR**.

## Out（Hard）

- S4-03 `[x]` this window · Stage-4 exit · S4-04…05 construction  
- HostFn+1 · F-08 · Wave / product-umbrella unpark  
- GC Nursery as concurrency axis · Language semantics greenfield  

## Ladder

- **S3-I98:** this charter + residual-scope ADR  
- **S3-I99:** dedicated S4-03 scoped `[x]` claim under Primary above — **landed**（[claim ADR](../adr/stage4-s4-03-claim.md)）  
- **S3-I100:** S4-04 open（next tip）  
