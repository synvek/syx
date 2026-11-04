# ADR: Stage-2 fingerprint product-sea select channel rename (S2-I90)

**Status:** Accepted (S2-I90)  
**Date:** 2026-11-04  
**Context:** After [S2-I89 async dist/cdn gate rename](stage2-fingerprint-product-sea-async-dist-rename.md), `syx-async` product-sea fingerprint surface is clear. Remaining product-sea mass includes the **select channel** mirror cluster: 7 defs in `syx-parser` + 7 in `syx-vm` `channel_hosts` (8 distinct symbols; census **Δ14**). Sibling pack under the product-sea rename family (track-l-fold §7 / I89 §5).

## Decision

1. **Authorize rename** of select-channel gate helpers from `*_fingerprint` → `*_digest` **without deleting** product APIs or changing return values/behavior beyond the symbol name.
2. **S2-I90 pack Δ14** — rename in:
   - `crates/syx-parser/src/lib.rs` (×7)
   - `crates/syx-vm/src/channel_hosts.rs` (×7) + `crates/syx-vm/src/lib.rs` re-exports
   - soft-nails: driver `channel_fairness` / `select_mixed_send_recv` / `select_deadline_strict`; LSP `lsp_phase109_real` string probe
   - Symbols:
     - `select_fair_fingerprint` → `select_fair_digest`
     - `select_mixed_poll_fingerprint` → `select_mixed_poll_digest`
     - `select_mixed_unified_fingerprint` → `select_mixed_unified_digest`
     - `select_mixed_await_fingerprint` → `select_mixed_await_digest`
     - `select_mixed_timeout_fingerprint` → `select_mixed_timeout_digest`
     - `select_timeout_syntax_fingerprint` → `select_timeout_syntax_digest`
     - `select_deadline_unify_fingerprint` → `select_deadline_unify_digest`
     - `select_deadline_strict_fingerprint` → `select_deadline_strict_digest`
3. **Do not rename** `SELECT_*_FINGERPRINT` consts this window (`fingerprint_const` axis deferred).
4. **Expected census:** `fingerprint_fn_matches` **145 → 131 (−14)**; report **0**; inventory **0**.
5. **MUST STAY:** `heap_v100_…`; `farm_lease_*_fingerprint`; capabilities API; `lang_props_v*`; gui dylib C-ABI / dlopen fingerprints; gui shape-core (cleared in S2-I91 sibling) / façade wrappers; async I88/I89 `*_digest`; five B-09 flags; `llvm_stub_audit_report`.
6. Later product-sea packs (gui non-C-ABI / generate) under sibling ADRs without claiming S2-01 `[x]` — **S2-I91:** [gui-shape rename](stage2-fingerprint-product-sea-gui-shape-rename.md).
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ deleting product APIs / ≠ folding `v100_c`.

## Consequences

- Census drops fourteen select-channel matches (parser+vm).
- Soft-nails and LSP string probes follow in the same window.
- **S2-I91:** gui shape-core ×9 renamed off census (`*_digest`); façades deferred.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-fingerprint-product-sea-async-dist-rename](stage2-fingerprint-product-sea-async-dist-rename.md) · S2-I89
- [stage2-fingerprint-product-sea-cdn-bgp-rename](stage2-fingerprint-product-sea-cdn-bgp-rename.md) · S2-I88
- [stage2-fingerprint-product-sea-gui-shape-rename](stage2-fingerprint-product-sea-gui-shape-rename.md) · S2-I91
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-01

