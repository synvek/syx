# R-26 frontend red census

**Date:** 2026-11-11  
**Scope:** Swift-style closure statement-body parse hole; frontend red accounting.

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-25 | exited (wasm **46/46**) |
| **R-26** | Swift `{ (…) in return … }` → Block via `looks_like_closure_stmt_body` |
| Object residual | was **198 / 14** (simd 159); **R-27** → **186 / 11** (Array/Dict/Set Sendable) |

## Delivery

1. [`parse_swift_closure`](../../crates/syx-parser/src/lib.rs) calls `looks_like_closure_stmt_body` (empty / nested `{` / stmt / expr).
2. Parser unit tests `r26_swift_closure_*`.
3. `hof_closure` temp dirs uniquified (parallel race → spurious P001).
4. `syx-mir` test-only: `ConstFnIndex` witness; `id(x: 42)` for S042.
5. Credibility `r26_*`; script [`scripts/r26_frontend_red_census.sh`](../../scripts/r26_frontend_red_census.sh).

## Gates (honest)

| KPI | Target | Notes |
|-----|--------|-------|
| `tutorial_p001` | **0** | all `tutorial/*.sy` |
| `lang_closure_failed` | **0** | map/filter/reduce/flatMap/sortBy/closures_book filter |
| `hof_closure` VM/MIR/WASM | green | native **L006** may fail without `full-native` |
| `mir_lib_failed` | **≤1** | prefer 0 |
| `wasm` | **46/46** | regression |
| Object | **198/14** | R-26 parked; see R-27 census for **186/11** |

## Still red (out of window)

- `lang` inheritance / poly / comparable clusters
- `browser` suite (~22)
- SIMD element typing / OrderedSet Object-shell (R-27 left parked)

**Next:** [`r-27-collections-object-census.md`](r-27-collections-object-census.md).
