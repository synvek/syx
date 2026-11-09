# ADR: Stage-3 S3-04 LoadLocal 标量 fast-path + StoreLocal move（S3-I59）

**Status:** Accepted (S3-I59) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I58 locals_buf_pool](stage3-s3-04-locals-frame-pool.md), `fib(30)` still ≈**1.65–1.7 s** (target **0.5**). Fib bytecode is LoadLocal-heavy（≈4/帧）with **zero** StoreLocal. This window cuts instruction-level clone tax: scalar **fast-path** for LoadLocal/Dup, and StoreLocal **move** parity with mir-run. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, `Value: Copy`, default JIT, or Stage-3 exit.

## Decision

1. Add `VmInstance::copy_value_for_stack` — scalar/`HeapSlot`/`Simd` field copy；`ObjectRef`/`Str`/`Array`/`Closure`/… **still `.clone()`**.
2. Wire `Instruction::LoadLocal` / `Dup` through `copy_value_for_stack`.
3. `Instruction::StoreLocal`：**move** into slot（`mem::replace(..., v)`）；`maybe_untrack` via pre-move `o != &v`（mir-run parity）.
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` for whole enum.

## Evidence

| Check | Evidence |
|-------|----------|
| Fastpath | `copy_value_for_stack` · `S3-I59` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| StoreLocal | census `store_local_slot_clone=0` |
| Census | `load_local_exec_fastpath≥1` · Current=I59 |
| Wall | `fib30_s` ≈**1.63–1.69**（有限/噪声；**仍远高于 0.5**） |
| Contract | `s3_04_loadlocal_fastpath_contract` |

## Consequences

- LoadLocal/Dup/StoreLocal hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
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
- Prior: [stage3-s3-04-locals-frame-pool](stage3-s3-04-locals-frame-pool.md) · S3-I58.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-locals-frame-pool](stage3-s3-04-locals-frame-pool.md) · S3-I58
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
