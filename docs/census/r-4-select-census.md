# R-4 Select public-surface census (post-SA residual)

**Status:** R-4 exit hinge met (R-4-01 + R-4-02 `[x]`) · next = **R-5 Mutex/Semaphore**（已由 R-5 接棒；见 [r-5 census](r-5-mutex-sem-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r4_select_census.sh`](../../scripts/r4_select_census.sh)

## Exit hinge (R-4)

| ID | Claim | Evidence |
|----|-------|----------|
| R-4-01 | Public `Select` off Object | KPI below; `Array<Channel>` + `Sendable` |
| R-4-02 | Select hard gate | LSP `channel_select_parse` + channel/m3 fixtures green |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r3_hinge_still_green` | **1** | Rollup of [`r3_channel_recv_census.sh`](../../scripts/r3_channel_recv_census.sh) |
| `select_public_object_residual` | **0** | Select method lines ≠ Object |
| `select_channels_array_channel_typed` | **1** | All three use `Array<Channel>` |
| `select_return_sendable_typed` | **1** | All three return `Sendable` |
| `select_fallback_sendable_typed` | **1** | `orDefault` `fallback: Sendable` |

### Select Object slice (R-4-01)

| API | Was | Now |
|-----|-----|-----|
| `Select.select` | `Array<Object> -> Object` | `Array<Channel> -> Sendable` **cleared** |
| `Select.orDefault` | `Array<Object>, fallback: Object -> Object` | `Array<Channel>, fallback: Sendable -> Sendable` **cleared** |
| `Select.timeout` | `Array<Object> -> Object` | `Array<Channel> -> Sendable` **cleared** |

Source: [`packages/sync/mod.sy`](../../packages/sync/mod.sy).

**Scoped honesty:** internal host `channel_select*` args may stay loosely typed (`channels: String` spelling); returns typed `Array` so Select can index. Unready/timeout still runtime `nil` (≠ typed Optional). `Channel.handle` cleared in **R-6** (`Int` dual-track). ≠ `Channel<T>`.

**Supporting fixes (same window):** VM `channel_id_from_value` unwraps class `ObjectRef.handle` (+ MutRefCell); `value_compare` resolves HeapSlot strings so miss sentinel `"-1"` compares correctly.
## How to run

```bash
bash scripts/r4_select_census.sh
```

## Next

**R-5** — Mutex/Semaphore → `Int`（已退出；见 [r-5 census](r-5-mutex-sem-census.md)）。**R-6** — `Channel.handle` → `Int`（已退出；见 [r-6 census](r-6-channel-handle-census.md)）。下一 = **R-7+**。
