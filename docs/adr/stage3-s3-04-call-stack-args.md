# ADR: Stage-3 S3-04 Call 栈参直绑（去中间 Vec）（S3-I60）

**Status:** Accepted (S3-I60) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I59 LoadLocal fastpath](stage3-s3-04-loadlocal-fastpath.md), `fib(30)` still ≈**1.63–1.78 s** (target **0.5**). Instruction-level clone hinges (I57–I59) no longer move the wall clock. Fib Direct `Call` still allocated a per-call `Vec` via `stack.drain(..).collect()` before owned bind. This window removes that intermediate alloc on the free-function hot path. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, `Value: Copy`, default JIT, or Stage-3 exit.

## Decision

1. Add `VmInstance::run_function_with_stack_args` — after frame install + `locals.resize`, **drain stack args straight into `locals`** (iterator move；**no intermediate `Vec`**).
2. Gate Direct `Instruction::Call` with `call_can_stack_bind`（自由函数；**≠** mangled instance / `_deinit`）→ stack-args；Lazy / virtual / ctor 慢路径仍 `drain→Vec` + `run_function_with_owned_args`.
3. Factor shared interpret/unwind into `run_function_body`.
4. JIT opt-in / suspend resume：仍退回 owned-args（需 `&[Value]` 或快照）。
5. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ interpreter `match` rewrite.

## Evidence

| Check | Evidence |
|-------|----------|
| Stack-args | `run_function_with_stack_args` · `call_can_stack_bind` · `S3-I60` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| Census | `call_stack_args_helper≥1` · Current=I60 |
| Wall | `fib30_s` 记方向（预期有限下降；**仍远高于 0.5**） |
| Contract | `s3_04_call_stack_args_contract` |

## Consequences

- Call free-function hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
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
- Prior: [stage3-s3-04-loadlocal-fastpath](stage3-s3-04-loadlocal-fastpath.md) · S3-I59.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-loadlocal-fastpath](stage3-s3-04-loadlocal-fastpath.md) · S3-I59
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
