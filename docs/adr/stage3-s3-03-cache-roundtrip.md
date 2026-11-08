# ADR: Stage-3 S3-03 cache roundtrip（S3-I53）

**Status:** Accepted (S3-I53) · S3-03 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I52 warm residual](stage3-s3-03-warm-residual.md), `hello_warm_s` ≈ `hello_cold_s` ≈ **0.35–0.36 s** while `hello_nostd_s` ≈ **0.01 s**. Prior knives (cache-first, OnceLock, lazy hosts, bake tag, plugin Once, trim skip) barely moved wall time because **entry/prelude `.bin` write succeeds but `bincode::deserialize` fails** (`UnexpectedEof` from `skip_serializing_if` on `FunctionUnit` / `CompiledProgram`). `purge_stale` then deletes the bad file → every enable-std run full-recompiles prelude. I49 `PreludeSnapshot` is the same write-only disease. nostd skips `ensure_prelude`, hence ~0.01 s.

## Decision

1. **Remove bincode-hostile `skip_serializing_if`** on `FunctionUnit` (`instruction_spans` / `local_names` / `local_types` / `source_name`) and `CompiledProgram::opaque_witnesses`; keep `#[serde(default)]`. Accept one-shot cache tip invalidation on layout change.
2. **Contract-nail roundtrip:** serialize→deserialize `FunctionUnit` / `CompiledProgram` (and census `cache_roundtrip_helper`) must pass.
3. **Non-claims (hard):** ≠ S3-03 `[x]` / ≠ claim `hello` &lt; 0.15 unless measured / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ S3-04…07 / ≠ prelude embed / ≠ language semantics.

## Evidence

| Check | Evidence |
|-------|----------|
| Serde fix | `FunctionUnit` / `opaque_witnesses` without `skip_serializing_if` |
| Roundtrip | `s3_03_cache_roundtrip_contract` / bincode serialize→deserialize |
| Census | Current=I53 · warm KPI |

## Consequences

- True disk cache hit pulls warm / prelude-hit under 0.15; cold needed I54 embed.
- **S3-I54 landed** as [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md)（`include_bytes` PreludeSnapshot fallback）。
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]`）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-03-warm-residual](stage3-s3-03-warm-residual.md) · S3-I52.
- Census Current → **S3-I55**.

## References

- [stage3-s3-03-warm-residual](stage3-s3-03-warm-residual.md) · S3-I52
- [stage3-stdlib-cache-census](../stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
