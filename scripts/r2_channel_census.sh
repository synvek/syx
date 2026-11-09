#!/usr/bin/env bash
# R-2 Channel send/trySend census (post-SA residual).
# KPIs: r1 hinge still green · public Channel.send/trySend value ≠ Object · Sendable-typed.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-2 Channel census =="

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

# --- Roll up R-1 (includes r0 + sa6 + withOpen) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r1_withopen_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r1_withopen_census failed (R-2 requires R-1 hinge stay green)")
else:
    ok("r1_hinge_still_green", 1)

# --- R-2-01: public send / trySend must not spell Object on value ---
sync = root / "packages/sync/mod.sy"
text = sync.read_text(encoding="utf-8")
obj_hits = []
sendable_hits = 0
for i, ln in enumerate(text.splitlines(), 1):
    if not re.search(r"func\s+(send|trySend)\b", ln):
        continue
    if re.search(r"\bObject\b", ln):
        obj_hits.append(f"{sync.relative_to(root)}:{i}:{ln.strip()}")
    if re.search(r"\bSendable\b", ln):
        sendable_hits += 1

if obj_hits:
    die(
        "Channel.send/trySend still Object-valued:\n  "
        + "\n  ".join(obj_hits)
    )
ok("channel_send_trysend_object_residual", 0)

# Both send and trySend must be Sendable-typed
if sendable_hits < 2:
    die(
        f"public send/trySend must use value: Sendable "
        f"(found Sendable on {sendable_hits} of 2 methods)"
    )
ok("channel_send_trysend_sendable_typed", 1)

print("current=R-2-exited")
print("next=R-3-recv-Select")
print("r2_exit_hinge=met")
sys.exit(fail)
PY
