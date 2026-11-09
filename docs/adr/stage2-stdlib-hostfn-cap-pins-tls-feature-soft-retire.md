# ADR: Stage-2 S2-02 stdlib hostfn cap pins + tls feature soft retire (S2-I128)

**Status:** Accepted (S2-I128) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I127 residual+product-caps soft retire](stage2-stdlib-residual-product-caps-soft-retire.md), remaining stdlib pure soft ≈200. Soft remains a primary test path; this window does **not** claim S2-02 readiness.

## Decision

1. **Authorize deletion** of **80** pure soft `#[test]` functions under [`crates/syx-driver/tests/stdlib/`](../../crates/syx-driver/tests/stdlib/) that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `compile_and_run` / compile-run paths; and
   - match one of:
     - live in `hostfn_*.rs` (all pure soft in those files); or
     - live in tls/net feature files (`tls_server_*`, `tcp_accept_*`, `sgate_tls_*`, `wss_*`) **except** charter KEEP below.
2. **Keep tls charter×9:** tests in / named for `tls_server_ga_thin`, `tls_server_ga_exit`, `tls_server_ga_met_exit`, `tls_server_hostfn_thin`.
3. **Keep** `inventory_meta`; `docs_and_capability`; `capability_thin` / floor / exit; props docs_honesty×3; `cache_v111_*`; `soft_debt_zero_meta_allowable_charter`; driver_gate floors (`#[test]≥450`, `behavioral≥350`); LSP KEEP stubs×10; hasher.
4. **Keep** colocated behavioral / `run()` / loopback tests in the same files (esp. hostfn_* behavioral).
5. **Defer** product_other / misc soft seas (jit/hosted/marketing/error_facade).
6. **Expected:** S2-01 census KPIs remain **0**; ≈ −80 soft-nail tests (hostfn×38 + tls feature×42; plan estimate ≈75); residual deletable stdlib soft still ~100+; tls charter×9 kept.
7. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness complete / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- hostfn cap-pin + tls feature soft subclass closes; residual soft path includes misc/product_other + tls charter tips + KEEP.
- Next: misc soft sea (minus capability_thin/floor/exit); claim-readiness only after soft is no longer a primary path.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-stdlib-residual-product-caps-soft-retire](stage2-stdlib-residual-product-caps-soft-retire.md) · S2-I127
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
