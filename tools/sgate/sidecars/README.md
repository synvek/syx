# sgate sidecars (optional)

`tls.sy` / `ws_proxy.sy` are **not** part of the main `sgate` package unit.
They document / retain socat·stunnel·websocat spawn helpers for when
`Process.spawn` is product-ready. Main gateway TLS uses `tls_native.sy`
(in-process PEM + `Http.serve`).

These files still use the older file-module style (`import config` etc.)
and are not compiled by `syx run tools/sgate`.
