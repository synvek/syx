# ADR: Stage-3 S3-04 locals 帧缓冲复用（S3-I58）

**Status:** Accepted (S3-I58) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I57 owned-args move](stage3-s3-04-call-args-move.md), Call still paid **empty-`Vec` + `resize` + drop callee buffer** on every nest (`mem::take` → capacity-0 `locals`). This window lands **`locals_buf_pool`** so callee buffers are cleared and reused. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, LoadLocal/StoreLocal de-clone, suspend-frame clone elimination, or Stage-3 exit.

## Decision

1. Add `VmInstance::locals_buf_pool: Vec<Vec<Value>>`（cleared buffers；**not** GC roots）.
2. Helpers: `checkout_locals_buf` / `recycle_locals_buf`（cap **64**）/ `push_caller_locals_frame` / `recycle_and_restore_locals`.
3. Wire `run_function_with_owned_args` / `run_closure_with_captures` / `run_init_on_existing_self`（init 成功：outer 入 pool，当前 init `locals` 保留）.
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ suspend locals/stack clone cut / ≠ LoadLocal·StoreLocal 大规模去 clone / ≠ slab ABI.

## Evidence

| Check | Evidence |
|-------|----------|
| Pool + helpers | `locals_buf_pool` · `push_caller_locals_frame` · `S3-I58` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| Census | `locals_frame_reuse_helper≥1` · Current=I58 |
| Wall | `fib30_s` ≈**1.65–1.7**（方向/噪声；**≠** &lt;0.5） |
| Contract | `s3_04_locals_frame_pool_contract` |

## Consequences

- Frame-buffer reuse hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
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
- Prior: [stage3-s3-04-call-args-move](stage3-s3-04-call-args-move.md) · S3-I57.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-call-args-move](stage3-s3-04-call-args-move.md) · S3-I57
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
