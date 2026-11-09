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
- **S3-I48 landed** as [stage3-s3-03-open](stage3-s3-03-open.md)（S3-03 `[~]` + stdlib-cache census）。
- **S3-I49 landed** as [stage3-s3-03-prelude-snapshot](stage3-s3-03-prelude-snapshot.md)（prelude 磁盘快照 hinge）。
- **S3-I50 landed** as [stage3-s3-03-warm-program-trim](stage3-s3-03-warm-program-trim.md)（entry reachability trim）。
- **S3-I51 landed** as [stage3-s3-03-warm-fixed-tax](stage3-s3-03-warm-fixed-tax.md)（cache-first + OnceLock + lazy hosts）。
- **S3-I52 landed** as [stage3-s3-03-warm-residual](stage3-s3-03-warm-residual.md)（bake tag + plugin Once + trim skip）。
- **S3-I53 landed** as [stage3-s3-03-cache-roundtrip](stage3-s3-03-cache-roundtrip.md)（bincode roundtrip）。
- **S3-I54 landed** as [stage3-s3-03-prelude-embed](stage3-s3-03-prelude-embed.md)（`include_bytes`）。
- **S3-I55 landed** as [stage3-s3-03-claim](stage3-s3-03-claim.md)（S3-03 `[x]`）。
- **S3-I56 landed** as [stage3-s3-04-open](stage3-s3-04-open.md)（S3-04 `[~]`）。
- Default next = **S3-I57** construct — not Stage-3 exit.
- Prior: [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md) · S3-I46.
- Mono-display census Current → **S3-I47**；stdlib-cache census Current → **S3-I55**.

## References

- [stage3-s3-02-open](stage3-s3-02-open.md) · S3-I42
- [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md) · S3-I43
- [stage3-s3-02-debug-stack-display](stage3-s3-02-debug-stack-display.md) · S3-I44
- [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md) · S3-I45
- [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md) · S3-I46
- [stage3-mono-display-census](../census/stage3-mono-display-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-02
