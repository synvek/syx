# Stage-3 LSP-book census (S3-06)

**ADR:** [open](adr/stage3-s3-06-open.md) · [s045-lsp-desugar](adr/stage3-s3-06-s045-lsp-desugar.md) · [residual-scope](adr/stage3-s3-06-residual-scope.md) · [**claim**](adr/stage3-s3-06-claim.md) · **Window:** S3-I81 · **Current:** S3-I81 · **Claimed:** scoped `[x]`

**Goal (S3-06 scoped `[x]`):** Primary = I79 desugar hinge + sample fixtures LSP 对齐；rename / signature / hover / full fence = **deferred supersede-only**.  
**This census:** S3-I81 scoped claim — S3-06 **`[x]`**；Stage-3 仍 **`[~]`**（**6/7**）；**≠** Stage-3 exit / **≠** 本窗 S3-07 施工.

## Command

```bash
bash scripts/s3_06_lsp_book_census.sh
```

## Classification (hard)

| Class | Counts as S3-06 debt? | Notes |
|-------|----------------------|-------|
| S045 LSP desugar mismatch | **Claimed** | I79 |
| Sample book LSP semantic mismatch | **Claimed** | Primary |
| Residual-scope / claim scaffolding | **Claimed** | I80–I81 |
| Rename honesty | **Deferred** | supersede-only |
| Signature help book 验收 | **Deferred** | supersede-only |
| Hover 书块产品轴 | **Deferred** | S3-02 demangle elsewhere |
| Full fence LSP sweep | **Deferred** | supersede-only |
| S3-07 init / Stage-3 exit | **No（OUT）** / **Next** | I82 open |

## KPIs（S3-I81 · Claim · 2026-11-08）

| KPI | Value | Notes |
|-----|------:|-------|
| `s3_06_claim_helper` | **≥1** | I81 hinge |
| `residual_scope_helper` | **≥1** | I80 |
| `s045_lsp_desugar_helper` | **≥1** | I79 |
| `s045_lsp_desugar_nail_count` | **0** | Primary claimed |
| `book_fixture_sample_n` | **≥3** | fixtures |
| `book_lsp_semantic_mismatch_count` | **0** | Primary claimed |
| `stage3_lsp_census_helper` | **≥1** | ADR anchors |
| `rename_provider_helper` | **≥1** | deferred axis inventory |

## Gap（诚实）

| 轴 | 状态 |
|----|------|
| S3-06 | **`[x]`** scoped；rename/sig/fence deferred |
| Stage-3 | 仍 **`[~]`**（**6/7**；余 S3-07） |
| Next | **S3-I82** S3-07 open — ≠ Stage-3 exit |

## Ladder (locked)

1. **S3-I78:** S3-06 open — landed.
2. **S3-I79:** S045 LSP desugar — landed.
3. **S3-I80:** residual-scope — landed.
4. **S3-I81:** S3-06 scoped claim — **this window**.
5. **S3-I82:** S3-07 open — next（≠ Stage-3 exit）。

## Non-claims

- **≠** Stage-3 exit  
- **≠** S3-07 construction this window  
- **≠** 全书零误报字面宣称  
- **≠** rename / signature / hover product knives  
- **≠** full fence LSP sweep  
- **≠** reopen S3-05 lint / S3-04 fib  

## References

- [stage3-s3-06-claim](adr/stage3-s3-06-claim.md) · S3-I81
- [stage3-s3-06-residual-scope](adr/stage3-s3-06-residual-scope.md) · S3-I80
- [plan/02](../plan/02-improvement-roadmap.md) S3-06
