# ADR: Stage-3 S3-02 runtime `__type` user-facing display（S3-I45）

**Status:** Accepted (S3-I45) · S3-02 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I44 debug-stack-display](stage3-s3-02-debug-stack-display.md), DAP/summary fn names demangle, but runtime `__type` / `object_ref_type_name` still exposed mangled identity (`Foo_Int`) on user-facing description / DAP type surfaces. This window adds a **display-only** runtime read path — still **≠** S3-02 `[x]`, still ≠ Stage-3 exit, still ≠ meta / claim.

## Decision

1. **Demangle SSOT:** move [`demangle_mono_name`](../../crates/syx-mono/src/lib.rs) (+ `is_mono_type_arg_segment`) into [`syx-mono`](../../crates/syx-mono/src/lib.rs) as inverse of `mangle_name`; [`syx-semantic`](../../crates/syx-semantic/src/semantic_type_codec.rs) re-exports.
2. **Runtime API:** keep [`object_ref_type_name`](../../crates/syx-runtime-core/src/value.rs) as **identity** (may be mangled; VM dispatch / deinit); add `object_ref_user_facing_type_name()` → `user_facing_runtime_type_name` / demangle.
3. **Display wire:** [`value_description`](../../crates/syx-runtime-core/src/protocols.rs) ObjectRef empty-fields branch + [`syx-debug::value_type_name`](../../crates/syx-debug/src/lib.rs) for object_ref / Dict/Object `__type` use user-facing.
4. **Gate KPI：** `user_facing_runtime_type_name_helper≥1`；`named_passthrough=0` 保持；`object_ref_type_name` = identity API 计数（≈9–10，**不**硬锁 =9）；meta KPI 不变。
5. **Ladder next：** **S3-I46** = meta `type_expr_to_type_name` 用户面（或 S3-02 claim-prep）— still ≠ S3-02 `[x]`.
6. **Non-claims (hard):** ≠ S3-02 `[x]` / ≠ Stage-3 exit / ≠ meta 专改 / ≠ 改 `__type` 写入 / ≠ 改 VM `resolve_virtual_method` / deinit 身份串 / ≠ open S3-03…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Demangle SSOT | `syx-mono::demangle_mono_name` + semantic re-export |
| Runtime API | `object_ref_type_name` mangled · `object_ref_user_facing_type_name` demangled |
| Display | protocols `value_description` · debug `value_type_name` |
| Census | `user_facing_runtime_type_name_helper≥1` · `named_passthrough=0` |
| Contract | `s3_02_mono_display_census_contract`（I45） |
| Plan status | S3-02 `[~]` · Stage-3 `[~]` |

## Consequences

- S3-02 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- Default next = **S3-03 open** — not Stage-3 exit.
- Prior: [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md) · S3-I44.
- Census Current → **S3-I47**.

## References

- [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md) · S3-I44
- [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md) · S3-I43
- [stage3-s3-02-open](stage3-s3-02-open.md) · S3-I42
- [stage3-mono-display-census](../census/stage3-mono-display-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-02
