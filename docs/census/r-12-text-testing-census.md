# R-12 Fmt + Testing census (post-SA residual)

**Status:** R-12 exit hinge met (R-12-01 + R-12-02 `[x]`) · next = **R-13+ residual**  
**Date:** 2026-11-10  
**Script:** [`scripts/r12_text_testing_census.sh`](../../scripts/r12_text_testing_census.sh)

## Exit hinge (R-12)

| ID | Claim | Evidence |
|----|-------|----------|
| R-12-01 | Fmt + Testing public off Object | KPI below; → Sendable / Array\<Sendable\> / Function |
| R-12-02 | Hard gate + credibility | fmt/testing fixtures + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r11_hinge_still_green` | **1** | Rollup of [`r11_sql_row_census.sh`](../../scripts/r11_sql_row_census.sh) |
| `fmt_public_object_residual` | **0** | `fmt.sy` + `text/mod.sy` Fmt methods ≠ Object |
| `fmt_format_args_array_sendable` | **1** | `format(..., args: Array<Sendable>)` |
| `testing_public_object_residual` | **0** | assertEq/expectEqual/test ≠ Object |
| `testing_test_fn_function` | **1** | `test(name:, fn: Function)` |
| `object_sig_residual_count` | **217** | was 230 (−13) |
| `object_sig_residual_files` | **17** | was 20 (−3) |

### Fmt / Testing slice (R-12-01)

| API | Was | Now |
|-----|-----|-----|
| `Fmt.print` / `printLine` / `debug` | `value: Object` | `value: Sendable` |
| `Fmt.format` | `args: Object` | `args: Array<Sendable>` |
| `assertEq` / `expectEqual` | `Object`×2 | `Sendable`×2 |
| `test` | `fn: Object` | `fn: Function` |

Sources: [`packages/text/fmt.sy`](../../packages/text/fmt.sy) · [`packages/text/mod.sy`](../../packages/text/mod.sy) · [`packages/testing/mod.sy`](../../packages/testing/mod.sy).

**Scoped honesty:** SyxHost may keep `fmt_print(value: Object)` / `testing_assert_eq(...: String)` / `testing_test(fn: String)`; public spelling Sendable/Function/Array\<Sendable\>. ≠ Fmt product GA; ≠ Testing auto-import; ≠ simd wipe; ≠ collections Element/Key.

## How to run

```bash
bash scripts/r12_text_testing_census.sh
```

## Next

**R-13** — Concurrency 任务袋 → `Dictionary`、值位 → `Sendable`（已交；见 [`r-13-concurrency-census.md`](r-13-concurrency-census.md)）。下一 = **R-14+**。
