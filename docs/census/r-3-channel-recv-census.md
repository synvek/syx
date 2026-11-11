# R-3 Channel recv/tryRecv census (post-SA residual)

**Status:** R-3 exit hinge met (R-3-01 + R-3-02 `[x]`) · next = **R-4 Select**（已由 R-4 接棒；见 [r-4 census](r-4-select-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r3_channel_recv_census.sh`](../../scripts/r3_channel_recv_census.sh)

## Exit hinge (R-3)

| ID | Claim | Evidence |
|----|-------|----------|
| R-3-01 | Public `Channel.recv` / `tryRecv` off Object | KPI below; `-> Sendable` |
| R-3-02 | Channel hard gate | `channel_send_recv` / m3_a04 fixtures green |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r2_hinge_still_green` | **1** | Rollup of [`r2_channel_census.sh`](../../scripts/r2_channel_census.sh) |
| `channel_recv_tryrecv_object_residual` | **0** | Public `recv`/`tryRecv` lines ≠ Object |
| `channel_recv_tryrecv_sendable_typed` | **1** | Both methods return `Sendable` |

### recv / tryRecv Object slice (R-3-01)

| API | Was | Now |
|-----|-----|-----|
| `Channel.recv` | `-> Object` | `-> Sendable` **cleared** |
| `Channel.tryRecv` | `-> Object` | `-> Sendable` **cleared** |

Source: [`packages/sync/mod.sy`](../../packages/sync/mod.sy).

**Scoped honesty:** [`packages/internal/system_host.sy`](../../packages/internal/system_host.sy) may keep `-> Object` (census does not scan internal; Rust host still returns any `Value`). Empty/closed sentinel remains runtime `nil` (≠ typed `Optional`). `Select` / `handle` remain Object → **R-4**. ≠ `Channel<T>`.

## How to run

```bash
bash scripts/r3_channel_recv_census.sh
```

## Next

**R-4** — Select 公开面 → `Array<Channel>`/`Sendable`（已退出；见 [r-4 census](r-4-select-census.md)）。下一 = **R-5** `handle` bag。
