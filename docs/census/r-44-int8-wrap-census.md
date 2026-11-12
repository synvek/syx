# R-44 Int8 contextual wrap census

**Date:** 2026-11-12  
**Scope:** Fixed-width Int8 wrap for `<<` and compound `+=` / `<<=` (contextual literals).  
**Script:** [`scripts/r44_int8_wrap_census.sh`](../../scripts/r44_int8_wrap_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-43 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**; simd **159**) |
| **R-44** | **exited** — Int8 wrap cluster green (3 nails) |
| LANG-POLY residual | user Comparable sort + JIT Suite-1 |

## Root cause

1. `BinOp::Shl`/`Shr` used `widen_for_binary(left, right)` so `Int8 << Int` ran at platform Int → printed `128` not `-128`.
2. Compound `x += 10` desugared to widened `Int8 + Int` then Assign; Binary has no `infer_expr_init_type` → no wrapping narrow cast → printed `130`.

## Delivery

1. MIR ([`mir_lower_expr_binary.rs`](../../crates/syx-mir/src/mir_lower_expr_binary.rs)): shifts use **left** operand kind only.
2. Semantic ([`semantic_analyze_expr.rs`](../../crates/syx-semantic/src/semantic_analyze_expr.rs)): shift result type follows left operand.
3. Compound ([`mir_lower_expr_compound_assign.rs`](../../crates/syx-mir/src/mir_lower_expr_compound_assign.rs)): fixed-width integer LHS ops at LHS width (wrap via `value_from_numeric`).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| `binary_int8_shl_wraps` | `-128` | **pass** |
| `compound_assign_int8_plus_wraps` | `-126` | **pass** |
| `compound_assign_int8_shl_wraps` | `-128` | **pass** |
| `contextual_int_literal` int8 filter | failed=0 | **pass** |
| browser_failed | **0** | hold |
| wasm | **46/46** | hold |
| Object | **≤180 / ≤10** | hold |
| simd residual | **≥159** | hold |

## Non-claims

- ≠ user Comparable `sorted` witness GA
- ≠ JIT Suite-1 / `SYX_JIT_TIER2` GA
- ≠ full inheritance / simd wipe / json bags
- ≠ changing free `Int8 + Int` widen (still widens; see `binary_int8_plus_int_widens`)

## Still red (→ later)

- ~~User-type `Comparable` sorted witness~~ → **cleared in [R-45](r-45-comparable-sort-census.md)**
- JIT Suite-1 differential → park

## References

- [`r-29-lang-super-census`](r-29-lang-super-census.md) · Int8 still-red cleared here
- [`r-43-poly-base-ref-census`](r-43-poly-base-ref-census.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-44 → R-45 Comparable → next **R-46**
