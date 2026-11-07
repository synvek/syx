# ADR: Stage-2 S2-07 open — single public version (S2-I263)

**Status:** Accepted (S2-I263) · S2-07 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I262 S2-06 claim](stage2-s2-06-claim.md), Stage-2 default work shifts to [plan/02 S2-07](../../plan/02-improvement-roadmap.md): one public version = crate semver; phase / marketing `117.0` internal-only or ungated. Honest inventory shows crates already unified at **`0.1.3`**; the remaining dual-track is product manifest / `build.rs` / evergreen nails still advertising **`117.0`** · phase **139**. This window **opens** the workstream with a version **census baseline** — it does **not** claim S2-07 `[x]`, rewrite version sources, or exit Stage-2.

## Gap: public version vs plan S2-07

| Claim | Today (honest) | plan/02 S2-07 target |
|-------|----------------|----------------------|
| Single public = crate semver | Dual: **`0.1.3`** vs **`117.0`** | One public number from `CARGO_PKG_VERSION` |
| phase/117.0 internal | Still public in manifest + evergreen | Internal fingerprint / narrative only |
| Crate pin divergence | **None** (already `0.1.3`) | Keep unified |

Baseline: [stage2-version-census](../stage2-version-census.md).

## Decision

1. **Open S2-07** as an active Stage-2 workstream; flip plan S2-07 `[ ]` → `[~]`.
2. **S2-I263 first slice** — publish version census (+ script); add a thin contract under `syx-driver/tests/` (no version-source edit this window).
3. **Ladder:** **S2-I264** = inject public surfaces from `CARGO_PKG_VERSION` / `0.1.3` (`build.rs` + manifest/capabilities generation + relax evergreen `117.0` nails; phase may remain internal salt) → residual docs honesty → dedicated **S2-07 `[x]` claim**.
4. **Non-claims (hard):** ≠ S2-07 `[x]` / ≠ Stage-2 exit / ≠ rewrite `build.rs` / manifest / evergreen this window / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ Frontend CST migrate / ≠ change language semantics.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Version census | [`docs/stage2-version-census.md`](../stage2-version-census.md) · `scripts/s2_07_version_census.sh` |
| Contract nail | `s2_07_version_census_contract` |
| Plan status | S2-07 `[~]` under this ADR |

## Consequences

- S2-07 moves `[ ]` → `[~]`; Stage-2 stays `[~]`.
- Decision §3 locked next = **S2-I264 inject `CARGO_PKG_VERSION`**; **landed** as [s2-07-inject-cargo-pkg-version](stage2-s2-07-inject-cargo-pkg-version.md)（S2-I264）→ [s2-07-claim](stage2-s2-07-claim.md)（S2-I265）；next = **S2-I266 Stage-2 exit**.
- Prior: [s2-06-claim](stage2-s2-06-claim.md) · S2-I262.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-06-claim](stage2-s2-06-claim.md) · S2-I262
- [stage2-version-census](../stage2-version-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-07
