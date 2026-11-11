# R-6 Channel.handle census (post-SA residual)

**Status:** R-6 exit hinge met (R-6-01 + R-6-02 `[x]`) · next = **R-7 UdpSocket**（已退出，见 [r-7 census](r-7-udp-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r6_channel_handle_census.sh`](../../scripts/r6_channel_handle_census.sh)

## Exit hinge (R-6)

| ID | Claim | Evidence |
|----|-------|----------|
| R-6-01 | `Channel.handle` Object→Int + create→Int dual-track | KPI below |
| R-6-02 | Hard gate + credibility | channel fixtures + Int-direct unit + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r5_hinge_still_green` | **1** | Rollup of [`r5_mutex_sem_census.sh`](../../scripts/r5_mutex_sem_census.sh) |
| `channel_handle_object_residual` | **0** | `Channel.handle: Int` |
| `channel_create_returns_int` | **1** | SyxHost + registry + VM |
| `channel_id_accepts_int` | **1** | bare `I64`/`I32` |
| `channel_id_bag_dual_track` | **1** | old `{__channel_id}` still accepted |

### Channel.handle slice (R-6-01)

| API | Was | Now |
|-----|-----|-----|
| `Channel.handle` | `Object` | `Int` |
| `SyxHost.channel_create` | `-> Object` | `-> Int` |
| `HostFnId::ChannelCreate` | `"Object"` | `"Int"` |
| `host_channel_create` | `{__channel_id}` bag | bare Int |
| `channel_id_from_value` | bag / ObjectRef only | Int \| bag dual-track |

Source: [`packages/sync/mod.sy`](../../packages/sync/mod.sy), [`crates/syx-vm/src/channel_hosts.rs`](../../crates/syx-vm/src/channel_hosts.rs).

**Scoped honesty:** ≠ `Channel<T>`; ≠ bag dual-track retirement; ≠ simd.

## How to run

```bash
bash scripts/r6_channel_handle_census.sh
```

## Next

**R-7** — `UdpSocket` → `Int`/`Dictionary`（已退出；见 [r-7 census](r-7-udp-census.md)）。下一 = **R-8+ residual**。
