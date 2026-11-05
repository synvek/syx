# ADR: Stage-2 S2-02 platform residual soft retire (S2-I130)

**Status:** Accepted (S2-I130) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I129 stdlib misc soft retire](stage2-stdlib-misc-soft-retire.md), stdlib deletable pure soft ≈0. Remaining Stage-2 soft accel is repo-level residual seas. This window retires the [I124](stage2-platform-fingerprint-surface-soft-retire.md) leftover under `tests/platform/` (dist/plugin/source_gate etc., ≈27). Soft remains as KEEP tips; **≠** S2-02 readiness.

## Decision

1. **Authorize deletion** of ≈**27** pure soft `#[test]` functions under [`crates/syx-driver/tests/platform/`](../../crates/syx-driver/tests/platform/) that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `compile_and_run` / compile-run paths; and
   - are **not** KEEP-named / KEEP-file listed below.
2. **Keep by name / file:** `cache_v111_*` (`cache_v111_invalidation.rs`); `props_*_docs_honesty*`; `soft_debt_zero_meta_allowable_charter`; driver_gate floors (`#[test]≥450`, `behavioral≥350`).
3. **Keep** stdlib KEEP tip (inventory/docs/capability/tls/tcp_accept charter); LSP KEEP stubs×10; hasher; colocated behavioral / `run()` tests.
4. **Do not** delete or relocate product `syx-rt` gate sources (`gate_b09_gc_narrative.rs`, `llvm_stub_audit.rs`) — S2-01 peel deferred.
5. **Expected:** S2-01 census KPIs remain **0**; ≈ −27 soft-nail tests; platform deletable pure soft ≈0; platform KEEP ≈11 remain. Cross-crate≈72 / LSP≈6 seas deferred.
6. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness **complete** / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Platform residual soft sea closes; residual soft = KEEP tips + cross-crate/LSP seas.
- Claim-readiness inventory opened in sibling ADR — **not** an automatic `[x]`.
- Open: [stage2-open](stage2-open.md) · [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-platform-fingerprint-surface-soft-retire](stage2-platform-fingerprint-surface-soft-retire.md) · S2-I124
- [stage2-stdlib-misc-soft-retire](stage2-stdlib-misc-soft-retire.md) · S2-I129
- [stage2-claim-readiness-inventory](stage2-claim-readiness-inventory.md) · S2-I130
- [stage2-governance-census](../stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
