# ADR: Stage-3 S3-04 locals 帧 swap 切换（S3-I63）

**Status:** Accepted (S3-I63) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I62 locals prep](stage3-s3-04-locals-prep.md), `fib(30)` still ≈**1.77 s** (target **0.5**). Frame switch still used `mem::take` (empty intermediate `locals`) around pool checkout/recycle. This window switches to **checkout + `mem::swap`** while keeping caller Vecs in `locals_frames` for GC. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, remove `locals_frames`, `Value: Copy`, default JIT, or Stage-3 exit.

## Decision

1. `push_caller_locals_frame`：`checkout` → `mem::swap(&mut locals, &mut callee)` → `locals_frames.push`（slot = caller）。
2. `recycle_and_restore_locals`：`pop` caller → `mem::swap` → `recycle_locals_buf`（vacated = callee）。
3. GC / `collect_live_roots` 仍遍历 `locals_frames` — **≠** 拆除帧根。
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ interpreter `match` rewrite / ≠ remove `locals_frames` GC roots.

## Evidence

| Check | Evidence |
|-------|----------|
| Frame swap | `mem::swap` in push/recycle · `S3-I63` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| Census | `locals_frame_swap_helper≥1` · Current=I63 |
| Wall | `fib30_s` 记方向（预期有限下降；**仍远高于 0.5**） |
| Contract | `s3_04_frame_swap_contract` |

## Consequences

- Frame-swap hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I64 landed** as [stage3-s3-04-begintry-cache](stage3-s3-04-begintry-cache.md)（BeginTry 编译期缓存）。
- **S3-I65 landed** as [stage3-s3-04-tryfree-dispatch](stage3-s3-04-tryfree-dispatch.md)（try-free 调度热路径）。
- **S3-I66 landed** as [stage3-s3-04-jit-env-gate-cache](stage3-s3-04-jit-env-gate-cache.md)（JIT env 门闩缓存）。
- **S3-I67 landed** as [stage3-s3-04-can-stack-bind-cache](stage3-s3-04-can-stack-bind-cache.md)（can_stack_bind 编译期缓存）。
- **S3-I68 landed** as [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md)（claim-prep）。
- **S3-I69 landed** as [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md)（wall-strategy）。
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-locals-prep](stage3-s3-04-locals-prep.md) · S3-I62.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-locals-prep](stage3-s3-04-locals-prep.md) · S3-I62
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
