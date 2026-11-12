# R-41 OrderedSet Object-shell census

**Date:** 2026-11-12  
**Scope:** R-27 park / todo-005 F4 thin slice — public OrderedSet Object-shell → `OrderedSet` / `Sendable`; inventory **−6**.  
**Script:** [`scripts/r41_ordered_set_census.sh`](../../scripts/r41_ordered_set_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-40 | exited (wasm **46/46**; browser failed=**0**) |
| **R-41** | **exited** — OrderedSet façade Sendable; inventory **180/10** |
| simd | **159** — still parked (**explicit slice**: not this window) |
| json Object bags | parked (**explicit slice**: not this window) |

## Charter (explicit slice)

This window clears **only** OrderedSet public Object signatures. It does **not** co-ship simd Object 159 honesty (R-42) or json Object bags. Ledger line “须与 simd/json 章程同窗或显式切片” → **显式切片**.

## Delivery

1. [`ordered_set.sy`](../../packages/core/builtins/collections/ordered_set.sy): `new`/`insert`/`contains` use `OrderedSet` + `Sendable`.
2. Light host façade align in [`system_host.sy`](../../packages/internal/system_host.sy); HostFn return types `OrderedSet` in registry; `TYPE_METHODS` for OrderedSet (Deque-parity).
3. Host `ordered_set_new` MutRefCell shaped with `__syx_ref` ObjectRef inner (field API); lang smoke `ordered_set_shell`.
4. R-27 census/contract: `ordered_set_object_shell_residual` → **0**; Object ceiling → **≤180/≤10**.

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `ordered_set_object_shell_residual` | **0** | **0** |
| `object_sig_residual_count` | **≤180** | **180** |
| `object_sig_residual_files` | **≤10** | **10** |
| `simd_object_residual` | **≥159** (parked) | **159** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |

## Non-claims

- ≠ `OrderedSet<T>` generics / product GA
- ≠ host capability flip to production-supported
- ≠ todo-005 F4 Collections package merge
- ≠ simd Object wipe / json bag rewrite
- ≠ LANG-POLY / JIT / cold-start E1

## References

- [`r-27-collections-object-census`](r-27-collections-object-census.md) · OrderedSet was out of window
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-41 → next **R-42**
