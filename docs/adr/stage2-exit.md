# ADR: Stage-2 exit (S2-I266)

**Status:** Accepted (S2-I266)  
**Date:** 2026-11-07  
**Context:** Credibility Stage-2 axis (architecture convergence) opened in [stage2-open](stage2-open.md) and completed its S2-01…S2-07 scoped plan claims through [S2-I265 S2-07 claim](stage2-s2-07-claim.md). Plan SSOT still marked Stage-2 `[~]` pending this explicit exit window — aligned with [stage1-exit](stage1-exit.md) (S1-I27).

## Decision

1. **Stage-2 exits.** `plan/01` and `plan/02` mark Stage-2 as **`[x]` 退出** as of S2-I266.
2. **Evidence (In):**
   - S2-01 / S2-02 `[x]` — [s2-01-s2-02-claim](stage2-s2-01-s2-02-claim.md) · S2-I135
   - S2-03 `[x]` — [s2-03-claim](stage2-s2-03-claim.md) · S2-I142
   - S2-04 `[x]` scoped — [s2-04-claim](stage2-s2-04-claim.md) · S2-I164
   - S2-05 `[x]` scoped — [s2-05-claim](stage2-s2-05-claim.md) · S2-I259
   - S2-06 `[x]` scoped — [s2-06-claim](stage2-s2-06-claim.md) · S2-I262
   - S2-07 `[x]` scoped — [s2-07-claim](stage2-s2-07-claim.md) · S2-I265
3. **Residual (post-exit; does not block this flip):**
   - Frontend CST migrate / dual-grammar retire / delete `syx-cst`;
   - docs/books「SYX 117.0」narrative; phase `139` field; fingerprint `117.0`/`phase117` salts;
   - S1-04 / S1-05 post-exit residuals; examples 拉满 / four-backend mono GA;
   - parked product umbrella / F-08 / HostFn+1.
4. **Non-claims:**
   - ≠ Stage-3 construction started this window;
   - ≠ product-umbrella / F-08 / HostFn+1 unpark;
   - ≠ Native / WASM product GA;
   - ≠ 「zero historical strings」 / 「Frontend already shared-tree」.

## Consequences

- Default next credibility work is **Stage-3**; **landed** through [span-host-driver-ops](stage3-s3-01-span-host-driver-ops.md)（S3-I7）；next = **S3-I8 llvm 定点**.
- Marketing / capability text may say Stage-2 exit **only** with the residual list above kept honest.

## References

- [stage2-open](stage2-open.md) · S2-I1
- [stage2-s2-07-claim](stage2-s2-07-claim.md) · S2-I265
- [stage1-exit](stage1-exit.md) · S1-I27 (exit pattern)
- [`plan/01-project-status.md`](../../plan/01-project-status.md)
- [`plan/02-improvement-roadmap.md`](../../plan/02-improvement-roadmap.md)
