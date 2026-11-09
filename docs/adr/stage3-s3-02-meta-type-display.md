# ADR: Stage-3 S3-02 meta type user-facing display（S3-I46）

**Status:** Accepted (S3-I46) · S3-02 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I45 runtime-type-display](stage3-s3-02-runtime-type-display.md), runtime `__type` display demangles, but meta [`type_expr_to_type_name`](../../crates/syx-meta/src/lib.rs) Named leaves still pass mangled identity into registry / reflection surfaces. This window adds a **display-only** meta read path — still **≠** S3-02 `[x]`, still ≠ Stage-3 exit, still ≠ claim.

## Decision

1. **Identity:** keep [`type_expr_to_type_name`](../../crates/syx-meta/src/lib.rs) + `field_types` storage **mangled/raw**（VM Codable `reflect_type(strip_optional(ft))` 查找键）.
2. **User-facing API:** add `type_expr_to_user_facing_type_name`（Named 叶 → [`demangle_mono_name`](../../crates/syx-mono/src/lib.rs)）+ `user_facing_meta_type_name`；[`TypeMetadata::user_facing_field_types`](../../crates/syx-meta/src/lib.rs).
3. **Display wire:** [`host_reflect_type`](../../crates/syx-vm/src/stdlib_phase85_hosts.rs) `name` demangle；增 `fieldTypes` 用户面数组。
4. **Gate KPI：** `user_facing_meta_type_name_helper≥1`；`named_passthrough=0` 保持；`meta_type_expr_to_type_name` = identity 计数（≈13，**不**硬锁归零）.
5. **Ladder next：** **S3-I47** = S3-02 scoped claim（**landed**）→ **S3-03 open** — still ≠ Stage-3 exit.
6. **Non-claims (hard):** ≠ S3-02 `[x]` / ≠ Stage-3 exit / ≠ 改 `field_types` 身份存储 / ≠ 改 VM Codable 查找语义 / ≠ open S3-03…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Meta API | identity `type_expr_to_type_name` · user-facing helper + unit tests |
| Reflect display | `host_reflect_type` name + `fieldTypes` |
| Census | `user_facing_meta_type_name_helper≥1` · `named_passthrough=0` |
| Contract | `s3_02_mono_display_census_contract`（I46） |
| Plan status | S3-02 `[~]` · Stage-3 `[~]` |

## Consequences

- S3-02 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- Default next = **S3-03 open** — not Stage-3 exit.
- Prior: [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md) · S3-I45.
- Census Current → **S3-I47**.

## References

- [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md) · S3-I45
- [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md) · S3-I44
- [stage3-mono-display-census](../census/stage3-mono-display-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-02
