# ADR: Stage-2 fingerprint foundation aggregator fold (S2-I86)

**Status:** Accepted (S2-I86)  
**Date:** 2026-11-04  
**Context:** After S2-I85, `heap_v111_*` tips are folded into `track_l`. Remaining on-aggregator slots include foundation×12 + `heap_v100_…_track_c` + `track_l`. The general [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md) required a **separate gate** before folding foundation. `module_hash` also calls three foundation helpers (`witness_table` / `vtable_layout` / `wasm_aot_tier`).

## Decision

1. **Authorize fold** of foundation `*_fingerprint` helpers under this ADR (merge their invalidation inputs into a kept successor; delete folded `*_fingerprint` defs; align `compute_cache_version_tag` `format!` in the same window). **Bare peel is forbidden.**
2. **S2-I86 pack Δ12** — merge into `heap_v100_phase128_track_c_wasm_fingerprint` (kept name); delete:
   - `host_feature_fingerprint`
   - `witness_table_fingerprint`
   - `vtable_layout_fingerprint`
   - `wasm_aot_tier_fingerprint`
   - `native_llvm_coverage_fingerprint`
   - `aot_fixture_set_fingerprint`
   - `sorted_codegen_fingerprint`
   - `tier2b_jit_llvm_fingerprint`
   - `tier2c_simd_browser_fingerprint`
   - `iterator_witness_native_tier2c_fingerprint`
   - `gc_arc_browser_prelude_fingerprint`
   - `unified_heap_native_exec_op_fingerprint`
3. **Byte order inside merge target:** host → witness → vtable → wasm_aot → native_llvm → aot_fixture → sorted → tier2b → tier2c → iterator → gc_arc → unified_heap → original v100_c statements.
4. **String-typed foundation** (e.g. `host_feature`, `wasm_aot_tier`): feed via `hasher.update(…as_bytes())`. **SHA-typed**: merge original `hasher.update` statements.
5. **`module_hash` same-window:** rename the three still-needed helpers to non-`*_fingerprint` digests (`witness_table_digest` / `vtable_layout_digest` / `wasm_aot_tier_digest`); `module_hash` and the merge target consume those digests' string bytes. **Do not** bind the whole `v100_c` into `module_hash`.
6. Soft-nail retarget → merge target: `reflect_witness_mandatory` (`witness_table_fingerprint`) + `inheritance` (`vtable_layout_fingerprint`).
7. **Expected census:** `fingerprint_fn_matches` **177 → 165 (−12)**; report **0**; inventory **0**.
8. **MUST STAY (at acceptance):** `heap_v111_…_track_l` (later folded under [stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md) in S2-I87); `farm_lease_*_fingerprint` product APIs; capabilities product API; `lang_props_v*`; five B-09 product flags; `llvm_stub_audit_report`.
9. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ product-sea carve this window.

## Consequences

- Outer cache tag loses twelve foundation slots; `v100_c` hex changes (expected invalidation).
- Aggregator-fold ADR foundation MUST STAY wording is superseded by this gate for the folded slots.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-aggregator-fold](stage2-fingerprint-aggregator-fold.md)
- [stage2-fingerprint-v111-fold](stage2-fingerprint-v111-fold.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
