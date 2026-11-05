# ADR: Stage-2 S2-02 driver_gate include_str surface retire + floor relax (S2-I114)

**Status:** Accepted (S2-I114) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I113 meta-allowable + native_locals retire](stage2-driver-gate-meta-allowable-native-locals-retire.md), remaining `driver_gate` soft-nails are mostly `include_str!(…)+contains` import/surface/abi pins. The `#[test]≥500` floor capped each knife at Δ≤16 and conflicts with Stage-2’s rejection of test-count floors as release KPI.

## Decision

1. **Relax** `driver_gate` meaningful-test floor from **500 → 450** in `dist_peer_announce_hold_driver_gate_count`, `macro_di_not_driver_gate_count`, `macro_di_wildcard_driver_gate_count`, and `p139_driver_gate_count`. **Keep** `behavioral >= 350`.
2. **Authorize deletion** of 25 pure `include_str` import / module-surface / abi-gui / source-fingerprint soft tests listed in the S2-I114 plan (from [`driver_gate.rs`](../../crates/syx-driver/tests/platform/driver_gate.rs)).
3. **Keep** `soft_debt_zero_meta_allowable_charter`; all `run()` / `*_exact_exec` / parity behavioral tests; `cache_v111`; hasher; manifest/audit/codable_l14 residual soft deferred to [S2-I115](stage2-driver-gate-manifest-audit-codable-residual-retire.md).
4. **Expected:** S2-01 census KPIs remain **0**; ≈ −25 soft-nail tests; `#[test]` ≈516→≈491 (≥450).
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Soft-nail retire accelerates; count floor no longer blocks mid-size knives.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-driver-gate-meta-allowable-native-locals-retire](stage2-driver-gate-meta-allowable-native-locals-retire.md) · S2-I113
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
