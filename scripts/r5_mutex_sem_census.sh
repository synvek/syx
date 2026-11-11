#!/usr/bin/env bash
# R-5 Mutex/Semaphore public-surface census (post-SA residual).
# KPIs: r4 hinge still green · Mutex/Semaphore methods ≠ Object · Int-typed.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-5 Mutex/Semaphore census =="

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

# --- Roll up R-4 (includes r3 + Select) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r4_select_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r4_select_census failed (R-5 requires R-4 hinge stay green)")
else:
    ok("r4_hinge_still_green", 1)

sync = root / "packages/sync/mod.sy"
text = sync.read_text(encoding="utf-8")

def class_body(name: str) -> str:
    m = re.search(
        rf"public class {name}\s*\{{(.*?)\n\}}",
        text,
        flags=re.S,
    )
    if not m:
        die(f"packages/sync/mod.sy: public class {name} not found")
        return ""
    return m.group(1)

mutex_body = class_body("Mutex")
sem_body = class_body("Semaphore")
bodies = [("Mutex", mutex_body), ("Semaphore", sem_body)]

obj_hits = []
for cname, body in bodies:
    for ln in body.splitlines():
        if not re.search(r"func\s+\w+", ln):
            continue
        if re.search(r"\bObject\b", ln):
            # locate absolute line
            for j, full in enumerate(text.splitlines(), 1):
                if full.strip() == ln.strip() and "func" in full:
                    obj_hits.append(f"{sync.relative_to(root)}:{j}:{ln.strip()}")
                    break

if obj_hits:
    die(
        "Mutex/Semaphore public methods still spell Object:\n  "
        + "\n  ".join(obj_hits)
    )
ok("mutex_sem_public_object_residual", 0)

# create returns Int
mutex_create_int = 1 if re.search(
    r"func\s+create\s*\(\s*\)\s*->\s*Int\b", mutex_body
) else 0
sem_create_int = 1 if re.search(
    r"func\s+create\s*\(\s*permits:\s*Int\s*\)\s*->\s*Int\b", sem_body
) else 0
if not mutex_create_int:
    die("Mutex.create must return Int")
if not sem_create_int:
    die("Semaphore.create must return Int")
ok("mutex_create_returns_int", 1)
ok("semaphore_create_returns_int", 1)

# All mutex/sem params that took Object must now be Int
param_ok = True
for cname, body in bodies:
    for ln in body.splitlines():
        if not re.search(r"func\s+(lock|unlock|lockGuard|wait|tryWait|signal|waitGuard)\b", ln):
            continue
        if re.search(r"\bObject\b", ln):
            param_ok = False
        if re.search(r"(mutex|sem):\s*Int\b", ln) is None and re.search(
            r"func\s+(lock|unlock|lockGuard|wait|tryWait|signal|waitGuard)\b", ln
        ):
            # methods must mention Int param
            if not re.search(r":\s*Int\b", ln):
                param_ok = False
if not param_ok:
    die("Mutex/Semaphore lock/wait/… params must be Int-typed")
ok("mutex_sem_params_int_typed", 1)

print("current=R-5-exited")
print("next=R-6-handle-bag")
print("r5_exit_hinge=met")
sys.exit(fail)
PY
