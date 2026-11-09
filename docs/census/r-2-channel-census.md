# R-2 Channel send/trySend census (post-SA residual)

**Status:** R-2 exit hinge met (R-2-01 + R-2-02 `[x]`) · next = **R-3 recv/Select**  
**Date:** 2026-11-10  
**Script:** [`scripts/r2_channel_census.sh`](../../scripts/r2_channel_census.sh)

## Exit hinge (R-2)

| ID | Claim | Evidence |
|----|-------|----------|
| R-2-01 | Public `Channel.send` / `trySend` off Object | KPI below; `value: Sendable` |
| R-2-02 | Channel hard gate | `channel_send_recv` / m3_a04 fixtures green |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r1_hinge_still_green` | **1** | Rollup of [`r1_withopen_census.sh`](../../scripts/r1_withopen_census.sh) |
| `channel_send_trysend_object_residual` | **0** | Public `send`/`trySend` lines ≠ Object |
| `channel_send_trysend_sendable_typed` | **1** | Both methods use `value: Sendable` |

### send / trySend Object slice (R-2-01)

| API | Was | Now |
|-----|-----|-----|
| `Channel.send` | `value: Object` | `value: Sendable` **cleared** |
| `Channel.trySend` | `value: Object` | `value: Sendable` **cleared** |

Source: [`packages/sync/mod.sy`](../../packages/sync/mod.sy).

**Scoped honesty:** [`packages/internal/system_host.sy`](../../packages/internal/system_host.sy) may keep `value: Object` (census does not scan internal; Rust host still accepts any `Value`). `recv` / `tryRecv` / `Select` / `handle` remain Object → **R-3**. ≠ `Channel<T>`.

## How to run

```bash
bash scripts/r2_channel_census.sh
```

## Next

**R-3** — Channel `recv` / Select / `handle` Object 切片（或 Channel 泛型候选；≠ 本窗）。
