#!/usr/bin/env bash
# R-10 WebSocket census (post-SA residual).
# KPIs: r9 hinge still green · connect/send/recv/close off Object → Dictionary.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-10 WebSocket census =="

python3 - <<'PY'
import re, subprocess, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg: str) -> None:
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

def ok(key: str, val: object = 1) -> None:
    print(f"{key}={val}")

# --- Roll up R-9 (includes r8…Url query) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r9_url_query_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r9_url_query_census failed (R-10 requires R-9 hinge stay green)")
else:
    ok("r9_hinge_still_green", 1)

src = root / "packages/net/websocket.sy"
text = src.read_text(encoding="utf-8")

m = re.search(
    r"public class WebSocket\s*\{(.*?)\n\}",
    text,
    flags=re.S,
)
if not m:
    die("packages/net/websocket.sy: public class WebSocket not found")
    body = ""
else:
    body = m.group(1)

obj_hits = []
for i, ln in enumerate(text.splitlines(), 1):
    if "func " in ln and re.search(r"\bObject\b", ln):
        obj_hits.append(f"{src.relative_to(root)}:{i}:{ln.strip()}")
if obj_hits:
    die("WebSocket public methods still spell Object:\n  " + "\n  ".join(obj_hits))
ok("websocket_public_object_residual", 0)

if not re.search(
    r"func\s+connect\s*\(\s*url:\s*String\s*\)\s*->\s*Dictionary\b", body
):
    die("WebSocket.connect must return Dictionary")
ok("websocket_connect_returns_dictionary", 1)

if not re.search(r"func\s+send\s*\(\s*socket:\s*Dictionary\b", body):
    die("WebSocket.send socket must be Dictionary")
if not re.search(r"func\s+recv\s*\(\s*socket:\s*Dictionary\b", body):
    die("WebSocket.recv socket must be Dictionary")
if not re.search(r"func\s+close\s*\(\s*socket:\s*Dictionary\b", body):
    die("WebSocket.close socket must be Dictionary")
ok("websocket_socket_params_dictionary", 1)

# SyxHost
host = (root / "packages/internal/system_host.sy").read_text(encoding="utf-8")
if not re.search(
    r"func\s+ws_connect\s*\(\s*url:\s*String\s*\)\s*->\s*Dictionary\b", host
):
    die("SyxHost.ws_connect must return Dictionary")
if not re.search(r"func\s+ws_send\s*\(\s*socket:\s*Dictionary\b", host):
    die("SyxHost.ws_send socket must be Dictionary")
if not re.search(r"func\s+ws_recv\s*\(\s*socket:\s*Dictionary\b", host):
    die("SyxHost.ws_recv socket must be Dictionary")
if not re.search(r"func\s+ws_close\s*\(\s*socket:\s*Dictionary\b", host):
    die("SyxHost.ws_close socket must be Dictionary")
ok("websocket_syxhost_dictionary", 1)

# registry
reg = (root / "crates/syx-stdlib/src/registry.rs").read_text(encoding="utf-8")
if not re.search(r'HostFnId::WsConnect\s*=>\s*"Dictionary"', reg):
    die('registry WsConnect must map to "Dictionary"')
if not re.search(r'HostFnId::WsConnectAsync\s*=>\s*"Dictionary"', reg):
    die('registry WsConnectAsync must map to "Dictionary"')

# networking mirror
net = (root / "packages/networking/mod.sy").read_text(encoding="utf-8")
nm = re.search(r"public class WebSocket\s*\{(.*?)\n\}", net, flags=re.S)
if not nm:
    die("packages/networking/mod.sy: WebSocket not found")
else:
    nb = nm.group(1)
    func_obj = any(
        "func " in ln and re.search(r"\bObject\b", ln) for ln in nb.splitlines()
    )
    if (
        func_obj
        or not re.search(r"func\s+connect\s*\([^)]*\)\s*->\s*Dictionary\b", nb)
        or not re.search(r"func\s+send\s*\(\s*socket:\s*Dictionary\b", nb)
    ):
        die("packages/networking/mod.sy WebSocket must mirror Dictionary (re-run gen)")
ok("networking_websocket_mirror_dictionary", 1)

print("current=R-10-exited")
print("next=R-11-plus-residual")
print("r10_exit_hinge=met")
sys.exit(fail)
PY
