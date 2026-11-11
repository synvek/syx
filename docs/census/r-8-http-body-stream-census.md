# R-8 HttpBodyStream census (post-SA residual)

**Status:** R-8 exit hinge met (R-8-01 + R-8-02 `[x]`) · next = **R-9 Url query**（已退出，见 [r-9 census](r-9-url-query-census.md)）  
**Date:** 2026-11-10  
**Script:** [`scripts/r8_http_body_stream_census.sh`](../../scripts/r8_http_body_stream_census.sh)

## Exit hinge (R-8)

| ID | Claim | Evidence |
|----|-------|----------|
| R-8-01 | HttpBodyStream open* off Object | KPI below; → Dictionary |
| R-8-02 | Hard gate + credibility | http_body_stream fixtures + census contract |

## KPI snapshot

| KPI | Value | Notes |
|-----|-------|-------|
| `r7_hinge_still_green` | **1** | Rollup of [`r7_udp_census.sh`](../../scripts/r7_udp_census.sh) |
| `http_body_stream_public_object_residual` | **0** | methods ≠ Object |
| `http_body_stream_open_returns_dictionary` | **1** | `open -> Dictionary` |
| `http_body_stream_open_with_options_returns_dictionary` | **1** | `openWithOptions -> Dictionary` |
| `http_body_stream_chunk_close_int_typed` | **1** | readChunk/close `handle: Int` |

### HttpBodyStream slice (R-8-01)

| API | Was | Now |
|-----|-----|-----|
| `open` / `openWithOptions` | `-> Object` | `-> Dictionary` |
| `SyxHost.http_body_stream_open` | `-> Object` | `-> Dictionary` |
| `SyxHost.http_body_stream_open_opts` | `options: Object -> Object` | `Dictionary -> Dictionary` |
| `HttpBodyStreamOpen*` registry | `"Object"` | `"Dictionary"` |

Source: [`packages/net/http_body_stream.sy`](../../packages/net/http_body_stream.sy).

**Scoped honesty:** VM may still push runtime Object map; public spelling `Dictionary` (R-0/R-7 pattern). ≠ WS Int dual-track; ≠ simd; ≠ Task.

## How to run

```bash
bash scripts/r8_http_body_stream_census.sh
```

## Next

**R-9** — `Url.encodeQuery`/`build` `query` → `Dictionary`（已退出；见 [r-9 census](r-9-url-query-census.md)）。下一 = **R-10+ residual**。
