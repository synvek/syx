# ADR: Stage-3 S3-03 warm floor — entry program reachability trim（S3-I50）

**Status:** Accepted (S3-I50) · S3-03 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I49 prelude disk snapshot](stage3-s3-03-prelude-snapshot.md), release census shows `hello_prelude_hit_s` ≈ `hello_warm_s` ≈ **0.41 s** while `hello_nostd_s` ≈ **0.04 s**. Warm skips prelude compile; the ~0.35 s gap is the **fat entry `CompiledProgram`** (~150KB merged stdlib bytecode in `CachedModuleUnit`) plus deserialize/VM. Embed alone cannot break the 0.41 floor. This window trims unreachable functions from the entry program and skips useless Service-DI refresh on cache hits.

## Decision

1. Add `trim_program_to_entry_reachability` in syx-codegen: BFS from `entry_point` + `__init_global` along `Call` / `CallSuper` / `ActorCall` / `ActorSend` / `TaskSpawn` / `MakeClosure` / `AwaitSuspend.resume_fn`; remap indices; drop unreachable `FunctionUnit`s. If reachable code uses `CallIndirect`, keep the full function table (conservative).
2. Apply trim before entry `bincode_encode` and at `run_compiled_program*` entry.
3. On incremental cache hit, skip `refresh_service_di_from_source` when source has no Service-DI markers.
4. **Non-claims (hard):** ≠ S3-03 `[x]` / ≠ claim `hello` &lt; 0.15 unless measured / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ S3-04…07 / ≠ `build.rs` embed / ≠ language semantics / ≠ change merge discovery.

## Evidence

| Check | Evidence |
|-------|----------|
| Trim API | `trim_program_to_entry_reachability` |
| Mounts | `compile_linked` encode · `run_exec` |
| DI skip | cache-hit without Service markers |
| Census | `hello_warm_s` / `entry_cache_bytes` · Current=I50 |
| Plan | S3-03 `[~]` · Stage-3 `[~]` |

## Consequences

- Warm / prelude-hit should drop toward nostd; empty-cache first cold still may stay high (I53 embed).
- **S3-I51 landed** as [stage3-s3-03-warm-fixed-tax](stage3-s3-03-warm-fixed-tax.md)（cache-first + OnceLock + lazy hosts）。
- **S3-I52 landed** as [stage3-s3-03-warm-residual](stage3-s3-03-warm-residual.md)（bake tag + plugin Once + trim skip）。
- **S3-I53 landed** as [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md)（bincode roundtrip）。
- **S3-I54 landed** as [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md)（`include_bytes`）。
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]`）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-03-prelude-snapshot](stage3-s3-03-prelude-snapshot.md) · S3-I49.
- Census Current → **S3-I55**.

## References

- [stage3-s3-03-prelude-snapshot](stage3-s3-03-prelude-snapshot.md) · S3-I49
- [stage3-stdlib-cache-census](../stage3-stdlib-cache-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-03
