# ADR: Stage-3 S3-03 plan claim（S3-I55）

**Status:** Accepted (S3-I55) · S3-03 `[x]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I54 prelude embed](stage3-s3-03-prelude-embed.md), construct hinges I48–I54 are landed and measured hello startup axes are under the S3-03 target: `hello_cold_s` ≈ **0.08**, `hello_prelude_hit_s` ≈ **0.08**, `hello_warm_s` ≈ **0.00** (all &lt; **0.15**); `hello_nostd_s` ≈ 0.00. This dedicated window flips the plan checkbox only — aligned with S3-01/S3-02 scoped claim. It does **not** exit Stage-3 or open S3-04 construction.

## Evidence

| Axis | Evidence |
|------|----------|
| Open | [S3-I48](stage3-s3-03-open.md) · stdlib-cache census |
| Prelude disk | [S3-I49](stage3-s3-03-prelude-snapshot.md) · `dedicated_prelude_cache_hit≥1` |
| Warm trim | [S3-I50](stage3-s3-03-warm-program-trim.md) · `program_trim_helper` / `entry_cache_bytes` |
| Fixed tax | [S3-I51](stage3-s3-03-warm-fixed-tax.md) · cache-first / OnceLock / lazy host |
| Residual | [S3-I52](stage3-s3-03-warm-residual.md) · bake tag / plugin Once / trim skip |
| Roundtrip | [S3-I53](stage3-s3-03-cache-roundtrip.md) · no bincode-hostile `skip_serializing_if` |
| Embed | [S3-I54](stage3-s3-03-prelude-embed.md) · `EMBEDDED_PRELUDE_SNAPSHOT` / `PRELUDE_EMBED_HIT` |
| Startup KPI | census `hello_cold_s` / `hello_warm_s` / `hello_prelude_hit_s` **&lt; 0.15**（measured） |
| Contract | `s3_03_claim_contract` · `s3_03_stdlib_cache_census_contract` |
| Honesty OUT | **≠** Stage-3 exit · **≠** S3-04…07 open this window |

## Scoped `[x]` meaning

Standard-library precompile/cache path makes **hello** cold / warm / prelude-hit startup **&lt; 0.15 s** (measured) via disk + embedded `PreludeSnapshot` — **not** 「zero `include_str!`」 / **not** 「build.rs compiles `.sy`」 / **not** 「Stage-3 exit」 / **not** 「zero residual stdlib source in binary」.

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S3-03 → `[x]`**.
2. Keep **Stage-3 → `[~]`** until an explicit Stage-3 exit ADR.
3. Update stdlib-cache census Current=I55 · **Claimed: scoped `[x]`**；链 open/I54 Consequences；next default = **S3-04 open** — **≠** Stage-3 exit / **≠** open S3-04 construction this window.
4. **Non-claims (hard):** ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-04…07 open or construct this window / ≠ language semantics / ≠ require zero `include_str!` / ≠ build.rs full `.sy` compile.

## Consequences

- S3-03 plan-claimed `[x]`; Stage-3 stays `[~]`（checklist ~**3/7 ≈ 43%**）.
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]` · VM-hotspot census）。
- Default next = **S3-I57** construct（call-path `Value` clone / frame）— not Stage-3 exit.
- Prior: [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md) · S3-I54 · open [S3-I48](stage3-s3-03-open.md).
- Stdlib-cache census Current → **S3-I55** · Claimed scoped `[x]`；VM-hotspot census Current → **S3-I56**.

## References

- [stage3-s3-03-open](stage3-s3-03-open.md) · S3-I48
- [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md) · S3-I54
- [stage3-stdlib-cache-census](../stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
