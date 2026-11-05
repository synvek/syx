# ADR: Stage-2 S2-02 stdlib misc soft retire (S2-I129)

**Status:** Accepted (S2-I129) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I128 hostfn+tls-feature soft retire](stage2-stdlib-hostfn-cap-pins-tls-feature-soft-retire.md), remaining stdlib pure soft ≈120 of which ≈67 are deletable misc/product/m2/net pins. Soft remains present as KEEP tips; this window does **not** claim S2-02 readiness.

## Decision

1. **Authorize deletion** of ≈**67** pure soft `#[test]` functions under [`crates/syx-driver/tests/stdlib/`](../../crates/syx-driver/tests/stdlib/) that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `compile_and_run` / compile-run paths; and
   - are **not** KEEP-named / charter-file listed below.
2. **Keep by name:** `inventory_meta`; `docs_and_capability`; `docs_honesty`; `capability_thin` / `capability_floor` / `capability_exit` patterns.
3. **Keep by file (charter):**
   - tls: `tls_server_ga_thin`, `tls_server_ga_exit`, `tls_server_ga_met_exit`, `tls_server_hostfn_thin`
   - tcp_accept: `tcp_accept_ga_thin`, `tcp_accept_ga_exit`
   - capability: `capability_thin`, `capability_floor_thin`, `capability_exit`, `website_capability_thin`, `website_capability_exit`
4. **Keep** `cache_v111_*`; `soft_debt_zero_meta_allowable_charter`; driver_gate floors (`#[test]≥450`, `behavioral≥350`); LSP KEEP stubs×10; hasher; colocated behavioral / `run()` tests.
5. **Expected:** S2-01 census KPIs remain **0**; ≈ −67 soft-nail tests; deletable stdlib soft ≈0; KEEP tip ≈53 remain. Soft primary path largely closes but **≠** claim-readiness / **≠** S2-02 `[x]`.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness complete / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Stdlib deletable misc soft sea closes; residual soft = KEEP tips only.
- Next: claim-readiness inventory / S2-01 gate narrative — not automatic S2-02 `[x]`.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-stdlib-hostfn-cap-pins-tls-feature-soft-retire](stage2-stdlib-hostfn-cap-pins-tls-feature-soft-retire.md) · S2-I128
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
