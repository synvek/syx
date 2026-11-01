# ADR: Stage-1 exit (S1-I27)

**Status:** Accepted (S1-I27)  
**Date:** 2026-11-01  
**Context:** Credibility Stage-1 exit triad — book pass ≥85% with parse→0, plus an examples runnable leg — is charter-complete after S1-I21…I26 scoped ADRs and CI gates. Plan SSOT still said “do not claim Stage-1 exit” pending an explicit flip window.

## Decision

1. **Stage-1 exits.** `plan/01` and `plan/02` mark Stage-1 as **`[x]` 退出** as of S1-I27.
2. **Evidence (In):**
   - Book KPI: en `book_pass_rate.py` → **339/339 = 100%**, `parse_fail_approx = 0` (local `/tmp/book_pass_s1i27.json`; CI job `book-pass`);
   - Examples leg: [exit examples-leg ADR](stage1-exit-examples-leg.md) = [S1-10 VM inventory](stage1-s1-10-vm-inventory-scope.md) + `s0_backend_smoke.sh` three-backend thin honesty;
   - Supporting scoped closures: [S1-07 wrap](stage1-s1-07-overflow-wrap-scope.md), [S1-08 multi-diag](stage1-s1-08-cli-multidiag-scope.md), [S1-09 book CI](stage1-s1-09-book-kpi-ci-scope.md).
3. **Residual (post-exit; does not block this flip):**
   - S1-04 diamond / Sequence·Collection full matrix;
   - S1-05 four-backend mono / opaque GA;
   - examples **拉满**; allowlist **full** native/wasm matrix;
   - build/run multi-diag; trap / R011 / `&+`/`&-`/`&*`.
4. **Non-claims:**
   - ≠ Stage-2 construction started;
   - ≠ product-umbrella / F-08 / HostFn+1 unpark;
   - ≠ Native / WASM product GA.

## Consequences

- Default next credibility work is **Stage-2 candidate** (S2-01…) — open only via a new window.
- Marketing / capability text may say Stage-1 exit **only** with the residual list above kept honest.

## References

- [`docs/adr/stage1-exit-examples-leg.md`](stage1-exit-examples-leg.md)
- [`scripts/stage1_exit_examples_leg.sh`](../../scripts/stage1_exit_examples_leg.sh)
- [`plan/01-project-status.md`](../../plan/01-project-status.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
