# R-46 books Ch17 Error surface census

**Date:** 2026-11-12  
**Scope:** Language-guide Ch17 Error Handling en+zh teaching surface (Error throw / `catch` / `if !x`).  
**Script:** [`scripts/r46_books_ch17_census.sh`](../../scripts/r46_books_ch17_census.sh) · sync nail [`scripts/r46_ch17_fence_sync.py`](../../scripts/r46_ch17_fence_sync.py)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-45 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**; simd **159**) |
| **R-46** | **exited** — Ch17 en+zh Error surface (≠ full-book F1) |
| Open | R-47…R-49；`func…async` parser / JIT **park** |

## Root cause

Ch17 taught non-Swift surfaces that still `check`: `throw "…"`, `catch (err)`, `if (!x)`, and String-as-primary error MVP narrative ([todo-005](../../plan/todo-005.md) F1).

## Delivery

1. Rewrite [`books/language-guide/17-error-handling.md`](../../books/language-guide/17-error-handling.md) + zh twin: `enum …: Error`, `catch` / `catch err`, `if !…`.
2. Narrative: string throw = residual MVP.
3. Keep SEM-THROW `sy,output` (`// prints: caught`).
4. Thin F5: en↔zh Ch17 `syx` fence bodies identical (`r46_ch17_fence_sync.py`).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| Ch17 fence `throw "` / `catch (` / `if (!` | **0** | **0** |
| en↔zh fence sync | identical bodies | **pass** (`fence_count=12`) |
| `sy,output` strict (en+zh) | **12/12** | **12/12** |
| browser_failed | **0** | **0** (46 passed) |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | **180 / 10** |
| simd residual | **≥159** | **159** |

## Non-claims

- ≠ `func f() async` / `catch let` parser productization
- ≠ Ch07/Ch18 async chapter rewrite / full-tree F5
- ≠ language ban of `throw String`
- ≠ R-47 package API / JIT unpark / simd wipe / hard book-pass 100%

## References

- [`r-38-book-run-census`](r-38-book-run-census.md) · opt-in output pins
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-46 → next **R-47**
