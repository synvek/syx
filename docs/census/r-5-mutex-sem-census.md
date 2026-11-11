# R-5 Mutex/Semaphore census (post-SA residual)

**Status:** R-5 exit hinge met (R-5-01 + R-5-02 `[x]`) · next = **R-6 handle Int**（已退出，见 [r-6 census](r-6-channel-handle-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r5_mutex_sem_census.sh`](../../scripts/r5_mutex_sem_census.sh)

## Exit hinge (R-5)

| ID | Claim | Evidence |
|----|-------|----------|
| R-5-01 | Public Mutex/Semaphore off Object | KPI below; factories/params `Int` |
| R-5-02 | RAII hard gate | `mutex_raii` / `semaphore_raii` green |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r4_hinge_still_green` | **1** | Rollup of [`r4_select_census.sh`](../../scripts/r4_select_census.sh) |
| `mutex_sem_public_object_residual` | **0** | Mutex/Semaphore method lines ≠ Object |
| `mutex_create_returns_int` | **1** | `Mutex.create() -> Int` |
| `semaphore_create_returns_int` | **1** | `Semaphore.create(permits:) -> Int` |
| `mutex_sem_params_int_typed` | **1** | lock/wait/… params are `Int` |

### Mutex / Semaphore Object slice (R-5-01)

| API | Was | Now |
|-----|-----|-----|
| `Mutex.create` | `-> Object` | `-> Int` **cleared** |
| `Mutex.lock` / `unlock` / `lockGuard` | `mutex: Object` | `mutex: Int` **cleared** |
| `Semaphore.create` | `-> Object` | `-> Int` **cleared** |
| `Semaphore.wait` / `tryWait` / `signal` / `waitGuard` | `sem: Object` | `sem: Int` **cleared** |

Source: [`packages/sync/mod.sy`](../../packages/sync/mod.sy).

**Scoped honesty:** Host already returned `Int` (`SyxHost.mutex_create` / `semaphore_create`). `Channel.handle` cleared in **R-6**. ≠ `Channel<T>`.

## How to run

```bash
bash scripts/r5_mutex_sem_census.sh
```

## Next

**R-6** — `Channel.handle` → `Int`（已退出；见 [r-6 census](r-6-channel-handle-census.md)）。下一 = **R-7+ residual**。
