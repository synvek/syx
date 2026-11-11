#!/usr/bin/env bash
# R-8 HttpBodyStream census (post-SA residual).
# KPIs: r7 hinge still green · open/openWithOptions → Dictionary · chunk/close Int.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-8 HttpBodyStream census =="

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

# --- Roll up R-7 (includes r6…UdpSocket) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r7_udp_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r7_udp_census failed (R-8 requires R-7 hinge stay green)")
else:
    ok("r7_hinge_still_green", 1)

src = root / "packages/net/http_body_stream.sy"
text = src.read_text(encoding="utf-8")

m = re.search(
    r"public class HttpBodyStream\s*\{(.*?)\n\}",
    text,
    flags=re.S,
)
if not m:
    die("packages/net/http_body_stream.sy: public class HttpBodyStream not found")
    body = ""
else:
    body = m.group(1)

obj_hits = []
for i, ln in enumerate(text.splitlines(), 1):
    if "func " in ln and re.search(r"\bObject\b", ln):
        obj_hits.append(f"{src.relative_to(root)}:{i}:{ln.strip()}")
if obj_hits:
    die(
        "HttpBodyStream public methods still spell Object:\n  "
        + "\n  ".join(obj_hits)
    )
ok("http_body_stream_public_object_residual", 0)

if not re.search(r"func\s+open\s*\(\s*url:\s*String\s*\)\s*->\s*Dictionary\b", body):
    die("HttpBodyStream.open must return Dictionary")
ok("http_body_stream_open_returns_dictionary", 1)

if not re.search(
    r"func\s+openWithOptions\s*\(\s*url:\s*String\s*\)\s*->\s*Dictionary\b", body
):
    die("HttpBodyStream.openWithOptions must return Dictionary")
ok("http_body_stream_open_with_options_returns_dictionary", 1)

if not re.search(r"func\s+readChunk\s*\(\s*handle:\s*Int\b", body):
    die("HttpBodyStream.readChunk handle must be Int")
if not re.search(r"func\s+close\s*\(\s*handle:\s*Int\b", body):
    die("HttpBodyStream.close handle must be Int")
ok("http_body_stream_chunk_close_int_typed", 1)

# networking aggregate mirror
net = (root / "packages/networking/mod.sy").read_text(encoding="utf-8")
nm = re.search(r"public class HttpBodyStream\s*\{(.*?)\n\}", net, flags=re.S)
if not nm:
    die("packages/networking/mod.sy: HttpBodyStream not found")
else:
    nb = nm.group(1)
    func_obj = any(
        "func " in ln and re.search(r"\bObject\b", ln) for ln in nb.splitlines()
    )
    if (
        func_obj
        or not re.search(r"func\s+open\s*\([^)]*\)\s*->\s*Dictionary\b", nb)
        or not re.search(r"func\s+openWithOptions\s*\([^)]*\)\s*->\s*Dictionary\b", nb)
    ):
        die("packages/networking/mod.sy HttpBodyStream must mirror Dictionary (re-run gen)")

# SyxHost
host = (root / "packages/internal/system_host.sy").read_text(encoding="utf-8")
if not re.search(
    r"func\s+http_body_stream_open\s*\(\s*url:\s*String\s*\)\s*->\s*Dictionary\b", host
):
    die("SyxHost.http_body_stream_open must return Dictionary")
if not re.search(
    r"func\s+http_body_stream_open_opts\s*\(\s*url:\s*String\s*,\s*options:\s*Dictionary\s*\)\s*->\s*Dictionary\b",
    host,
):
    die("SyxHost.http_body_stream_open_opts must be Dictionary → Dictionary")

# registry
reg = (root / "crates/syx-stdlib/src/registry.rs").read_text(encoding="utf-8")
if not re.search(r'HostFnId::HttpBodyStreamOpen\s*=>\s*"Dictionary"', reg):
    die('registry HttpBodyStreamOpen must map to "Dictionary"')
if not re.search(r'HostFnId::HttpBodyStreamOpenOpts\s*=>\s*"Dictionary"', reg):
    die('registry HttpBodyStreamOpenOpts must map to "Dictionary"')

print("current=R-8-exited")
print("next=R-9-plus-residual")
print("r8_exit_hinge=met")
sys.exit(fail)
PY
