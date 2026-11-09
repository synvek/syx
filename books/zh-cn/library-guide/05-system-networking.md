# System I/O 与 Networking

## System I/O（`import System`）

**实现状态：** `File` 读写/存在/复制 **已实现**；`readAsync` 走 blocking pool awaitable，**不是** epoll reactor GA。`Http.serve` 在 Networking 下（默认后端 = hyper+tokio）。同端口 HMR 产品路径自动化、重叠 accept、进程内 HTTPS GET、以及 `upgradeTerminate` 帧路径已按钉点 met — 仍 **≠** 浏览器 Playwright GA / 全量并发 SLA / WSS 产品 GA。

```syx
import System

func main() {
    File.write(path: "/tmp/syx-io-demo.txt", content: "hi")
    print(File.read(path: "/tmp/syx-io-demo.txt"))
}
```

范例：[examples/file_tool/](../../../examples/file_tool/) · [examples/io_demo.sy](../../../examples/io_demo.sy)

公开类型：`File`、`Directory`、`Stdin`、`FilePath`（`import System`）。

## Networking（`import Networking`）

**实现状态：** `Http` / `Url` 返回 `HTTPResponse` / `Url`；TLS **客户端** 真路径；**不是** server TLS GA，**不是** streaming body。WebSocket / gRPC 收窄面以 opt-in 实验路径存在；**仍非** 产品 GA。

```syx
import Networking

func main() {
    let r = Http.fetch(url: "https://example.com")
    print(r.status)
}
```

范例：[examples/http_tool/](../../../examples/http_tool/) · [examples/http_server_tool/](../../../examples/http_server_tool/) · [examples/httpclient_demo.sy](../../../examples/httpclient_demo.sy)

| API | 说明 |
|-----|------|
| `Http.fetch(url:)` | GET；单标签参数 `url` |
| `Http.post(url:body:)` | POST，body 为 `String` |
| `Url` | 解析 / 组件 |

环回正例可设 `SYX_EC_M2_1_HTTP_BASE`。`Http.serve` 的请求次数上限见 `SYX_HTTP_SERVE_MAX_REQUESTS`（≠ graceful `Http.shutdown` GA）。
