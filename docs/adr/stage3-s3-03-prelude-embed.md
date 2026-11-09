# ADR: Stage-3 S3-03 prelude embed（S3-I54）

**Status:** Accepted (S3-I54) · S3-03 `[x]`（via I55 claim）· Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I53 cache roundtrip](stage3-s3-03-cache-roundtrip.md), warm ≈ **0.00 s** and prelude_hit ≈ **0.08 s** (&lt;0.15) while cold ≈ **0.37 s**. Disk hit works; empty-`target/cache` cold still full-compiles prelude from `include_str!`. Goal: embed a `PreludeSnapshot` bincode so disk miss behaves like disk hit.

## Decision

1. **Check in** `crates/syx-driver/assets/prelude_snapshot.bin` produced by `scripts/gen_prelude_snapshot.sh` (run release `syx` once → copy `prelude_*.bin`). **Do not** compile `.sy` inside `build.rs`.
2. **`load_prelude_snapshot` embed fallback:** on disk miss/bad file, deserialize `include_bytes!(…)` / `EMBEDDED_PRELUDE_SNAPSHOT`; require `cache_version_tag` match; on success fill graph and bump `PRELUDE_EMBED_HIT`.
3. **`build.rs`:** `rerun-if-changed` the asset only.
4. **Non-claims (hard):** ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ S3-04…07 this construct window / ≠ language semantics / ≠ build.rs full `.sy` compile.

## Evidence

| Check | Evidence |
|-------|----------|
| Asset | `assets/prelude_snapshot.bin` / gen script |
| Fallback | `load_prelude_snapshot` → embed path |
| Hit | `PRELUDE_EMBED_HIT` |
| Census | Current=I54 · cold KPI |

## Consequences

- Cold measured ≈0.08 (&lt;0.15).
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]` scoped claim）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md) · S3-I53.
- Census Current → **S3-I55**.

## References

- [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md) · S3-I53
- [stage3-stdlib-cache-census](../census/stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
