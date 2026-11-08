# ADR: Stage-3 S3-04 BeginTry 编译期缓存（S3-I64）

**Status:** Accepted (S3-I64) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I63 frame-swap](stage3-s3-04-frame-swap.md), `fib(30)` still ≈**1.7+ s** (target **0.5**). I61 try-free Call still paid a **per-Call full-table `BeginTry` scan** via `instructions.iter().any(...)`. This window caches that fact at emit time on `FunctionUnit.has_begin_try`. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, `Value: Copy`, default JIT, interpreter match rewrite, or Stage-3 exit.

## Decision

1. [`FunctionUnit`](../../crates/syx-codegen/src/lib.rs) 新增 `#[serde(default)] pub has_begin_try: bool`（S3-I53 bincode 兼容；旧 cache → `false`）。
2. MIR→bytecode / AST `finish_fn` 等封函路径扫一次 `Instruction::BeginTry` 写入该位（`compute_has_begin_try`）。
3. VM `stack_args_needs_try_frame`：`try_handlers` 非空 → true；否则读 **`func.has_begin_try`**（**去掉**热路径全表 `any`）。
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ interpreter `match` rewrite.

## Evidence

| Check | Evidence |
|-------|----------|
| BeginTry cache | `has_begin_try` · `compute_has_begin_try` · `S3-I64` in codegen + VM |
| Census | `begin_try_cache_helper≥1` · Current=I64 |
| Wall | `fib30_s` 记方向（预期有限下降；**仍远高于 0.5**） |
| Contract | `s3_04_begintry_cache_contract` |

## Consequences

- BeginTry cache hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I65 landed** as [stage3-s3-04-tryfree-dispatch](stage3-s3-04-tryfree-dispatch.md)（try-free 调度热路径）。
- **S3-I66 landed** as [stage3-s3-04-jit-env-gate-cache](stage3-s3-04-jit-env-gate-cache.md)（JIT env 门闩缓存）。
- **S3-I67 landed** as [stage3-s3-04-can-stack-bind-cache](stage3-s3-04-can-stack-bind-cache.md)（can_stack_bind 编译期缓存）。
- **S3-I68 landed** as [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md)（claim-prep）。
- **S3-I69 landed** as [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md)（wall-strategy）。
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-frame-swap](stage3-s3-04-frame-swap.md) · S3-I63.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-frame-swap](stage3-s3-04-frame-swap.md) · S3-I63
- [stage3-s3-04-tryfree-call](stage3-s3-04-tryfree-call.md) · S3-I61
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
