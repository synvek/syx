# ADR: Stage-3 S3-04 can_stack_bind 编译期缓存（S3-I67）

**Status:** Accepted (S3-I67) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I66 JIT env gate cache](stage3-s3-04-jit-env-gate-cache.md), `fib(30)` still ≈**1.7 s** (target **0.5**). Every Direct Call still paid `ends_with("_deinit")` + `parse_mangled_instance_method` via `call_can_stack_bind`. This window caches that fact at emit time on `FunctionUnit.can_stack_bind` (mirror of I64 `has_begin_try`). It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, `Value: Copy`, default JIT, or Stage-3 exit.

## Decision

1. [`FunctionUnit`](../../crates/syx-codegen/src/lib.rs) 新增 `#[serde(default)] pub can_stack_bind: bool`（S3-I53 bincode 兼容；旧 cache → `false` → 慢 Call，安全）。
2. `compute_can_stack_bind(name)`：`!ends_with("_deinit") && parse_mangled_instance_method(name).is_none()`；封函路径写入。
3. VM `call_can_stack_bind`：读 **`func.can_stack_bind`**（去掉热路径字符串税）。
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ 全量 match 重写 / ≠ criterion/`examples/fib` 本窗。

## Evidence

| Check | Evidence |
|-------|----------|
| can_stack_bind cache | `can_stack_bind` · `compute_can_stack_bind` · `S3-I67` in codegen + VM |
| Census | `can_stack_bind_cache_helper≥1` · Current=I67 |
| Wall | `fib30_s` 记方向（预期有限下降；**仍远高于 0.5**；prefer release） |
| Contract | `s3_04_can_stack_bind_cache_contract` |

## Consequences

- can_stack_bind cache hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I68 landed** as [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md)（claim-prep：examples/fib + criterion）。
- **S3-I69 landed** as [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md)（wall-strategy）。
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-jit-env-gate-cache](stage3-s3-04-jit-env-gate-cache.md) · S3-I66.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-jit-env-gate-cache](stage3-s3-04-jit-env-gate-cache.md) · S3-I66
- [stage3-s3-04-begintry-cache](stage3-s3-04-begintry-cache.md) · S3-I64
- [stage3-s3-04-call-stack-args](stage3-s3-04-call-stack-args.md) · S3-I60
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
