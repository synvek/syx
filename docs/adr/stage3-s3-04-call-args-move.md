# ADR: Stage-3 S3-04 call-boundary owned-args move（S3-I57）

**Status:** Accepted (S3-I57) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I56 S3-04 open](stage3-s3-04-open.md), census shows `fib30_s`≈**1.7** (target **0.5**) and Call paths `stack.drain` into a `Vec<Value>` then re-bind via `arg.clone()` into `locals`. This window lands the **first construct hinge**: owned-args move-bind. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, frame-buffer reuse, or Stage-3 exit.

## Decision

1. Add `WasmVM::run_function_with_owned_args(func, program, args: Vec<Value>)` that **move**-binds into `locals[i]` after `resize`（no per-arg bind clone）.
2. Wire `Instruction::Call` / `CallSuper` / `CallIndirect` / `CallWitness` / `CallClosure` / Actor·Task drain paths（and `run_init_on_existing_self`）to the owned API.
3. Keep `run_function_with_args(&[Value])` as a thin `to_vec()` wrapper for slice / JIT-adjacent one-off callers.
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ locals frame-pool this window.

## Evidence

| Check | Evidence |
|-------|----------|
| Owned API | `run_function_with_owned_args` · `S3-I57` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| Bind clone | census `arg_bind_clone=0` |
| Wall | census `fib30_s`（direction only；**≠** &lt;0.5 claim） |
| Contract | `s3_04_call_args_move_contract` |
| Census | Current=I57 |

## Consequences

- Call-boundary bind clone hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I58 landed** as [stage3-s3-04-locals-frame-pool](stage3-s3-04-locals-frame-pool.md)（`locals_buf_pool`）。
- **S3-I59 landed** as [stage3-s3-04-loadlocal-fastpath](stage3-s3-04-loadlocal-fastpath.md)（LoadLocal fastpath + StoreLocal move）。
- **S3-I60 landed** as [stage3-s3-04-call-stack-args](stage3-s3-04-call-stack-args.md)（Call 栈参直绑）。
- **S3-I61 landed** as [stage3-s3-04-tryfree-call](stage3-s3-04-tryfree-call.md)（try-free Call）。
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
- Prior: [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
