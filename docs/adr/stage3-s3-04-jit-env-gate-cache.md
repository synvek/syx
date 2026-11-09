# ADR: Stage-3 S3-04 JIT env 门闩缓存（S3-I66）

**Status:** Accepted (S3-I66) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I65 try-free dispatch](stage3-s3-04-tryfree-dispatch.md), `fib(30)` still ≈**1.7+ s** (target **0.5**). Every recursive Call still paid **`fastpath_enabled` + `tier2_opt_in_enabled` → 2× `std::env::var`** even when JIT stays default-off. This window caches those gates process-locally. It does **not** claim S3-04 `[x]`, `fib(30)` &lt; 0.5, enable production-default JIT, `Value: Copy`, or Stage-3 exit.

## Decision

1. [`jit.rs`](../../crates/syx-vm/src/jit.rs)：`SYX_JIT_FASTPATH` / `SYX_JIT_TIER2` 首次查询后写入 **`AtomicU8` 三态**（unset / false / true）；热路径命中缓存后不再 `env::var`。
2. `#[cfg(test)] reset_jit_env_gates_for_test()`：会改 env 的单测在 `set_var`/`remove_var` 后清缓存再读。
3. VM Call 门闩语义不变（默认 **off**）；**≠** 生产默认 JIT。
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ F-08 / ≠ Wave / ≠ S3-05…07 / ≠ **production-default JIT** / ≠ `Value: Copy` / ≠ 全量 match 重写 / ≠ `can_stack_bind` 位本窗。

## Evidence

| Check | Evidence |
|-------|----------|
| Gate cache | `JIT_FASTPATH_GATE` / `JIT_TIER2_GATE` · `load_or_init_gate` · `S3-I66` in jit.rs |
| Census | `jit_env_gate_cache_helper≥1` · Current=I66 |
| Wall | `fib30_s` 记方向（预期可见下降；**仍远高于 0.5**；prefer release） |
| Contract | `s3_04_jit_env_gate_cache_contract` |

## Consequences

- JIT env gate cache hinge landed；S3-04 stays `[~]`；Stage-3 stays `[~]`.
- **S3-I67 landed** as [stage3-s3-04-can-stack-bind-cache](stage3-s3-04-can-stack-bind-cache.md)（can_stack_bind 编译期缓存）。
- **S3-I68 landed** as [stage3-s3-04-claim-prep](stage3-s3-04-claim-prep.md)（claim-prep）。
- **S3-I69 landed** as [stage3-s3-04-wall-strategy](stage3-s3-04-wall-strategy.md)（wall-strategy）。
- **S3-I70 landed** as [stage3-s3-04-residual-scope](stage3-s3-04-residual-scope.md)（residual-scope）。
- **S3-I71 landed** as [stage3-s3-04-claim](stage3-s3-04-claim.md)（S3-04 `[x]` scoped claim）。
- Default next = **S3-I72**（S3-05 open）— not Stage-3 exit.
- Prior: [stage3-s3-04-tryfree-dispatch](stage3-s3-04-tryfree-dispatch.md) · S3-I65.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-s3-04-tryfree-dispatch](stage3-s3-04-tryfree-dispatch.md) · S3-I65
- [stage3-s3-04-open](stage3-s3-04-open.md) · S3-I56
- [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
