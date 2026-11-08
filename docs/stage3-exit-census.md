# Stage-3 exit census

**ADR:** [exit-open](adr/stage3-exit-open.md) · [**exit**](adr/stage3-exit.md) · **Window:** S3-I87 · **Current:** S3-I87 · **exit_status:** exited  
**Prior claim chain:** [S3-01](adr/stage3-s3-01-claim.md)…[S3-07](adr/stage3-s3-07-claim.md) · open [S3-I86](adr/stage3-exit-open.md)

**Goal (exit):** Stage-3 **`[x]` 退出** with Evidence In = S3-01…07 scoped claims + I86 scaffolding；deferred / parked = **post-exit Residual**.  
**This census:** S3-I87 Stage-3 exit — Stage-3 **`[x]` 退出**；下一 = **Stage-4 open**（S3-I88 命名；≠ 本窗施工）。

## Command

```bash
bash scripts/s3_exit_census.sh
```

## Evidence In（Primary claimed）

| Axis | Claim window | Notes |
|------|--------------|-------|
| S3-01 Span | S3-I41 | face gates + census |
| S3-02 mono display | S3-I47 | demangle / runtime / meta |
| S3-03 stdlib cache | S3-I55 | prelude/warm/cache |
| S3-04 VM hotspot | S3-I71 | Hard OUT ceiling；fib&lt;0.5 deferred |
| S3-05 lint AST | S3-I77 | L007/L012=0；hybrid deferred |
| S3-06 LSP book | S3-I81 | S045 + sample fixtures；rename/fence deferred |
| S3-07 init VM default | S3-I85 | default-vm hinge；UsageGuide/crate deferred |
| Exit-open | S3-I86 | census + open ADR |
| Exit | S3-I87 | [stage3-exit](adr/stage3-exit.md) |

## Classification（exit honesty）

| Class | Blocks Stage-3 exit? | Notes |
|-------|----------------------|-------|
| S3-01…07 scoped `[x]` | **No — Evidence In** | Required；landed |
| Deferred supersede-only（per claim） | **No — Residual** | Do not restore as exit gate |
| Stage-1/2 post-exit residual | **No — Residual** | See stage1/2-exit |
| HostFn / F-08 / Wave park | **No — Parked** | Explicit unpark ADR only |
| Stage-4 open construction | **OUT this window** | Next naming only |

## Post-exit Residual（canonical；from I86 draft）

| Source | Residual |
|--------|----------|
| S3-04 | 字面 `fib(30) < 0.5` |
| S3-05 | L010–L012 text-hybrid / full CST lint / S2-04 Frontend rewrite |
| S3-06 | rename / signature / hover book / full fence LSP sweep |
| S3-07 | UsageGuide `--template` 叙事 / Init 抽 crate / CLI 子进程 init→run / `_`→hard error |
| Stage-1/2 | examples 拉满 / Frontend CST migrate / docs 117.0 / fingerprint salts |
| Parked | HostFn+1 / F-08 / Wave / product umbrella |

## KPIs（S3-I87 · Exit · 2026-11-09）

| KPI | Value | Notes |
|-----|------:|-------|
| `stage3_exit_helper` | **≥1** | I87 hinge |
| `stage3_exit_open_helper` | **≥1** | I86 prior |
| `stage3_claim_chain_helper` | **≥7** | S3-01…07 claim ADR anchors |
| `stage3_checklist` | **7/7** | plan rows |
| `stage3_status` | **x** | exited |
| `exit_status` | **exited** | stage3-exit landed |
| `stage3_exit_adr_present` | **1** | `docs/adr/stage3-exit.md` |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| Checklist | **7/7** S3-01…07 `[x]` |
| Stage-3 | **`[x]` 退出**（S3-I87） |
| Exit | **exited** |
| Next | **Stage-4 open**（S3-I88）— ≠ Stage-4 construction this window |

## Ladder (locked)

1. **S3-I85:** S3-07 scoped claim — landed.
2. **S3-I86:** Stage-3 exit open — landed.
3. **S3-I87:** write `stage3-exit` — **this window**.
4. **S3-I88:** Stage-4 open — next（naming only）。

## Non-claims

- **≠** Stage-4 construction this window  
- **≠** restore deferred as post-exit hard gate  
- **≠** HostFn / F-08 / Wave unpark  
- **≠** reopen S3-04…07 product knives  

## References

- [stage3-exit](adr/stage3-exit.md) · S3-I87
- [stage3-exit-open](adr/stage3-exit-open.md) · S3-I86
- [stage3-s3-07-claim](adr/stage3-s3-07-claim.md) · S3-I85
- [stage2-exit](adr/stage2-exit.md) · S2-I266
- [plan/02](../plan/02-improvement-roadmap.md) Stage-3
