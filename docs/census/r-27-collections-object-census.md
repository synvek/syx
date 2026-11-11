# R-27 Collections Object→Sendable census

**Date:** 2026-11-11  
**Scope:** Public Array / Dictionary / Set element·key façades off `Object`.  
**Script:** [`scripts/r27_collections_object_census.sh`](../../scripts/r27_collections_object_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-26 | exited (wasm **46/46**) |
| **R-27** | Array/Dict/Set `item`/`key: Object` → `Sendable` |
| Object residual | **186 / 11** (was 198/14; −12 / −3) |
| simd | **159** — still parked |
| OrderedSet | Object-shell MVP — out of window |

## Delivery

1. R-13 inventory pin retired to **ceiling** (`≤198` / `≤14`); absolute owned by R-27.
2. Spelling: [`array.sy`](../../packages/core/builtins/collections/array.sy), [`dictionary.sy`](../../packages/core/builtins/collections/dictionary.sy), [`set.sy`](../../packages/core/builtins/collections/set.sy).
3. Credibility [`r27_collections_sendable_contract.rs`](../../crates/syx-driver/tests/credibility/r27_collections_sendable_contract.rs).

## Gates (honest)

| KPI | Target |
|-----|--------|
| `collections_array_dict_set_object` | **0** |
| `object_sig_residual_count` | **≤186** |
| `object_sig_residual_files` | **≤11** |
| `simd_object_residual` | **≥159** |
| wasm | **46/46** regression |

**Scoped honesty:** SyxHost may keep `Object` (or Dict key `String`) internals. ≠ `Element`/`Key` generics GA; ≠ OrderedSet product type; ≠ simd wipe; ≠ lang inheritance / browser green.

## Still red (out of window)

- `lang` inheritance / poly / comparable
- simd / OrderedSet / json Object bags

**Next:** [`r-28-browser-red-census.md`](r-28-browser-red-census.md) closed browser **22→0**.
