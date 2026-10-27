# SYX 网络标准库

> **现状对齐：** SYX **117.0** · Phase **139** · 包面 [`packages/net/`](../packages/net/) · 能力摘要 [capabilities-summary.md](adr/capabilities-summary.md)  
> WebSocket / gRPC 等为**收窄面**；**≠** 产品 GA。详见 [implementation-status](../books/shared/implementation-status.md)。

Node.js / fetch 风格的 HTTP 客户端、URL 工具、blocking HTTP 服务端，以及 TCP / WebSocket 等收窄模块。

## HTTP 命名类型（ADR-236 / `M2-S4-01`）

| 类型 | 说明 |
|------|------|
| `HTTPResponse` | `Http.get`/`post`/`fetch`/`request`/`response` 返回类型（非糊墙 `Object`） |
| `Url` | `Url.parse` 返回类型；字段 `href`/`host`/`port`/`path`/`query` |
| `HttpRequest` | 请求形状标记（method/path/query/body） |
| `HttpHeaders` / `HttpBody` | Headers/Body 类型面（流式 body → `M2-S4-04`） |

运行时值形状仍可为 Object 袋 + `__http_response` / MethodId。**非** streaming（`M2-S4-04`）；**非** `EC-M2-4-Z`。钉测：`m2_s4_01_http_types`。

## HTTP 客户端

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

### 便捷方法

| 方法 | 说明 |
|------|------|
| `Http.get(url) -> HTTPResponse` | GET 请求 |
| `Http.post(url, body) -> HTTPResponse` | POST；body 为 `String` 或对象 |
| `Http.fetch(url, options) -> HTTPResponse` | 通用 fetch 风格请求 |
| `Http.request(url, options) -> HTTPResponse` | 同 `fetch` |

### Request options

| 字段 | 类型 | 说明 |
|------|------|------|
| `method` | `String` | GET / POST / PUT / PATCH / DELETE / HEAD（默认 GET） |
| `headers` | `Object` | 请求头 |
| `body` | `String` \| `Object` | 请求体；对象为 JSON 时自动设置 `Content-Type: application/json` |
| `query` | `Object` | 追加 query string |
| `timeout_ms` | `Int` | 超时（毫秒） |

### Response 方法

| 方法 | 返回 | 说明 |
|------|------|------|
| `status()` | `Int` | HTTP 状态码 |
| `ok()` | `Bool` | 2xx 为 true |
| `text()` / `body()` | `String` | 响应体文本 |
| `json()` | `Object` | 解析 JSON |
| `header(name)` | `String?` | 单个响应头 |
| `headers()` | `Object` | 全部响应头 |

### HTTPS / TLS client（ADR-237 / `M2-S4-05`）

客户端使用 `reqwest` + `rustls`，支持 HTTPS GET/POST 等。默认**校验证书**（系统根）。CI/诊断：`SYX_HTTP_EXTRA_ROOT_PEM`（PEM 文件路径）可**追加**信任锚，仍走校验路径（≠ skip-verify / ≠ `danger_accept_invalid_certs`）。正例钉：`m2_s4_05_client_tls`。负例（self-signed / bad cert fail-closed）：ADR-235 / `m2_s4_02b_client_timeout_tls`。**非** skip-verify 产品 API；server TLS 产品 status raise **met**（ADR-779 / W435）；handler application-data **met**（ADR-780 / W436；`HANDLED`）；multi-conn N=2 **met**（ADR-781 / W437）；ALPN `syx/1` **met**（ADR-782 / W438）；`TlsServe` HostFn **implemented**（ADR-785 / W441）；unbounded idle-timeout multi-conn **met**（ADR-786 / W442）；env configurable ALPN **met**（ADR-787 / W443；`SYX_TLS_SERVE_ALPN`）；concurrent thread-per-conn **met**（ADR-788 / W444）；language String alpn 5th arg **met**（ADR-789 / W445；`serveOnceAlpn`）；env no-idle first-slice **met**（ADR-801 / W457；`SYX_TLS_SERVE_NO_IDLE` + optional `SYX_TLS_SERVE_MAX_HANDLED`）；language noIdle 6th Int arg **met**（ADR-802 / W458；`serveOnceNoIdle` / `serveOnceAlpnNoIdle`；`0`→env/默认 idle；`≠0`→跳过 idle；≠ Array alpn API / ≠ daemon GA / ≠ thread-pool API）。

### Timeout（ADR-235）

`fetch` / `request` options 的 `timeout_ms`：真超时 → 诊断码 **R012**（`reqwest::Error::is_timeout`）。其它运输失败仍 **R003**。`get`/`post` 便捷方法无 per-call timeout（用默认 client）。钉测：`m2_s4_02b_client_timeout_tls`。

### 响应 body 分块读（ADR-239 / `M2-S4-04`）

默认 `Http.get` / `fetch` **仍整缓冲**。opt-in 会话：

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

VM 保留 `reqwest` Response，用 `std::io::Read` 增量读（该路径**不**调用 `response.bytes()`）。流路径**不**做 zstd/gzip 整缓冲解压。钉测：`m2_s4_04_http_body_stream`。**非** server 流式；**非** wire `Transfer-Encoding: chunked` 产品 API；**非** File `M2-S3-03`；**非** `JSONStream`（S7）；**非** `EC-M2-4-Z`。

