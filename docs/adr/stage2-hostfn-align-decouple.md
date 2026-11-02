# ADR: Stage-2 HostFn←capability_align decouple (S2-I17)

**Status:** Accepted (S2-I17)  
**Date:** 2026-11-02  
**Context:** After S2-I16, census is **83**. Remaining mass is `gate_b09`≈74 and HostFn/`capability_align`≈9. `llvm_stub_capability_align_inventory` is the only live reader path into HostFn thin/deepen `_once` (B-06 W314 shell). Wave B / B-06 / B-07 axes are already signed; align is process-governance feed, not product emit. Precedent: [stage2-cutover-decouple](stage2-cutover-decouple.md), [stage2-wave-b-inventory-retire](stage2-wave-b-inventory-retire.md).

## Decision

1. **Product HostFn inventories no longer walk align.** In `gate_hostfn_real_lowering_horizon_{thin,deepen}_inventory_once`, align-derived relational floors and counts are **policy-fixed** to prior live values (`hostfn_llvm_real=247`, `partial=0`, `named_aot_wired=6`, `call_host_path_wired=true`; relational preds `true`).
2. **This window deletes only the align inventory** (`llvm_stub_capability_align_inventory` + struct + `count_hostfn_llvm_backends` + unit + driver W314 + re-export). Expected census **83 → 82 (−1)**.
3. **MUST STAY:** HostFn×8 full chain (including live wire probes such as `http_get_aot_first_slice_wired`), `llvm_stub_audit_report` / `classify_llvm_stub_block`, entire `gate_b09`.
4. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ B-06·B-07 product rewrite / ≠ deleting HostFn×8 or `gate_b09` / ≠ HostFn+1 / ≠ emit·ABI change.

## Consequences

- Later windows may slim HostFn inner chain or migrate `gate_b09` only under a heavier ADR.
- gate-b06 W314 / gate-b07 “from align inventory” docs soft-archive to HostFn policy floors.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-cutover-decouple](stage2-cutover-decouple.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01
