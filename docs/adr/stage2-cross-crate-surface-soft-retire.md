# ADR: Stage-2 S2-02 cross-crate surface/fingerprint soft retire (S2-I125)

**Status:** Accepted (S2-I125) · S2-02 `[~]`  
**Date:** 2026-11-05  
**Context:** After [S2-I124 platform fingerprint/surface soft retire](stage2-platform-fingerprint-surface-soft-retire.md), remaining large soft-nail mass includes pure `include_str!(…).contains(…)` surface/fingerprint nails under `tests/{gui,runtime,data,native,browser}/`. Soft is still a primary test path overall (stdlib residual sea); this window does **not** claim S2-02 readiness.

## Decision

1. **Authorize deletion** of **43** pure soft `#[test]` functions under [`crates/syx-driver/tests/`](../../crates/syx-driver/tests/) in `gui` / `runtime` / `data` / `native` / `browser` that:
   - contain `include_str!(…).contains(…)`; and
   - do **not** invoke `run_*` / `run(` compile-run paths; and
   - have names containing `fingerprint(s)` / `surface`, **or** are browser `playwright_*` / `gui_smoke_playwright_or_compile_marker` column pins.
2. **Keep** `cache_v111_*`; `soft_debt_zero_meta_allowable_charter`; driver_gate floors (`#[test]≥450`, `behavioral≥350`); LSP KEEP stubs×10; props docs_honesty×3; stdlib `*_docs_and_capability` / `inventory_meta`; hasher.
3. **Keep** colocated behavioral / `run()` / e2e tests in the same files.
4. **Expected:** S2-01 census KPIs remain **0**; ≈ −43 soft-nail tests.
5. **Non-claims:** ≠ S2-01 `[x]` / ≠ S2-02 `[x]` / ≠ Stage-2 exit / ≠ claim-readiness complete / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Consequences

- Cross-crate surface/fp/playwright soft subclass closes; residual soft path includes stdlib sea + platform misc + KEEP tips.
- Next: claim-readiness inventory only after soft is no longer a primary path; stdlib soft knives deferred.
- Open: [stage2-open](stage2-open.md).

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-02-open](stage2-s2-02-open.md) · S2-I108
- [stage2-platform-fingerprint-surface-soft-retire](stage2-platform-fingerprint-surface-soft-retire.md) · S2-I124
- [stage2-governance-census](../census/stage2-governance-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-02
