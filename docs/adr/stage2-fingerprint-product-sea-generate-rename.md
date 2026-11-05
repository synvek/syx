# ADR: Stage-2 fingerprint product-sea generate helper rename (S2-I98)

**Status:** Accepted (S2-I98)  
**Date:** 2026-11-04  
**Context:** After [S2-I97 VM synui rename](stage2-fingerprint-product-sea-vm-synui-rename.md), three private driver generate helpers remain on-census. Sibling pack: rename Rust symbols only; leave hash inputs, C-ABI, farm_lease, and `heap_v100_*` untouched.

## Decision

1. **Authorize rename** of three private generate helpers from `*_fingerprint` → `*_digest` **without deleting** APIs or changing stamp/`module_hash` byte inputs.
2. **S2-I98 pack Δ3** — rename in `crates/syx-driver/src/lib.rs` + soft-nail `query.rs`:
   - `native_input_fingerprint` → `native_input_digest`
   - `unified_mir_fn_count_fingerprint` → `unified_mir_fn_count_digest`
   - `dependency_tree_fingerprint` → `dependency_tree_digest`
3. **Do not rename:** `syx_gui_dylib_*_fingerprint`; `farm_lease_*`; `heap_v100_phase128_track_c_wasm_fingerprint`; orphan×2; driver_gate string probes; `lang_props_v*`.
4. **Expected census:** `fingerprint_fn_matches` **67 → 64 (−3)**; report **0**; inventory **0**.
5. Later packs (orphan / test-noise) under sibling ADRs without claiming S2-01 `[x]` — **S2-I99:** [orphan rename](stage2-fingerprint-product-sea-orphan-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops three driver generate matches; cache semantics unchanged.
- **S2-I99:** orphan tips ×2 renamed off census (`*_digest`); hasher bytes / C-ABI / farm_lease kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-vm-synui-rename](stage2-fingerprint-product-sea-vm-synui-rename.md) · S2-I97
- [stage2-fingerprint-product-sea-orphan-rename](stage2-fingerprint-product-sea-orphan-rename.md) · S2-I99
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

