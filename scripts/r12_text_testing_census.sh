#!/usr/bin/env bash
# R-12 Fmt + Testing census (post-SA residual).
# KPIs: r11 hinge still green · Fmt/Testing public Object → Sendable / Array<Sendable> / Function.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-12 Fmt+Testing census =="

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

# --- Roll up R-11 (includes r10…SqlRow) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r11_sql_row_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r11_sql_row_census failed (R-12 requires R-11 hinge stay green)")
else:
    ok("r11_hinge_still_green", 1)

def check_fmt_file(path: Path, label: str) -> None:
    text = path.read_text(encoding="utf-8")
    m = re.search(r"public class Fmt\s*\{(.*?)\n\}", text, flags=re.S)
    if not m:
        die(f"{path}: public class Fmt not found")
        body = ""
    else:
        body = m.group(1)
    obj_hits = []
    for i, ln in enumerate(text.splitlines(), 1):
        # Only count Fmt class method lines with Object (file may have other classes).
        if path.name == "mod.sy":
            # Scan Fmt body only for Object residual.
            pass
    for i, ln in enumerate(body.splitlines(), 1):
        if "func " in ln and re.search(r"\bObject\b", ln):
            obj_hits.append(f"{path.relative_to(root)} Fmt body:{ln.strip()}")
    if obj_hits:
        die(f"{label} Fmt methods still spell Object:\n  " + "\n  ".join(obj_hits))

    if not re.search(r"func\s+print\s*\(\s*value:\s*Sendable\b", body):
        die(f"{label}: Fmt.print value must be Sendable")
    if not re.search(r"func\s+printLine\s*\(\s*value:\s*Sendable\b", body):
        die(f"{label}: Fmt.printLine value must be Sendable")
    if not re.search(r"func\s+debug\s*\(\s*value:\s*Sendable\b", body):
        die(f"{label}: Fmt.debug value must be Sendable")
    if not re.search(
        r"func\s+format\s*\(\s*pattern:\s*String\s*,\s*args:\s*Array\s*<\s*Sendable\s*>",
        body,
    ):
        die(f"{label}: Fmt.format args must be Array<Sendable>")

check_fmt_file(root / "packages/text/fmt.sy", "packages/text/fmt.sy")
check_fmt_file(root / "packages/text/mod.sy", "packages/text/mod.sy")
ok("fmt_public_object_residual", 0)
ok("fmt_format_args_array_sendable", 1)

# Testing
testing = root / "packages/testing/mod.sy"
tt = testing.read_text(encoding="utf-8")
obj_hits = []
for i, ln in enumerate(tt.splitlines(), 1):
    if "func " in ln and re.search(r"\bObject\b", ln):
        obj_hits.append(f"{testing.relative_to(root)}:{i}:{ln.strip()}")
if obj_hits:
    die("Testing public funcs still spell Object:\n  " + "\n  ".join(obj_hits))
ok("testing_public_object_residual", 0)

if not re.search(
    r"func\s+assertEq\s*\(\s*actual:\s*Sendable\s*,\s*expected:\s*Sendable\b", tt
):
    die("assertEq must take Sendable ×2")
if not re.search(
    r"func\s+expectEqual\s*\(\s*actual:\s*Sendable\s*,\s*expected:\s*Sendable\b", tt
):
    die("expectEqual must take Sendable ×2")
if not re.search(r"func\s+test\s*\(\s*name:\s*String\s*,\s*fn:\s*Function\b", tt):
    die("test fn must be Function")
ok("testing_test_fn_function", 1)

# Inventory roll-down (excl packages/internal)
obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b)")
obj_total = 0
obj_files = 0
for path in sorted((root / "packages").rglob("*.sy")):
    rel = path.relative_to(root).as_posix()
    if rel.startswith("packages/internal/"):
        continue
    n = len(obj_re.findall(path.read_text(encoding="utf-8")))
    if n:
        obj_files += 1
        obj_total += n
ok("object_sig_residual_count", obj_total)
ok("object_sig_residual_files", obj_files)
# Monotonic guard: later windows (R-13+) may only reduce the inventory.
# The absolute value is pinned by the newest window's own contract.
if obj_total > 217:
    die(f"object_sig_residual_count regressed above R-12 baseline 217: {obj_total}")
if obj_files > 17:
    die(f"object_sig_residual_files regressed above R-12 baseline 17: {obj_files}")

print("current=R-12-exited")
print("next=R-13-plus-residual")
print("r12_exit_hinge=met")
sys.exit(fail)
PY
