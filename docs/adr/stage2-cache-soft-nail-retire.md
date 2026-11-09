# ADR: Stage-2 S2-02 cache soft-nail retire (S2-I109)

**Status:** Accepted (S2-I109) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I108 S2-02 open](stage2-s2-02-open.md) (props fingerprint soft-nail Δ5), remaining pure historical cache invalidation soft-nails are `cache_v108`–`v110`. Current SSOT tip is `cache_v111_invalidation` (`syx-cache-v1`).

## Decision

1. **Authorize deletion** of `cache_v108_invalidation.rs` / `cache_v109_invalidation.rs` / `cache_v110_invalidation.rs` and their `mod` lines in `platform/main.rs`.
2. **Retarget** `driver_gate.rs` fixture `.exists()` asserts that named v108/v110 files → `cache_v111_invalidation.rs`.
3. **Keep** `cache_v111_invalidation.rs`; do **not** change `lib.rs` hasher bytes / `lang_surface_v1`.
4. **Expected:** S2-01 census KPIs remain **0**; ≈ −21 pure soft-nail tests.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Historical cache-tag soft-nail layers drop; current tip remains.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
