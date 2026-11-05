# ADR: Stage-2 fingerprint farm_lease digest rename (S2-I101)

**Status:** Accepted (S2-I101)  
**Date:** 2026-11-04  
**Context:** After [S2-I100 test-noise](stage2-fingerprint-test-noise-rename.md), `fingerprint_fn` sits on the MUST-STAY floor (farm×36 + C-ABI×19 + heap×1). I100 §6 requires sibling ADRs with a different strategy for MUST-STAY packs. `farm_lease_*` is Rust-only product surface (no C-ABI / no HostFn strings); authorize product-sea digest rename for this pack only.

## Decision

1. **Authorize rename** of all thirty-six `farm_lease_*_fingerprint` helpers to `farm_lease_*_digest` **without deleting** APIs or changing gate return string literals / cache semantics.
2. **S2-I101 pack Δ36** — rename in `crates/syx-driver/src/farm_lease_auth.rs` + soft-nail `lib.rs` re-exports/hasher call-sites + farm tests.
3. **Do not rename:** `syx_gui_dylib_*_fingerprint` C-ABI / dlsym; `heap_v100_phase128_track_c_wasm_fingerprint` (deferred → [S2-I105](stage2-fingerprint-heap-tip-digest-rename.md)); `lang_props_v*`; HostFn registration strings.
4. **Do not** change returned `&'static str` gate literals (e.g. `"farm-lease-raft-v1:…"`).
5. **Expected census:** `fingerprint_fn_matches` **56 → 20 (−36)**; report **0**; inventory **0**.
6. Later packs (C-ABI / heap_v100 / fingerprint_const / lang_props) under sibling ADRs without claiming S2-01 `[x]` — **S2-I102:** [fingerprint-const gui-leaf rename](stage2-fingerprint-const-gui-leaf-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ folding `v100_c`.

## Consequences

- Census drops thirty-six farm_lease matches; remaining 20 = C-ABI×19 + heap×1.
- **S2-I102:** gui leaf `*_FINGERPRINT` consts renamed to `*_DIGEST` (fingerprint_const axis opened); C-ABI/fn floor kept.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-test-noise-rename](stage2-fingerprint-test-noise-rename.md) · S2-I100
- [stage2-fingerprint-const-gui-leaf-rename](stage2-fingerprint-const-gui-leaf-rename.md) · S2-I102
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

