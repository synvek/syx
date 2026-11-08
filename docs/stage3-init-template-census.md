# Stage-3 init-template census (S3-07)

**ADR:** [open](adr/stage3-s3-07-open.md) · [default-vm](adr/stage3-s3-07-default-vm.md) · [residual-scope](adr/stage3-s3-07-residual-scope.md) · [**claim**](adr/stage3-s3-07-claim.md) · **Window:** S3-I85 · **Current:** S3-I85 · **Claimed:** scoped `[x]`

**Goal (S3-07 scoped `[x]`):** Primary = I83 default-vm hinge + sample init→run nail；UsageGuide / Init crate / CLI 子进程 / `_` hard-error = **deferred supersede-only**.  
**This census:** S3-I85 scoped claim — S3-07 **`[x]`**；Stage-3 **`[x]` 退出**（**7/7** · **S3-I87**；见 [exit census](stage3-exit-census.md)）。

## Command

```bash
bash scripts/s3_07_init_template_census.sh
```

## Classification (hard)

| Class | Counts as S3-07 debt? | Notes |
|-------|----------------------|-------|
| Default template named `native` | **Claimed** | default = `vm` |
| Explicit `vm` template arm | **Claimed** | named match |
| init→run integration nail | **Claimed** | construct contract |
| Residual-scope / claim scaffolding | **Claimed** | I84–I85 |
| UsageGuide rewrite | **Deferred** | supersede-only |
| Init crate extract | **Deferred** | supersede-only |
| CLI subprocess init→run | **Deferred** | driver nail OK |
| `_` → hard error | **Deferred** | supersede-only |
| Stage-3 exit | **No（OUT）** / **Next** | I86 exit open |

## KPIs（S3-I85 · Claim · 2026-11-08）

| KPI | Value | Notes |
|-----|------:|-------|
| `s3_07_claim_helper` | **≥1** | I85 hinge |
| `residual_scope_helper` | **≥1** | I84 |
| `stage3_init_census_helper` | **≥1** | ADR / census anchors |
| `init_commands_helper` | **≥1** | `Commands::Init` |
| `init_default_template_native` | **0** | Primary claimed |
| `init_default_template_vm` | **≥1** | Primary claimed |
| `init_vm_named_template_count` | **≥1** | Primary claimed |
| `init_integration_test_count` | **≥1** | Primary claimed |
| `init_inline_match_arm_helper` | **≥1** | inventory |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S3-07 | **`[x]`** scoped；UsageGuide/crate deferred |
| Stage-3 | **`[x]` 退出**（S3-I87） |
| Next | **S3-I86…I87** exit ladder **landed**；默认 = **Stage-4 open**（S3-I88） |

## Ladder (locked)

1. **S3-I82:** S3-07 open — landed.
2. **S3-I83:** default-vm construct — landed.
3. **S3-I84:** residual-scope — landed.
4. **S3-I85:** S3-07 scoped claim — **claimed**（this census Current）。
5. **S3-I86:** Stage-3 exit open — **landed**（[exit census](stage3-exit-census.md)）。
6. **S3-I87:** write `stage3-exit` — **landed**.
7. **S3-I88:** Stage-4 open — next（naming only）。

## Non-claims

- **≠** Stage-3 exit  
- **≠** write `stage3-exit` this window  
- **≠** UsageGuide 大改 / 抽 Init crate  
- **≠** HostFn+1 / F-08 / Wave  
- **≠** reopen S3-06 LSP / S3-05 lint / S3-04 fib  

## References

- [stage3-s3-07-claim](adr/stage3-s3-07-claim.md) · S3-I85
- [stage3-s3-07-residual-scope](adr/stage3-s3-07-residual-scope.md) · S3-I84
- [plan/02](../plan/02-improvement-roadmap.md) S3-07
