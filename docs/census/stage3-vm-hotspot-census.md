# Stage-3 VM-hotspot census (S3-04)

**ADR:** [open](../adr/stage3-s3-04-open.md) · [call-args-move](../adr/stage3-s3-04-call-args-move.md) · [locals-frame-pool](../adr/stage3-s3-04-locals-frame-pool.md) · [loadlocal-fastpath](../adr/stage3-s3-04-loadlocal-fastpath.md) · [call-stack-args](../adr/stage3-s3-04-call-stack-args.md) · [tryfree-call](../adr/stage3-s3-04-tryfree-call.md) · [locals-prep](../adr/stage3-s3-04-locals-prep.md) · [frame-swap](../adr/stage3-s3-04-frame-swap.md) · [begintry-cache](../adr/stage3-s3-04-begintry-cache.md) · [tryfree-dispatch](../adr/stage3-s3-04-tryfree-dispatch.md) · [jit-env-gate-cache](../adr/stage3-s3-04-jit-env-gate-cache.md) · [can-stack-bind-cache](../adr/stage3-s3-04-can-stack-bind-cache.md) · [claim-prep](../adr/stage3-s3-04-claim-prep.md) · [wall-strategy](../adr/stage3-s3-04-wall-strategy.md) · [residual-scope](../adr/stage3-s3-04-residual-scope.md) · [claim](../adr/stage3-s3-04-claim.md) · **Window:** S3-I71 · **Current:** S3-I71 · **Claimed:** scoped `[x]`

**Goal (S3-04 scoped `[x]`):** Hard OUT 下 VM 热点 construct/policy + claim 脚手架齐；JIT-off 主轴诚实落在 **`fib30_interp_ceiling_s ≈ 1.5–1.8`**；`fib_criterion` 绿。  
**Deferred literal:** 历史字面 **`fib(30) < 0.5 s`** = supersede-only（须 superseding ADR）。  
**This census:** S3-I71 scoped claim — S3-04 **`[x]`**；Stage-3 仍 **`[~]`**（**4/7**）；**≠** Stage-3 exit / **≠** I72 本窗施工。

## Command

```bash
bash scripts/s3_04_vm_hotspot_census.sh
```

## Classification (hard)

| Class | Counts as S3-04 debt? | Notes |
|-------|----------------------|-------|
| `fib(30)` wall &lt;0.5 literal | **Deferred** | supersede-only |
| interpret residual ceiling | Hinged (I69–I70) | ≈1.5–1.8 |
| Call / locals / dispatch / cache | Hinged (I57–I67) | |
| examples / criterion | Hinged (I68) | |
| wall-strategy / residual-scope | Hinged (I69–I70) | |
| scoped `[x]` claim | Hinged (I71) | this window |
| S3-05 lint AST/CST | **Next workstream** | I72 open landed · next I73 |
| HostFn / Wave / F-08 / Stage-3 exit | **No（OUT）** | Hard OUT |

## KPIs（S3-I71 · Claim · 2026-11-08）

Machine-local（prefer release `syx`）。**≠** claim &lt;0.5。

| KPI | Value | Notes |
|-----|------:|-------|
| `s3_04_claim_helper` | **≥1** | I71 hinge |
| `residual_scope_helper` | **≥1** | I70 |
| `wall_strategy_helper` | **≥1** | I69 |
| `fib30_interp_ceiling_s` | **≈1.5–1.8** | interpret-only Hard OUT 上限 |
| `claim_prep_helper` | **≥1** | I68 |
| `examples_fib` | **≥1** | `examples/fib*` |
| `fib_criterion_bench` | **≥1** | `benches/*fib*` |
| `can_stack_bind_cache_helper` | **≥1** | I67 |
| `jit_env_gate_cache_helper` | **≥1** | I66 |
| `tryfree_dispatch_helper` | **≥1** | I65 |
| `begin_try_cache_helper` | **≥1** | I64 |
| `locals_frame_swap_helper` | **≥1** | I63 |
| `locals_prep_helper` | **≥1** | I62 |
| `try_free_call_helper` | **≥1** | I61 |
| `call_stack_args_helper` | **≥1** | I60 |
| `load_local_exec_fastpath` | **≥1** | I59 |
| `store_local_slot_clone` | **0** | I59 |
| `locals_frame_reuse_helper` | **≥1** | I58 |
| `arg_bind_clone` | **0** | I57 |
| `owned_args_helper` | **≥1** | I57 |
| `fib30_s` | **≈1.74** | JIT-off；落在 ceiling |
| `fib30_warm_s` | **≈1.56** | |
| `fib30_tier2_s` | **≈2.65** | 次轴；≠ claim |
| `hot_loop_s` | **护栏** | |
| `fib30_target_s` | **0.5** | deferred literal |
| `value_clone_mentions` | **≈380+** | 余 heap/其他路径 |
| `call_path_helper` | **≥1** | |
| `hotspot_count_helper` | **≥1** | 结构护栏 |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S3-04 | **`[x]`** scoped；字面 &lt;0.5 deferred |
| Stage-3 | 仍 **`[~]`**（**4/7**；余 S3-05…07） |
| Next | **S3-I72** S3-05 open **landed**（[lint-ast census](stage3-lint-ast-census.md)）；下一 construct = **S3-I73** — ≠ Stage-3 exit |

## Ladder (locked)

1. **S3-I56:** S3-04 open — landed.
2. **S3-I57:** call-args-move — landed.
3. **S3-I58:** locals-frame-pool — landed.
4. **S3-I59:** loadlocal-fastpath — landed.
5. **S3-I60:** call-stack-args — landed.
6. **S3-I61:** tryfree-call — landed.
7. **S3-I62:** locals-prep — landed.
8. **S3-I63:** frame-swap — landed.
9. **S3-I64:** begintry-cache — landed.
10. **S3-I65:** tryfree-dispatch — landed.
11. **S3-I66:** jit-env-gate-cache — landed.
12. **S3-I67:** can-stack-bind-cache — landed.
13. **S3-I68:** claim-prep — landed.
14. **S3-I69:** wall-strategy — landed.
15. **S3-I70:** residual-scope — landed.
16. **S3-I71:** S3-04 scoped claim — landed.
17. **S3-I72:** S3-05 open — landed（[lint-ast census](stage3-lint-ast-census.md)）。
18. **S3-I73:** S3-05 construct — next（≠ Stage-3 exit）。

## Claim criteria（met · scoped）

- I57–I70 hinges + helpers hold
- `examples_fib≥1` · `fib_criterion_bench≥1`
- JIT-off `fib30_s` within `fib30_interp_ceiling_s ≈ 1.5–1.8`
- Dedicated claim window = **this window**
- Literal &lt;0.5 **deferred** — ≠ scoped claim requirement

## Non-claims

- **≠** Stage-3 exit  
- **≠** claim `fib(30)` &lt; 0.5 / Node parity  
- **≠** HostFn+1 / F-08 / Wave  
- **≠** S3-05…07 本窗开工  
- **≠** `Value: Copy` / 生产默认 JIT / micro-knife  

## References

- [stage3-s3-04-claim](../adr/stage3-s3-04-claim.md) · S3-I71
- [stage3-s3-04-residual-scope](../adr/stage3-s3-04-residual-scope.md) · S3-I70
- [plan/02](../../plan/02-improvement-roadmap.md) S3-04
