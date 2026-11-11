#!/usr/bin/env bash
# R-7 UdpSocket census (post-SA residual).
# KPIs: r6 hinge still green · UdpSocket ≠ Object · bind→Int · socket Int · recvFrom Dictionary.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-7 UdpSocket census =="

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

# --- Roll up R-6 (includes r5…Channel.handle) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r6_channel_handle_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r6_channel_handle_census failed (R-7 requires R-6 hinge stay green)")
else:
    ok("r6_hinge_still_green", 1)

udp = root / "packages/net/udp.sy"
text = udp.read_text(encoding="utf-8")

m = re.search(
    r"public class UdpSocket\s*\{(.*?)\n\}",
    text,
    flags=re.S,
)
if not m:
    die("packages/net/udp.sy: public class UdpSocket not found")
    body = ""
else:
    body = m.group(1)

obj_hits = []
for i, ln in enumerate(text.splitlines(), 1):
    if "func " in ln and re.search(r"\bObject\b", ln):
        obj_hits.append(f"{udp.relative_to(root)}:{i}:{ln.strip()}")
if obj_hits:
    die("UdpSocket public methods still spell Object:\n  " + "\n  ".join(obj_hits))
ok("udp_public_object_residual", 0)

if not re.search(r"func\s+bind\s*\(\s*host:\s*String\s*,\s*port:\s*Int\s*\)\s*->\s*Int\b", body):
    die("UdpSocket.bind must return Int")
ok("udp_bind_returns_int", 1)

if not re.search(r"func\s+sendTo\s*\(\s*socket:\s*Int\b", body):
    die("UdpSocket.sendTo socket must be Int")
if not re.search(r"func\s+recvFrom\s*\(\s*socket:\s*Int\b", body):
    die("UdpSocket.recvFrom socket must be Int")
ok("udp_socket_params_int_typed", 1)

if not re.search(
    r"func\s+recvFrom\s*\(\s*socket:\s*Int\s*,\s*maxBytes:\s*Int\s*\)\s*->\s*Dictionary\b",
    body,
):
    die("UdpSocket.recvFrom must return Dictionary")
ok("udp_recv_from_dictionary_typed", 1)

# networking aggregate must mirror (regen source of truth)
net = (root / "packages/networking/mod.sy").read_text(encoding="utf-8")
nm = re.search(r"public class UdpSocket\s*\{(.*?)\n\}", net, flags=re.S)
if not nm:
    die("packages/networking/mod.sy: UdpSocket not found")
else:
    nb = nm.group(1)
    func_obj = any(
        "func " in ln and re.search(r"\bObject\b", ln) for ln in nb.splitlines()
    )
    if (
        func_obj
        or not re.search(r"func\s+bind\s*\([^)]*\)\s*->\s*Int\b", nb)
        or not re.search(r"func\s+recvFrom\s*\([^)]*\)\s*->\s*Dictionary\b", nb)
    ):
        die("packages/networking/mod.sy UdpSocket must mirror Int/Dictionary (re-run gen)")

# SyxHost
host = (root / "packages/internal/system_host.sy").read_text(encoding="utf-8")
if not re.search(r"func\s+udp_bind\s*\(\s*host:\s*String\s*,\s*port:\s*Int\s*\)\s*->\s*Int\b", host):
    die("SyxHost.udp_bind must return Int")
if not re.search(r"func\s+udp_send_to\s*\(\s*socket:\s*Int\b", host):
    die("SyxHost.udp_send_to socket must be Int")
if not re.search(r"func\s+udp_recv_from\s*\(\s*socket:\s*Int\b", host):
    die("SyxHost.udp_recv_from socket must be Int")
if not re.search(r"func\s+udp_recv_from\s*\([^)]*\)\s*->\s*Dictionary\b", host):
    die("SyxHost.udp_recv_from must return Dictionary")

# registry
reg = (root / "crates/syx-stdlib/src/registry.rs").read_text(encoding="utf-8")
if not re.search(r'HostFnId::UdpRecvFrom\s*=>\s*"Dictionary"', reg):
    die('registry UdpRecvFrom must map to "Dictionary"')

print("current=R-7-exited")
print("next=R-8-plus-residual")
print("r7_exit_hinge=met")
sys.exit(fail)
PY
