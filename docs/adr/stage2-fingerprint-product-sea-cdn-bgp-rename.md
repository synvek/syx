# ADR: Stage-2 fingerprint product-sea cdn_bgp rename (S2-I88)

**Status:** Accepted (S2-I88)  
**Date:** 2026-11-04  
**Context:** After S2-I87, on-aggregator tips are a single kept helper (`heap_v100_…`). Remaining S2-01 `fingerprint_fn` mass is dominated by product-sea `*_fingerprint` helpers outside the cache aggregator. [stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md) required a **separate ADR** before product-sea carve. Precedent: S2-I84 capabilities surface rename; foundation `*_digest` off-census helpers.

## Decision

1. **Authorize rename** of selected product-sea `*_fingerprint` helpers to non-census suffixes (`*_digest`) **without deleting** product capability APIs or changing return values/behavior beyond the symbol name.
2. **S2-I88 pack Δ11** — rename in `crates/syx-async/src/dist_transport.rs` (+ `lib.rs` re-exports + same-crate call sites):
   - `cdn_bgp_sticky_ttl_fingerprint` → `cdn_bgp_sticky_ttl_digest`
   - `cdn_bgp_sticky_persist_fingerprint` → `cdn_bgp_sticky_persist_digest`
   - `cdn_bgp_sticky_multi_snapshot_fingerprint` → `cdn_bgp_sticky_multi_snapshot_digest`
   - `cdn_bgp_peer_announce_sticky_fingerprint` → `cdn_bgp_peer_announce_sticky_digest`
   - `cdn_bgp_peer_announce_ack_fingerprint` → `cdn_bgp_peer_announce_ack_digest`
   - `cdn_bgp_peer_announce_fanout_fingerprint` → `cdn_bgp_peer_announce_fanout_digest`
   - `cdn_bgp_peer_announce_conflict_epoch_fingerprint` → `cdn_bgp_peer_announce_conflict_epoch_digest`
   - `cdn_bgp_peer_announce_epoch_persist_fingerprint` → `cdn_bgp_peer_announce_epoch_persist_digest`
   - `cdn_bgp_peer_announce_ttl_fingerprint` → `cdn_bgp_peer_announce_ttl_digest`
   - `cdn_bgp_peer_announce_priority_fingerprint` → `cdn_bgp_peer_announce_priority_digest`
   - `cdn_bgp_peer_announce_hold_fingerprint` → `cdn_bgp_peer_announce_hold_digest`
3. **Expected census:** `fingerprint_fn_matches` **164 → 153 (−11)**; report **0**; inventory **0**.
4. **MUST STAY:** `heap_v100_…`; `farm_lease_*_fingerprint`; capabilities API; `lang_props_v*`; gui dylib C-ABI fingerprints; other unlisted `cdn_bgp_*` / `cdn_pop_*` names (cleared in S2-I89 sibling); five B-09 flags; `llvm_stub_audit_report`.
5. Later product-sea rename packs may continue under sibling ADRs (or amend this family) without claiming S2-01 `[x]` — **S2-I89:** [async-dist rename](stage2-fingerprint-product-sea-async-dist-rename.md).
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops eleven product-sea matches confined to `syx-async`.
- External importers of the old symbol names (if any out-of-tree) must follow the rename; in-tree call sites update in the same window.
- **S2-I89:** remaining async dist/cdn gates ×8 renamed off census (`*_digest`).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-track-l-fold](stage2-fingerprint-track-l-fold.md)
- [stage2-capabilities-report-surface](stage2-capabilities-report-surface.md)
- [stage2-fingerprint-product-sea-async-dist-rename](stage2-fingerprint-product-sea-async-dist-rename.md) · S2-I89
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

