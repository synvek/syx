# ADR: Stage-2 S2-02 driver_gate manifest/audit/codable residual soft retire (S2-I115)

**Status:** Accepted (S2-I115) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I114 include_str surface retire](stage2-driver-gate-include-str-surface-retire.md), `driver_gate` pure `include_str` soft-nails remaining were manifest phase pins, audit-script floor oracles, and codable registry/inventory soft checks (7 tests). Floor count ≈497 with headroom to 450; no further floor relax needed.

## Decision

1. **Authorize deletion** of 7 pure soft tests from [`driver_gate.rs`](../../crates/syx-driver/tests/platform/driver_gate.rs):
   - `p99_driver_codable_l14_only_gate`
   - `p98_driver_codable_l14_host_registry`
   - `p98_driver_codable_host_body_prune_inventory_exact`
   - `p101_driver_manifest_79`
   - `p102_driver_manifest_80`
   - `p98_driver_audit_1550_1950_script_exact`
   - `p98_driver_audit_1700_2000_script_exact`
2. **Keep** `#[test]≥450` and `behavioral≥350` floors (no change this window).
3. **Keep** `soft_debt_zero_meta_allowable_charter`; guidemo / gui_dlopen hybrids; `ensure_gui_loaded_api_exact`; `codable_hostfn_prune_inventory_exact`; all four `*_driver_gate_count` gates; `cache_v111`; hasher; all `run()` / `*_exact_exec` / parity behavioral tests.
4. **Expected:** S2-01 census KPIs remain **0**; ≈ −7 soft-nail tests; floor count ≈497→≈490 (≥450).
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Remaining `driver_gate` include_str soft path is essentially hybrids + `soft_debt_zero` meta; backends honesty cell continued in [S2-I116](stage2-backends-honesty-fences-retire.md).
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-driver-gate-include-str-surface-retire](stage2-driver-gate-include-str-surface-retire.md) · S2-I114
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
