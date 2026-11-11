#!/usr/bin/env bash
# R-11 SqlRow census (post-SA residual).
# KPIs: r10 hinge still green · getString/getInt/getFloat/getBlob/isNull/columnNames row off Object → Dictionary.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-11 SqlRow census =="

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

# --- Roll up R-10 (includes r9…WebSocket) ---
r = subprocess.run(
    ["bash", str(root / "scripts/r10_websocket_census.sh")],
    cwd=root,
    capture_output=True,
    text=True,
)
print(r.stdout, end="")
if r.stderr:
    print(r.stderr, end="", file=sys.stderr)
if r.returncode != 0:
    die("r10_websocket_census failed (R-11 requires R-10 hinge stay green)")
else:
    ok("r10_hinge_still_green", 1)

src = root / "packages/data/sql_row.sy"
text = src.read_text(encoding="utf-8")

m = re.search(
    r"public class SqlRow\s*\{(.*?)\n\}",
    text,
    flags=re.S,
)
if not m:
    die("packages/data/sql_row.sy: public class SqlRow not found")
    body = ""
else:
    body = m.group(1)

obj_hits = []
for i, ln in enumerate(text.splitlines(), 1):
    if "func " in ln and re.search(r"\bObject\b", ln):
        obj_hits.append(f"{src.relative_to(root)}:{i}:{ln.strip()}")
if obj_hits:
    die("SqlRow public methods still spell Object:\n  " + "\n  ".join(obj_hits))
ok("sql_row_public_object_residual", 0)

required = [
    ("getString", r"func\s+getString\s*\(\s*row:\s*Dictionary\b"),
    ("getInt", r"func\s+getInt\s*\(\s*row:\s*Dictionary\b"),
    ("getFloat", r"func\s+getFloat\s*\(\s*row:\s*Dictionary\b"),
    ("getBlob", r"func\s+getBlob\s*\(\s*row:\s*Dictionary\b"),
    ("isNull", r"func\s+isNull\s*\(\s*row:\s*Dictionary\b"),
    ("columnNames", r"func\s+columnNames\s*\(\s*row:\s*Dictionary\b"),
]
for name, pat in required:
    if not re.search(pat, body):
        die(f"SqlRow.{name} row must be Dictionary")
ok("sql_row_params_dictionary", 1)

# SyxHost
host = (root / "packages/internal/system_host.sy").read_text(encoding="utf-8")
host_required = [
    ("sql_row_get_string", r"func\s+sql_row_get_string\s*\(\s*row:\s*Dictionary\b"),
    ("sql_row_get_int", r"func\s+sql_row_get_int\s*\(\s*row:\s*Dictionary\b"),
    ("sql_row_get_float", r"func\s+sql_row_get_float\s*\(\s*row:\s*Dictionary\b"),
    ("sql_row_get_blob", r"func\s+sql_row_get_blob\s*\(\s*row:\s*Dictionary\b"),
    ("sql_row_is_null", r"func\s+sql_row_is_null\s*\(\s*row:\s*Dictionary\b"),
    ("sql_row_column_names", r"func\s+sql_row_column_names\s*\(\s*row:\s*Dictionary\b"),
]
for name, pat in host_required:
    if not re.search(pat, host):
        die(f"SyxHost.{name} row must be Dictionary")
ok("sql_row_syxhost_dictionary", 1)

print("current=R-11-exited")
print("next=R-12-plus-residual")
print("r11_exit_hinge=met")
sys.exit(fail)
PY
