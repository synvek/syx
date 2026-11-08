# ADR: Stage-3 S3-02 demangle semantic hinge（S3-I43）

**Status:** Accepted (S3-I43) · S3-02 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I42 S3-02 open](stage3-s3-02-open.md), census showed `type_id_to_name_named_passthrough=1` and `user_facing_type_name_helper=0`. This window adds demangle helpers and flips the Named hinge for **user-facing** display — still **≠** S3-02 `[x]`, still ≠ Stage-3 exit, still ≠ debug/runtime dedicated faces.

## Decision

1. **API:** add [`demangle_mono_name`](../../crates/syx-semantic/src/semantic_type_codec.rs) / [`user_facing_type_name`](../../crates/syx-semantic/src/semantic_type_codec.rs) / [`type_id_to_ir_name`](../../crates/syx-semantic/src/semantic_type_codec.rs); inverse of [`mangle_name`](../../crates/syx-mono/src/lib.rs) for builtin type-arg suffixes; nested `_` → `.` heuristic; else raw.
2. **铰链（IN）：** [`type_id_to_name`](../../crates/syx-semantic/src/semantic_type_codec.rs) demangles `TypeId::Named`; codegen uses **`type_id_to_ir_name`** so IR identity stays mangled.
3. **Gate KPI：** `type_id_to_name_named_passthrough=0` · `user_facing_type_name_helper≥1`; debug/runtime KPIs unchanged.
4. **Ladder next：** **S3-I44** = debug stack user-facing names（or LSP residual）— still ≠ S3-02 `[x]`.
5. **Non-claims (hard):** ≠ S3-02 `[x]` / ≠ Stage-3 exit / ≠ debug `Greeter_hello` face / ≠ `object_ref_type_name` / ≠ meta / ≠ change `mangle_name` / ≠ open S3-03…07 / ≠ HostFn+1 / ≠ F-08 / ≠ change diagnostics defaults.

## Evidence

| Check | Evidence |
|-------|----------|
| Named passthrough = 0 | census `type_id_to_name_named_passthrough=0` |
| Helper present | `user_facing_type_name_helper≥1`（`demangle_mono_name` / `user_facing_type_name`） |
| Unit tests | `semantic_type_codec::demangle_tests` |
| Contract | `s3_02_mono_display_census_contract`（I43 assertions） |
| Plan status | S3-02 `[~]` · Stage-3 `[~]` |

## Consequences

- S3-02 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I44 landed** as [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md)（DAP/summary 用户面名）.
- **S3-I45 landed** as [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md)（runtime `__type` 用户面）.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- Default next after I46 = **S3-I47** claim-prep（或 scoped claim）— not S3-02 claim.
- Prior: [stage3-s3-02-open](stage3-s3-02-open.md) · S3-I42.
- Census Current → **S3-I47**.

## References

- [stage3-s3-02-open](stage3-s3-02-open.md) · S3-I42
- [stage3-mono-display-census](../stage3-mono-display-census.md)
- [stage3-open](stage3-open.md) · S3-I1
- [plan/02](../../plan/02-improvement-roadmap.md) S3-02
