# ADR: Language surface SSOT = language guide (Swift-style)

**Status:** Accepted (S1-01)  
**Date:** 2026-10-30  
**Context:** Stage 1 language-core credibility. Parser/semantic historically accepted a simplified subset that diverges from `books/language-guide` (and `books/zh-cn/language-guide`). Analysis baseline: ~28% of guide code blocks pass `syx check`.

## Decision

1. **Book is SSOT.** When book examples and the current toolchain disagree on core syntax/semantics, the guide wins. Implementation work (S1-02…) closes the gap toward the book, not the reverse.
2. **One-cycle compatibility.** Existing simplified spellings that the toolchain already accepts remain parseable for **one release cycle**, but emit a **deprecation diagnostic** (warn-level) naming the book form and pointing at the relevant chapter.
3. **No separate syntax vote.** This ADR records the Stage-1 charter choice; subsequent windows do not re-litigate “book vs simplified” without a new ADR that supersedes this one.
4. **KPI.** Primary Stage-1 KPI is **book code-block pass rate** (`scripts/book_pass_rate.py`), not test-count floors.

## Compatibility / migration

| Area | Book form | Legacy (one cycle) | Notes |
|------|-----------|--------------------|-------|
| Struct fields | `var` / `let` + type | bare `name: Type` | S1-02 first slice accepts both |
| Mutating methods | `mutating func` | unmarked mutating | Deprecation when unmarked path is detected (deepen) |
| Module APIs | `Math.add` / `Url.build` / `Concurrency.sleep` | bare `add` / `import URL` | examples fixed in S0-I2 |

Provide `syx migrate` in a later window if rewrite volume warrants it; not required to accept this ADR.

## Consequences

- Parser/semantic PRs for Stage 1 cite this ADR and the chapter under repair.
- Capability / appendix narrative must not claim book GA until pass-rate exit thresholds in `plan/02` are met.
- Private ledger may track window tips; this public ADR is the durable decision record.

## References

- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md) · S1-01
- [`books/language-guide/10-structures-and-classes.md`](../../books/language-guide/10-structures-and-classes.md)
- Analysis: [`plan/ProjectAnalysisAndImprovementPlan.zh-CN.md`](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) §1.1
