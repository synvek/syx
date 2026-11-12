# R-43 polymorphic_base_ref / vtable census

**Date:** 2026-11-12  
**Scope:** Erased base-class reference virtual dispatch (`make() -> Animal` → `a.speak()` → `woof`); MIR `__vtable_*` + `CallIndirect`; wasm AOT table dispatch.  
**Script:** [`scripts/r43_poly_base_ref_census.sh`](../../scripts/r43_poly_base_ref_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-42 | exited (wasm **46/46**; browser failed=**0**; Object **≤180/10**; simd **159**) |
| **R-43** | **exited** — erased poly base_ref embed+wasm green; call-site vtable |
| LANG-POLY residual | R-44 JIT / Int8 wrap / user Comparable sort |

## Root cause

1. Weak fixture `let a: Animal = Dog()` could retain concrete `Dog` → static `Call Dog_speak` (wasm printed `woof` by luck).
2. Erased factory path lowered to static base `Call Animal_speak` on wasm/LLVM (no VM `resolve_virtual_method`).
3. MIR `CallIndirect` was unmapped on wasm AOT sync path (`W001`).

## Delivery

1. Harden fixture + tests to erased `make() -> Animal`; drop `;` P001 / native-strict L006 false reds from hard gate.
2. MIR ([`mir_lower_expr_call_member.rs`](../../crates/syx-mir/src/mir_lower_expr_call_member.rs)): class receiver with vtable slot → `ObjectGet("__vtable_*")` + `CallIndirect` (R-34 witness shape; skip init/deinit/platform).
3. Wasm AOT ([`mir_emit.rs`](../../crates/syx-wasm-emit/src/mir_emit.rs)): `CallIndirect` → `TakeRtI32` + table `call_indirect` (same as static `Call`).
4. MIR asserts require call-site `CallIndirect` **and** `__vtable_*` (not init-slot alone).

## Gates (honest)

| KPI | Target | Result |
|-----|--------|--------|
| erased embed `woof` | green | **pass** |
| erased wasm `woof` | green | **pass** |
| `lang_poly_base_ref_failed` | **0** | **0** (≥8 passed) |
| call-site `CallIndirect` + `__vtable_*` | required | **pass** |
| browser_failed | **0** | **0** |
| wasm | **46/46** | **46/46** |
| Object | **≤180 / ≤10** | hold |
| simd residual | **≥159** | **159** |

## Non-claims

- ≠ full inheritance GA
- ≠ native GA without `--features full-native` (soft; not exit gate)
- ≠ R-44 JIT differential / Int8 wrap / user Comparable sort
- ≠ simd Object wipe / json bags / cold-start E1

## Still red (→ later windows)

- ~~contextual Int8 wrap~~ → **cleared in [R-44](r-44-int8-wrap-census.md)**
- User-type `Comparable` sorted witness → **next default**
- JIT differential → park
- simd Object façade / json bags → later backlog

## References

- [`r-29-lang-super-census`](r-29-lang-super-census.md) · still-red poly cluster cleared here
- [`r-34-existential-shape-census`](r-34-existential-shape-census.md) · witness CallIndirect shape
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · R-43 → R-44 Int8 → next Comparable
