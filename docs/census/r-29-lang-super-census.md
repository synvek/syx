# R-29 Lang honesty + MIR `super.method` census

**Date:** 2026-11-11  
**Scope:** Manifest/Int spelling honesty; MIR `super.method` → `CallSuper`; array/set `var`+Sendable triage.  
**Script:** [`scripts/r29_lang_super_census.sh`](../../scripts/r29_lang_super_census.sh)

## Progress

| Track | Status |
|-------|--------|
| Cred / SA / R-0…R-28 | exited (wasm **46/46**; browser **46/0/2**) |
| **R-29** | phase **139** / Int `SyxHost.int_compare` / MIR CallSuper any method |
| Object / simd | **186/11** / **159** — unchanged |

## Delivery

1. Honesty pins: inheritance/comparable manifest → 139 / `0.2.0`; Int compare spelling.
2. MIR [`mir_lower_expr_call_member.rs`](../../crates/syx-mir/src/mir_lower_expr_call_member.rs): any `super.method` → `CallSuper`; `init` self-storeback retained.
3. Element triage: array/set mutate via `var` (S060); comment Object→Sendable.
4. Credibility [`r29_lang_super_contract.rs`](../../crates/syx-driver/tests/credibility/r29_lang_super_contract.rs).

## Gates (honest)

| KPI | Target |
|-----|--------|
| `lang_super_failed` (filter) | **0** |
| browser_failed | **0** |
| wasm | **46/46** |
| Object | **≤186 / ≤11** |

**Scoped honesty:** ≠ poly_base_ref / JIT / Int8 wrap / user Comparable sort GA; ≠ simd wipe.

## Still red (out of window)

- `polymorphic_base_ref` / wasm vtable
- JIT differential / contextual Int8 wrap
- User-type `Comparable` sorted witness (e.g. `struct_with_compare_witness_sorted`)
- simd / OrderedSet / json Object bags
