# ADR: Stage-3 S3-03 open — 标准库预编译/缓存（S3-I48）

**Status:** Accepted (S3-I48) · S3-03 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I47 S3-02 claim](stage3-s3-02-claim.md), Stage-3 default work shifts to [plan/02 S3-03](../../plan/02-improvement-roadmap.md): stdlib precompile/cache so `hello` starts **&lt; 0.15 s**. Honest inventory shows project `CachedModuleUnit` / `syx-cache-v*` for disk modules, while [embedded_std](../../crates/syx-driver/src/embedded_std.rs) still feeds **source** via `include_str!` and [ensure_prelude_modules_in_graph](../../crates/syx-driver/src/prelude_glue.rs) re-prepares core modules into the graph. This window **opens** the workstream with a stdlib-cache / hello-startup **census baseline** — it does **not** claim S3-03 `[x]`, ship precompiled std snapshots, or exit Stage-3.

## Gap: hello startup vs plan S3-03

| Claim | Today (honest) | plan/02 S3-03 target |
|-------|----------------|----------------------|
| `hello` wall time | Census ~**1.6–1.8 s** (cold≈warm; see baseline) · analysis doc historically **0.95 s** | **&lt; 0.15 s** |
| Incremental cache | `CachedModuleUnit` + `target/cache/*.bin` for project modules | Stdlib **precompiled** snapshot reusable on cold start |
| Stdlib load | `embedded_std` source embed → parse/semantic/MIR each graph build | Build-time serialize semantic/MIR (or equiv.) → load/lazy-load |
| Prelude | `ensure_prelude_modules_in_graph` may (re)compile many core modules | Shared warm snapshot; hello path avoids full re-prepare |

Baseline: [stage3-stdlib-cache-census](../census/stage3-stdlib-cache-census.md).

## Decision

1. **Open S3-03** as an active Stage-3 workstream; flip plan S3-03 `[ ]` → `[~]`.
2. **S3-I48 first slice** — publish stdlib-cache census (+ script); add a thin contract under `syx-driver/tests/` (**no** product cache/precompile edit this window).
3. **Ladder:** **S3-I49** = first construct knife (priority: **prelude/stdlib precompiled snapshot** loaded from `ensure_prelude_modules_in_graph` — census shows `--enable-std=false` already &lt;0.15 while `dedicated_prelude_cache_hit=0`) → residual cold/warm cuts → dedicated **S3-03 `[x]` claim** (measured `hello` &lt; 0.15 s).
4. **Non-claims (hard):** ≠ S3-03 `[x]` / ≠ claim `hello` &lt; 0.15 s this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ Wave·F-08 / ≠ open S3-04…07 construction / ≠ change language semantics / ≠ expand HostFn surface.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| Stdlib-cache census | [`docs/census/stage3-stdlib-cache-census.md`](../census/stage3-stdlib-cache-census.md) · `scripts/s3_03_stdlib_cache_census.sh` |
| Contract nail | `s3_03_stdlib_cache_census_contract` |
| Plan status | S3-03 `[x]` via I55 claim · Stage-3 `[~]` · S3-01/S3-02 `[x]` |

## Consequences

- S3-03 moves `[ ]` → `[~]`; Stage-3 stays `[~]`; S3-01/S3-02 stay `[x]`.
- **S3-I49 landed** as [stage3-s3-03-prelude-snapshot](stage3-s3-03-prelude-snapshot.md)（prelude 磁盘快照 hinge；`dedicated_prelude_cache_hit≥1`）。
- **S3-I50 landed** as [stage3-s3-03-warm-program-trim](stage3-s3-03-warm-program-trim.md)（entry reachability trim）。
- **S3-I51 landed** as [stage3-s3-03-warm-fixed-tax](stage3-s3-03-warm-fixed-tax.md)（cache-first + OnceLock + lazy hosts）。
- **S3-I52 landed** as [stage3-s3-03-warm-residual](stage3-s3-03-warm-residual.md)（bake tag + plugin Once + trim skip）。
- **S3-I53 landed** as [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md)（bincode roundtrip / 去掉 `skip_serializing_if`）。
- **S3-I54 landed** as [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md)（`include_bytes` PreludeSnapshot）。
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]` scoped claim）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-02-claim](stage3-s3-02-claim.md) · S3-I47.
- Stdlib-cache census Current → **S3-I55** · Claimed scoped `[x]`；VM-hotspot census Current → **S3-I56**.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-02-claim](stage3-s3-02-claim.md) · S3-I47
- [stage3-stdlib-cache-census](../census/stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) hello KPI
