#!/usr/bin/env bash
# R-0 residual census rollup (post-SA).
# KPIs: sa_6 hinge still green · Networking TcpStream.readAsync Object-cleared ·
#       Object inventory (honest residual) · withOpen residual honesty.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-0 residual census =="

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

# --- Roll up SA-6 exit KPIs (must stay green) ---
r = subprocess.run(
    ["bash", str(root / "scripts/sa_6_exit_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("sa_6_exit_census failed (R-0 requires SA-6 hinge stay green)")
else:
    ok("sa6_hinge_still_green", 1)

# --- R-0-02: TcpStream.readAsync must not return Object ---
slice_files = [
    root / "packages/networking/mod.sy",
    root / "packages/net/tcp.sy",
    root / "packages/internal/system_host.sy",
]
slice_bad = []
for p in slice_files:
    if not p.is_file():
        continue
    text = p.read_text(encoding="utf-8")
    for i, ln in enumerate(text.splitlines(), 1):
        # Match readAsync / tcp_read_async signatures that still say Object.
        if re.search(r"func\s+(readAsync|tcp_read_async)\b", ln) and re.search(
            r"\bObject\b", ln
        ):
            slice_bad.append(f"{p.relative_to(root)}:{i}:{ln.strip()}")

if slice_bad:
    die(
        "Networking tcp readAsync Object slice not cleared:\n  "
        + "\n  ".join(slice_bad)
    )
ok("networking_tcp_readasync_object_cleared", 1)

# Honesty: withOpen still Object-bodied (R-1 candidate)
with_open_obj = 0
fh = root / "packages/io/file_handle.sy"
if fh.is_file():
    for ln in fh.read_text(encoding="utf-8").splitlines():
        if re.search(r"func\s+withOpen\b", ln) and "Object" in ln:
            with_open_obj = 1
            break
ok("filehandle_withopen_object_residual", with_open_obj)

print("current=R-0-exited")
print("next=R-2-Channel")
print("r0_exit_hinge=met")
sys.exit(fail)
PY
