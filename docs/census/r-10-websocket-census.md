# R-10 WebSocket census (post-SA residual)

**Status:** R-10 exit hinge met (R-10-01 + R-10-02 `[x]`) · next = **R-11+ residual**  
**Date:** 2026-11-10  
**Script:** [`scripts/r10_websocket_census.sh`](../../scripts/r10_websocket_census.sh)

## Exit hinge (R-10)

| ID | Claim | Evidence |
|----|-------|----------|
| R-10-01 | WebSocket public off Object | KPI below; → Dictionary |
| R-10-02 | Hard gate + credibility | WS fixtures + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r9_hinge_still_green` | **1** | Rollup of [`r9_url_query_census.sh`](../../scripts/r9_url_query_census.sh) |
| `websocket_public_object_residual` | **0** | methods ≠ Object |
| `websocket_connect_returns_dictionary` | **1** | `connect -> Dictionary` |
| `websocket_socket_params_dictionary` | **1** | send/recv/close `socket: Dictionary` |
| `websocket_syxhost_dictionary` | **1** | SyxHost aligned |
| `networking_websocket_mirror_dictionary` | **1** | gen networking |

### WebSocket slice (R-10-01)

| API | Was | Now |
|-----|-----|-----|
| `connect` | `-> Object` | `-> Dictionary` |
| `send`/`recv`/`close` socket | `Object` | `Dictionary` |
| `SyxHost.ws_connect` | `-> Object` | `-> Dictionary` |
| `SyxHost.ws_send/recv/close` socket | `String` | `Dictionary` |
| `WsConnect` / `WsConnectAsync` registry | `"Object"` | `"Dictionary"` |

Source: [`packages/net/websocket.sy`](../../packages/net/websocket.sy).

**Scoped honesty:** VM still pushes/reads runtime bag; public spelling `Dictionary`. ≠ WS→Int; ≠ WSS GA; ≠ simd.

## How to run

```bash
bash scripts/r10_websocket_census.sh
```

## Next

**R-11** — SqlRow `row` → Dictionary（已交；见 [`r-11-sql-row-census.md`](r-11-sql-row-census.md)）。下一 = **R-12+**。
