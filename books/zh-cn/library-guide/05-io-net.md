# IO 与 Net

## IO

**实现状态：**：`Http.serve` **默认后端 = hyper+tokio**。：同端口 HMR **产品路径**自动化钉 met（；≠ 浏览器 Playwright GA）。：多连接 **重叠 accept** met（；≠ 全量并发 SLA）。：进程内 **HTTPS GET** met（；env PEM + rustls；≠ 产品 GA）。：重叠×HMR **交叉钉**met（；≠ SLA）。：Upgrade-over-TLS **HMR-over-HTTPS** met（；≠ WSS 产品 GA ≠ 去 sidecar）。：**帧终结 / serve 自算 Accept** met（；`upgradeTerminate`；≠ WSS 产品 GA）。：upgradeTerminate **close-path** met（；≠ 完整 codec）。：upgradeTerminate **binary echo** met（；≠ fragment/ping）。：upgradeTerminate **ping→pong** met（；≠ fragment）。：upgradeTerminate **分片 text** met（；≠ binary 分片）。：upgradeTerminate **binary 分片** met（；≠ 多段 continuation）。：upgradeTerminate **多段 continuation（N=3）** met（；≠ 任意段数）。：upgradeTerminate **control-during-fragment** met（；N=3 text frag1→ping→mid；≠ 任意段数 / binary mid-control）。：upgradeTerminate **binary mid-control** met（；N=3 binary frag1→ping→mid；≠ 任意段数）。：upgradeTerminate **任意段数（N=4 text）** met（；≠ 无界 N / binary 任意-N） … ≠ Playwright）。：todo 8 **总 soft-close / 项收口** met（完成面冻结；Playwright/SLA/websocat/extension 等明示 OUT；项 8 不再开片）。 `File` 读写/存在/复制 **已实现**；`readAsync` 走 blocking pool awaitable，**不是** epoll reactor GA。

```syx
import IO

func main() {
    File.write("/tmp/syx-io-demo.txt", "hi")
    print(File.read("/tmp/syx-io-demo.txt"))
}
```

范例：[examples/file_tool/](../../../examples/file_tool/) · [examples/io_demo.sy](../../../examples/io_demo.sy)

公开类型：`File`、`Directory`、`Stdin`（`packages/io/mod.sy`）。

## Net

**实现状态：** `Http` / `Url` 返回 `HTTPResponse` / `Url`；TLS **客户端** 真路径；**不是** server TLS GA，**不是** streaming body。E-01 WebSocket **scaffold** 已交；E-01 **真 WS 施工** 已交（；窄 `ws: / ` RFC6455 opt-in）；E-01 **负例** 已交（；握手 fail-closed + close-path）；E-01 **互操作** 已交（；命名 loopback echo peer）；E-01 **wss ** 已交（；命名 loopback rustls+RFC6455）；E-01 **收窄面翻行章程** 已交；E-01 **收窄面 soft-close ** 已交（；`E-01` `[x]`）；E-02 **真 gRPC 执行 ADR ** 已交；E-02 **scaffold ** 已交；E-02 **真 gRPC 施工** 已交（；窄 loopback H2+gRPC framing；opt-in `SYX_GRPC_REAL=1`；默认仍 HTTP/1.1 facade）；E-02 **互操作** 已交（；命名 loopback gRPC echo peer verbatim）；E-02 **grpcs ** 已交（；命名 loopback rustls+H2+gRPC framing；仍非产品 GA / streaming / 行勾选）；E-02 **收窄面翻行章程** 已交；E-02 **收窄面 soft-close ** 已交（；`E-02` `[x]`；仍非产品 GA / streaming）；E-02 **负例** 已交（；缺 PREFACE / 截断 DATA fail-closed；仍非负例全集 / streaming / 产品 GA）；E-02 **server-streaming ** 已交（；窄 loopback h2c 多 DATA；仍非 bidi / streaming GA / … ≠ 互操作全集）；**仍非** 真 gRPC 产品 GA（`net.grpc` 仍 `experimental`）。

范例：[examples/http_tool/](../../../examples/http_tool/) · [examples/http_server_tool/](../../../examples/http_server_tool/) · [examples/httpclient_demo.sy](../../../examples/httpclient_demo.sy)

环回正例可设 `SYX_EC_M2_1_HTTP_BASE`。`Http.serve` 的请求次数上限见 `SYX_HTTP_SERVE_MAX_REQUESTS`（≠ graceful `Http.shutdown` GA）。
