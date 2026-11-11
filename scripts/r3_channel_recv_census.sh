#!/usr/bin/env bash
# R-3 Channel recv/tryRecv census (post-SA residual).
# KPIs: r2 hinge still green · public Channel.recv/tryRecv ≠ Object · Sendable-typed.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-3 Channel recv census =="

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

# --- Roll up R-2 (includes r1 + send/trySend) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r2_channel_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r2_channel_census failed (R-3 requires R-2 hinge stay green)")
else:
    ok("r2_hinge_still_green", 1)

# --- R-3-01: public recv / tryRecv must not spell Object on return ---
sync = root / "packages/sync/mod.sy"
text = sync.read_text(encoding="utf-8")
obj_hits = []
sendable_hits = 0
for i, ln in enumerate(text.splitlines(), 1):
    if not re.search(r"func\s+(recv|tryRecv)\b", ln):
        continue
    if re.search(r"\bObject\b", ln):
        obj_hits.append(f"{sync.relative_to(root)}:{i}:{ln.strip()}")
    if re.search(r"\bSendable\b", ln):
        sendable_hits += 1

if obj_hits:
    die(
        "Channel.recv/tryRecv still Object-returning:\n  "
        + "\n  ".join(obj_hits)
    )
ok("channel_recv_tryrecv_object_residual", 0)

# Both recv and tryRecv must be Sendable-typed
if sendable_hits < 2:
    die(
        f"public recv/tryRecv must return Sendable "
        f"(found Sendable on {sendable_hits} of 2 methods)"
    )
ok("channel_recv_tryrecv_sendable_typed", 1)

print("current=R-3-exited")
print("next=R-4-Select-handle")
print("r3_exit_hinge=met")
sys.exit(fail)
PY
