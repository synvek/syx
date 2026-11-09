# R-1 withOpen census (post-SA residual)

**Status:** R-1 exit hinge met (R-1-01 + R-1-02 `[x]`) · next = **R-2 Channel**（已由 R-2 接棒；见 [r-2 census](r-2-channel-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r1_withopen_census.sh`](../../scripts/r1_withopen_census.sh)

## Exit hinge (R-1)

| ID | Claim | Evidence |
|----|-------|----------|
| R-1-01 | Public `withOpen(body:)` off Object | KPI below; `body: Function` |
| R-1-02 | RAII hard gate | `filehandle_raii_with` green |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r0_hinge_still_green` | **1** | Rollup of [`r0_residual_census.sh`](../../scripts/r0_residual_census.sh) |
| `filehandle_withopen_object_residual` | **0** | Public `withOpen` line ≠ Object |
| `filehandle_withopen_object_cleared` | **1** | Same claim, positive key |
| `filehandle_withopen_function_typed` | **1** | `body: Function` (param `(Int)->Int` → P001; Array HOF shape) |

### withOpen Object slice (R-1-01)

| API | Was | Now |
|-----|-----|-----|
| `FileHandle.withOpen` | `body: Object` | `body: Function` **cleared** |

Sources: [`packages/io/file_handle.sy`](../../packages/io/file_handle.sy) · regen [`packages/system/mod.sy`](../../packages/system/mod.sy) via [`scripts/gen_system_mod.py`](../../scripts/gen_system_mod.py).

**Scoped honesty:** [`packages/internal/system_host.sy`](../../packages/internal/system_host.sy) may keep `body: Object` (census does not scan internal; Rust host still expects Closure).

## How to run

```bash
bash scripts/r1_withopen_census.sh
```

## Next

**R-2** — Channel `send`/`trySend` Object 切片（≠ Channel 泛型全清）。
