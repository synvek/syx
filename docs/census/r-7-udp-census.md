# R-7 UdpSocket census (post-SA residual)

**Status:** R-7 exit hinge met (R-7-01 + R-7-02 `[x]`) · next = **R-8 HttpBodyStream**（已退出，见 [r-8 census](r-8-http-body-stream-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r7_udp_census.sh`](../../scripts/r7_udp_census.sh)

## Exit hinge (R-7)

| ID | Claim | Evidence |
|----|-------|----------|
| R-7-01 | UdpSocket public off Object | KPI below; bind/socket Int; recvFrom Dictionary |
| R-7-02 | Hard gate + credibility | UDP bind/echo fixtures + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r6_hinge_still_green` | **1** | Rollup of [`r6_channel_handle_census.sh`](../../scripts/r6_channel_handle_census.sh) |
| `udp_public_object_residual` | **0** | `packages/net/udp.sy` methods ≠ Object |
| `udp_bind_returns_int` | **1** | `UdpSocket.bind -> Int` |
| `udp_socket_params_int_typed` | **1** | sendTo/recvFrom `socket: Int` |
| `udp_recv_from_dictionary_typed` | **1** | `recvFrom -> Dictionary` |

### UdpSocket slice (R-7-01)

| API | Was | Now |
|-----|-----|-----|
| `UdpSocket.bind` | `-> Object` | `-> Int` |
| `sendTo(socket:)` | `Object` | `Int` |
| `recvFrom(socket:)` | `Object` | `Int` |
| `recvFrom` return | `Object` | `Dictionary` |

Source: [`packages/net/udp.sy`](../../packages/net/udp.sy) → [`packages/networking/mod.sy`](../../packages/networking/mod.sy).

**Scoped honesty:** VM recv bag remains runtime Object map; public spelling `Dictionary` (R-0 pattern). ≠ simd; ≠ WebSocket; ≠ Channel bag retirement.

## How to run

```bash
bash scripts/r7_udp_census.sh
```

## Next

**R-8** — `HttpBodyStream.open*` → `Dictionary`（已退出；见 [r-8 census](r-8-http-body-stream-census.md)）。下一 = **R-9+ residual**。
