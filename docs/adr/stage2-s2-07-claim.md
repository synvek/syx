# ADR: Stage-2 S2-07 plan claim (S2-I265)

**Status:** Accepted (S2-I265) · S2-07 `[x]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I263 version census open](stage2-s2-07-open.md) and [S2-I264 inject CARGO_PKG_VERSION](stage2-s2-07-inject-cargo-pkg-version.md), the S2-07 construction ladder meets the scoped acceptance bar: public product version follows crate semver (`0.1.3` / `CARGO_PKG_VERSION`). This dedicated window flips the plan checkbox only — aligned with [S2-I262 S2-06 claim](stage2-s2-06-claim.md) / [S2-I259 S2-05 claim](stage2-s2-05-claim.md). It does **not** exit Stage-2, rewrite docs/books marketing strings, or delete the phase field.

## Evidence

| Axis | Evidence |
|------|----------|
| Open baseline | [stage2-s2-07-open](stage2-s2-07-open.md) · [stage2-version-census](../stage2-version-census.md) · S2-I263 |
| Public inject | `build.rs` ← `CARGO_PKG_VERSION` · manifest/capabilities `0.1.3` · S2-I264 |
| Evergreen | version asserts follow `CARGO_PKG_VERSION` |
| Contract chain | `s2_07_version_census` · `s2_07_inject_cargo_pkg_version` · this claim |
| Honesty OUT | **≠** mass docs rewrite · **≠** delete phase · **≠** Stage-2 exit |

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S2-07 → `[x]`**.
2. Keep **Stage-2 → `[~]`** until an explicit Stage-2 exit ADR.
3. **Scoped `[x]` meaning:** public surfaces (`build.rs` / checked-in manifest / capabilities / evergreen) advertise crate semver via `CARGO_PKG_VERSION`; phase `139`, fingerprint `117.0`/`phase117` salts, and docs/books「SYX 117.0」narrative residual **OK** — **not** 「every historical string rewritten」 / **not** 「phase field deleted」.
4. Update [stage2-s2-07-open](stage2-s2-07-open.md) / [stage2-s2-07-inject-cargo-pkg-version](stage2-s2-07-inject-cargo-pkg-version.md) / [stage2-open](stage2-open.md) Consequences; census Current=I265; next default = **Stage-2 exit**.
5. **Non-claims (hard):** ≠ Stage-2 exit this window / ≠ open Stage-3 construction / ≠ mass docs/books rewrite / ≠ delete phase / ≠ rewrite fingerprint salts / ≠ HostFn+1 / ≠ product umbrella / F-08 / ≠ change language semantics.

## Consequences

- S2-07 plan-claimed; Stage-2 exit **landed** as [stage2-exit](stage2-exit.md)（S2-I266）；next = **Stage-3 open**.
- Decision §4 locked next = **Stage-2 exit**; **landed** S2-I266 — not Stage-3 construction in the claim window.
- Prior: [s2-07-inject-cargo-pkg-version](stage2-s2-07-inject-cargo-pkg-version.md) · S2-I264.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-07-open](stage2-s2-07-open.md) · S2-I263
- [stage2-s2-07-inject-cargo-pkg-version](stage2-s2-07-inject-cargo-pkg-version.md) · S2-I264
- [stage2-s2-06-claim](stage2-s2-06-claim.md) · S2-I262 (claim pattern)
- [stage2-version-census](../stage2-version-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-07
