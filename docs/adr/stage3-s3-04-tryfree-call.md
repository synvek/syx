# ADR: Stage-3 S3-04 try-free Call（跳过 try 帧记账）（S3-I61）

**Status:** Accepted (S3-I61) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I60 Call stack-args](stage3-s3-04-call-stack-args.md), `fib(30)` still ≈**1.6–2.0 s** (target **0.5**). Fib Direct Call still paid `try_handler_frames.push` / `pop_try_handler_frame` on every recursion despite **no `BeginTry`**. This window skips that bookkeeping on the stack-args hot path when no try boundary is needed. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, `Value: Copy`, default JIT, interpreter match rewrite, or Stage-3 exit.

## Decision

1. On `run_function_with_stack_args`：仅当 `stack_args_needs_try_frame`（活跃 `try_handlers` **或** 函数体含 `BeginTry`）时 `try_handler_frames.push`。
2. `run_function_body(..., track_try_frame)`：对称地仅在 push 过时 `pop_try_handler_frame`。
3. `run_function_with_owned_args` 慢路径仍 **always** track（本窗不扩面）。
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ interpreter `match` rewrite / ≠ `locals_frames`/`mem::take` rewrite this window.

## Evidence

| Check | Evidence |
|-------|----------|
| Try-free | `stack_args_needs_try_frame` · `track_try_frame` · `S3-I61` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| Census | `try_free_call_helper≥1` · Current=I61 |
| Wall | `fib30_s` 记方向（预期有限下降；**仍远高于 0.5**） |
| Contract | `s3_04_tryfree_call_contract` |

## Consequences

- Try-free stack-args hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I62 landed** as [stage3-s3-04-locals-prep](stage3-s3-04-locals-prep.md)（locals 绑参优先）。
- **S3-I63 landed** as [stage3-s3-04-frame-swap](stage3-s3-04-frame-swap.md)（locals 帧 swap）。
- **S3-I64 landed** as [stage3-s3-04-begintry-cache](stage3-s3-04-begintry-cache.md)（BeginTry 编译期缓存）。
- **S3-I65 landed** as [stage3-s3-04-tryfree-dispatch](stage3-s3-04-tryfree-dispatch.md)（try-free 调度热路径）。
- **S3-I66 landed** as [stage3-s3-04-jit-env-gate-cache](stage3-s3-04-jit-env-gate-cache.md)（JIT env 门闩缓存）。
- **S3-I67 landed** as [stage3-s3-04-can-stack-bind-cache](stage3-s3-04-can-stack-bind-cache.md)（can_stack_bind 编译期缓存）。
- **S3-I68 landed** as [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md)（claim-prep）。
- **S3-I69 landed** as [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md)（wall-strategy）。
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-call-stack-args](stage3-s3-04-call-stack-args.md) · S3-I60.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-call-stack-args](stage3-s3-04-call-stack-args.md) · S3-I60
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
