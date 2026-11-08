# ADR: Stage-3 S3-03 warm residual（S3-I52）

**Status:** Accepted (S3-I52) · S3-03 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I51 warm fixed tax](stage3-s3-03-warm-fixed-tax.md), `hello_warm_s` ≈ `hello_cold_s` ≈ `hello_prelude_hit_s` ≈ **0.36–0.38 s** while `hello_nostd_s` ≈ **0.01 s**. Cache-first / digest OnceLock / lazy hosts landed but wall time barely moved: the remaining tax is **per-process first-touch** — especially `cache_entry_fresh` → `cache_version_tag` → `heap_v100_phase128_track_c_wasm_digest` (~194× `HostFnId::all()`), duplicate `load_from_env` directory probes, and a redundant warm `trim_program_to_entry_reachability`. Embed alone cannot close the warm≃cold gap.

## Decision

1. **Bake `SYX_CACHE_VERSION_TAG` at build time** (`build.rs` + `cache_version_tag`): runtime prefers `option_env!("SYX_CACHE_VERSION_TAG")` — zero first-touch heap digest on hello warm; tip invalidates when driver rebuilds (fingerprint / HostFn registry content hash + pkg + rustc). Fallback remains `compute_cache_version_tag` / OnceLock.
2. **`load_from_env` Once** (`syx-host-plugin`): memoize first probe; `run_exec` drops the duplicate call on `run_compiled_program_with_strict` when the outer `run_file*` already loaded.
3. **Warm trim skip:** `run_compiled_program_with_strict` no longer unconditionally re-trims (I50 already trims before cache write); keep trim on paths that may still hold a fat program (`run_file_printed`).
4. **Non-claims (hard):** ≠ S3-03 `[x]` / ≠ claim `hello` &lt; 0.15 unless measured / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ S3-04…07 / ≠ prelude embed / ≠ language semantics / ≠ weaken cache invalidation intent.

## Evidence

| Check | Evidence |
|-------|----------|
| Bake tag | `SYX_CACHE_VERSION_TAG` / `cache_version_tag` |
| Plugin Once | `load_from_env` OnceLock |
| Trim skip | `run_compiled_program_with_strict` without unconditional trim |
| Census | Current=I52 · warm KPI |

## Consequences

- Warm / prelude-hit should drop toward nostd/0.15; cold may still need I54 embed.
- **S3-I53 landed** as [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md)（bincode roundtrip / 去掉 `skip_serializing_if`）。
- **S3-I54 landed** as [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md)（`include_bytes`）。
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]`）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-03-warm-fixed-tax](stage3-s3-03-warm-fixed-tax.md) · S3-I51.
- Census Current → **S3-I55**.

## References

- [stage3-s3-03-warm-fixed-tax](stage3-s3-03-warm-fixed-tax.md) · S3-I51
- [stage3-stdlib-cache-census](../stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
