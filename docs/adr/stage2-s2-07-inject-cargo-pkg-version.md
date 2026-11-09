# ADR: Stage-2 S2-07 inject CARGO_PKG_VERSION (S2-I264)

**Status:** Accepted (S2-I264) · S2-07 `[~]` · Stage-2 `[~]`  
**Date:** 2026-11-07  
**Context:** After [S2-I263 S2-07 open](stage2-s2-07-open.md), public surfaces still advertised marketing **`117.0`** while crates were already **`0.1.3`**. This window injects the public version from `CARGO_PKG_VERSION`, relaxes evergreen nails, and keeps phase **139** as an internal census value. It does **not** claim S2-07 `[x]` or exit Stage-2.

## Decision

1. [`syx-driver/build.rs`](../../crates/syx-driver/build.rs): manifest `"version"` ← `env!("CARGO_PKG_VERSION")`; keep `"phase": 139`.
2. Align checked-in [`packages/internal/manifest.json`](../../packages/internal/manifest.json) + capabilities `syx_version` / generated meta to **`0.1.3`**.
3. Evergreen `smoke` / `stdlib_parity` / stale `integration::manifest_v41`: assert version == `env!("CARGO_PKG_VERSION")`; phase 139 OK.
4. Thin: faerlin `min_syx_version` → `0.1.3`.
5. **Non-claims (hard):** ≠ S2-07 `[x]` / ≠ Stage-2 exit / ≠ mass docs/books rewrite / ≠ delete phase / ≠ rewrite fingerprint salts / ≠ HostFn+1 / ≠ product umbrella / F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Inject | `build.rs` uses `CARGO_PKG_VERSION` |
| Manifest | `packages/internal/manifest.json` `"version": "0.1.3"` |
| Evergreen | `manifest_` filter green against semver |
| Census | [stage2-version-census](../census/stage2-version-census.md) Current=I264 |
| Plan | S2-07 remains `[~]` |

## Consequences

- Public product version = crate semver; phase/fingerprint/docs narrative residual OK.
- S2-07 stays `[~]`; Stage-2 stays `[~]`.
- Default next = **S2-I265 S2-07 `[x]` claim**; **landed** as [s2-07-claim](stage2-s2-07-claim.md)（S2-I265）；next = **S2-I266 Stage-2 exit**.
- Prior: [s2-07-open](stage2-s2-07-open.md) · S2-I263.

## References

- [stage2-open](stage2-open.md)
- [stage2-s2-07-open](stage2-s2-07-open.md) · S2-I263
- [stage2-version-census](../census/stage2-version-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S2-07
