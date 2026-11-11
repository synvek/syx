#!/usr/bin/env bash
# R-4 Select public-surface census (post-SA residual).
# KPIs: r3 hinge still green · Select methods ≠ Object · Array<Channel> + Sendable returns.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-4 Select census =="

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

# --- Roll up R-3 (includes r2 + recv/tryRecv) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r3_channel_recv_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r3_channel_recv_census failed (R-4 requires R-3 hinge stay green)")
else:
    ok("r3_hinge_still_green", 1)

# --- R-4-01: public Select.select / orDefault / timeout must not spell Object ---
sync = root / "packages/sync/mod.sy"
text = sync.read_text(encoding="utf-8")
# Extract Select class body (between "public class Select" and next "public class")
m = re.search(
    r"public class Select\s*\{(.*?)\n\}",
    text,
    flags=re.S,
)
if not m:
    die("packages/sync/mod.sy: public class Select not found")
    sys.exit(1)
select_body = m.group(1)
obj_hits = []
array_channel = 0
return_sendable = 0
fallback_sendable = 0
for i, ln in enumerate(select_body.splitlines(), 1):
    if not re.search(r"func\s+(select|orDefault|timeout)\b", ln):
        continue
    # Absolute line number approx: find in full text
    abs_i = None
    for j, full_ln in enumerate(text.splitlines(), 1):
        if full_ln.strip() == ln.strip() and re.search(
            r"func\s+(select|orDefault|timeout)\b", full_ln
        ):
            abs_i = j
            break
    loc = f"{sync.relative_to(root)}:{abs_i or '?'}:{ln.strip()}"
    if re.search(r"\bObject\b", ln):
        obj_hits.append(loc)
    if re.search(r"Array\s*<\s*Channel\s*>", ln):
        array_channel += 1
    if re.search(r"->\s*Sendable\b", ln):
        return_sendable += 1
    if re.search(r"fallback:\s*Sendable\b", ln):
        fallback_sendable += 1

if obj_hits:
    die(
        "Select public methods still spell Object:\n  "
        + "\n  ".join(obj_hits)
    )
ok("select_public_object_residual", 0)

# select + orDefault + timeout all need Array<Channel>
if array_channel < 3:
    die(
        f"Select methods must use channels: Array<Channel> "
        f"(found on {array_channel} of 3)"
    )
ok("select_channels_array_channel_typed", 1)

# select + orDefault + timeout all return Sendable
if return_sendable < 3:
    die(
        f"Select methods must return Sendable "
        f"(found on {return_sendable} of 3)"
    )
ok("select_return_sendable_typed", 1)

# orDefault fallback must be Sendable
if fallback_sendable < 1:
    die("Select.orDefault must use fallback: Sendable")
ok("select_fallback_sendable_typed", 1)

print("current=R-4-exited")
print("next=R-5-handle-bag")
print("r4_exit_hinge=met")
sys.exit(fail)
PY
