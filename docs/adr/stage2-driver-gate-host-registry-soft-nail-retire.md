# ADR: Stage-2 S2-02 driver_gate host_registry soft-nail retire (S2-I112)

**Status:** Accepted (S2-I112) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I111 invalidation-exact retire](stage2-driver-gate-invalidation-exact-retire.md), remaining pure soft-nails include five `host_registry_*` tests that only `include_str!(registry.rs)+contains`, each duplicated by `*_host` / parity behavioral coverage.

## Decision

1. **Authorize deletion** of `p98_driver_host_registry_reflect_type` / `tcp_connect` / `udp_bind` / `ed25519` / `array_hof` from [`driver_gate.rs`](../../crates/syx-driver/tests/platform/driver_gate.rs).
2. **Keep** all corresponding `*_host` / parity behavioral tests; `soft_debt_zero_meta_allowable_charter`; other meta_allowable; `native_llvm_locals_v7_invalidation_exact`; `cache_v111`; hasher; `lang_surface_v1`.
3. **Expected:** S2-01 census KPIs remain **0**; ≈ −5 soft-nail tests; `#[test]` stays above ≥500.
4. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Registry string soft-nails drop; host/parity behavior remains the SSOT for these surfaces.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-driver-gate-invalidation-exact-retire](stage2-driver-gate-invalidation-exact-retire.md) · S2-I111
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
