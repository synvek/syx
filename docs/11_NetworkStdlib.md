# SYX networking standard library

[简体中文](./zh/11_NetworkStdlib.zh-CN.md)

> **Current alignment:** SYX **117.0** · Phase **139** · package surface [`packages/net/`](../packages/net/) · capability summary [capabilities-summary.md](adr/capabilities-summary.md)  
> WebSocket / gRPC are **narrow surfaces**; **≠** product GA. See [implementation-status](../books/shared/implementation-status.md).

Node.js / fetch-style HTTP client, URL helpers, blocking HTTP server, plus narrow TCP / WebSocket modules.

## Named HTTP types (ADR-236 / `M2-S4-01`)

| Type | Description |
|------|-------------|
| `HTTPResponse` | Return type of `Http.get` / `post` / `fetch` / `request` / `response` (not a bare `Object`) |
| `Url` | Return type of `Url.parse`; fields `href` / `host` / `port` / `path` / `query` |
| `HttpRequest` | Request shape marker (method / path / query / body) |
| `HttpHeaders` / `HttpBody` | Headers / body type surface (streaming body → `M2-S4-04`) |

Runtime values may still be Object bags + `__http_response` / MethodId. **Not** streaming (`M2-S4-04`); **not** `EC-M2-4-Z`. Nail: `m2_s4_01_http_types`.

## HTTP client

```syx
import Net

let resp: HTTPResponse = Http.fetch("https://api.example.com/users", {
    "method": "POST",
    "headers": { "Authorization": "Bearer tok" },
    "body": { "name": "syx" },
    "timeout_ms": 5000
})

if resp.ok() {
    print(resp.json())
}
print(resp.status())
print(resp.text())
print(resp.header("Content-Type"))
print(resp.headers())
```

### Convenience methods

| Method | Description |
|--------|-------------|
| `Http.get(url) -> HTTPResponse` | GET |
| `Http.post(url, body) -> HTTPResponse` | POST; body is `String` or object |
| `Http.fetch(url, options) -> HTTPResponse` | Generic fetch-style request |
| `Http.request(url, options) -> HTTPResponse` | Same as `fetch` |

### Request options

| Field | Type | Description |
|-------|------|-------------|
| `method` | `String` | GET / POST / PUT / PATCH / DELETE / HEAD (default GET) |
| `headers` | `Object` | Request headers |
| `body` | `String` \| `Object` | Body; JSON objects set `Content-Type: application/json` |
| `query` | `Object` | Appended query string |
| `timeout_ms` | `Int` | Timeout in milliseconds |

### Response methods

| Method | Returns | Description |
|--------|---------|-------------|
| `status()` | `Int` | HTTP status code |
| `ok()` | `Bool` | True for 2xx |
| `text()` / `body()` | `String` | Response body text |
| `json()` | `Object` | Parsed JSON |
| `header(name)` | `String?` | Single response header |
| `headers()` | `Object` | All response headers |

### HTTPS / TLS client (ADR-237 / `M2-S4-05`)

Client uses `reqwest` + `rustls` for HTTPS GET/POST etc. Certificates are **verified by default** (system roots). CI/diagnostics: `SYX_HTTP_EXTRA_ROOT_PEM` (PEM path) may **append** trust anchors while still verifying (≠ skip-verify / ≠ `danger_accept_invalid_certs`). Positive nail: `m2_s4_05_client_tls`. Negative (self-signed / bad cert fail-closed): ADR-235 / `m2_s4_02b_client_timeout_tls`. **Not** a product skip-verify API. Server TLS product status raise **met** (ADR-779 / W435); handler application-data **met** (ADR-780 / W436; `HANDLED`); multi-conn N=2 **met** (ADR-781 / W437); ALPN `syx/1` **met** (ADR-782 / W438); `TlsServe` HostFn **implemented** (ADR-785 / W441); unbounded idle-timeout multi-conn **met** (ADR-786 / W442); env-configurable ALPN **met** (ADR-787 / W443; `SYX_TLS_SERVE_ALPN`); concurrent thread-per-conn **met** (ADR-788 / W444); language String ALPN 5th arg **met** (ADR-789 / W445; `serveOnceAlpn`); env no-idle first-slice **met** (ADR-801 / W457; `SYX_TLS_SERVE_NO_IDLE` + optional `SYX_TLS_SERVE_MAX_HANDLED`); language noIdle 6th Int arg **met** (ADR-802 / W458; `serveOnceNoIdle` / `serveOnceAlpnNoIdle`; `0`→env/default idle; `≠0`→skip idle; ≠ Array ALPN API / ≠ daemon GA / ≠ thread-pool API).

### Timeout (ADR-235)

`fetch` / `request` option `timeout_ms`: true timeout → diagnostic **R012** (`reqwest::Error::is_timeout`). Other transport failures remain **R003**. Convenience `get`/`post` have no per-call timeout (default client). Nail: `m2_s4_02b_client_timeout_tls`.

### Chunked response body reads (ADR-239 / `M2-S4-04`)

Default `Http.get` / `fetch` still **fully buffers**. Opt-in session:

```syx
import Net

let opened = HttpBodyStream.open(url: "http://127.0.0.1:8080/big")
let handle = opened.handle
print(opened.status)
var chunk = HttpBodyStream.readChunk(handle: handle, maxBytes: 1024)
while chunk != "" {
    print(chunk)
    chunk = HttpBodyStream.readChunk(handle: handle, maxBytes: 1024)
}
HttpBodyStream.close(handle: handle)
```

