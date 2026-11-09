# ADR: Stage-3 S3-04 open — VM 热点 / `fib(30)`（S3-I56）

**Status:** Accepted (S3-I56) · S3-04 `[~]` · Stage-3 `[~]`  
**Date:** 2026-11-08  
**Context:** After [S3-I55 S3-03 claim](stage3-s3-03-claim.md), Stage-3 default work shifts to [plan/02 S3-04](../../plan/02-improvement-roadmap.md): VM hotspot optimization so **`fib(30)` &lt; 0.5 s** with criterion regression. Honest inventory shows **no** Stage-3 VM-hotspot census, **no** checked-in `examples/fib*`, **no** fib criterion bench; [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) historically recorded **`fib(30)` ≈ 5.4 s**. This window **opens** the workstream with a VM-hotspot / fib **census baseline** — it does **not** claim S3-04 `[x]`, ship VM/JIT product cuts, or exit Stage-3.

## Gap: fib wall time vs plan S3-04

| Claim | Today (honest) | plan/02 S3-04 target |
|-------|----------------|----------------------|
| `fib(30)` wall time | Census baseline (see census) · analysis historically **5.4 s** | **&lt; 0.5 s** |
| Criterion regression | Host I/O / compile benches only · **`fib_criterion_bench=0`** | fib (or equiv.) criterion gate |
| Call / `Value` tax | Widespread `.clone()` on call/arg paths; analysis: 传参 / `Value` 克隆 / 帧 | Cut call-path clone/frame tax |
| JIT | Experimental `SYX_JIT_*` / `hotspot_count` · **not** production default | Optional later; **≠** this open |

Baseline: [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md).

## Decision

1. **Open S3-04** as an active Stage-3 workstream; flip plan S3-04 `[ ]` → `[~]`.
2. **S3-I56 first slice** — publish VM-hotspot census (+ script); add a thin contract under `syx-driver/tests/` (**no** product VM/JIT edit this window).
3. **Ladder:** **S3-I57** = first construct knife (priority: **call-path `Value` clone / frame tax** per analysis) → residual cuts → dedicated **S3-04 `[x]` claim** (measured `fib(30)` &lt; 0.5 s + criterion).
4. **Non-claims (hard):** ≠ S3-04 `[x]` / ≠ claim `fib(30)` &lt; 0.5 s this window / ≠ Stage-3 exit / ≠ HostFn+1 / ≠ Wave·F-08 / ≠ open S3-05…07 construction / ≠ change language semantics / ≠ expand HostFn surface / ≠ production-default JIT.

## Evidence (this window)

| Check | Evidence |
|-------|----------|
| VM-hotspot census | [`docs/census/stage3-vm-hotspot-census.md`](../census/stage3-vm-hotspot-census.md) · `scripts/s3_04_vm_hotspot_census.sh` |
| Contract nail | `s3_04_vm_hotspot_census_contract` |
| Plan status | S3-04 `[~]` under this ADR · Stage-3 `[~]` · S3-01…03 `[x]` |

## Consequences

- S3-04 moves `[ ]` → `[~]`; Stage-3 stays `[~]`; S3-01…03 stay `[x]`.
- **S3-I57 landed** as [stage3-s3-04-call-args-move](stage3-s3-04-call-args-move.md)（owned-args move-bind）。
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
- Prior: [stage3-s3-03-claim](stage3-s3-03-claim.md) · S3-I55.
- Census Current → **S3-I71** · Claimed scoped `[x]`.

## References

- [stage3-open](stage3-open.md) · S3-I1
- [stage3-s3-03-claim](stage3-s3-03-claim.md) · S3-I55
- [stage3-vm-hotspot-census](../census/stage3-vm-hotspot-census.md)
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
- [ProjectAnalysis](../../plan/ProjectAnalysisAndImprovementPlan.zh-CN.md) fib KPI
