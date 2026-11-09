# ADR: Stage-3 S3-03 prelude disk snapshot（S3-I49）

**Status:** Accepted (S3-I49) · S3-03 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I48 S3-03 open](stage3-s3-03-open.md), census shows `hello_nostd_s` already &lt; 0.15 while `dedicated_prelude_cache_hit=0`. Cold `syx run` rebuilds embedded prelude via [`ensure_prelude_modules_in_graph`](../../crates/syx-driver/src/prelude_glue.rs). Entry `target/cache/*.bin` skips the whole graph on warm hit, but there is **no** dedicated prelude artifact. This window lands the **first construct hinge**: disk prelude snapshot keyed by `cache_version_tag`. It does **not** claim S3-03 `[x]`, `hello` &lt; 0.15, or empty-cache first-cold parity (that needs embed — I50+).

## Decision

1. Persist a `PreludeSnapshot` (`cache_version_tag` + ordered `CachedModuleUnit` list) under `project_cache_dir(root)/prelude_{sanitized_tag}.bin`.
2. At the top of `ensure_prelude_modules_in_graph`, `load_prelude_snapshot` when tag matches; on hit fill `graph` and count `PRELUDE_SNAPSHOT_HIT`.
3. On miss, keep today’s compile path; after successful ensure, `store_prelude_snapshot`.
4. Freshness = `cache_version_tag()` only (same family as entry incremental cache). No `build.rs` embed this window.

## Evidence

| Check | Evidence |
|-------|----------|
| Load/store API | `load_prelude_snapshot` / `store_prelude_snapshot` / `prelude_cache_path` |
| Hit nail | `PRELUDE_SNAPSHOT_HIT` · census `dedicated_prelude_cache_hit≥1` |
| Timing axis | `hello_prelude_hit_s`（删入口 `.bin`、保留 prelude） |
| Contract | `s3_03_*` census / prelude snapshot contracts |
| Plan | S3-03 remains `[~]` · Stage-3 `[~]` |

## Non-claims (hard)

≠ S3-03 `[x]` / ≠ claim `hello` &lt; 0.15 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ Wave·F-08 / ≠ S3-04…07 / ≠ language semantics / ≠ change entry-cache early-return / ≠ `build.rs` embed.

## Consequences

- `dedicated_prelude_cache_hit` moves **0 → ≥1**.
- Empty-`target/cache` **first** cold may still be slow; second cold / prelude-hit path improves — honest in census.
- **S3-I50 landed** as [stage3-s3-03-warm-program-trim](stage3-s3-03-warm-program-trim.md)（entry reachability trim；`entry_cache_bytes` ↓）。
- **S3-I51 landed** as [stage3-s3-03-warm-fixed-tax](stage3-s3-03-warm-fixed-tax.md)（cache-first + OnceLock + lazy hosts）。
- **S3-I52 landed** as [stage3-s3-03-warm-residual](stage3-s3-03-warm-residual.md)（bake tag + plugin Once + trim skip）。
- **S3-I53 landed** as [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md)（bincode roundtrip）。
- **S3-I54 landed** as [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md)（`include_bytes`）。
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]`）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-03-open](stage3-s3-03-open.md) · S3-I48.
- Census Current → **S3-I55**.

## References

- [stage3-s3-03-open](stage3-s3-03-open.md) · S3-I48
- [stage3-stdlib-cache-census](../census/stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
