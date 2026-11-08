# ADR: Stage-3 S3-02 debug stack user-facing names（S3-I44）

**Status:** Accepted (S3-I44) · S3-02 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I43 demangle-semantic](stage3-s3-02-demangle-semantic.md), type diagnostics demangle Named forms, but DAP / `stack_summary` still exposed MIR mangled fn names (`Greeter_hello`). This window wires **display-only** demangle for debug stacks — still **≠** S3-02 `[x]`, still ≠ Stage-3 exit, still ≠ runtime `__type`.

## Decision

1. **API:** add [`user_facing_mir_fn_name`](../../crates/syx-semantic/src/semantic_type_codec.rs) combining `parse_mangled_instance_method` + [`demangle_mono_name`](../../crates/syx-semantic/src/semantic_type_codec.rs)（`_init` → `.init`；mono type-arg RHS stays type demangle）.
2. **定点修（IN）：** [`stack_frames_dap`](../../crates/syx-debug/src/lib.rs) / [`stack_summary`](../../crates/syx-debug/src/lib.rs) 展示串走 helper；**断点 / `FunctionUnit.name` 匹配保持 MIR mangled**.
3. **Gate KPI：** `user_facing_mir_fn_name_helper≥1`；`object_ref_type_name=9` 不变；`named_passthrough=0` 保持；**不**要求 `debug_mangled_fn_nail→0`（断点字面量可留）。
4. **Ladder next：** **S3-I45** = runtime `object_ref_type_name` 用户面 demangle — still ≠ S3-02 `[x]`.
5. **Non-claims (hard):** ≠ S3-02 `[x]` / ≠ Stage-3 exit / ≠ `object_ref_type_name` 施工 / ≠ meta / ≠ 改 MIR `f.name` / ≠ open S3-03…07 / ≠ HostFn+1 / ≠ F-08.

## Evidence

| Check | Evidence |
|-------|----------|
| Helper | `user_facing_mir_fn_name` + unit tests |
| DAP display | `dap_stack_frame_name_is_user_facing_mir_fn` |
| Census | `user_facing_mir_fn_name_helper≥1` · `object_ref_type_name=9` |
| Contract | `s3_02_mono_display_census_contract`（I44） |
| Plan status | S3-02 `[~]` · Stage-3 `[~]` |

## Consequences

- S3-02 stays `[~]`; Stage-3 stays `[~]`.
- **S3-I45 landed** as [stage3-s3-02-runtime-type-display](stage3-s3-02-runtime-type-display.md)（runtime `__type` 用户面）.
- **S3-I46 landed** as [stage3-s3-02-meta-type-display](stage3-s3-02-meta-type-display.md)（meta 用户面）.
- **S3-I47 landed** as [stage3-s3-02-claim](stage3-s3-02-claim.md)（S3-02 `[x]` scoped claim）.
- Default next = **S3-03 open** — not Stage-3 exit.
- Prior: [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md) · S3-I43.
- Census Current → **S3-I47**.

## References

- [stage3-s3-02-demangle-semantic](stage3-s3-02-demangle-semantic.md) · S3-I43
- [stage3-s3-02-open](stage3-s3-02-open.md) · S3-I42
- [stage3-mono-display-census](../stage3-mono-display-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-02
