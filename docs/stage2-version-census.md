# Stage-2 version census (S2-07)

**ADR:** [stage2-s2-07-claim](adr/stage2-s2-07-claim.md) · **Window:** S2-I265 · **Current:** S2-I265  
**Open baseline:** [stage2-s2-07-open](adr/stage2-s2-07-open.md) · S2-I263 · **Inject:** [stage2-s2-07-inject-cargo-pkg-version](adr/stage2-s2-07-inject-cargo-pkg-version.md) · S2-I264

**Goal (S2-07):** single **public** version = crate semver (`0.x.y`); phase / marketing `117.0` internal-only or ungated.  
**Status:** S2-07 **`[x]`** scoped (S2-I265) — **≠** Stage-2 exit.

## Command

```bash
bash scripts/s2_07_version_census.sh
```

## S2-I263 open baseline (2026-11-07)

| Metric | Value |
|--------|------:|
| Workspace crate semver | **`0.1.3`** |
| Product manifest version | **`117.0`** (dual-track) |
| Capabilities `syx_version` | **`117.0`** |
| Driver `build.rs` embed | hardcoded **`117.0`** + phase **139** |
| Evergreen nails | asserted literal **`117.0`** |

## S2-I264 inject (2026-11-07)

| Metric | Value |
|--------|------:|
| `build.rs` public version | **`env!("CARGO_PKG_VERSION")`** |
| Product manifest / capabilities | **`0.1.3`** |
| Evergreen version nails | assert **`CARGO_PKG_VERSION`** |
| Phase surface | **139** retained (internal OK) |

## S2-I265 claim (2026-11-07) — Current

| Metric | Value |
|--------|------:|
| Plan S2-07 | **`[x]`** scoped |
| Stage-2 | **`[~]`** (exit pending) |
| Public SSOT | crate semver / `CARGO_PKG_VERSION` = **`0.1.3`** |
| Residual OK | phase 139; fingerprint salts; docs/books「SYX 117.0」narrative |

## Classification (honest)

| Class | Notes |
|-------|-------|
| **Public SSOT** | Cargo / embedded manifest **`0.1.3`** via `CARGO_PKG_VERSION` |
| **Internal OK residual** | phase **139**; fingerprint strings; historical job/file names |
| **Narrative residual** | docs/books/ADR charter may still say SYX 117.0 — OK for scoped `[x]` |

## Gap vs plan/02 S2-07

| Claim | Today (honest) | plan/02 S2-07 target |
|-------|----------------|----------------------|
| Single public version = crate semver | **Done** · **claimed** S2-I265 | One public number from `CARGO_PKG_VERSION` |
| phase/117.0 only internal | phase field + docs residual OK under scoped `[x]` | Internal fingerprint / narrative only |
| Contributor / release honesty | release.yml + product surfaces on `0.1.3` | Align product surfaces to crate semver |

## Ladder

1. **S2-I263:** version census + open ADR — landed.
2. **S2-I264:** inject public surfaces from `CARGO_PKG_VERSION` — landed.
3. **S2-I265:** dedicated **S2-07 `[x]` claim** — **this window**; ≠ Stage-2 exit.
4. Next default: **S2-I266 Stage-2 exit**.

## Non-claims (post-claim honesty)

- **≠** Stage-2 exit this window  
- **≠** mass docs/books rewrite required for scoped `[x]`  
- **≠** HostFn+1 / product umbrella unpark / F-08 / Stage-3 construction this window  

## References

- [stage2-s2-07-claim](adr/stage2-s2-07-claim.md) · S2-I265
- [stage2-s2-07-inject-cargo-pkg-version](adr/stage2-s2-07-inject-cargo-pkg-version.md) · S2-I264
- [stage2-s2-07-open](adr/stage2-s2-07-open.md) · S2-I263
- [plan/02](../plan/02-improvement-roadmap.md) S2-07
