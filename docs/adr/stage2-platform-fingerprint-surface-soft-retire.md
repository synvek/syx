# ADR: Stage-2 S2-02 platform fingerprint/surface soft retire (S2-I124)

**Status:** Accepted (S2-I124) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I123 hybrid fingerprint stub retire](stage2-lsp-hybrid-fingerprint-stub-retire.md), remaining large soft-nail mass includes pure `include_str!(…).contains(…)` fingerprint/surface nails under [`crates/syx-driver/tests/platform/`](../../crates/syx-driver/tests/platform/). The `cache_v111` tip nails stay as the active cache soft tip.

## Decision

1. **Authorize deletion** of **31** pure fingerprint/surface soft `#[test]` functions under `tests/platform/` that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `run(` compile-run paths; and
   - have names containing `fingerprint(s)` / `_surface(s)`; and
   - are **not** under `cache_v111_invalidation.rs` / `cache_v111_*`.
2. **Keep** all `cache_v111_*` tip nails; `soft_debt_zero_meta_allowable_charter`; driver_gate floors (`#[test]≥450`, `behavioral≥350`); LSP KEEP stubs×10; props docs_honesty×3; product caps/inventory; hasher.
3. **Keep** colocated behavioral / `run()` / e2e tests in the same platform files.
4. **Expected:** S2-01 census KPIs remain **0**; ≈ −31 soft-nail tests.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Platform fingerprint/surface soft sea closes (except `cache_v111` tip); residual soft path = cache tip + LSP KEEP×10 + product honesty/caps (+ tiny docs_honesty crumb).
- Next: claim-readiness inventory / other crate soft seas — not an automatic S2-02 `[x]`.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-lsp-hybrid-fingerprint-stub-retire](stage2-lsp-hybrid-fingerprint-stub-retire.md) · S2-I123
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
