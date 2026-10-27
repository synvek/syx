# sgate — a minimal nginx-like HTTP gateway in SYX

`sgate` is a small reverse-proxy / static-file / canned-response
gateway written in [SYX 116.0](../../README.md) and configured entirely
by a single JSON file. It demonstrates how the SYX stdlib can be
combined into a non-trivial server-side application: a real HTTP
listener (via the `Net` package), JSON config parsing, dynamic
routing, round-robin upstream selection, prefix-stripping, static
file serving, and access logging.

> **Phase / version**
> - Built for SYX 116.0 (manifest phase 138).
> - Designed against `docs/9_UsageGuide.md` HTTP server API and
>   `packages/{net,io,datetime,log}` stdlib surfaces.
> - `sgate` is a *tools/* entry — no `syx.toml`, no multi-package
>   workspace. All sources live in this directory and are loaded by
>   `syx run tools/sgate/main.sy [CONFIG_PATH]`.

## Features

| Capability | Where |
|---|---|
| JSON config (no code change for new routes) | `sgate.json` |
| Path-based routing: `exact` + `prefix` matching | `router.sy::matchLocation` |
| Canned response: fixed `status` + `body` (with optional headers) | `router.sy::serveStaticResponse` |
| Static file serving: `prefix` + `root` + auto index | `router.sy::serveStatic` |
| Reverse proxy: `prefix` + `upstream` + `strip_prefix` | `router.sy::proxyToUpstream` |
| Round-robin load balancing across backend pool | `upstream.sy::nextServerIdx` |
| Combined-format access log with elapsed-ms | `log.sy::writeAccess` |
| Error pages: status → body map (404 / 502 / 504 …) | `config.sy::error_body_for` |
| Custom header forwarding (Host, UA, X-Real-IP, X-Forwarded-*) | `router.sy::buildForwardHeaders` |
| Configurable request timeouts per upstream | `config.sy::timeoutForUpstream` |

## Run

```bash
# Build SYX once (only needed the first time)
cargo build -p syx-cli --release

# Start sgate with the default ./sgate.json config
./target/release/syx run tools/sgate

# Or point at a different config file
./target/release/syx run tools/sgate /etc/sgate/prod.json
```

You should see a startup banner like:

```
sgate sgate-1 starting on 127.0.0.1:8080
    [health] exact=/health (canned)
    [version] exact=/version (canned)
    [echo] prefix=/echo (canned)
    [api_proxy] prefix=/api/ -> api (proxy)
    [web_proxy] prefix=/web/ -> web (proxy)
    [static_files] prefix=/static/ root=./public
    [static_root] prefix=/ root=./public
    upstreams:
      [0] [127.0.0.1:3001, 127.0.0.1:3002]
      [1] [127.0.0.1:4000]
    error pages: 404="Not Found" 502="Bad Gateway" 504="Gateway Timeout"
```

## Try it

```bash
# Health endpoint (canned 200, X-Sgate-Handler: health)
curl -i http://127.0.0.1:8080/health

# Version (canned body)
curl -i http://127.0.0.1:8080/version

# Static file (the index page)
curl -i http://127.0.0.1:8080/

# Or via the explicit /static/ prefix
curl -i http://127.0.0.1:8080/static/index.html

# JSON asset
curl -i http://127.0.0.1:8080/static/assets/data.json

# Echo location (matched by prefix)
curl -i http://127.0.0.1:8080/echo

# Reverse proxy (no upstream running → 502)
curl -i http://127.0.0.1:8080/api/users

# 404 with custom body
curl -i http://127.0.0.1:8080/nope
```

To exercise the reverse proxy, start a tiny upstream first:

```bash
# in a separate terminal — listens on :3001
python3 -m http.server 3001

# then
curl -i http://127.0.0.1:8080/api/
```

The request is forwarded to `127.0.0.1:3001/api/` with the
`/api/` prefix preserved (or stripped if `strip_prefix` is set in
the location). Round-robin alternates between `3001` and `3002`
on each call to `/api/...`.

## Config schema

```jsonc
{
  "server": {
    "listen":      "127.0.0.1",        // bind address
    "port":        8080,                // bind port
    "access_log":  true,                // enable combined-format access log
    "worker_name": "sgate-1"            // banner / log prefix
  },

  "upstreams": [
    {
      "name":       "api",             // referenced by `upstream` in a location
      "servers":    ["127.0.0.1:3001", "127.0.0.1:3002"],
      "strategy":   "round_robin",     // (only "round_robin" is wired today)
      "timeout_ms": 5000               // forwarded to Http.fetch
    }
  ],

  "locations": [
    // Static (canned) response:
    { "name": "health", "exact": "/health", "status": 200, "body": "ok",
      "headers": { "X-Sgate-Handler": "health" } },

    // Static file serving:
    { "name": "docs",  "prefix": "/static/", "root": "./public" },

    // Reverse proxy with prefix strip:
    { "name": "api", "prefix": "/api/", "upstream": "api", "proxy": true,
      "strip_prefix": "/api/" }
  ],

  "error_pages": {
    "404": "Not Found",
    "502": "Bad Gateway",
    "504": "Gateway Timeout"
  }
}
```

### Location dispatch order

For every request, the router walks `locations` in order and
chooses the first that matches:

1. **`exact` matches** — `req.path == location.exact`
2. **`prefix` matches** — `req.path` starts with `location.prefix`

Once a location is chosen, the router decides what to do based on
the location's fields:

| Field set | Behavior |
|---|---|
| `status` + `body` (+ optional `headers`) | Return canned response |
| `root` | Serve static file from `<root><stripped path>` |
| `upstream` + `proxy: true` | Forward to upstream (round-robin pick) |
| anything else | 500 with explanation |

### Upstream forwarding

For a proxy location, the request path is matched against the
location's `prefix`. If the location sets `strip_prefix`, that
prefix is removed before forwarding; otherwise the full path is
forwarded. The query string is preserved. Selected request headers
(Host, User-Agent, Accept, Authorization, Cookie, …) are forwarded,
and `X-Real-IP` / `X-Forwarded-For` are added if the client did
not send them. `X-Forwarded-Proto` is set to `http`.

The upstream URL is selected by the round-robin counter. The
counter is per-upstream-name, so two locations pointing at the
same upstream share the load.

## Module layout

```
tools/sgate/
├── main.sy           // entry point: parse args / SYX_SGATE_CONFIG, load config, TLS native or sidecar skip, Http.serve
├── config.sy         // JSON load + typed accessors (server, upstreams, locations, tls, hot_reload, ws_routes)
├── router.sy         // path matching, dispatch, static file / proxy / canned reply, opportunistic reload
├── upstream.sy       // round-robin pool + URL builder
├── log.sy            // access log + error log helpers
├── reload.sy         // mtime polling + atomic CONFIG reassignment
├── tls_native.sy     // W1249: mode native → Env.set SYX_HTTP_SERVE_TLS_* + Http.serve on tls.port
├── tls.sy            // TLS sidecar helpers (socat / stunnel) + historical __syx_tls_serve / if false honesty
├── ws_proxy.sy       // WebSocket pass-through sidecar (websocat) — full handshake + frame codec is forward-declared
├── sha1.sy           // pure-SYX SHA-1 (kept around for the eventual in-process WS handshake)
├── mock_upstream.sy  // tiny SYX mock backend (used by test.sh)
├── sgate.json         // default config (this directory)
├── sgate.test.json    // config used by the extended (--full) test section
├── public/            // static file root
│   ├── index.html
│   ├── hello.txt
│   └── assets/data.json
├── test.sh            // smoke test (basic + --full for TLS/hot-reload/WS)
└── README.md          // you are here
```

## Advanced features

sgate layers TLS / hot-reload / dedicated WS ports on the
basic HTTP gateway.

- **TLS native (W1249 / ADR-1595):** production HTTPS can use
  in-process rustls via `Http.serve` + PEM env — **no socat**.
- **TLS / WS sidecars:** optional dual-port TLS (`socat` /
  `stunnel`) and dedicated `ws_routes` (`websocat`) remain for
  shapes that still want an external terminator; `Process.spawn`
  may be unavailable in some runtime builds (honest skip).

### 1. TLS termination

Configure in `server.tls`:

```jsonc
"server": {
  "port": 8080,
  "tls": {
    "enabled":  true,
    "port":     8443,                  // public HTTPS port (native binds here)
    "cert":     "./certs/server.pem",   // PEM cert
    "key":      "./certs/server.key",   // PEM key
    "backend_port": 8080,              // sidecar only: plain-HTTP port to forward to
    "mode":     "native"               // "native" (recommended) or "sidecar"
  }
}
```

Pass an alternate config with `SYX_SGATE_CONFIG=/path/to.json`
(`syx run` accepts a single PATH; the project dir is `tools/sgate`).

#### `mode: "native"` (W1249 product path)

When `enabled: true` and `mode: "native"`, `tls_native.sy`
loads the PEM paths from config, calls
`Env.set("SYX_HTTP_SERVE_TLS_CERT" / "SYX_HTTP_SERVE_TLS_KEY")`,
and `main.sy` binds `Http.serve` on **`server.tls.port`**
(single-port HTTPS, in-process rustls — same runtime as the
stdlib HTTPS nails). **No socat / stunnel.**

**Not claimed:** `net.tls.server` / `__syx_tls_serve` HostFn GA
(that placeholder stays under `if false` in `tls.sy` for
honesty deepen); deleting sidecar helpers; `ws_routes`
websocat removal; Playwright HMR GA.
(WSS marketing claim boundary: W1250 / ADR-1596 soft-close.)

#### `mode: "sidecar"`

When `mode: "sidecar"`, `tls.sy` would spawn a TLS terminator
in front of plain `server.port`. Tool pick order:

1. `socat` — `OPENSSL-LISTEN:8443,… TCP:127.0.0.1:8080`
2. `stunnel` / `stunnel4` (experimental)
3. error + plain HTTP if neither is installed

Today `main.sy` **honestly skips** sidecar bring-up when
`Process.spawn` is unavailable in the runtime build (plain
`Http.serve` on `server.port`). Prefer `mode: "native"` for
production HTTPS without an external TLS tool.

#### Generating a self-signed cert for the demo

```bash
mkdir -p tools/sgate/certs
openssl req -x509 -newkey rsa:2048 -nodes \
    -keyout tools/sgate/certs/server.key \
    -out    tools/sgate/certs/server.pem \
    -days 365 -subj "/CN=localhost"
```

### 2. Hot reload (in-process)

Configure in `server.hot_reload`:

```jsonc
"server": {
  "hot_reload": {
    "enabled":     true,
    "interval_ms": 1000
  }
}
```

`reload.sy` keeps the mtime of the config file in a
module-level var. After every HTTP request, the router calls
`reloadMaybe(intervalMs)`; if `intervalMs` has
elapsed since the last check, it stats the file and compares
mtime. On change, it re-parses the JSON and atomically
reassigns `config.CONFIG = parsed`.

Atomicity: SYX's event loop is single-threaded, so a
`var CONFIG: Object = {}` reassignment cannot interleave
with a reader. Every reader sees either the old config or
the new one, never a torn snapshot.

OS-accelerated kick: the poll also tries `File.watch(path)`
(inotify on Linux, FSEvents on macOS, kqueue on BSD). If
that returns any changed paths, we stat the file even
before `interval_ms` has elapsed — this gives sub-second
reload latency without a background thread.

Limits: a quiet server (no requests) won't notice a config
change until the next request. The `interval_ms` knob caps
how long that worst case can be.

### 3. WebSocket pass-through (sidecar)

Configure in `ws_routes`:

```jsonc
"ws_routes": [
  {
    "name":        "ws_echo",
    "path":        "/ws/",
    "upstream":    "wsapi",
    "listen_port": 5001,
    "strip_prefix":"/ws/"
  }
]
```

For each entry, `ws_proxy.sy` spawns one `websocat`
process:

```text
websocat -s tcp-l:127.0.0.1:5001 ws://<upstream-host>:<upstream-port>/
```

The upstream is selected by the same round-robin counter
used by HTTP (`upstream.sy::nextServerIdx`), so a `wsapi`
pool with N servers load-balances just like the `api` HTTP
pool.

#### Why a sidecar for `ws_routes`?

Same-origin HTTP `Upgrade` on the sgate listen port does
**not** need websocat: when a proxy location sees an
`Upgrade` header, `router.sy` returns an Object with
`upgradeUpstream` (same upstream URL shape as `Http.get`).
`Http.serve` dials that host, forwards the request line +
headers, and on `101` uses `Request.upgrade` plus a
background bidirectional byte copy. Upstream 101 response
headers (including `Sec-WebSocket-Accept`) are **relayed**
as-is — serve does **not** compute Accept from the client
Key. Non-101 upstream statuses are mapped to the client via
`status.max(400)` (no tunnel). Frames (including
client-masked RFC6455 text frames) pass through **opaque**;
serve does **not** parse or rewrite them, terminate TLS, or
replace hyper. Stdlib nails:
`http_serve_upgrade_proxy_masked_text_frame_roundtrip`,
`http_serve_upgrade_proxy_relays_sec_websocket_accept`,
`http_serve_upgrade_proxy_non_101_maps_status_max_400`,
`http_serve_upgrade_proxy_hmr_path_same_port_roundtrip` (W1177).

`ws_routes` + websocat still exist for a different job:
dedicated listen ports (e.g. `:5001` for `/ws/`) where
clients connect to a separate socket that speaks WebSocket
end-to-end. SYX `TcpStream` remains client-only (no
stdlib `listen` / `accept`), so those path-based listeners
stay as sidecars.

`sha1.sy` remains scaffolding if a future path wants
in-process WS framing (serve-computed `Sec-WebSocket-Accept`);
that is not required for the byte-tunnel Upgrade path above.

#### Vite HMR through the HTTP proxy (`:8080` → `:9999`)

Browsers open the HMR WebSocket on the **same origin** as
the page (`ws://127.0.0.1:8080/...`). That hits a proxy
location; sgate returns `upgradeUpstream` and
`Http.serve` tunnels bytes to the Vite upstream after 101.
Same-origin HMR can stay on `:8080` without a Vite
`clientPort` change.

**Optional fallback** if you prefer the browser to talk to
Vite's port directly (bypassing sgate for the socket only):

```ts
// vite.config.ts
export default defineConfig({
  server: {
    port: 9999,
    hmr: {
      protocol: "ws",
      host: "127.0.0.1",
      port: 9999,
      clientPort: 9999,
    },
  },
})
```

Limits (still true): no WS frame **terminator** / codec in
sgate or `Http.serve` (passthrough only); Upgrade still a byte
tunnel. Serve backend default is `hyper+tokio` (W1176 /
ADR-1522). Multi-connection **overlap** is available (W1178 /
ADR-1524; ≠ full concurrency SLA). In-process **HTTPS GET**
first slice is available via diagnostic PEM env on
`Http.serve` (W1179 / ADR-1525; ≠ WSS / HMR-over-TLS GA;
sgate production may still use an external TLS sidecar).
`ws_routes` / websocat remain for explicit separate listen
ports, not as a substitute for same-origin Upgrade.

**Todo 8 third slice (W1177 / ADR-1523):** same-port HMR
**product path** is **nailed** — client dials the serve port
only; handler returns `upgradeUpstream` to a Vite-like
upstream; 101 + Accept relay + masked text-frame round-trip
(`http_serve_upgrade_proxy_hmr_path_same_port_roundtrip`).
**Not** claimed: real-browser / Playwright HMR GA; in-process
frame terminator / serve-computed Accept; server TLS.

**Todo 8 fourth slice (W1178 / ADR-1524):** overlapping
accept / multi-connection first slice is **nailed**
(`http_serve_overlap_slow_fast_plain_get`). Multi-tab /
page+HMR no longer serialize on a single in-flight
connection. **Not** claimed: production concurrency SLA.

**Todo 8 fifth slice (W1179 / ADR-1525):** in-process
HTTPS GET first slice is **nailed**
(`http_serve_https_loopback_get_with_extra_root`; env PEM +
`SYX_HTTP_EXTRA_ROOT_PEM`). **Not** claimed (at fifth slice): WSS / HMR-over-TLS
(seventh slice nails HMR-over-HTTPS automation); dropping
the TLS sidecar for production sgate;
`net.http.server` / `net.tls.server` product GA.

**Todo 8 sixth slice (W1180 / ADR-1526):** overlap × HMR
cross nail is **nailed**
(`http_serve_overlap_slow_plain_and_hmr_upgrade` — slow
plain GET holds while same-port HMR Upgrade completes).
**Not** claimed: production concurrency SLA.

**Todo 8 seventh slice (W1181 / ADR-1527):** Upgrade-over-TLS
/ same-port HMR-over-HTTPS first slice is **nailed**
(`http_serve_https_hmr_path_same_port_roundtrip` — rustls
terminate → Upgrade tunnel → Vite-like upstream; 101 +
Accept relay + masked text-frame). **Not** claimed: WSS
product GA; dropping the TLS sidecar for production sgate;
Playwright HMR GA.

**Todo 8 eighth slice (W1182 / ADR-1528):** in-process frame
terminate / serve-computed Accept first slice is **nailed**
(`http_serve_upgrade_terminate_accept_and_text_echo` —
`upgradeTerminate: true` → Accept from Key + text-frame
echo). HMR still uses `upgradeUpstream` relay. **Not**
claimed: WSS product GA; dropping TLS sidecar; Playwright
HMR GA.

**Todo 8 ninth slice (W1183 / ADR-1529):** upgradeTerminate
close-path first slice is **nailed**
(`http_serve_upgrade_terminate_accept_text_and_close` —
text echo then client close → server unmasked close).
**Not** claimed: WSS product GA; full RFC6455 codec
(binary/fragment/ping); dropping TLS sidecar; Playwright
HMR GA.

**Todo 8 tenth slice (W1184 / ADR-1530):** upgradeTerminate
binary echo first slice is **nailed**
(`http_serve_upgrade_terminate_accept_text_binary_and_close` —
text → binary opcode 2 → close). **Not** claimed: WSS
product GA; full RFC6455 (fragment/ping); dropping TLS
sidecar; Playwright HMR GA.

**Todo 8 eleventh slice (W1185 / ADR-1531):** upgradeTerminate
ping→pong first slice is **nailed**
(`http_serve_upgrade_terminate_accept_text_binary_ping_and_close` —
text → binary → ping opcode 9 → unmasked pong). **Not**
claimed: WSS product GA; full RFC6455 (fragment/keepalive);
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 twelfth slice (W1186 / ADR-1532):** upgradeTerminate
fragmented-text first slice is **nailed**
(`http_serve_upgrade_terminate_accept_fragmented_text_binary_ping_and_close` —
FIN=0 text + continuation → single echo). **Not** claimed:
WSS product GA; binary fragment / multi-continuation; dropping
TLS sidecar; Playwright HMR GA.

**Todo 8 thirteenth slice (W1187 / ADR-1533):** upgradeTerminate
fragmented-binary first slice is **nailed**
(`http_serve_upgrade_terminate_accept_fragmented_text_fragmented_binary_ping_and_close`
— FIN=0 binary + continuation → single 0x82). **Not** claimed:
WSS product GA; multi-continuation; dropping TLS sidecar;
Playwright HMR GA.

**Todo 8 fourteenth slice (W1188 / ADR-1534):** upgradeTerminate
multi-continuation N=3 first slice is **nailed**
(`http_serve_upgrade_terminate_accept_multi_continuation_n3_ping_and_close`
— FIN=0 data + FIN=0 mid cont + FIN=1 final cont). **Not**
claimed: WSS product GA; arbitrary-N / control-during-fragment;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 fifteenth slice (W1189 / ADR-1535):** upgradeTerminate
control-during-fragment first slice is **nailed**
(`http_serve_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close`
— N=3 text frag1 → ping/pong → mid cont → final → echo; binary
N=3 without mid-control). **Not** claimed: WSS product GA;
arbitrary-N / binary mid-control; dropping TLS sidecar;
Playwright HMR GA.

**Todo 8 sixteenth slice (W1190 / ADR-1536):** upgradeTerminate
binary mid-control first slice is **nailed**
(`http_serve_upgrade_terminate_accept_control_during_fragment_n3_binary_ping_and_close`
— N=3 binary frag1 → ping/pong → mid cont → final → 0x82).
**Not** claimed: WSS product GA; arbitrary-N / mid↔final control;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 seventeenth slice (W1191 / ADR-1537):** upgradeTerminate
arbitrary-N (N=4 text) first slice is **nailed**
(`http_serve_upgrade_terminate_accept_multi_continuation_n4_text_ping_and_close`
— FIN=0 data + 2× FIN=0 cont + FIN=1 cont → 0x81; binary
lifecycle still N=3). **Not** claimed: WSS product GA;
unbounded-N / binary arbitrary-N / mid↔final control; dropping
TLS sidecar; Playwright HMR GA.

**Todo 8 eighteenth slice (W1192 / ADR-1538):** upgradeTerminate
binary arbitrary-N (N=4) first slice is **nailed**
(`http_serve_upgrade_terminate_accept_multi_continuation_n4_text_and_binary_ping_and_close`
— N=4 text + N=4 binary → 0x81/0x82). **Not** claimed: WSS
product GA; unbounded-N / mid↔final control; dropping TLS
sidecar; Playwright HMR GA.

**Todo 8 nineteenth slice (W1193 / ADR-1539):** upgradeTerminate
mid↔final control first slice is **nailed**
(`http_serve_upgrade_terminate_accept_mid_final_control_n4_text_ping_and_close`
— N=4 text cont1 → ping/pong → cont2 → final). **Not** claimed:
WSS product GA; unbounded-N / binary mid↔final; dropping TLS
sidecar; Playwright HMR GA.

**Todo 8 twentieth slice (W1194 / ADR-1540):** upgradeTerminate
binary mid↔final control first slice is **nailed**
(`http_serve_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close`
— N=4 binary cont1 → ping/pong → cont2 → final). **Not** claimed:
WSS product GA; unbounded-N / extension; dropping TLS
sidecar; Playwright HMR GA.

**Todo 8 twenty-first slice (W1195 / ADR-1541):** upgradeTerminate
unbounded-N (N=5 text) first slice is **nailed**
(`http_serve_upgrade_terminate_accept_multi_continuation_n5_text_ping_and_close`
— FIN=0 + 3× FIN=0 cont + FIN=1 → 0x81). **Not** claimed:
WSS product GA; unbounded-N matrix GA / N=5 binary / extension; dropping TLS
sidecar; Playwright HMR GA.

**Todo 8 twenty-second slice (W1196 / ADR-1542):** upgradeTerminate
unbounded-N (N=5 binary) first slice is **nailed**
(`http_serve_upgrade_terminate_accept_multi_continuation_n5_text_and_binary_ping_and_close`
— N=5 text + N=5 binary → 0x81/0x82). **Not** claimed:
WSS product GA; unbounded-N matrix GA / extension; dropping TLS
sidecar; Playwright HMR GA.

**Todo 8 twenty-third slice (W1197 / ADR-1543):** upgradeTerminate
close status+reason echo first slice is **nailed**
(`http_serve_upgrade_terminate_accept_text_close_status_reason_echo`
— close(1000, "bye") → unmasked echo). **Not** claimed:
WSS product GA; RSV / extension; dropping TLS
sidecar; Playwright HMR GA.

**Todo 8 twenty-fourth slice (W1198 / ADR-1544):** upgradeTerminate
RSV fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_rsv_text_fail_closed`
— RSV1 text → no echo / connection abort). **Not** claimed:
WSS product GA; extension negotiation; protocol close(1002) GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 twenty-fifth slice (W1199 / ADR-1545):** upgradeTerminate
close body len=1 fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_close_len1_fail_closed`
— close payload len=1 → no close echo / abort). **Not** claimed:
WSS product GA; unmasked client; extension; protocol close(1002) GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 twenty-sixth slice (W1200 / ADR-1546):** upgradeTerminate
unmasked-client fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_unmasked_text_fail_closed`
— MASK=0 text → no echo / abort). **Not** claimed:
WSS product GA; fragmented control; extension; protocol close(1002) GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 twenty-seventh slice (W1201 / ADR-1547):** upgradeTerminate
fragmented-control (FIN=0 ping) fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_fin0_ping_fail_closed`
— FIN=0 ping → no pong / abort). **Not** claimed:
WSS product GA; extension; reserved opcode; protocol close(1002) GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 33–35 cluster (W1207 / ADR-1553):** upgradeTerminate
UTF-8 text + close-1004 fail-closed first cluster is **nailed**
(`accept_text_non_utf8_fail_closed` / `accept_fragmented_text_non_utf8_fail_closed`
/ `accept_close_status_1004_fail_closed`). **Not** claimed:
WSS product GA; UTF-8 text product GA; extension; protocol close(1002) product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 36–39 cluster (W1208 / ADR-1554):** upgradeTerminate
framing negative fail-closed first cluster is **nailed**
(`accept_fin0_close_fail_closed` / `accept_unexpected_continuation_fail_closed`
/ `accept_unsolicited_pong_fail_closed` / `accept_fin0_close_during_fragment_fail_closed`).
**Not** claimed: WSS product GA; extension; protocol close(1002) product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 40–43 cluster (W1209 / ADR-1555):** upgradeTerminate
close(1002) product reply first cluster is **nailed**
(`accept_rsv_text_close_1002` / `accept_reserved_opcode_0xb_close_1002`
/ `accept_fin0_ping_close_1002` / `accept_unexpected_continuation_close_1002`
— protocol `InvalidData` → unmasked close(status=1002) then abort).
Prior abort-only nails now expect the 1002 reply. **Not** claimed:
WSS product GA; extension; close-code GA (full set); dropping TLS sidecar;
Playwright HMR GA.

**Todo 8 slices 44–47 cluster (W1210 / ADR-1556):** upgradeTerminate
close(1002) product reply second cluster is **nailed**
(`accept_unmasked_text_close_1002` / `accept_oversized_ping_close_1002`
/ `accept_close_len1_close_1002` / `accept_close_status_1005_close_1002`).
Prior `*_fail_closed` nails retained. **Not** claimed:
WSS product GA; extension; close-code GA (full set); UTF-8 text product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 48–51 cluster (W1211 / ADR-1557):** upgradeTerminate-over-TLS /
WSS terminate product first cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_and_text_echo` /
`…_accept_rsv_text_close_1002` / `…_accept_text_binary_ping_and_close` /
`http_serve_https_hmr_path_accept_still_relayed_not_serve_computed`).
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 52–55 cluster (W1212 / ADR-1558):** upgradeTerminate
close(1002) product reply third cluster is **nailed**
(`accept_text_non_utf8_close_1002` / `accept_close_reason_non_utf8_close_1002`
/ `accept_unsolicited_pong_close_1002` / `accept_close_status_1004_close_1002`).
Prior `*_fail_closed` nails retained. **Not** claimed:
WSS product GA; extension; close-code GA (full set); UTF-8 text product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 56–59 cluster (W1213 / ADR-1559):** upgradeTerminate
close(1002) product reply fourth cluster is **nailed**
(`accept_fragmented_text_non_utf8_close_1002` / `accept_fin0_close_close_1002`
/ `accept_fin0_close_during_fragment_close_1002` / `accept_reserved_opcode_0x3_close_1002`).
Plain fail_closed↔close_1002 matrix aligned 16/16. Prior `*_fail_closed` retained.
**Not** claimed: WSS product GA; extension; close-code GA (full set);
UTF-8 text product GA; dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 60–63 cluster (W1214 / ADR-1560):** upgradeTerminate-over-TLS
close(1002) product reply first cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_close_1002` /
`…_accept_fin0_ping_close_1002` / `…_accept_unexpected_continuation_close_1002` /
`…_accept_unmasked_text_close_1002`).
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA;
TLS close(1002) full matrix; dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 64–67 cluster (W1215 / ADR-1561):** upgradeTerminate-over-TLS
close(1002) product reply second cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_oversized_ping_close_1002` /
`…_accept_close_len1_close_1002` / `…_accept_close_status_1005_close_1002` /
`…_accept_text_non_utf8_close_1002`).
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA;
TLS close(1002) full matrix; UTF-8 text product GA; dropping TLS sidecar;
Playwright HMR GA.

**Todo 8 slices 68–71 cluster (W1216 / ADR-1562):** upgradeTerminate-over-TLS
close(1002) product reply third cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_close_1002` /
`…_accept_unsolicited_pong_close_1002` / `…_accept_close_status_1004_close_1002` /
`…_accept_fragmented_text_non_utf8_close_1002`).
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA;
TLS close(1002) full matrix; UTF-8 text product GA; dropping TLS sidecar;
Playwright HMR GA.

**Todo 8 slices 72–74 cluster (W1217 / ADR-1563):** upgradeTerminate-over-TLS
close(1002) product reply fourth cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_fin0_close_close_1002` /
`…_accept_fin0_close_during_fragment_close_1002` /
`…_accept_reserved_opcode_0x3_close_1002`).
TLS close(1002) dedicated-nail matrix vs plain is **16/16**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA;
close-code GA (full set); UTF-8 text product GA; dropping TLS sidecar;
Playwright HMR GA.

**Todo 8 slices 75–78 cluster (W1218 / ADR-1564):** upgradeTerminate
illegal close-status deepen first cluster is **nailed**
(`http_serve_upgrade_terminate_accept_close_status_1006_close_1002` /
`…_accept_close_status_1015_close_1002` /
`…_accept_close_status_999_close_1002` /
`http_serve_https_upgrade_terminate_accept_close_status_1006_close_1002`).
**Not** claimed: close-code GA (full set); WSS / WebSocket /
`net.tls.server` product GA; dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 79–82 cluster (W1219 / ADR-1565):** upgradeTerminate
illegal close-status deepen second cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_1015_close_1002` /
`…_accept_close_status_999_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_0_close_1002` /
`…_accept_close_status_500_close_1002`).
TLS illegal five codes (1004/1005/1006/1015/999) align with plain.
**Not** claimed: close-code GA (full set); WSS / WebSocket /
`net.tls.server` product GA; dropping TLS sidecar; Playwright HMR GA.


**Todo 8 slices 83–86 cluster (W1220 / ADR-1566):** upgradeTerminate-over-TLS
positive codec deepen first cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_text_close_status_reason_echo` /
`…_accept_fragmented_text_fragmented_binary_ping_and_close` /
`…_accept_multi_continuation_n3_ping_and_close` /
`…_accept_mid_final_control_n4_text_ping_and_close`).
**Not** claimed: TLS positive codec full set; WSS / WebSocket /
`net.tls.server` product GA; close-code GA (full set); dropping TLS sidecar;
Playwright HMR GA.


**Todo 8 slices 87–90 cluster (W1221 / ADR-1567):** upgradeTerminate-over-TLS
positive codec deepen second cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_control_during_fragment_n3_ping_and_close` /
`…_accept_control_during_fragment_n3_binary_ping_and_close` /
`…_accept_multi_continuation_n4_text_ping_and_close` /
`…_accept_multi_continuation_n4_text_and_binary_ping_and_close`).
**Not** claimed: TLS positive codec full set; WSS / WebSocket /
`net.tls.server` product GA; close-code GA (full set); dropping TLS sidecar;
Playwright HMR GA.


**Todo 8 slices 91–94 cluster (W1222 / ADR-1568):** upgradeTerminate-over-TLS
positive codec deepen third cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_mid_final_control_n4_binary_ping_and_close` /
`…_accept_multi_continuation_n5_text_ping_and_close` /
`…_accept_multi_continuation_n5_text_and_binary_ping_and_close` /
`…_accept_fragmented_text_binary_ping_and_close`).
**Not** claimed: TLS positive codec full set; N5 unbounded-N full-set GA;
WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 95–98 cluster (W1223 / ADR-1569):** upgradeTerminate-over-TLS
positive twin closeout + illegal-status TLS pad is **nailed**
(`http_serve_https_upgrade_terminate_accept_text_and_close` /
`…_accept_text_binary_and_close` /
`…_accept_close_status_0_close_1002` /
`…_accept_close_status_500_close_1002`).
**Not** claimed: TLS positive codec full set; close-code GA (full set);
WSS / WebSocket / `net.tls.server` product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 slices 99–102 cluster (W1224 / ADR-1570):** upgradeTerminate-over-TLS
fail_closed name-table first cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed` /
`…_accept_fin0_ping_fail_closed` /
`…_accept_unexpected_continuation_fail_closed` /
`…_accept_unmasked_text_fail_closed`). Soft-twins (behavior = close(1002)).
**Not** claimed: fail_closed TLS name-table full set (16/16);
WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 overall soft-close / item closeout (W1251 / ADR-1597):** todo 8 **soft-close met** — charter done-face frozen (hyper+tokio, same-port HMR, overlap, HTTPS/sgate native, upgradeTerminate matrix, W1248/W1250 soft-closes). Further slices under todo 8 are **OUT**. Inventory nail `todo8_overall_soft_close_first_slice`.
**Not** claimed delivered: Playwright HMR GA; concurrency SLA; dropping websocat `ws_routes`; RFC6455 extension; close-code full set; `net.tls.server` HostFn GA; full WSS/WebSocket product GA.

**Todo 8 WSS marketing product soft-close (W1250 / ADR-1596):** product floor SSOT is **frozen** — `upgradeUpstream`/HMR + `upgradeTerminate` plain/over-TLS + sgate native HTTPS; further “WSS product GA” marketing slices are **OUT**. Inventory nail `wss_marketing_product_soft_close_first_slice`.
**Not** claimed: full WSS/WebSocket product GA; RFC6455 extension; close-code full set; `net.tls.server` HostFn GA; Playwright HMR GA; concurrency SLA; dropping websocat `ws_routes`; todo 8 overall closeout.

**Todo 8 sgate TLS native first slice (W1249 / ADR-1595):** production HTTPS
without socat is **nailed** — `mode: "native"` → `tls_native.sy`
`Env.set(SYX_HTTP_SERVE_TLS_*)` + `Http.serve` on `tls.port`
(in-process rustls). Integration: `test.sh --full` native section and/or
`sgate_tls_native_first_slice` stdlib nail.
**Not** claimed: deleting sidecar helpers; `ws_routes` websocat removal;
`net.tls.server` / `__syx_tls_serve` HostFn GA; Playwright HMR GA;
todo 8 overall closeout; dual-port plain+TLS same-process.
(WSS marketing claim boundary: W1250 / ADR-1596 soft-close.)

**Todo 8 illegal near-bound sampling axis soft-close (W1248 / ADR-1594):** W1247 soft-debt + TLS edge-align to 9/990 is **nailed**; further near-bound `N/999−N` deepen is **OUT** (`http_serve_https_upgrade_terminate_accept_close_status_8_fail_closed` /
`…_991_fail_closed` /
`…_9_close_1002` /
`…_990_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_9_fail_closed` /
`…_990_fail_closed`). Soft-twins (behavior = close(1002)); axis soft-close met — no further deepen slices on this axis.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
Playwright HMR GA; todo 8 overall closeout; new abort-only semantics.
(TLS native product path: see W1249.)

**Todo 8 slices 189–192 cluster (W1247 / ADR-1593):** upgradeTerminate
illegal close-status TLS 8/991 close(1002) twin closeout + plain 9/990 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_8_close_1002` /
`…_991_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_9_close_1002` /
`…_990_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 185–188 cluster (W1246 / ADR-1592):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1245) is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_7_fail_closed` /
`…_992_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_8_fail_closed` /
`…_991_fail_closed`). Soft-twins (behavior = close(1002));
W1245 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 181–184 cluster (W1245 / ADR-1591):** upgradeTerminate
illegal close-status TLS 7/992 close(1002) twin closeout + plain 8/991 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_7_close_1002` /
`…_992_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_8_close_1002` /
`…_991_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 177–180 cluster (W1244 / ADR-1590):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1243) is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_6_fail_closed` /
`…_993_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_7_fail_closed` /
`…_992_fail_closed`). Soft-twins (behavior = close(1002));
W1243 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 173–176 cluster (W1243 / ADR-1589):** upgradeTerminate
illegal close-status TLS 6/993 close(1002) twin closeout + plain 7/992 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_6_close_1002` /
`…_993_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_7_close_1002` /
`…_992_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 169–172 cluster (W1242 / ADR-1588):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1241) is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_5_fail_closed` /
`…_994_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_6_fail_closed` /
`…_993_fail_closed`). Soft-twins (behavior = close(1002));
W1241 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 165–168 cluster (W1241 / ADR-1587):** upgradeTerminate
illegal close-status TLS 5/994 close(1002) twin closeout + plain 6/993 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_5_close_1002` /
`…_994_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_6_close_1002` /
`…_993_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 161–164 cluster (W1240 / ADR-1586):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1239) is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_4_fail_closed` /
`…_995_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_5_fail_closed` /
`…_994_fail_closed`). Soft-twins (behavior = close(1002));
W1239 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 157–160 cluster (W1239 / ADR-1585):** upgradeTerminate
illegal close-status TLS 4/995 close(1002) twin closeout + plain 5/994 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_4_close_1002` /
`…_995_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_5_close_1002` /
`…_994_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 153–156 cluster (W1238 / ADR-1584):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1237) is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_3_fail_closed` /
`…_996_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_4_fail_closed` /
`…_995_fail_closed`). Soft-twins (behavior = close(1002));
W1237 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 149–152 cluster (W1237 / ADR-1583):** upgradeTerminate
illegal close-status TLS 3/996 close(1002) twin closeout + plain 4/995 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_3_close_1002` /
`…_996_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_4_close_1002` /
`…_995_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 145–148 cluster (W1236 / ADR-1582):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1235) is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_2_fail_closed` /
`…_997_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_3_fail_closed` /
`…_996_fail_closed`). Soft-twins (behavior = close(1002));
W1235 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 141–144 cluster (W1235 / ADR-1581):** upgradeTerminate
illegal close-status TLS 2/997 close(1002) twin closeout + plain 3/996 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_2_close_1002` /
`…_997_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_3_close_1002` /
`…_996_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 137–140 cluster (W1234 / ADR-1580):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1233) is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_100_fail_closed` /
`…_998_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_2_fail_closed` /
`…_997_fail_closed`). Soft-twins (behavior = close(1002));
W1233 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 133–136 cluster (W1233 / ADR-1579):** upgradeTerminate
illegal close-status TLS 100/998 close(1002) twin closeout + plain 2/997 deepen is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_100_close_1002` /
`…_998_close_1002` /
`http_serve_upgrade_terminate_accept_close_status_2_close_1002` /
`…_997_close_1002`). ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 129–132 cluster (W1232 / ADR-1578):** upgradeTerminate
illegal close-status fail_closed soft-twin (align W1231) is **nailed**
(`http_serve_upgrade_terminate_accept_close_status_1_fail_closed` /
`…_100_fail_closed` /
`…_998_fail_closed` /
`http_serve_https_upgrade_terminate_accept_close_status_1_fail_closed`). Soft-twins (behavior = close(1002));
W1231 sample soft-twins **4/4**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 125–128 cluster (W1231 / ADR-1577):** upgradeTerminate
illegal close-status deepen third cluster is **nailed**
(`http_serve_upgrade_terminate_accept_close_status_1_close_1002` /
`…_100_close_1002` /
`…_998_close_1002` /
`http_serve_https_upgrade_terminate_accept_close_status_1_close_1002`).
Sample points in already-rejected `0..=999` → close(1002); ≠ close-code full GA.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 123–124 (W1230 / ADR-1576):** upgradeTerminate
illegal close-status fail_closed soft-twin closeout is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_0_fail_closed` /
`…_500_fail_closed`). Soft-twins (behavior = close(1002));
illegal fail_closed name-table **10/10**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 119–122 cluster (W1229 / ADR-1575):** upgradeTerminate
illegal close-status fail_closed soft-twin second cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_status_1015_fail_closed` /
`…_999_fail_closed` /
`http_serve_upgrade_terminate_accept_close_status_0_fail_closed` /
`…_500_fail_closed`). Soft-twins (behavior = close(1002));
illegal fail_closed name-table **8/10**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
illegal fail_closed full set (10/10); dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 115–118 cluster (W1228 / ADR-1574):** upgradeTerminate
illegal close-status fail_closed soft-twin first cluster is **nailed**
(`http_serve_upgrade_terminate_accept_close_status_1006_fail_closed` /
`…_1015_fail_closed` /
`…_999_fail_closed` /
`http_serve_https_upgrade_terminate_accept_close_status_1006_fail_closed`). Soft-twins (behavior = close(1002));
illegal fail_closed name-table **4/10**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 111–114 cluster (W1227 / ADR-1573):** upgradeTerminate-over-TLS
fail_closed name-table fourth/final cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_fin0_close_fail_closed` /
`…_accept_fin0_close_during_fragment_fail_closed` /
`…_accept_reserved_opcode_0x3_fail_closed` /
`…_accept_rsv_text_fail_closed`). Soft-twins (behavior = close(1002));
TLS fail_closed name-table **16/16**.
**Not** claimed: WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 107–110 cluster (W1226 / ADR-1572):** upgradeTerminate-over-TLS
fail_closed name-table third cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_close_reason_non_utf8_fail_closed` /
`…_accept_unsolicited_pong_fail_closed` /
`…_accept_close_status_1004_fail_closed` /
`…_accept_fragmented_text_non_utf8_fail_closed`). Soft-twins (behavior = close(1002)).
**Not** claimed: fail_closed TLS name-table full set (16/16);
WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 slices 103–106 cluster (W1225 / ADR-1571):** upgradeTerminate-over-TLS
fail_closed name-table second cluster is **nailed**
(`http_serve_https_upgrade_terminate_accept_oversized_ping_fail_closed` /
`…_accept_close_len1_fail_closed` /
`…_accept_close_status_1005_fail_closed` /
`…_accept_text_non_utf8_fail_closed`). Soft-twins (behavior = close(1002)).
**Not** claimed: fail_closed TLS name-table full set (16/16);
WSS / WebSocket / `net.tls.server` product GA; close-code GA (full set);
dropping TLS sidecar; Playwright HMR GA; new abort-only semantics.

**Todo 8 thirty-second slice (W1206 / ADR-1552):** upgradeTerminate
close-reason-utf8 fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_close_reason_non_utf8_fail_closed`
— close(1000, reason=`[0xFF]`) → no echo / abort). **Not** claimed:
WSS product GA; UTF-8 text; extension; protocol close(1002) product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 thirty-first slice (W1205 / ADR-1551):** upgradeTerminate
illegal-close-status fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_close_status_1005_fail_closed`
— close(1005) → no echo / abort). **Not** claimed:
WSS product GA; extension; protocol close(1002) product GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 thirtieth slice (W1204 / ADR-1550):** upgradeTerminate
oversized-control-payload (>125) fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_oversized_ping_fail_closed`
— ping len7=126 → no pong / abort). **Not** claimed:
WSS product GA; extension; protocol close(1002) GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 twenty-ninth slice (W1203 / ADR-1549):** upgradeTerminate
data-reserved-opcode (0x3–0x7) fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_reserved_opcode_0x3_fail_closed`
— opcode 0x3 → no echo / abort). **Not** claimed:
WSS product GA; extension; protocol close(1002) GA;
dropping TLS sidecar; Playwright HMR GA.

**Todo 8 twenty-eighth slice (W1202 / ADR-1548):** upgradeTerminate
reserved-control-opcode (0xB–0xF) fail-closed first slice is **nailed**
(`http_serve_upgrade_terminate_accept_reserved_opcode_0xb_fail_closed`
— opcode 0xB → no echo / abort). **Not** claimed:
WSS product GA; extension; protocol close(1002) GA;
dropping TLS sidecar; Playwright HMR GA.

## Smoke test

Same-port Upgrade / HMR / overlap / HTTPS / HMR-over-HTTPS /
terminate(+N3/N4-frag/ctrl-during-frag/ping/close) is covered by the driver stdlib nails,
not by `test.sh` / a full Vite HMR browser run:

```bash
cargo test -p syx-driver --test stdlib http_serve_upgrade --release -- --test-threads=1
cargo test -p syx-driver --test stdlib hmr_path --release -- --test-threads=1
cargo test -p syx-driver --test stdlib http_serve_overlap --release -- --test-threads=1
cargo test -p syx-driver --test stdlib http_serve_tls --release -- --test-threads=1
```

The basic `test.sh` covers the same canned / static / proxy /
404 / round-robin checks as before. Run it with `--full`
to also exercise the three advanced features:

```bash
# minimal — just the basic HTTP gateway
./tools/sgate/test.sh

# also exercise hot reload / TLS / WS (skipped if the
# relevant external tool is missing)
./tools/sgate/test.sh --full

# same but fail the run if any extended test fails
./tools/sgate/test.sh --full --strict
```

The extended section starts a **second** sgate instance on
port 8081 / 8082 / 8083 with a temp copy of the config
(`sgate.test.json`), runs the test, and tears it down — so
the original `sgate.json` on port 8080 is never touched.

The hot-reload test mutates the temp config in place and
hits the affected endpoint up to 5 times within 1.5s; the
PASS line confirms the new value made it through.

The TLS test generates a self-signed cert with `openssl`,
points the temp config at it, and probes
`https://127.0.0.1:8443/health` with `curl -k`. A 200 means
the sidecar is forwarding correctly.

The WebSocket test spins up a `websocat` echo server on
5002, points sgate at it, and does a true round-trip via
the sidecar:

```text
echo 'hello-ws' | websocat --no-close --text ws://127.0.0.1:5001/
# -> hello-ws
```

If `openssl`, `socat`, or `websocat` is missing, the
corresponding section is skipped (and reported as such) so
the script still returns 0.

## Smoke test

`test.sh` is a self-contained black-box test. It boots three
mock backends (an SYX script on 3001, 3002, 4000), starts
sgate, and then curls every route to verify the response:

```bash
# Build SYX once if you don't have a binary yet
cargo build -p syx-cli --release

# Run the full smoke test
./tools/sgate/test.sh
```

The script will:

1. Pick the first `syx` binary it finds (`target/release/syx`
   preferred, then `target/debug/syx`; override with `SYX=...`).
2. Launch `mock_upstream.sy` three times — once per port —
   in the background, with `MOCK_PORT=3001 MOCK_NODE_ID=api-a`
   and so on. Each mock stamps its identity into every reply
   so we can prove round-robin actually rotates.
3. Launch sgate in the background, wait for port 8080.
4. Run ~20 curl checks covering canned routes, static files,
   404 handling, the reverse proxy path, POST body forwarding,
   and round-robin rotation.
5. Trap `EXIT` / `INT` / `TERM` so all the background processes
   are killed even if you `Ctrl-C` in the middle.

Expected output (truncated):

```
==> canned routes (no upstream needed)
  PASS  GET /health -> 200  (200)
  PASS  GET /health body == 'ok'  (body contains "ok")
  ...
==> reverse proxy (requires mocks up)
  PASS  GET /api/users  -> 200
  PASS  api pool rotates  (saw 2 distinct ports over 6 calls:  3001 3002 3001 3002 3001 3002)
  ...

==> 17 / 17 passed, 0 failed
```

Useful flags:

```bash
# Use an already-running mock set on 3001/3002/4000 instead
# of booting fresh mocks.
./tools/sgate/test.sh --no-mocks

# Pin a specific syx binary.
SYX=./target/debug/syx ./tools/sgate/test.sh
```

The mock upstream itself is also usable as a stand-alone
script — handy for poking at sgate by hand:

```bash
# Three terminals (or backgrounded with &):
MOCK_PORT=3001 MOCK_NODE_ID=api-a syx run tools/sgate/mock_upstream.sy
MOCK_PORT=3002 MOCK_NODE_ID=api-b syx run tools/sgate/mock_upstream.sy
MOCK_PORT=4000 MOCK_NODE_ID=web-a syx run tools/sgate/mock_upstream.sy

# then
syx run tools/sgate
curl -s http://127.0.0.1:8080/api/whoami   # alternates 3001/3002
```

The mock supports a small but useful set of paths:

| Path | Returns |
|---|---|
| `GET /` or `/whoami` | `{ node, port, method }` |
| `GET /users` | fake user list (alice, bob) |
| `GET /users/<id>` | single user |
| `POST /echo` | echoes body + forwarded headers |
| `GET /slow` | sleeps 1s (for timeout drill) |
| anything else | 404 |

## Extending sgate

| Want to add… | Where |
|---|---|
| A new route | Add a `locations` entry in `sgate.json` |
| A new upstream | Add a `upstreams` entry, reference it from a location |
| A new response header on a static route | Add `headers: { ... }` to the location |
| A new error page | Add the status code to `error_pages` |
| A new log field | `log.sy::writeAccess` + the `dispatchRequest` call site |
| A new match strategy (regex, host, etc.) | `router.sy::matchLocation` |

If you add a new field to the config, make sure to expose a
typed accessor in `config.sy` (since SYX 116.0 can't dispatch
dynamic-key subscripts cleanly on `Object`).

## Limitations

These are deliberate trade-offs of the current implementation, not
of the SYX language; the SYX stdlib is rich enough to do more, but
the goal here is a *clean* demonstration rather than a feature
parity with nginx.

- **Round-robin only** — `random` / `ip_hash` / `least_conn` are
  reserved future strategies. The `strategy` field is read but not
  branched on.
- **No TLS / HTTPS** — sgate is plain HTTP. Front it with a TLS
  terminator (Caddy, nginx, haproxy) for production.
- **No path-rewriting beyond `strip_prefix`** — a full `rewrite`
  directive with regex would need SYX's regex machinery in
  `packages/text`.
- **No WebSocket frame terminator** — same-port HTTP `Upgrade`
  is an opaque byte tunnel via `upgradeUpstream` (frames pass
  through; no in-process codec). Upstream `Sec-WebSocket-Accept`
  is relayed, not computed. Path-based WS on separate ports
  still uses the websocat sidecar (`ws_routes`).
- **No keepalive / connection pooling** — sgate uses a single
  `Http.fetch` per request; backend keep-alive is the stdlib's
  default.
- **No hot reload** — restart on config change.

## See also

- [SYX 完整使用指南](../../docs/9_UsageGuide.md) — CLI, project
  layout, native vs WASM, plugins.
- [`books/language-guide/29-modules-and-packages.md`](../../books/language-guide/29-modules-and-packages.md) —
  multi-file `import` rules in SYX.
- [`books/language-guide/18-concurrency.md`](../../books/language-guide/18-concurrency.md) —
  how `Http.serve` blocks the main thread on the event loop.
- [examples/httpserver.sy](../../examples/httpserver.sy) — the
  minimal example this project is built on.
