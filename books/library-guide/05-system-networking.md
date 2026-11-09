# System I/O and Networking

## System I/O (`import System`)

**Implementation status:** `File` read/write/exists/copy **Implemented**; `readAsync` goes to the blocking pool awaitable, **not** an epoll reactor GA. `Http.serve` lives under Networking (default back end = hyper+tokio). Same-port HMR product-path automation, overlapping accept, in-process HTTPS GET, and `upgradeTerminate` frame paths are met as pinned elsewhere — still **≠** browser Playwright GA / full concurrency SLA / WSS product GA.

```syx
import System

func main() {
    File.write(path: "/tmp/syx-io-demo.txt", content: "hi")
    print(File.read(path: "/tmp/syx-io-demo.txt"))
}
```

Examples: [examples/file_tool/](../../examples/file_tool/) · [examples/io_demo.sy](../../examples/io_demo.sy)

Public types: `File`, `Directory`, `Stdin`, `FilePath` (`import System`).

## Networking (`import Networking`)

**Implementation status:** `Http` / `Url` return `HTTPResponse` / `Url`; TLS **client** real path; **not** server TLS GA, **not** streaming body. WebSocket / gRPC narrowed surfaces exist as opt-in experimental paths; **still not** product GA.

```syx
import Networking

func main() {
    let r = Http.fetch(url: "https://example.com")
    print(r.status)
}
```

Examples: [examples/http_tool/](../../examples/http_tool/) · [examples/http_server_tool/](../../examples/http_server_tool/) · [examples/httpclient_demo.sy](../../examples/httpclient_demo.sy)

| API | Notes |
|-----|-------|
| `Http.fetch(url:)` | GET; single labeled `url` argument |
| `Http.post(url:body:)` | POST body as `String` |
| `Url` | Parse / components |

Loopback positive cases can be configured with `SYX_EC_M2_1_HTTP_BASE`. The request-count ceiling of `Http.serve` is set by `SYX_HTTP_SERVE_MAX_REQUESTS` (≠ graceful `Http.shutdown` GA).
