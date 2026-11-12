# R-45 user Comparable sort census

**Date:** 2026-11-12  
**Scope:** User-type `Comparable` + `Array.sorted()` via `__witness_Comparable_compare` (order `1/2/3`).  
**Script:** [`scripts/r45_comparable_sort_census.sh`](../../scripts/r45_comparable_sort_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-44 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**; simd **159**) |
| **R-45** | **exited** — Comparable class sort order-strict green |
| LANG-POLY residual | JIT Suite-1 **parked** |
| P2 books…docs | renumbered **R-46…R-49** |

## Root cause

1. [`emit_protocol_witnesses`](../../crates/syx-mir/src/mir_lower_class_init.rs) / vtable `ObjectSet` without `StoreLocal(self)` → stack garbage; `[Score(...),…]` ArrayBuild aliased last instance → `2/2/2`.
2. [`comparable_witness_fn_index`](../../crates/syx-vm/src/lib.rs) only bare `Object`/`Dict`+`I32`; class is `ObjectRef`/`MutRefCell`; `ConstFnIndex` often `I64` → no witness reorder.

## Delivery

1. MIR: `ObjectSet` + `StoreLocal(self_slot)` for protocol witnesses and class vtable slots.
2. VM + mir-run: `mut_ref_cell_get_field("__witness_Comparable_compare")` + `I32`/`I64` (R-33 Iterator shape).
3. Nail: order-strict `["1","2","3"]` on `struct_with_compare_witness_sorted`.

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `struct_with_compare_witness_sorted` | order `1/2/3` | **pass** |
| Int/String + parity fixture | green | **pass** |
| browser_failed | **0** | hold |
| wasm | **46/46** | hold |
| Object | **≤180 / ≤10** | hold |
| simd residual | **≥159** | hold |

## Non-claims

- ≠ JIT Suite-1 / `SYX_JIT_TIER2` GA
- ≠ Equatable protocol GA / full inheritance
- ≠ simd wipe / json bags / R-46 books Swift surface

## Still red (→ later)

- JIT Suite-1 differential → park
- simd Object façade / json bags → later backlog

## References

- [`r-33-iterator-witness-census`](r-33-iterator-witness-census.md) · ObjectRef witness pattern
- [`r-44-int8-wrap-census`](r-44-int8-wrap-census.md) · left Comparable residual here
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-45 → next **R-46**
