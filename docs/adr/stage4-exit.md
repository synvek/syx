# ADR: Stage-4 exit (S3-I109)

**Status:** Accepted (S3-I109)  
**Date:** 2026-11-09  
**Context:** Credibility Stage-4 axis (ecosystem & language enhancement) opened in [stage4-open](stage4-open.md) and completed its S4-01…S4-05 scoped plan claims through [S3-I107 S4-05 claim](stage4-s4-05-claim.md), with exit scaffolding in [S3-I108 exit-open](stage4-exit-open.md). Plan SSOT still marked Stage-4 `[~]` · **5/5** · exit-open pending this explicit exit window — aligned with [stage3-exit](stage3-exit.md) (S3-I87) / [stage2-exit](stage2-exit.md) / [stage1-exit](stage1-exit.md).

## Decision

1. **Stage-4 exits.** `plan/01` and `plan/02` mark Stage-4 as **`[x]` 退出** as of S3-I109.
2. **Evidence (In):**
   - S4-01 `[x]` scoped — [s4-01-claim](stage4-s4-01-claim.md) · S3-I91
   - S4-02 `[x]` scoped — [s4-02-claim](stage4-s4-02-claim.md) · S3-I95
   - S4-03 `[x]` scoped — [s4-03-claim](stage4-s4-03-claim.md) · S3-I99
   - S4-04 `[x]` scoped — [s4-04-claim](stage4-s4-04-claim.md) · S3-I103
   - S4-05 `[x]` scoped — [s4-05-claim](stage4-s4-05-claim.md) · S3-I107
   - Exit-open scaffolding — [stage4-exit-open](stage4-exit-open.md) · S3-I108 · [exit census](../census/stage4-exit-census.md)
3. **Residual (post-exit; does not block this flip):**
   - S4-01：manifest schema / doctest CLI productize；
   - S4-02：公网 registry GA / lock trim / CLI UX rewrite；
   - S4-03：Sendable deepen / `withTaskGroup` GA / TSAN·loom；
   - S4-04：`Sequence.makeIterator` GA / 用户 map·filter / Collection Index；
   - S4-05：`syx doctor --native-sdk` CLI wire / release smoke / official sysroot / marketplace / 全 profile CI；
   - Stage-0…3 post-exit residuals；
   - parked product umbrella / F-08 / HostFn+1 / Wave.
4. **Non-claims:**
   - ≠ Stage-5 / new plan row S4-06；
   - ≠ product-umbrella / F-08 / HostFn+1 unpark；
   - ≠ restore deferred supersede-only axes as post-exit hard gates；
   - ≠ reopen S4-01…05 product knives this window；
   - ≠ wire `doctor --native-sdk` this window.

## Consequences

- Credibility stages **0–4** all **`[x]` 退出**（macro ladder closed）.
- Default next = **post-exit Residual / 维护** — not Stage-5 / not product knife / not deferred-as-gate.
- Marketing / capability text may say Stage-4 exit **only** with the residual list above kept honest.
- Exit census Current → **S3-I109** · `exit_status=exited`.

## References

- [stage4-exit-open](stage4-exit-open.md) · S3-I108
- [stage4-s4-05-claim](stage4-s4-05-claim.md) · S3-I107
- [stage4-open](stage4-open.md) · S3-I88
- [stage3-exit](stage3-exit.md) · S3-I87（exit pattern）
- [stage2-exit](stage2-exit.md) · S2-I266
- [stage1-exit](stage1-exit.md) · S1-I27
- [stage4-exit-census](../census/stage4-exit-census.md)
- [`plan/01-project-status.md`](../../plan/01-project-status.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