The VM keeps the `reqwest` Response and reads incrementally via `std::io::Read` (this path does **not** call `response.bytes()`). The stream path does **not** fully-buffer zstd/gzip decompression. Nail: `m2_s4_04_http_body_stream`. **Not** server streaming; **not** wire `Transfer-Encoding: chunked` product API; **not** File `M2-S3-03`; **not** `JSONStream` (S7); **not** `EC-M2-4-Z`.

## URL module

```syx
import Net

let u: Url = Url.parse("https://example.com/path?q=1")
print(u.host)
print(u.path)
print(u.query)
print(Url.encodeQuery({ "a": "b", "c": "2" }))
print(Url.build("https://host", "/api", { "page": "1" }))
```

| Method | Description |
|--------|-------------|
| `Url.parse(text) -> Url` | Parse URL (`href`/`host`/`port`/`path`/`query`; runtime bag also has `protocol`) |
| `Url.encodeQuery(dict)` | Dictionary → query string |
| `Url.build(base, path, query)` | Join base + path + query |

## HTTP server (blocking)

Synchronous HTTP server based on `tiny_http`. First slice is HTTP only (no TLS); for production HTTPS terminate TLS at nginx / Caddy / etc.

```syx
import Net

func handle(req: HttpRequest) -> HTTPResponse {
    if req.path == "/shutdown" {
        Http.shutdown()
        return Http.response(200, "bye")
    }
    return Http.response(200, "ok")
}

func main() {
    // Blocks until Http.shutdown() (or process exit)
    Http.serve("127.0.0.1", 8080, "handle")
}
```

**Graceful shutdown (ADR-238 / `M2-S4-03b`):** `Http.shutdown()` calls `tiny_http::Server::unblock()` on the active `Http.serve`; `serve` returns **after the current request**. Nail: `m2_s4_03b_server_graceful_shutdown`. `SYX_HTTP_SERVE_MAX_REQUESTS` remains a separate test/ops counter hook (≠ graceful). **Not** streaming body (`M2-S4-04`); **not** TLS handler/ALPN GA; **not** `EC-M2-4-Z`.

### Request object (`HttpRequest` surface)

| Field | Type | Description |
|-------|------|-------------|
| `method` | `String` | HTTP method |
| `path` | `String` | Path (no query) |
| `query` | `String` | Query string (no `?`) |
| `headers` | `Object` | Headers (`HttpHeaders` marker type) |
| `body` | `String` | Body (`HttpBody` text MVP) |

### Building responses

```syx
Http.response(status, body) -> HTTPResponse            // body: String or Object
```

Returns `HTTPResponse` for server handlers.

### Stopping the server

`Http.serve` blocks in the accept loop; **do not** expect `__syx_task_cancel` to interrupt it (the async runtime skips functions containing `__syx_http_serve`). Call `Http.shutdown()` from a handler or another thread:

```syx
// inside handler:
Http.shutdown()
return Http.response(200, "bye")
```

## TCP (M0-B02 · VM MVP)

`TcpStream.connect(host:port:)` returns a stream with integer `handle` (`>= 0` ok, `-1` fail).
`__syx_tcp_connect` returns an **Int** handle — not a `"tcp-ok:{n}"` status string.
Read/write take `(handle, …)`; unknown handles fail (no fake success).

## WebSocket / gRPC / TLS server (honest boundaries · M0-A04–A06)

| Surface | Status |
|---------|--------|
| WebSocket | **experimental / mock** — not production wire protocol |
| gRPC | **experimental facade** — not gRPC/HTTP2 wire protocol |
| TLS server | **implemented** product status (ADR-779 / W435) + app-data (ADR-780 / W436) + multi-conn N=2 (ADR-781 / W437) + ALPN `syx/1` (ADR-782 / W438) + `TlsServe` HostFn **implemented** (ADR-785 / W441) + unbounded idle (ADR-786 / W442) + env ALPN (ADR-787 / W443) + concurrent thread-per-conn (ADR-788 / W444) + language String ALPN 5th arg (ADR-789 / W445; `serveOnceAlpn`) + env no-idle first-slice (ADR-801 / W457) + language noIdle 6th Int (ADR-802 / W458; `serveOnceNoIdle`; ≠ Array ALPN / ≠ daemon GA / ≠ thread-pool API) |
| HTTPS client | **implemented** path (ADR-237 / `M2-S4-05`; reqwest+rustls verify by default; ≠ skip-verify) |

## Diagnostic codes

| Code | Description |
|------|-------------|
| R003 | HTTP request failed / invalid method / TLS verify failure, etc. |
| R005 | JSON parse failure |
| R011 | HTTP client build failure |
| R012 | True HTTP timeout (`is_timeout` · ADR-235); also serve missing program context |
| R013 | URL parse failure |
| U001 | Unknown IANA timezone (known zones implemented · ADR-233; narrow calendar core · ADR-222) |

## Explicit non-goals (current · do not market as GA)

- Production WebSocket / gRPC; TLS server Array ALPN API; daemon GA (language/env noIdle first-slice ≠ GA); thread-pool product API
- Full IANA DateTime / DST
- Connection pools, cookie jars, cert pinning, user-facing skip-verify

## Follow-ons

- `[network]` manifest section: timeout, proxy, TLS options
- Async fetch: `Concurrency.detach` + join or dedicated async host
- WASM: register via host imports like File/HTTP
