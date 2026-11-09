# ADR: Stage-3 S3-04 try-free 调度热路径（S3-I65）

**Status:** Accepted (S3-I65) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I64 BeginTry cache](stage3-s3-04-begintry-cache.md), `fib(30)` still ≈**1.7 s** (target **0.5**). Call/locals 表层税已尽；每指令仍付 catch/throw 轮询 + 巨型 `exec_instruction` match。This window adds a **try-free interpret path** with no per-insn catch/throw polls and a closed hot-opcode early match. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, `Value: Copy`, default JIT, full ISA match rewrite, or Stage-3 exit.

## Decision

1. `run_function_body`：当 `track_try_frame == false` → **`run_function_body_tryfree`**；否则保持原循环。
2. `run_function_body_tryfree`：**去掉**每指令 `pending_catch` / `propagating_throw` 轮询。
3. 热 opcode 闭集早匹配：`ConstI32` · `LoadLocal` · `StoreLocal` · `I32*` 算术/比较 · `Jump` · `JumpIfFalse` · `Dup` · `Pop` · `Return` · `Call`（仅 `call_can_stack_bind` → `run_function_with_stack_args` + `finish_call`）。
4. 其余 opcode → 现有 `exec_instruction`；冷路径后仍检查 `suspended` / `propagating_throw`。
5. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ production-default JIT / ≠ `Value: Copy` / ≠ 全量 `exec_instruction` 重写 / ≠ `can_stack_bind` 编译期位 / ≠ criterion/`examples/fib` 本窗。

## Evidence

| Check | Evidence |
|-------|----------|
| Try-free dispatch | `run_function_body_tryfree` · `S3-I65` in [`syx-vm/src/lib.rs`](../../crates/syx-vm/src/lib.rs) |
| Census | `tryfree_dispatch_helper≥1` · Current=I65 |
| Wall | `fib30_s` 记方向（预期可见下降；**仍可能远高于 0.5**） |
| Contract | `s3_04_tryfree_dispatch_contract` |

## Consequences

- Try-free dispatch hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I66 landed** as [stage3-s3-04-jit-env-gate-cache](stage3-s3-04-jit-env-gate-cache.md)（JIT env 门闩缓存）。
- **S3-I67 landed** as [stage3-s3-04-can-stack-bind-cache](stage3-s3-04-can-stack-bind-cache.md)（can_stack_bind 编译期缓存）。
- **S3-I68 landed** as [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md)（claim-prep）。
- **S3-I69 landed** as [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md)（wall-strategy）。
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-begintry-cache](stage3-s3-04-begintry-cache.md) · S3-I64.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-begintry-cache](stage3-s3-04-begintry-cache.md) · S3-I64
- [stage3-s3-04-tryfree-call](stage3-s3-04-tryfree-call.md) · S3-I61
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
