# Stage-4 exit census

**ADR:** [exit-open](adr/stage4-exit-open.md) · [**exit**](adr/stage4-exit.md) · **Window:** S3-I109 · **Current:** S3-I109 · **exit_status:** exited  
**Prior claim chain:** [S4-01](adr/stage4-s4-01-claim.md)…[S4-05](adr/stage4-s4-05-claim.md) · open [S3-I88](adr/stage4-open.md) · exit-open [S3-I108](adr/stage4-exit-open.md)

**Goal (exit):** Stage-4 **`[x]` 退出** with Evidence In = S4-01…05 scoped claims + I108 scaffolding；deferred / parked = **post-exit Residual**.  
**This census:** S3-I109 Stage-4 exit — Stage-4 **`[x]` 退出** · checklist **5/5**；下一 = **post-exit Residual / 维护**（≠ Stage-5）。

## Command

```bash
bash scripts/s4_exit_census.sh
```

## Evidence In（Primary claimed）

| Axis | Claim window | Notes |
|------|--------------|-------|
| S4-01 stdlib stability | S3-I91 | SSOT + residual；manifest/doctest deferred |
| S4-02 package registry | S3-I95 | file-registry hinge；公网 registry deferred |
| S4-03 structured concurrency | S3-I99 | cancel/TaskGroup hinge；Sendable/withTaskGroup deferred |
| S4-04 Sequence/Collection | S3-I103 | Iterator for-in hinge；makeIterator/map·filter deferred |
| S4-05 prebuilt release | S3-I107 | layout/full-native Primary；doctor CLI / sysroot deferred |
| Exit-open | S3-I108 | census + open ADR |
| Exit | S3-I109 | [stage4-exit](adr/stage4-exit.md) |

## Classification（exit honesty）

| Class | Blocks Stage-4 exit? | Notes |
|-------|----------------------|-------|
| S4-01…05 scoped `[x]` | **No — Evidence In** | Required；landed |
| Deferred supersede-only（per claim） | **No — Residual** | Do not restore as exit gate |
| Stage-0…3 post-exit residual | **No — Residual** | See prior exit ADRs |
| HostFn / F-08 / Wave park | **No — Parked** | Explicit unpark ADR only |
| Stage-5 construction | **OUT** | No default Stage-5 |

## Post-exit Residual（canonical）

| Source | Residual |
|--------|----------|
| S4-01 | manifest schema / doctest CLI productize |
| S4-02 | 公网 `packages.sy.dev` GA / lock trim / CLI UX rewrite |
| S4-03 | Sendable deepen / `withTaskGroup` GA / TSAN·loom |
| S4-04 | `Sequence.makeIterator` GA / 用户 map·filter / Collection Index |
| S4-05 | `syx doctor --native-sdk` CLI wire / release smoke / official sysroot / marketplace / 全 profile CI |
| Parked | HostFn+1 / F-08 / Wave / product umbrella |

## KPIs（S3-I109 · Exit · 2026-11-09）

| KPI | Value | Notes |
|-----|------:|-------|
| `stage4_exit_helper` | **≥1** | I109 hinge |
| `stage4_exit_open_helper` | **≥1** | I108 prior |
| `stage4_claim_chain_helper` | **≥5** | S4-01…05 claim ADR anchors |
| `stage4_checklist` | **5/5** | plan rows |
| `stage4_status` | **x** | exited |
| `exit_status` | **exited** | stage4-exit landed |
| `stage4_exit_adr_present` | **1** | `docs/adr/stage4-exit.md` |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| Checklist | **5/5** S4-01…05 `[x]` |
| Stage-4 | **`[x]` 退出**（S3-I109） |
| Exit | **exited** |
| Next | **post-exit Residual / 维护** |

## Ladder (locked)

1. **S3-I107:** S4-05 scoped claim — landed.
2. **S3-I108:** Stage-4 exit open — landed.
3. **S3-I109:** write `stage4-exit` — **this window**.
4. **Next:** post-exit Residual / 维护（≠ Stage-5）。

## Non-claims

- ≠ Stage-5 · ≠ deferred as exit gate · ≠ HostFn/F-08/Wave unpark  
- ≠ wire `doctor --native-sdk` · ≠ S4-01…05 product knives  

## References

- [stage4-exit](adr/stage4-exit.md) · S3-I109
- [stage4-exit-open](adr/stage4-exit-open.md) · S3-I108
- [stage4-s4-05-claim](adr/stage4-s4-05-claim.md) · S3-I107
- [stage3-exit-census](stage3-exit-census.md) · S3-I86/I87 形态
- [plan/02](../plan/02-improvement-roadmap.md) Stage-4
