# ADR: Stage-2 B-09 product-flag tip decouple (S2-I18)

**Status:** Accepted (S2-I18)  
**Date:** 2026-11-02  
**Context:** After S2-I17, census is **82** (`gate_b09`≈74 + HostFn×8). Five readonly product flags (`objectref_rc_migrate_enabled`, `storage_cleanup_enabled`, `unified_mir_runtime_heap_enabled`, `b09_axis_exit_enabled`, `gc_ga_claimed`) only walk four tip inventories (W555–W558 product flips). Emit/VM/LLVM paths do not call these flags. HostFn thin ADR yields only −1; unlocking meaningful S2-01 progress requires opening the b09 tip. Precedent: [stage2-hostfn-align-decouple](stage2-hostfn-align-decouple.md).

## Decision

1. **Product flags no longer walk tip inventories.** The five flags are **policy-fixed `true`** (matching W555–W558 live product flips).
2. **This window deletes tip product inventories×4** (struct / `_once` / pub fn / unit / re-export / driver companions):
   - `gate_b09_objectref_rc_migrate_product_first_slice_inventory`
   - `gate_b09_storage_cleanup_product_first_slice_inventory`
   - `gate_b09_unified_mir_runtime_heap_enable_product_first_slice_inventory`
   - `gate_b09_axis_exit_gc_ga_product_first_slice_inventory`
3. **Expected census:** **82 → 78 (−4)**.
4. **MUST STAY:** remaining b09 construction-chain inventories (~70), HostFn×8, `llvm_stub_audit_report`, the five flag **API names** (implementation only changes).
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ rewriting ManagedHeap/ObjectRef/GC collect product behavior / ≠ B-09 GA semantic change / ≠ deleting HostFn×8 or construction chain / ≠ HostFn+1.

## Consequences

- Later windows may delete orphaned construction-chain packs from the new tip (`post_mutrefcell_peel_refresh` region) under follow-on ADRs.
- Historical W555–W558 / gate docs may still name deleted tip inventory symbols as archival evidence.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-hostfn-align-decouple](stage2-hostfn-align-decouple.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
