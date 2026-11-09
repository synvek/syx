# ADR: Stage-3 exit open（S3-I86）

**Status:** Accepted (S3-I86) · Stage-3 `[~]` · exit-open  
**Date:** 2026-11-09  
**Context:** After [S3-I85 S3-07 claim](stage3-s3-07-claim.md), plan checklist is **7/7**（S3-01…07 scoped `[x]`）while Stage-3 remains `[~]` pending an explicit exit ADR — aligned with [stage2-exit](stage2-exit.md) / [stage1-exit](stage1-exit.md). This window **opens** the Stage-3 exit workstream with an exit **census baseline**（Evidence rollup + post-exit Residual draft）— it does **not** write `stage3-exit`, flip Stage-3 to `[x] 退出`, restore deferred axes as exit gates, or open Stage-4.

## Gap: checklist complete vs stage exit

| Claim | Today (honest) | Exit target |
|-------|----------------|-------------|
| S3-01…07 plan rows | All scoped `[x]`（I41…I85） | Evidence In for exit ADR |
| Stage-3 status | `[~]` · **7/7** | Dedicated `[x] 退出` via `stage3-exit` |
| Deferred supersede-only | S3-04 fib&lt;0.5 · S3-05 hybrid · S3-06 rename/fence · S3-07 UsageGuide/crate | **post-exit Residual**（do not block exit） |
| Parked / prior residual | Stage-1/2 post-exit · HostFn/F-08/Wave | Remain parked / residual |

Baseline: [stage3-exit-census](../census/stage3-exit-census.md).

## Decision

1. **Open Stage-3 exit** as an active credibility workstream（**not** a new plan row S3-08）.
2. **S3-I86 first slice** — publish exit census (+ script); add a thin contract under `syx-driver/tests/`（**no** product knives；**no** `docs/adr/stage3-exit.md` this window）.
3. **Ladder:** **S3-I87** = write [`stage3-exit`](stage3-exit.md)（Evidence In = S3 claim 链；Residual = this census draft；flip plan Stage-3 → `[x] 退出`）— still ≠ Stage-4 construction in that window unless exit ADR says otherwise.
4. **Non-claims (hard):** ≠ write `stage3-exit` this window / ≠ flip Stage-3 → `[x] 退出` / ≠ restore deferred as exit hard gate / ≠ HostFn+1 / ≠ F-08 / ≠ Wave unpark / ≠ reopen S3-04…07 product knives / ≠ open Stage-4 construction / ≠ language semantics.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Exit census | [`docs/census/stage3-exit-census.md`](../census/stage3-exit-census.md) · `scripts/s3_exit_census.sh` |
| Claim chain | S3-01 [I41](stage3-s3-01-claim.md) · S3-02 [I47](stage3-s3-02-claim.md) · S3-03 [I55](stage3-s3-03-claim.md) · S3-04 [I71](stage3-s3-04-claim.md) · S3-05 [I77](stage3-s3-05-claim.md) · S3-06 [I81](stage3-s3-06-claim.md) · S3-07 [I85](stage3-s3-07-claim.md) |
| Contract nail | `s3_exit_open_contract` |
| Plan status | Stage-3 `[~]` · **7/7** · exit-open under this ADR |

## Consequences

- Stage-3 exit workstream opened；Stage-3 was `[~]`（checklist **7/7**）under this window.
- **S3-I87 landed** as [stage3-exit](stage3-exit.md)（Stage-3 `[x]` 退出）。
- Default next = **Stage-4 open**（S3-I88 naming）— not Stage-4 construction this prior window.
- Prior: [stage3-s3-07-claim](stage3-s3-07-claim.md) · S3-I85 · [stage3-open](stage3-open.md) · S3-I1.
- Exit census Current → **S3-I87** · exit_status=exited.

## References

- [stage3-exit](stage3-exit.md) · S3-I87
- [stage3-s3-07-claim](stage3-s3-07-claim.md) · S3-I85
- [stage2-exit](stage2-exit.md) · S2-I266（exit 形态）
- [stage1-exit](stage1-exit.md) · S1-I27
- [stage3-open](stage3-open.md) · S3-I1
- [stage3-exit-census](../census/stage3-exit-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) Stage-3

