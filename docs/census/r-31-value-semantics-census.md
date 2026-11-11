# R-31 值语义 census

**Date:** 2026-11-11  
**Scope:** struct/dataclass assign copy + Array/Dict/Set Ident mutating-call uniquify.  
**Script:** [`scripts/r31_value_semantics_census.sh`](../../scripts/r31_value_semantics_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-30 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-31** | SEM-STRUCT / SEM-ARRAY (+ Dict/Set) cleared |
| Object / simd | **186/11** / **159** — unchanged |

## Root cause

Assignment copied `HeapSlot` indices / `ObjectRef` Arcs. Field writes and collection `write_back_heap_slot` mutated the shared payload.

## Delivery

1. [`mir_lower_ctx.rs`](../../crates/syx-mir/src/mir_lower_ctx.rs) / [`mir_lower_stmt.rs`](../../crates/syx-mir/src/mir_lower_stmt.rs): dataclass/struct `StoreLocal` preceded by `Call("{T}_copy")` (existing shell).
2. [`HostFnId::HeapSlotUniquify`](../../crates/syx-stdlib/src/registry.rs) + [`host_heap_slot_uniquify`](../../crates/syx-vm/src/hosts.rs): clone slot payload into a fresh index (no write-back).
3. [`emit_hosted_collection_call`](../../crates/syx-mir/src/mir_host_cast_helpers.rs): Ident + S060 mutators (`append`/`insert`/`remove`/…) uniquify then call.
4. Fixtures + [`value_semantics.rs`](../../crates/syx-driver/tests/lang/value_semantics.rs).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `value_semantics::*` | 4/4 | **pass** |
| `array_element_check` | pass | **pass** |
| r20 / r22 copy contracts | pass | **pass** |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |

## Non-claims

- ≠ parameter aliasing (`func f(_ p: Point)` still shares)
- ≠ nested deep copy (struct field that is Array/struct may still alias)
- ≠ subscript assign CoW (`b[i] = …`)
- ≠ heap refcount / optimistic CoW
- ≠ enum-as-dataclass assign copy
- ≠ native/wasm AOT-only paths beyond shared Value/hosts
