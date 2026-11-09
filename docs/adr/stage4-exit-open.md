# ADR: Stage-4 exit open（S3-I108）

**Status:** Accepted (S3-I108) · Stage-4 `[~]` · exit-open · **5/5**  
**Date:** 2026-11-09  
**Context:** After [S3-I107 S4-05 claim](stage4-s4-05-claim.md), plan checklist is **5/5**（S4-01…05 scoped `[x]`）while Stage-4 remains `[~]` pending an explicit exit ADR — aligned with [stage3-exit-open](stage3-exit-open.md) / [stage3-exit](stage3-exit.md). This window **opens** the Stage-4 exit workstream with an exit **census baseline**（Evidence rollup + post-exit Residual draft）— it does **not** write `stage4-exit`, flip Stage-4 to `[x] 退出`, restore deferred axes as exit gates, or open a new plan row S4-06.

## Gap: checklist complete vs stage exit

| Claim | Today (honest) | Exit target |
|-------|----------------|-------------|
| S4-01…05 plan rows | All scoped `[x]`（I91…I107） | Evidence In for exit ADR |
| Stage-4 status | `[~]` · **5/5** | Dedicated `[x] 退出` via `stage4-exit` |
| Deferred supersede-only | S4-01 doctest/manifest · S4-02 public registry · S4-03 Sendable/withTaskGroup · S4-04 makeIterator/map·filter · S4-05 doctor CLI / sysroot / marketplace | **post-exit Residual**（do not block exit） |
| Parked / prior residual | Stage-0…3 post-exit · HostFn/F-08/Wave | Remain parked / residual |

Baseline: [stage4-exit-census](../stage4-exit-census.md).

## Decision

1. **Open Stage-4 exit** as an active credibility workstream（**not** a new plan row S4-06）.
2. **S3-I108 first slice** — publish exit census (+ script); add a thin contract under `syx-driver/tests/`（**no** product knives；**no** `docs/adr/stage4-exit.md` this window）.
3. **Ladder:** **S3-I109** = write [`stage4-exit`](stage4-exit.md)（Evidence In = S4 claim 链；Residual = this census draft；flip plan Stage-4 → `[x] 退出`）。
4. **Non-claims (hard):** ≠ write `stage4-exit` this window / ≠ flip Stage-4 → `[x] 退出` / ≠ restore deferred as exit hard gate / ≠ HostFn+1 / ≠ F-08 / ≠ Wave unpark / ≠ reopen S4-01…05 product knives / ≠ wire `doctor --native-sdk` / ≠ language semantics.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Exit census | [`docs/stage4-exit-census.md`](../stage4-exit-census.md) · `scripts/s4_exit_census.sh` |
| Claim chain | S4-01 [I91](stage4-s4-01-claim.md) · S4-02 [I95](stage4-s4-02-claim.md) · S4-03 [I99](stage4-s4-03-claim.md) · S4-04 [I103](stage4-s4-04-claim.md) · S4-05 [I107](stage4-s4-05-claim.md) |
| Contract nail | `s4_exit_open_contract` |
| Plan status | Stage-4 `[~]` · **5/5** · exit-open under this ADR |

## Consequences

- Stage-4 exit workstream opened；Stage-4 was `[~]`（checklist **5/5**）under this window.
- **S3-I109 landed** as [stage4-exit](stage4-exit.md)（Stage-4 `[x]` 退出）。
- Default next = **post-exit Residual / 维护** — not Stage-5 / not product knife.
- Prior: [stage4-s4-05-claim](stage4-s4-05-claim.md) · S3-I107 · [stage4-open](stage4-open.md) · S3-I88.
- Exit census Current → **S3-I109** · exit_status=exited · `stage4_exit_adr_present=1`.

## References

- [stage4-s4-05-claim](stage4-s4-05-claim.md) · S3-I107
- [stage3-exit-open](stage3-exit-open.md) · S3-I86（exit-open 形态）
- [stage3-exit](stage3-exit.md) · S3-I87
- [stage4-open](stage4-open.md) · S3-I88
- [stage4-exit-census](../stage4-exit-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) Stage-4