## URL 模块

```syx
import Net

let u: Url = Url.parse("https://example.com/path?q=1")
print(u.host)
print(u.path)
print(u.query)
print(Url.encodeQuery({ "a": "b", "c": "2" }))
print(Url.build("https://host", "/api", { "page": "1" }))
```

| 方法 | 说明 |
|------|------|
| `Url.parse(text) -> Url` | 解析 URL（字段 `href`/`host`/`port`/`path`/`query`；运行时袋另有 `protocol`） |
| `Url.encodeQuery(dict)` | 字典 → query string |
| `Url.build(base, path, query)` | 拼接 base + path + query |

## HTTP 服务端（blocking）

基于 `tiny_http` 的同步 HTTP 服务器。首期仅 HTTP（无 TLS）；生产 HTTPS 请用 nginx / Caddy 等反向代理终止 TLS。

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
    // 阻塞直到 Http.shutdown()（或进程退出）
    Http.serve("127.0.0.1", 8080, "handle")
}
```

**Graceful shutdown（ADR-238 / `M2-S4-03b`）：** `Http.shutdown()` 对活跃 `Http.serve` 调用 `tiny_http::Server::unblock()`；**完成当前请求后** `serve` 返回。钉测：`m2_s4_03b_server_graceful_shutdown`。`SYX_HTTP_SERVE_MAX_REQUESTS` 仍是独立的测试/运维计数钩（≠ graceful）。**非** streaming body（`M2-S4-04`）；**非** TLS handler/ALPN GA；**非** `EC-M2-4-Z`。

### Request 对象（`HttpRequest` 类型面）

| 字段 | 类型 | 说明 |
|------|------|------|
| `method` | `String` | HTTP 方法 |
| `path` | `String` | 路径（不含 query） |
| `query` | `String` | query string（不含 `?`） |
| `headers` | `Object` | 请求头（`HttpHeaders` 标记类型） |
| `body` | `String` | 请求体（`HttpBody` 文本 MVP） |

### Response 构建

```syx
Http.response(status, body) -> HTTPResponse            // body: String 或 Object
```

返回 `HTTPResponse`，供服务端 handler 使用。

### 停服用法

`Http.serve` 在 accept 循环中阻塞；**不要**指望 `__syx_task_cancel` 打断它（async 运行时会跳过含 `__syx_http_serve` 的函数）。从 handler 或其它线程调用 `Http.shutdown()`：

```syx
// handler 内：
Http.shutdown()
return Http.response(200, "bye")
```

## TCP（M0-B02 · VM MVP）

`TcpStream.connect(host:port:)` returns a stream with integer `handle` (`>= 0` ok, `-1` fail).
`__syx_tcp_connect` returns **Int** handle — not a `"tcp-ok:{n}"` status string.
Read/write take `(handle, …)`; unknown handles fail (no fake success).

## WebSocket / gRPC / TLS server（诚实边界 · M0-A04–A06）

| 表面 | 状态 |
|------|------|
| WebSocket | **experimental / mock** — not production wire protocol |
| gRPC | **experimental facade** — not gRPC/HTTP2 wire protocol |
| TLS server | **implemented** product status（ADR-779 / W435）+ app-data（ADR-780 / W436）+ multi-conn N=2（ADR-781 / W437）+ ALPN `syx/1`（ADR-782 / W438）+ `TlsServe` HostFn **implemented**（ADR-785 / W441）+ unbounded idle（ADR-786 / W442）+ env ALPN（ADR-787 / W443）+ concurrent thread-per-conn（ADR-788 / W444）+ language String alpn 5th arg（ADR-789 / W445；`serveOnceAlpn`）+ env no-idle first-slice（ADR-801 / W457）+ language noIdle 6th Int（ADR-802 / W458；`serveOnceNoIdle`；≠ Array alpn / ≠ daemon GA / ≠ thread-pool API） |
| HTTPS client | **implemented** path（ADR-237 / `M2-S4-05`；reqwest+rustls 默认校验；≠ skip-verify） |

## 诊断码

| 代码 | 说明 |
|------|------|
| R003 | HTTP 请求失败 / 无效 method / TLS verify 失败等运输错误 |
| R005 | JSON 解析失败 |
| R011 | HTTP client 构建失败 |
| R012 | HTTP 请求真超时（`is_timeout` · ADR-235）；亦用于 serve 缺 program context |
| R013 | URL 解析失败 |
| U001 | DateTime 未知 IANA 区名（已知区已 implemented · ADR-233；窄历法核心 · ADR-222） |

## 明确不做（当前版本 · 勿宣传为 GA）

- 生产级 WebSocket / gRPC；TLS server Array alpn API；daemon GA（语言/env noIdle first-slice ≠ GA）；thread-pool 配置产品 API
- 完整 IANA DateTime / DST
- 连接池、Cookie jar、证书 pinning、用户面 skip-verify

## 后续扩展

- `[network]` manifest 段：timeout、proxy、TLS 选项
- async fetch：`Concurrency.detach` + join 或专用 async host
- WASM：与 File/HTTP 相同，通过 host import 注册
