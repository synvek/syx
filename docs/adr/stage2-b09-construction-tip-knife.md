# ADR: Stage-2 B-09 construction tip knife (S2-I19+)

**Status:** Accepted (S2-I19)  
**Date:** 2026-11-02  
**Context:** After S2-I18, five B-09 product flags are policy-fixed and tip product inventories×4 are gone (census **78**). Remaining b09 mass is a linear construction-chain tip starting at `post_mutrefcell_peel_refresh`. HostFn×8 still only yields tip −1. Precedent: [stage2-b09-product-flag-decouple](stage2-b09-product-flag-decouple.md), [stage2-cutover-decouple](stage2-cutover-decouple.md).

## Decision

1. **After product-flag decouple, orphaned construction tip packs may be deleted sequentially** without rewiring flags or product emit/GC behavior.
2. **S2-I19 deletes tip peel 两联×6** (mutrefcell peel triad + reentry peel triad):
   - `gate_b09_objectref_rc_migrate_readiness_post_mutrefcell_peel_refresh_thin_inventory`
   - `gate_b09_mutrefcell_maybe_track_peel_delete_first_slice_inventory`
   - `gate_b09_mutrefcell_maybe_track_peel_readiness_thin_inventory`
   - `gate_b09_objectref_rc_migrate_readiness_post_reentry_peel_refresh_thin_inventory`
   - `gate_b09_objectref_reentry_maybe_track_peel_delete_first_slice_inventory`
   - `gate_b09_objectref_reentry_maybe_track_peel_readiness_thin_inventory`
3. **Expected census (S2-I19):** **78 → 72 (−6)**. New tip ≈ `post_collapse_refresh` / dual_track triad.
4. **Later windows may continue tip packs under this ADR** (e.g. dual_track 三联) without a new decision document, while keeping HostFn×8 / `gc_narrative_thin` / deeper chain OUT unless separately decided.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ rewriting maybe_track/ObjectRef/GC product semantics / ≠ B-09 GA change / ≠ deleting HostFn×8 / ≠ HostFn+1.

## Consequences

- Driver companions for deleted delete-slices are removed with the inventories.
- Historical Wave docs may still name deleted symbols as archival evidence.
- **S2-I20:** dual_track+mirop tip packs×6 deleted; census **72 → 66**; new tip ≈ objectref peel triad.
- **S2-I21:** objectref peel tip pack×3 deleted; census **66 → 63**; new tip ≈ deep_field 两联.
- **S2-I22:** deep_field+strip_rc tip packs×4 deleted; census **63 → 59**; new tip ≈ handle 三联.
- **S2-I23:** handle tip pack×3 deleted; census **59 → 56**; new tip ≈ nested_field.
- **S2-I24:** nested_field tip pack×1 deleted; census **56 → 55**; new tip ≈ refresh_thin.
- **S2-I25:** refresh+wrap+value tip packs×3 deleted; census **55 → 52**; new tip ≈ storelocal_deinits.
- **S2-I26:** death-path tip packs×4 deleted; census **52 → 48**; new tip ≈ rc_release_opcode.
- **S2-I27:** opcode retire tip packs×3 deleted; census **48 → 45**; new tip ≈ rc_retire_readiness_thin.
- **S2-I28:** retire-readiness tip packs×3 deleted; census **45 → 42**; new tip ≈ rc_migrate_readiness_thin.
- **S2-I29:** migrate_readiness tip pack×1 deleted; census **42 → 41**; new tip ≈ ssa_loadlocal_heap_slots.
- **S2-I30:** heap_slots tip packs×3 deleted; census **41 → 38**; new tip ≈ llvm_objectref_wrap_emit_retire.
- **S2-I31:** llvm wrap emit tip pack×1 deleted; census **38 → 37**; new tip ≈ arrayget_heap_slots.
- **S2-I32:** arrayget+residual_birth tip packs×2 deleted; census **37 → 35**; new tip ≈ llvm_wasm_objectref_rc_emit_retire.
- **S2-I33:** llvm_wasm emit-retire tip pack×1 deleted; census **35 → 34**; new tip ≈ retain_catch_all_heap_slots.
- **S2-I34:** retain/method/actor tip packs×3 deleted; census **34 → 31**; new tip ≈ mir_run_call_return_heap_slots.
- **S2-I35:** call-return tip packs×2 deleted; census **31 → 29**; new tip ≈ mutrefcell_mir_run_callhost_heap_slots.
- **S2-I36:** mutrefcell CallHost tip packs×2 deleted; census **29 → 27**; new tip ≈ mutrefcell_mir_run_dictbuild_align.
- **S2-I37:** mutrefcell DictBuild tip packs×2 deleted; census **27 → 25**; new tip ≈ objectref_mir_run_callhost_heap_slots.
- **S2-I38:** objectref CallHost tip packs×2 deleted; census **25 → 23**; new tip ≈ objectref_mir_run_dictbuild_align.
- **S2-I39:** objectref DictBuild tip pack×1 deleted; census **23 → 22**; new tip ≈ objectref_vm_heap_slots.
- **S2-I40:** objectref vm_heap_slots tip pack×1 deleted; census **22 → 21**; new tip ≈ objectref_vm_managed_heap_probe.
- **S2-I41:** objectref vm_managed_heap_probe tip pack×1 deleted; census **21 → 20**; new tip ≈ objectref_rc_migrate.
- **S2-I42:** objectref_rc_migrate tip pack×1 deleted; census **20 → 19**; new tip ≈ memory_pool_dead_shell.
- **S2-I43:** memory_pool_dead_shell tip pack×1 deleted; census **19 → 18**; new tip ≈ storage_cleanup.
- **S2-I44:** storage_cleanup tip pack×1 deleted; census **18 → 17**; new tip ≈ enable_readiness.
- **S2-I45:** tip packs×4 deleted (enable_readiness+storage_honesty+fully_converged+dual_heap); census **17 → 13**; new tip ≈ actor_mailbox; **later windows default tip packs Δ3–4**.
- **S2-I46:** tip packs×4 deleted (actor_mailbox+vm_gccollect+mir_runtime_root_set+gc_narrative_deepen); census **13 → 9**; remaining ≈ gc_narrative_thin + HostFn×8.
- **S2-I47:** orphan thin + HostFn tip packs moved to [stage2-hostfn-construction-tip-knife](stage2-hostfn-construction-tip-knife.md); census **9 → 5**.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-b09-product-flag-decouple](stage2-b09-product-flag-decouple.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
