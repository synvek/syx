# Stage-4 stdlib stability / doctest census (S4-01)

**ADR:** [open](adr/stage4-open.md) · [stability-ssot](adr/stage4-s4-01-stability-ssot.md) · [residual-scope](adr/stage4-s4-01-residual-scope.md) · [**claim**](adr/stage4-s4-01-claim.md) · **Window:** S3-I91 · **Current:** S3-I91 · **Claimed:** scoped `[x]`

**Goal (S4-01 scoped `[x]`):** Primary = I89 `stability.toml` plan ternary（`stable` | `preview` | `experimental`）+ I90 scoped acceptance；manifest wiring / `syx doctest` / packages doctest gate = **deferred supersede-only** — **≠** Stage-4 exit / **≠** schema / CLI product knife.  
**This census:** S3-I91 scoped claim — Stage-4 **`[~]`** · S4-01 **`[x]`** · checklist **1/5**；下一 = **S3-I92** S4-02 open.

## Command

```bash
bash scripts/s4_01_stdlib_stability_doctest_census.sh
```

## Classification (hard)

| Class | Counts as S4-01 debt? | Notes |
|-------|----------------------|-------|
| README 五档 ≠ plan 三元 | **Mapped** | v0 + SSOT；README 仍人类叙事 |
| Manifest 无 `stability` 字段 | **Deferred / supersede-only** | keep 0；≠ claim blocker |
| 无 `syx doctest` CLI | **Deferred / supersede-only** | keep 0；≠ claim blocker |
| packages 无 doctest 闸 | **Deferred / supersede-only** | ≠ claim blocker |
| `stability.toml` SSOT | **Primary hinge** | I89 |
| scoped acceptance charter | **Primary hinge** | I90 |
| S4-01 plan claim | **Claimed scoped `[x]`** | I91 |
| 书块 doctest（en CI） | **Inventory** | S1-09 |
| S4-02…05 / Stage-4 exit | **No（OUT）** | |

## KPIs（S3-I91 · Claim · 2026-11-09）

| KPI | Value | Notes |
|-----|------:|-------|
| `s4_01_claim_helper` | **≥1** | I91 hinge |
| `residual_scope_helper` | **≥1** | I90 |
| `stability_ssot_helper` | **≥1** | I89 Primary |
| `stability_ssot_package_rows` | **≥19** | `[[package]]` count |
| `stage4_open_helper` | **≥1** | I88 |
| `s4_01_census_helper` | **≥1** | census anchors |
| `packages_readme_status_rows` | **≥1** | README Status 列 |
| `packages_readme_plan_ternary_preview` | **0** | README 无字面 preview 档 |
| `manifest_stability_field_count` | **0** | Hard OUT / deferred |
| `syx_doctest_subcommand_count` | **0** | Hard OUT / deferred |
| `doctest_annotations_doc_helper` | **≥1** | book annotations |
| `capabilities_experimental_helper` | **≥1** | inventory |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S4-01 | **`[x]`** scoped；SSOT Primary 绿；manifest/doctest deferred supersede-only |
| Stage-4 | **`[~]`** · **1/5** |
| Next | **S3-I92** S4-02 open — ≠ Stage-4 exit / ≠ S4-02 construct this window |

## Ladder (locked)

1. **S3-I87:** Stage-3 exit — landed.
2. **S3-I88:** Stage-4 + S4-01 open — landed.
3. **S3-I89:** S4-01 stability SSOT — landed.
4. **S3-I90:** S4-01 residual-scope — landed.
5. **S3-I91:** S4-01 scoped `[x]` claim — **this window**.
6. **S3-I92:** S4-02 open — next。

## Non-claims

- **≠** Stage-4 exit / write `stage4-exit` this window  
- **≠** `manifest.json` schema / driver 读 SSOT  
- **≠** `syx doctest` / packages doctest 闸  
- **≠** S4-02…05 construct / HostFn / F-08 unpark  

## References

- [stage4-s4-01-claim](adr/stage4-s4-01-claim.md) · S3-I91
- [stage4-s4-01-residual-scope](adr/stage4-s4-01-residual-scope.md) · S3-I90
- [s4-01-manifest-doctest-scope-v0](s4-01-manifest-doctest-scope-v0.md)
- [stage4-s4-01-stability-ssot](adr/stage4-s4-01-stability-ssot.md) · S3-I89
- [stage4-open](adr/stage4-open.md) · S3-I88
- [stdlib-stability-v0](stdlib-stability-v0.md)
- [plan/02](../plan/02-improvement-roadmap.md) S4-01
