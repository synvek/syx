# IO and Net

## IO

**Implementation status:** `Http.serve` **default back end = hyper+tokio**. Same-port HMR **product path** automation pinned met (≠ browser Playwright GA). Multi-connection **overlapping accept** met (≠ full concurrency SLA). In-process **HTTPS GET** met (env PEM + rustls; ≠ product GA). Overlap×HMR **cross pin** met (≠ SLA). Upgrade-over-TLS **HMR-over-HTTPS** met (≠ WSS product GA ≠ removing the sidecar). **Frame termination / self-computed Accept by serve** met (`upgradeTerminate`; ≠ WSS product GA). `upgradeTerminate` **close-path** met (≠ complete codec). `upgradeTerminate` **binary echo** met (≠ fragment/ping). `upgradeTerminate` **ping→pong** met (≠ fragment). `upgradeTerminate` **text fragmentation** met (≠ binary fragmentation). `upgradeTerminate` **binary fragmentation** met (≠ multi-segment continuation). `upgradeTerminate` **multi-segment continuation (N=3)** met (≠ arbitrary segment count). `upgradeTerminate` **control-during-fragment** met (N=3 text frag1→ping→mid; ≠ arbitrary segment count / binary mid-control). `upgradeTerminate` **binary mid-control** met (N=3 binary frag1→ping→mid; ≠ arbitrary segment count). `upgradeTerminate` **arbitrary segment count (N=4 text)** met (≠ unbounded N / binary arbitrary-N) … ≠ Playwright). todo 8 **overall soft-close / item close-out** met (surface frozen; Playwright/SLA/websocat/extension and others explicitly OUT; item 8 opens no further slices). `File` read/write/exists/copy **Implemented**; `readAsync` goes to the blocking pool awaitable, **not** an epoll reactor GA.

```syx
import IO

func main() {
    File.write("/tmp/syx-io-demo.txt", "hi")
    print(File.read("/tmp/syx-io-demo.txt"))
}
```

Examples: [examples/file_tool/](../../examples/file_tool/) · [examples/io_demo.sy](../../examples/io_demo.sy)

Public types: `File`, `Directory`, `Stdin` (`packages/io/mod.sy`).

## Net

**Implementation status:** `Http` / `Url` return `HTTPResponse` / `Url`; TLS **client** real path; **not** server TLS GA, **not** streaming body. E-01 WebSocket **scaffold** delivered; E-01 **real WS construction** delivered (narrow `ws: / ` RFC6455 opt-in); E-01 **negative cases** delivered (handshake fail-closed + close path); E-01 **interop** delivered (named loopback echo peer); E-01 **wss** delivered (named loopback rustls+RFC6455); E-01 **narrowed-surface flip charter** delivered; E-01 **narrowed-surface soft-close** delivered (`E-01` `[x]`); E-02 **real gRPC execution ADR** delivered; E-02 **scaffold** delivered; E-02 **real gRPC construction** delivered (narrow loopback H2+gRPC framing; opt-in `SYX_GRPC_REAL=1`; default is still the HTTP/1.1 facade); E-02 **interop** delivered (named loopback gRPC echo peer verbatim); E-02 **grpcs** delivered (named loopback rustls+H2+gRPC framing; still not product GA / streaming / row check); E-02 **narrowed-surface flip charter** delivered; E-02 **narrowed-surface soft-close** delivered (`E-02` `[x]`; still not product GA / streaming); E-02 **negative cases** delivered (missing PREFACE / truncated DATA fail-closed; still not the full negative set / streaming / product GA); E-02 **server-streaming** delivered (narrow loopback h2c multiple DATA; still not bidi / streaming GA / … ≠ full interop set); **still not** real gRPC product GA (`net.grpc` remains `experimental`).

Examples: [examples/http_tool/](../../examples/http_tool/) · [examples/http_server_tool/](../../examples/http_server_tool/) · [examples/httpclient_demo.sy](../../examples/httpclient_demo.sy)

Loopback positive cases can be configured with `SYX_EC_M2_1_HTTP_BASE`. The request-count ceiling of `Http.serve` is set by `SYX_HTTP_SERVE_MAX_REQUESTS` (≠ graceful `Http.shutdown` GA).
