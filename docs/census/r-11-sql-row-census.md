# R-11 SqlRow census (post-SA residual)

**Status:** R-11 exit hinge met (R-11-01 + R-11-02 `[x]`) · next = **R-12+ residual**  
**Date:** 2026-11-10  
**Script:** [`scripts/r11_sql_row_census.sh`](../../scripts/r11_sql_row_census.sh)

## Exit hinge (R-11)

| ID | Claim | Evidence |
|----|-------|----------|
| R-11-01 | SqlRow public `row` off Object | KPI below; → Dictionary |
| R-11-02 | Hard gate + credibility | SQL fixtures + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r10_hinge_still_green` | **1** | Rollup of [`r10_websocket_census.sh`](../../scripts/r10_websocket_census.sh) |
| `sql_row_public_object_residual` | **0** | methods ≠ Object |
| `sql_row_params_dictionary` | **1** | six accessors `row: Dictionary` |
| `sql_row_syxhost_dictionary` | **1** | SyxHost aligned |

### SqlRow slice (R-11-01)

| API | Was | Now |
|-----|-----|-----|
| `getString` / `getInt` / `getFloat` / `getBlob` / `isNull` / `columnNames` | `row: Object` | `row: Dictionary` |
| `SyxHost.sql_row_*` | `row: Object` | `row: Dictionary` |

Source: [`packages/data/sql_row.sy`](../../packages/data/sql_row.sy).

**Scoped honesty:** VM still reads `__type=SqlRow` runtime bag; public spelling `Dictionary`. ≠ SqlRow value-type GA; ≠ query builder; ≠ Task→Int; ≠ simd wipe.

## How to run

```bash
bash scripts/r11_sql_row_census.sh
```

## Next

**R-12** — Fmt + Testing 公开 Object → Sendable/Function（已交；见 [`r-12-text-testing-census.md`](r-12-text-testing-census.md)）。下一 = **R-13+**。
