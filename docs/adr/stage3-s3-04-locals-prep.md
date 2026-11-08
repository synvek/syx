# ADR: Stage-3 S3-04 locals 绑参优先（去全槽 resize Nil）（S3-I62）

**Status:** Accepted (S3-I62) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I61 try-free Call](stage3-s3-04-tryfree-call.md), `fib(30)` still ≈**1.71 s** (target **0.5**). Stack-args still did `locals.resize(local_count, Nil)` then immediately overwrote `0..arg_count`. This window binds stack args first and Nil-pads only the remaining slots. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, `Value: Copy`, default JIT, `locals_frames`/`mem::take` rewrite, or Stage-3 exit.

## Decision

1. Add `VmInstance::bind_stack_args_into_locals(arg_count, local_count)` — drain stack args into pooled `locals`, then `resize` **only** to pad remaining Nil slots (or `truncate` if oversize).
2. Wire `run_function_with_stack_args` through that helper（replace pre-bind full-slot `resize`）。
3. Owned-args slow path may keep its own resize（本窗不以 owned 为主验收）。
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ interpreter `match` rewrite / ≠ `locals_frames`/`mem::take` rewrite this window.

## Evidence

| Check | Evidence |
|-------|----------|
| Locals prep | `bind_stack_args_into_locals` · `S3-I62` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| Census | `locals_prep_helper≥1` · Current=I62 |
| Wall | `fib30_s` 记方向（预期有限下降；**仍远高于 0.5**） |
| Contract | `s3_04_locals_prep_contract` |

## Consequences

- Locals prep hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
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
- Prior: [stage3-s3-04-tryfree-call](stage3-s3-04-tryfree-call.md) · S3-I61.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-tryfree-call](stage3-s3-04-tryfree-call.md) · S3-I61
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
