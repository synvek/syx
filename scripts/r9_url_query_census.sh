#!/usr/bin/env bash
# R-9 Url query census (post-SA residual).
# KPIs: r8 hinge still green · encodeQuery/build query → Dictionary · SyxHost + networking mirror.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-9 Url query census =="

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

# --- Roll up R-8 (includes r7…HttpBodyStream) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r8_http_body_stream_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r8_http_body_stream_census failed (R-9 requires R-8 hinge stay green)")
else:
    ok("r8_hinge_still_green", 1)

src = root / "packages/net/url.sy"
text = src.read_text(encoding="utf-8")

m = re.search(
    r"public class Url\s*\{(.*?)\n\}",
    text,
    flags=re.S,
)
if not m:
    die("packages/net/url.sy: public class Url not found")
    body = ""
else:
    body = m.group(1)

obj_hits = []
for i, ln in enumerate(text.splitlines(), 1):
    if "func " in ln and re.search(r"\bObject\b", ln):
        obj_hits.append(f"{src.relative_to(root)}:{i}:{ln.strip()}")
if obj_hits:
    die("Url public methods still spell Object:\n  " + "\n  ".join(obj_hits))
ok("url_public_object_residual", 0)

if not re.search(
    r"func\s+encodeQuery\s*\(\s*query:\s*Dictionary\s*\)\s*->\s*String\b", body
):
    die("Url.encodeQuery query must be Dictionary")
ok("url_encode_query_param_dictionary", 1)

if not re.search(
    r"func\s+build\s*\(\s*base:\s*String\s*,\s*path:\s*String\s*,\s*query:\s*Dictionary\s*\)\s*->\s*String\b",
    body,
):
    die("Url.build query must be Dictionary")
ok("url_build_query_param_dictionary", 1)

# SyxHost
host = (root / "packages/internal/system_host.sy").read_text(encoding="utf-8")
if not re.search(
    r"func\s+url_encode_query\s*\(\s*query:\s*Dictionary\s*\)\s*->\s*String\b", host
):
    die("SyxHost.url_encode_query query must be Dictionary")
if not re.search(
    r"func\s+url_build\s*\(\s*base:\s*String\s*,\s*path:\s*String\s*,\s*query:\s*Dictionary\s*\)\s*->\s*String\b",
    host,
):
    die("SyxHost.url_build must be base:String + query:Dictionary")
ok("url_syxhost_query_dictionary", 1)

# networking mirror
net = (root / "packages/networking/mod.sy").read_text(encoding="utf-8")
nm = re.search(r"public class Url\s*\{(.*?)\n\}", net, flags=re.S)
if not nm:
    die("packages/networking/mod.sy: Url not found")
else:
    nb = nm.group(1)
    func_obj = any(
        "func " in ln and re.search(r"\bObject\b", ln) for ln in nb.splitlines()
    )
    if (
        func_obj
        or not re.search(r"func\s+encodeQuery\s*\(\s*query:\s*Dictionary\b", nb)
        or not re.search(r"func\s+build\s*\([^)]*query:\s*Dictionary\b", nb)
    ):
        die("packages/networking/mod.sy Url must mirror Dictionary query (re-run gen)")
ok("networking_url_mirror_dictionary", 1)

print("current=R-9-exited")
print("next=R-10-plus-residual")
print("r9_exit_hinge=met")
sys.exit(fail)
PY
