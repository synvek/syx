# ADR: Stage-2 HostFn construction tip knife + orphan thin (S2-I47+)

**Status:** Accepted (S2-I47)  
**Date:** 2026-11-03  
**Context:** After S2-I46, b09 construction tip packs are exhausted (census **9** = `gc_narrative_thin`×1 + HostFn×8). `gate_b09_gc_narrative_thin_inventory` is orphaned (no product readers beyond lib re-export/unit; five policy-fixed flags remain in the same module). HostFn×8 is a linear tip chain starting at `exit`. [stage2-hostfn-align-decouple](stage2-hostfn-align-decouple.md) foreshadowed: later windows may slim HostFn inner chain only under a heavier ADR. Precedent: [stage2-b09-construction-tip-knife](stage2-b09-construction-tip-knife.md).

## Decision

1. **Orphan `gate_b09_gc_narrative_thin_inventory` may be deleted** without rewiring the five product flags (`objectref_rc_migrate_enabled`, `storage_cleanup_enabled`, `unified_mir_runtime_heap_enabled`, `b09_axis_exit_enabled`, `gc_ga_claimed`).
2. **HostFn inventory tip packs may be deleted tip→deep** (tip = `exit` → `fifth` → `fourth` → `third` → `second` → `first` → `deepen` → `thin`) without enabling global lowering, rewriting stubs/abi/emit, or claiming B-07 GA.
3. **S2-I47 deletes tip packs×4:**
   - `gate_b09_gc_narrative_thin_inventory`
   - `gate_hostfn_real_lowering_horizon_exit_inventory`
   - `gate_hostfn_real_lowering_horizon_fifth_slice_wire_inventory`
   - `gate_hostfn_real_lowering_horizon_fourth_slice_wire_inventory`
4. **Expected census (S2-I47):** **9 → 5 (−4)**. New HostFn tip ≈ `third_slice_wire`.
5. **Later windows may continue HostFn tip packs under this ADR** (default Δ3–4) without a new decision document, while keeping deeper HostFn chain / `llvm_stub_audit_report` / live stub implementations / five flags OUT unless separately decided.
6. **MUST STAY:** HostFn deeper inventories remaining after each knife, `llvm_stub_audit_report` / `classify_llvm_stub_block`, live `syx_rt_*` stub implementations, five product flag APIs.
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ enabling global lowering or batch rewrite / ≠ emit·ABI rewrite / ≠ B-07·B-09 GA semantic change.

## Consequences

- Driver companions for deleted HostFn wire slices are removed with the inventories (`tcp_handle_release_wire`, `tcp_write_wire` this window).
- Historical Wave / gate docs may still name deleted symbols as archival evidence.
- **S2-I47:** orphan thin + HostFn tip 三联 deleted; census **9 → 5**; new tip ≈ HostFn `third`.
- **S2-I48:** tip packs×4 deleted (third+second+first+deepen); census **5 → 1**; remaining ≈ HostFn `thin`.
- **S2-I49:** HostFn thin 末刀 deleted; census **1 → 0**; HostFn inventory chain exhausted (≠ S2-01 `[x]`).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-hostfn-align-decouple](stage2-hostfn-align-decouple.md)
- [stage2-b09-construction-tip-knife](stage2-b09-construction-tip-knife.md)
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
