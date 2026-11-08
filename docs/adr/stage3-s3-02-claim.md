# ADR: Stage-3 S3-02 plan claim（S3-I47）

**Status:** Accepted (S3-I47) · S3-02 `[x]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I46 meta-type-display](stage3-s3-02-meta-type-display.md), user-facing mono-display axes ①–⑤ are hinged (I43–I46) with census helpers ≥1 and `named_passthrough=0`. This dedicated window flips the plan checkbox only — **no** separate claim-prep window (construct debt already cleared). It does **not** exit Stage-3 or open S3-03 construction.

## Evidence

| Axis | Evidence |
|------|----------|
| ① 诊断类型串 | `type_id_to_name_named_passthrough=0` · `user_facing_type_name_helper≥1`（[I43](stage3-s3-02-demangle-semantic.md)） |
| ② LSP/IDE | 同 `type_id_to_name` / demangle 路径（I43 附带） |
| ③ Debug 栈 | `user_facing_mir_fn_name_helper≥1` · DAP / `stack_summary`（[I44](stage3-s3-02-debug-stack-display.md)） |
| ④ Runtime 展示 | `user_facing_runtime_type_name_helper≥1`（[I45](stage3-s3-02-runtime-type-display.md)） |
| ⑤ Meta/reflect | `user_facing_meta_type_name_helper≥1` · `host_reflect_type`（[I46](stage3-s3-02-meta-type-display.md)） |
| ⑥ 本窗 | plan S3-02 → `[x]`；Stage-3 仍 `[~]` |
| Prior | [meta-type-display](stage3-s3-02-meta-type-display.md) · S3-I46 · open [S3-I42](stage3-s3-02-open.md) |
| Contract | `s3_02_claim_contract` · `s3_02_mono_display_census_contract` |
| Honesty OUT | **≠** Stage-3 exit · **≠** S3-03 open/complete this window |

## Scoped `[x]` meaning

User-facing surfaces show generic / method forms via demangle helpers — **not** 「断点 / MIR `f.name` 归零」 / **not** 「identity API demangle」 / **not** 「停 `mangle_name`」 / **not** 「Stage-3 exit」.

## Intentional OUT（不挡 `[x]`）

- 断点 / `FunctionUnit.name` / `debug_mangled_fn_nail≥1`（非归零闸）
- Identity：`object_ref_type_name` · `type_id_to_ir_name` · `type_expr_to_type_name` / `field_types` · `mangle_name` · MIR `f.name`
- 内部 dump / VM cast·deinit·dispatch 身份串；`__type` **写入**可仍 mangled

## Decision

1. Flip [`plan/02`](../../plan/02-improvement-roadmap.md) **S3-02 → `[x]`**.
2. Keep **Stage-3 → `[~]`** until an explicit Stage-3 exit ADR.
3. Update mono-display census Current=I47；链 ADR Consequences；next default = **S3-03 open**（标准库预编译/缓存）— **≠** Stage-3 exit this window.
4. **Non-claims (hard):** ≠ Stage-3 exit / ≠ open or complete S3-03 construction this window / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ `debug_mangled_fn_nail→0` / ≠ 改身份存储 / ≠ 停 `mangle_name`.

## Consequences

- S3-02 plan-claimed `[x]`; Stage-3 stays `[~]`.
- Default next = **S3-03 open** — not Stage-3 exit.
- Prior: [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md) · S3-I46.
- Census Current → **S3-I47**.

## References

- [stage3-s3-02-open](stage3-s3-02-open.md) · S3-I42
- [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md) · S3-I43
- [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md) · S3-I44
- [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md) · S3-I45
- [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md) · S3-I46
- [stage3-mono-display-census](../stage3-mono-display-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-02
