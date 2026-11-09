#!/usr/bin/env python3
from pathlib import Path
ROOT = Path(__file__).resolve().parents[1]
# Prefer matching crates/syx-driver/src/embedded_std.rs `NET_SOURCE` for the
# core HTTP/TCP surface; extra TLS/stream/gRPC files are overlay-only until
# embedded concat is expanded in a follow-up.
PARTS = [
    "packages/io/types/http_response.sy",
    "packages/net/http_types.sy",
    "packages/net/mod.sy",
    "packages/net/url.sy",
    "packages/net/websocket.sy",
    "packages/net/tcp.sy",
    "packages/net/udp.sy",
    "packages/net/dns.sy",
    "packages/net/tls.sy",
    "packages/net/http_body_stream.sy",
    "packages/net/grpc.sy",
]
hdr = (
    "/// SA-3-02: `import Networking` aggregate.\n"
    "/// Edit packages/net/*; re-run scripts/gen_networking_mod.py; sync embedded_std.\n"
    "/// Runtime/workspace load prepends packages/internal/system_host.sy (SA-4-04).\n\n"
)
out = ROOT / "packages/networking/mod.sy"
out.parent.mkdir(exist_ok=True)
body = "\n".join((ROOT / p).read_text() for p in PARTS if (ROOT / p).is_file())
assert "__syx_" not in body, "networking aggregate still has __syx_"
out.write_text(hdr + body + "\n")
print(f"wrote {out.relative_to(ROOT)} ({out.stat().st_size} bytes)")
