# ADR: Stage-2 fingerprint product-sea async dist/cdn gate rename (S2-I89)

**Status:** Accepted (S2-I89)  
**Date:** 2026-11-04  
**Context:** After [S2-I88 cdn_bgp sticky/peer-announce rename](stage2-fingerprint-product-sea-cdn-bgp-rename.md), `syx-async` still contributed **8** on-census `*_fingerprint` product-sea gates (TLS pin, cdn_bgp/pop mains, MP, anycast/nonloopback/multihost, tcp peer). Sibling pack under the product-sea rename family authorized by track-l-fold §7.

## Decision

1. **Authorize rename** of the remaining `syx-async` dist/cdn gate helpers from `*_fingerprint` → `*_digest` **without deleting** product APIs or changing return values/behavior beyond the symbol name.
2. **S2-I89 pack Δ8** — rename in `crates/syx-async/src/dist_transport.rs` (+ `lib.rs` re-exports + same-crate call sites) and soft-nail driver platform tests:
   - `dist_actor_tls_rustls_pin_fingerprint` → `dist_actor_tls_rustls_pin_digest`
   - `cdn_bgp_fingerprint` → `cdn_bgp_digest`
   - `cdn_pop_fingerprint` → `cdn_pop_digest`
   - `dist_actor_mp_fingerprint` → `dist_actor_mp_digest`
   - `dist_region_anycast_fingerprint` → `dist_region_anycast_digest`
   - `dist_region_nonloopback_fingerprint` → `dist_region_nonloopback_digest`
   - `dist_region_multihost_fingerprint` → `dist_region_multihost_digest`
   - `cdn_tcp_peer_fingerprint` → `cdn_tcp_peer_digest`
3. **Expected census:** `fingerprint_fn_matches` **153 → 145 (−8)**; `syx-async` product-sea fingerprint defs **8 → 0**; report **0**; inventory **0**.
4. **MUST STAY:** `heap_v100_…`; `farm_lease_*_fingerprint`; capabilities API; `lang_props_v*`; gui dylib C-ABI fingerprints; `select_*_fingerprint` (cleared in S2-I90 sibling); I88 sticky/peer-announce `*_digest` (already off-census); five B-09 flags; `llvm_stub_audit_report`.
5. Later product-sea packs (select / gui non-C-ABI / generate) under sibling ADRs without claiming S2-01 `[x]` — **S2-I90:** [select-channel rename](stage2-fingerprint-product-sea-select-channel-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops eight matches; `syx-async` on-census product-sea fingerprint surface clears.
- Soft-nails: `syx-driver/tests/platform/dist_bgp_anycast_pop.rs`, `dist_bgp_peer_exchange.rs`.
- **S2-I90:** select channel ×14 renamed off census (`*_digest`).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-cdn-bgp-rename](stage2-fingerprint-product-sea-cdn-bgp-rename.md) · S2-I88
- [stage2-fingerprint-product-sea-select-channel-rename](stage2-fingerprint-product-sea-select-channel-rename.md) · S2-I90
- [stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md)
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

