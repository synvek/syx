# ADR: Stage-2 S2-06 open — workspace slim (S2-I260)

**Status:** Accepted (S2-I260) · S2-06 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I259 S2-05 claim](stage2-s2-05-claim.md), Stage-2 default work shifts to [plan/02 S2-06](../../plan/02-improvement-roadmap.md): shrink the default workspace so Farm/Dist/HSM/CDN/GUI-class surfaces leave the default contributor build (or live under `experimental/`). Honest inventory shows Farm/Dist/HSM/CDN **already absent** as crates; the remaining default-member product face is [`syx-gui`](../../crates/syx-gui). This window **opens** the workstream with a workspace **census baseline** — it does **not** claim S2-06 `[x]`, move crates, or exit Stage-2.

## Gap: membership vs plan S2-06

| Claim | Today (honest) | plan/02 S2-06 target |
|-------|----------------|----------------------|
| Default workspace slim | `members = ["crates/*"]` · **37** crates | Product/optional out of default |
| GUI | `syx-gui` in members; CI check/clippy excludes it | Out of default members / `experimental/` |
| Farm/Dist/HSM/CDN | **No** named crates | Stay out; narrative/test residual only |

Baseline: [stage2-workspace-census](../stage2-workspace-census.md).

## Decision

1. **Open S2-06** as an active Stage-2 workstream; flip plan S2-06 `[ ]` → `[~]`.
2. **S2-I260 first slice** — publish workspace census (+ script); add a thin contract under `syx-driver/tests/` (no membership edit this window).
3. **Ladder:** **S2-I261** = exclude **`syx-gui`** from default workspace (keep path/feature opt-in) → residual honesty → dedicated **S2-06 `[x]` claim**.
4. **Non-claims (hard):** ≠ S2-06 `[x]` / ≠ Stage-2 exit / ≠ move or delete `syx-gui` this window / ≠ open S2-07 / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ Frontend CST migrate / ≠ change language semantics.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Workspace census | [`docs/stage2-workspace-census.md`](../stage2-workspace-census.md) · `scripts/s2_06_workspace_census.sh` |
| Contract nail | `s2_06_workspace_census_contract` |
| Plan status | S2-06 `[~]` under this ADR |

## Consequences

- S2-06 moves `[ ]` → `[~]`; Stage-2 stays `[~]`.
- Decision §3 locked next = **S2-I261 exclude `syx-gui`**; **landed** as [s2-06-exclude-syx-gui](stage2-s2-06-exclude-syx-gui.md)（S2-I261）→ [s2-06-claim](stage2-s2-06-claim.md)（S2-I262）；next = **S2-I263 S2-07 open**.
- Prior: [s2-05-claim](stage2-s2-05-claim.md) · S2-I259.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-05-claim](stage2-s2-05-claim.md) · S2-I259
- [stage2-workspace-census](../stage2-workspace-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-06
