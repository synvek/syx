# R-38 book run+stdout census

**Date:** 2026-11-11  
**Scope:** todo-005 B1–B2 / E1 constraint — `book_pass_rate` opt-in `sy,output` + stdout compare; SEM-* book pins; cold-start cost evaluated first.  
**Script:** [`scripts/r38_book_run_census.sh`](../../scripts/r38_book_run_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-37 | exited (wasm **46/46**; browser **46/0/2**; Object **186/11**) |
| **R-38** | **exited** — cold-start eval → opt-in run+stdout; SEM book/independent pins |
| Object / simd | **186/11** / unchanged |

## Cold-start evaluation (before / with runner)

Measured with `target/release/syx` on macOS (2026-11-11):

| Sample | Wall (s) | Notes |
|--------|----------|-------|
| hello cold (census script) | **2.637** | fresh temp path |
| hello warmish (new temp, OS cache) | **2.610** | still cold-ish; true warm same-path ≈0.00 earlier probe |
| try_cross_fn / SEM fixtures (pre-window) | **≈2.6–3.1** | each fresh temp dir |

**Decision:** cold start ≈ **2.6–3.1 s/block**. Full language-guide (~300+ fences) × cold ≈ tens of minutes → **not CI-viable**. This window ships **opt-in only** (`sy,output` / `// prints:`); default `sy` remains `check`. Product cold-start embed → **R-40**. Full-book run KPI → later.

## Delivery

1. Cold-start numbers + opt-in-only policy.
2. [`scripts/book_pass_rate.py`](../../scripts/book_pass_rate.py): `output` annotation + `// prints:` expected; `syx run` + stdout compare; `--only-output` for pin census.
3. [`docs/doctest-annotations.md`](../doctest-annotations.md): document opt-in run.
4. SEM-* book pins (en + zh) — 6 en / 6 zh = **12/12** `run_pass`.
5. Independent regressions claimed from R-30…R-36 lang tests (no rewrite).
6. CI `book-pass` **remains soft** (`continue-on-error: true`).

## Independent SEM regressions (already green)

| SEM | Window | Surface |
|-----|--------|---------|
| SEM-THROW | R-30 | `crates/syx-driver/tests/lang/errors.rs` + `fixtures/errors/try_cross_fn*.sy` |
| SEM-STRUCT/ARRAY | R-31 | `…/lang/value_semantics.rs` + `fixtures/lang/value_semantics_*.sy` |
| SEM-INTSTR | R-32 | `…/lang/int_from_string.rs` + fixtures |
| SEM-ITER | R-33 | `…/lang/protocol_iterator.rs` |
| SEM-EXIST | R-34 | `…/lang/protocol_shape_array.rs` |

## Book `sy,output` pins

| SEM | Chapter (en/zh) | Expected |
|-----|-----------------|----------|
| SEM-THROW | `17-error-handling.md` | `caught` |
| SEM-STRUCT | `10-structures-and-classes.md` | `1\n9` |
| SEM-ARRAY | `05-collection-types.md` | `1\n2` |
| SEM-INTSTR | `02-the-basics.md` | `42\n43\nnil` |
| SEM-ITER | `23-protocols.md` | `6` |
| SEM-EXIST | `23-protocols.md` | `circle\nsquare\n2156.636` |

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| cold-start eval recorded | yes | **yes** (hello_cold_s=**2.637**) |
| opt-in `output` blocks green | all | **12/12** (en 6 + zh 6) |
| browser_failed | **0** | **0** (46/0/2) |
| wasm | **46/46** | **46/46** |
| Object | **≤186 / ≤11** | **186/11** |
| book-pass CI | soft (R-37) | **remains soft** |

## Non-claims

- ≠ Full-book default `run` / book-pass 100% hard again
- ≠ R-40 E1 prelude embed / cold-start &lt;0.3 s product
- ≠ R-39 full-native link
- ≠ Object/simd / LANG-POLY / zh CI matrix
