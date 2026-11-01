# ADR: S1-09 scoped = en book KPI CI hard gate (≠ stage-1 exit / ≠ zh matrix)

**Status:** Accepted (S1-I23)  
**Date:** 2026-11-01  
**Context:** Stage-1 primary KPI is language-guide code-block pass rate (`scripts/book_pass_rate.py`). Local en rate reached **339/339 = 100%** with `parse_fail_approx = 0`, and annotation draft + script already existed, but CI only gated examples VM contract — the book KPI itself remained a draft hard gate.

## Decision

1. **S1-09 acceptance is scoped.** Completing S1-09 means:
   - Annotation draft + `scripts/book_pass_rate.py` remain the measurement tool;
   - CI job **`book-pass`** runs **en-only** `book_pass_rate.py` after `cargo build -p syx-cli`;
   - Hard fail unless `passed == total`, `rate_pct == 100`, and `parse_fail_approx == 0`;
   - JSON report is uploaded as a CI artifact (including on failure when present).
2. **Out / defer (not required for S1-09 `[x]`):**
   - `--zh` book matrix in CI;
   - Broader doctest productization / annotation grammar expansion beyond the current draft;
   - Treating book CI green as Stage-1 exit.
3. **Stage-1 exit honesty.** Book ≥85% and parse→0 remain exit legs; this ADR locks the **en KPI regression gate**. Examples 拉满 / 三后端 and remaining language debts (trap, project multi-diag, mono, diamond) still block claiming Stage-1 exit. **S1-09 scoped `[x]` alone does not authorize Stage-1 exit.**

## Consequences

- `plan/02` marks **S1-09 `[x]` (scoped)**; narrative must keep “≠ 阶段1退出 / ≠ zh CI / ≠ S1-08 `[x]`”.
- Do not weaken the gate to &lt;100% without a superseding ADR while local KPI is 100%.
- Adding zh CI or changing the threshold requires updating this ADR (or a follow-on).

## References

- [`scripts/book_pass_rate.py`](../../scripts/book_pass_rate.py)
- [`.github/workflows/ci.yml`](../../.github/workflows/ci.yml) · job `book-pass`
- [`docs/doctest-annotations.md`](../doctest-annotations.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · S1-09
