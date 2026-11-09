# ADR: Stage-3 exit (S3-I87)

**Status:** Accepted (S3-I87)  
**Date:** 2026-11-09  
**Context:** Credibility Stage-3 axis (DX & performance) opened in [stage3-open](stage3-open.md) and completed its S3-01…S3-07 scoped plan claims through [S3-I85 S3-07 claim](stage3-s3-07-claim.md), with exit scaffolding in [S3-I86 exit-open](stage3-exit-open.md). Plan SSOT still marked Stage-3 `[~]` · **7/7** · exit-open pending this explicit exit window — aligned with [stage2-exit](stage2-exit.md) (S2-I266) / [stage1-exit](stage1-exit.md) (S1-I27).

## Decision

1. **Stage-3 exits.** `plan/01` and `plan/02` mark Stage-3 as **`[x]` 退出** as of S3-I87.
2. **Evidence (In):**
   - S3-01 `[x]` scoped — [s3-01-claim](stage3-s3-01-claim.md) · S3-I41
   - S3-02 `[x]` scoped — [s3-02-claim](stage3-s3-02-claim.md) · S3-I47
   - S3-03 `[x]` scoped — [s3-03-claim](stage3-s3-03-claim.md) · S3-I55
   - S3-04 `[x]` scoped — [s3-04-claim](stage3-s3-04-claim.md) · S3-I71
   - S3-05 `[x]` scoped — [s3-05-claim](stage3-s3-05-claim.md) · S3-I77
   - S3-06 `[x]` scoped — [s3-06-claim](stage3-s3-06-claim.md) · S3-I81
   - S3-07 `[x]` scoped — [s3-07-claim](stage3-s3-07-claim.md) · S3-I85
   - Exit-open scaffolding — [stage3-exit-open](stage3-exit-open.md) · S3-I86 · [exit census](../census/stage3-exit-census.md)
3. **Residual (post-exit; does not block this flip):**
   - S3-04：字面 `fib(30) < 0.5`；
   - S3-05：L010–L012 text-hybrid / full CST lint / S2-04 Frontend rewrite；
   - S3-06：rename / signature / hover book / full fence LSP sweep；
   - S3-07：UsageGuide `--template` 叙事 / Init 抽 crate / CLI 子进程 init→run / `_`→hard error；
   - Stage-1/2 post-exit residuals（examples 拉满 / Frontend CST migrate / docs 117.0 / fingerprint salts）；
   - parked product umbrella / F-08 / HostFn+1 / Wave.
4. **Non-claims:**
   - ≠ Stage-4 construction started this window；
   - ≠ product-umbrella / F-08 / HostFn+1 unpark；
   - ≠ restore deferred supersede-only axes as post-exit hard gates；
   - ≠ reopen S3-04…07 product knives this window.

## Consequences

- **S3-I88 landed** as [stage4-open](stage4-open.md)（Stage-4 `[~]` · S4-01 `[~]`）。
- Default next = **S3-I89**（S4-01 first construct）— not Stage-4 exit / not S4-01 `[x]` this prior window.
- Marketing / capability text may say Stage-3 exit **only** with the residual list above kept honest.
- Exit census Current → **S3-I87** · `exit_status=exited`；Stage-4 census Current → **S3-I88**.

## References

- [stage4-open](stage4-open.md) · S3-I88
- [stage3-exit-open](stage3-exit-open.md) · S3-I86
- [stage3-s3-07-claim](stage3-s3-07-claim.md) · S3-I85
- [stage3-open](stage3-open.md) · S3-I1
- [stage2-exit](stage2-exit.md) · S2-I266（exit pattern）
- [stage1-exit](stage1-exit.md) · S1-I27
- [stage3-exit-census](../census/stage3-exit-census.md)
- [`plan/01-project-status.md`](../../plan/01-project-status.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
