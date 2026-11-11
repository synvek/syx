# R-9 Url query census (post-SA residual)

**Status:** R-9 exit hinge met (R-9-01 + R-9-02 `[x]`) · next = **R-10 WebSocket**（已退出，见 [r-10 census](r-10-websocket-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r9_url_query_census.sh`](../../scripts/r9_url_query_census.sh)

## Exit hinge (R-9)

| ID | Claim | Evidence |
|----|-------|----------|
| R-9-01 | Url encodeQuery/build query off Object | KPI below; → Dictionary |
| R-9-02 | Hard gate + credibility | sa5_net_overlay / Url fixtures + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r8_hinge_still_green` | **1** | Rollup of [`r8_http_body_stream_census.sh`](../../scripts/r8_http_body_stream_census.sh) |
| `url_public_object_residual` | **0** | methods ≠ Object |
| `url_encode_query_param_dictionary` | **1** | `encodeQuery(query: Dictionary)` |
| `url_build_query_param_dictionary` | **1** | `build(..., query: Dictionary)` |
| `url_syxhost_query_dictionary` | **1** | SyxHost aligned (+ `url_build` base:String) |
| `networking_url_mirror_dictionary` | **1** | gen networking |

### Url query slice (R-9-01)

| API | Was | Now |
|-----|-----|-----|
| `Url.encodeQuery` | `query: Object` | `query: Dictionary` |
| `Url.build` | `query: Object` | `query: Dictionary` |
| `SyxHost.url_encode_query` | `query: Object` | `query: Dictionary` |
| `SyxHost.url_build` | `base: Float64`, `query: Object` | `base: String`, `query: Dictionary` |

Source: [`packages/net/url.sy`](../../packages/net/url.sy).

**Scoped honesty:** VM already accepts Object\|Dict bags. ≠ WS Int dual-track; ≠ simd; ≠ Task.

## How to run

```bash
bash scripts/r9_url_query_census.sh
```

## Next

**R-10** — `WebSocket` → `Dictionary`（已退出；见 [r-10 census](r-10-websocket-census.md)）。下一 = **R-11+ residual**。
