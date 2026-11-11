#!/usr/bin/env bash
# R-13 Concurrency census (post-SA residual).
# KPIs: r12 hinge still green · Concurrency task bag → Dictionary · any-value slots → Sendable.
# Counting scope: code lines only (comment lines starting with `//` are excluded).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-13 Concurrency census =="

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

# --- Roll up R-12 (includes r11…Fmt/Testing) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r12_text_testing_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r12_text_testing_census failed (R-13 requires R-12 hinge stay green)")
else:
    ok("r12_hinge_still_green", 1)

src = root / "packages/core/concurrency.sy"
text = src.read_text(encoding="utf-8")

m = re.search(r"public class Concurrency\s*\{(.*)$", text, flags=re.S)
body = m.group(1) if m else ""
if not body:
    die("packages/core/concurrency.sy: public class Concurrency not found")

# Public Object residual in Concurrency (code lines only).
obj_hits = []
for i, ln in enumerate(text.splitlines(), 1):
    s = ln.strip()
    if s.startswith("//"):
        continue
    if "func " in ln and re.search(r"\bObject\b", ln):
        obj_hits.append(f"{src.relative_to(root)}:{i}:{s}")
if obj_hits:
    die("Concurrency public methods still spell Object:\n  " + "\n  ".join(obj_hits))
ok("concurrency_public_object_residual", 0)

# Task/group bags → Dictionary
bag_required = [
    ("detach", r"func\s+detach\s*\(\s*_\s*name:\s*String\s*\)\s*->\s*Dictionary\b"),
    (
        "detach(args)",
        r"func\s+detach\s*\(\s*_\s*name:\s*String\s*,\s*args:\s*Array\s*<\s*Sendable\s*>\s*\)\s*->\s*Dictionary\b",
    ),
    ("join", r"func\s+join\s*\(\s*_\s*task:\s*Dictionary\s*\)\s*->\s*Dictionary\b"),
    ("cancel", r"func\s+cancel\s*\(\s*_\s*task:\s*Dictionary\s*\)\s*->\s*Bool\b"),
    ("taskGroupCreate", r"func\s+taskGroupCreate\s*\(\s*\)\s*->\s*Dictionary\b"),
    (
        "taskGroupAdd",
        r"func\s+taskGroupAdd\s*\(\s*_\s*group:\s*Dictionary\s*,\s*_\s*task:\s*Dictionary\s*\)\s*->\s*Bool\b",
    ),
    ("taskGroupWait", r"func\s+taskGroupWait\s*\(\s*_\s*group:\s*Dictionary\s*\)"),
    ("taskGroupCancel", r"func\s+taskGroupCancel\s*\(\s*_\s*group:\s*Dictionary\s*\)"),
]
for name, pat in bag_required:
    if not re.search(pat, body):
        die(f"Concurrency.{name} must use Dictionary task/group bags")
ok("concurrency_task_handle_dictionary", 1)

# Any-value slots → Sendable
sendable_required = [
    (
        "awaitValue",
        r"func\s+awaitValue\s*\(\s*_\s*task:\s*Dictionary\s*\)\s*->\s*Sendable\b",
    ),
    ("mainActorRun", r"func\s+mainActorRun\s*\(\s*_\s*name:\s*String\s*\)\s*->\s*Sendable\b"),
    (
        "taskLocalSet",
        r"func\s+taskLocalSet\s*\(\s*_\s*key:\s*String\s*,\s*_\s*value:\s*Sendable\s*\)",
    ),
    ("taskLocalGet", r"func\s+taskLocalGet\s*\(\s*_\s*key:\s*String\s*\)\s*->\s*Sendable\b"),
]
for name, pat in sendable_required:
    if not re.search(pat, body):
        die(f"Concurrency.{name} must use Sendable for any-value slots")
ok("concurrency_any_value_sendable", 1)

# Inventory roll-down: code lines only (excl packages/internal).
obj_re = re.compile(
    r"(?::\s*Object\b|->\s*Object\b|Array\s*<\s*Object\s*>)"
)
obj_total = 0
obj_files = 0
for path in sorted((root / "packages").rglob("*.sy")):
    rel = path.relative_to(root).as_posix()
    if rel.startswith("packages/internal/"):
        continue
    code = "\n".join(
        ln
        for ln in path.read_text(encoding="utf-8").splitlines()
        if not ln.strip().startswith("//")
    )
    n = len(obj_re.findall(code))
    if n:
        obj_files += 1
        obj_total += n
ok("object_sig_residual_count", obj_total)
ok("object_sig_residual_files", obj_files)
# Monotonic guard (R-27): absolute pin owned by newest window (r27 contract).
if obj_total > 198:
    die(f"object_sig_residual_count regressed above R-13 baseline 198: {obj_total}")
if obj_files > 14:
    die(f"object_sig_residual_files regressed above R-13 baseline 14: {obj_files}")

print("current=R-13-exited")
print("next=R-14-plus-residual")
print("r13_exit_hinge=met")
sys.exit(fail)
PY
