# ADR: Stage-3 S3-03 warm fixed tax（S3-I51）

**Status:** Accepted (S3-I51) · S3-03 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I50 warm program trim](stage3-s3-03-warm-program-trim.md), `entry_cache_bytes` dropped ~150KB→~5KB but `hello_warm_s` stayed ≈0.39–0.40s while `hello_nostd_s` ≈0.04s. Warm entry hits skip prelude compile; the gap is **driver fixed tax before cache hit** (`module_hash` re-hashing embedded prelude, runtime `rustc -V`, purge/manifest) plus **VM `register_default_hosts` (~630)** on every `WasmVM::new`.

## Decision

1. **`compile_linked` cache-first:** try incremental cache before plugin/manifest/purge heavy setup; on hit return early (Service-DI skip retained).
2. **Fingerprint OnceLock:** memoize `witness_table_digest` / `unified_mir_fn_count_digest`; bake `rustc_version_tag` at **build time** (no runtime `rustc -V` spawn).
3. **VM lazy hosts:** do not register all HostFnId at `new()`; resolve via `host_fn_for` on `CallHost` miss (and optionally seed from `program.host_fn_names`).
4. **Non-claims (hard):** ≠ S3-03 `[x]` / ≠ claim `hello` &lt; 0.15 unless measured / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ S3-04…07 / ≠ embed / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Cache-first | `compile_linked_with_options` early cache probe |
| OnceLock digests | `witness_table_digest` / `unified_mir_fn_count_digest` |
| Compile-time rustc | `SYX_RUSTC_VERSION` / `rustc_version_tag` |
| Lazy hosts | `ensure_host_fn` / no full `register_default_hosts` in `new` |
| Census | Current=I51 · warm KPI |

## Consequences

- Warm / prelude-hit should drop toward nostd/0.15; cold may still need I53 embed.
- **S3-I52 landed** as [stage3-s3-03-warm-residual](stage3-s3-03-warm-residual.md)（bake tag + plugin Once + trim skip）。
- **S3-I53 landed** as [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md)（bincode roundtrip）。
- **S3-I54 landed** as [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md)（`include_bytes`）。
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]`）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-03-warm-program-trim](stage3-s3-03-warm-program-trim.md) · S3-I50.
- Census Current → **S3-I55**.

## References

- [stage3-s3-03-warm-program-trim](stage3-s3-03-warm-program-trim.md) · S3-I50
- [stage3-stdlib-cache-census](../stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
