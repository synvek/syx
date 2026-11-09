#!/usr/bin/env bash
# R-1 withOpen census (post-SA residual).
# KPIs: r0/sa6 hinge still green · public FileHandle.withOpen body ≠ Object.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-1 withOpen census =="

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

# --- Roll up R-0 (includes sa_6 + tcp readAsync) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r0_residual_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r0_residual_census failed (R-1 requires R-0 hinge stay green)")
else:
    ok("r0_hinge_still_green", 1)

# --- R-1-01: public withOpen must not spell Object ---
# Census authority: packages/io/file_handle.sy only (system aggregate is gen'd).
fh = root / "packages/io/file_handle.sy"
text = fh.read_text(encoding="utf-8")
with_open_obj = 0
bad = []
for i, ln in enumerate(text.splitlines(), 1):
    if re.search(r"func\s+withOpen\b", ln) and re.search(r"\bObject\b", ln):
        with_open_obj = 1
        bad.append(f"{fh.relative_to(root)}:{i}:{ln.strip()}")

# Also fail if aggregate still has Object on withOpen (gen drift)
sys_mod = root / "packages/system/mod.sy"
if sys_mod.is_file():
    for i, ln in enumerate(sys_mod.read_text(encoding="utf-8").splitlines(), 1):
        if re.search(r"func\s+withOpen\b", ln) and re.search(r"\bObject\b", ln):
            with_open_obj = 1
            bad.append(f"{sys_mod.relative_to(root)}:{i}:{ln.strip()}")

if with_open_obj:
    die(
        "FileHandle.withOpen still Object-bodied:\n  "
        + "\n  ".join(bad)
    )
ok("filehandle_withopen_object_residual", 0)
ok("filehandle_withopen_object_cleared", 1)

# Honesty: typed as Function on public surface (param (Int)->Int not yet parseable)
typed = 0
for ln in text.splitlines():
    if re.search(r"func\s+withOpen\b", ln) and re.search(r"\bFunction\b", ln):
        typed = 1
        break
ok("filehandle_withopen_function_typed", typed)
if not typed:
    die("public withOpen must use body: Function")

print("current=R-1-exited")
print("next=R-2-Channel")
print("r1_exit_hinge=met")
sys.exit(fail)
PY
