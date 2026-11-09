#!/usr/bin/env bash
# SA-3 module remap census.
# Hinges: import System/Networking live; old Os/Net/IO/… imports = 0; SSOT importable match.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== SA-3 module remap census =="

python3 - <<'PY'
import json, re, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg):
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

manifest = json.loads((root / "packages/internal/manifest.json").read_text())
imp = list(manifest["importable_modules"])

for need in ("System", "Networking", "Synchronization", "Logging", "Tracing", "Profiling", "SQL", "DI", "Numerics"):
    if need not in imp:
        die(f"missing importable {need}")
print("new_importables=1")

for old in ("Os", "Process", "Env", "Path", "IO", "Time", "DateTime", "Net", "Sync", "Sql", "Log", "Trace", "Profiler", "Di", "Linalg"):
    if old in imp:
        die(f"old importable still listed: {old}")
print("old_importables_cleared=1")

# OrderedSet/Deque remain until Collections merge (deferred)
for keep in ("OrderedSet", "Deque"):
    if keep not in imp:
        die(f"{keep} must remain importable (Collections merge deferred)")
print("collections_deferred_keep=1")

emb = (root / "crates/syx-driver/src/embedded_std.rs").read_text()
im = re.search(r"const IMPORTABLE: &\[\(&str, &str\)\] = &\[(.*?)\];", emb, re.S)
emb_names = re.findall(r'\(\s*"([^"]+)"\s*,', im.group(1)) if im else []
if emb_names != imp:
    die(f"IMPORTABLE != manifest\n  emb={emb_names}\n  man={imp}")
print("importable_match=1")

old_import = re.compile(
    r"^\s*import\s+(Os|Process|Env|Path|IO|Time|DateTime|Net|Sync|Sql|Log|Trace|Profiler|Di|Linalg)\s*$",
    re.M,
)
hits = []
for base in ("packages", "examples", "tools", "tutorial", "books", "crates"):
    b = root / base
    if not b.exists():
        continue
    for p in b.rglob("*"):
        if not p.is_file() or p.suffix not in {".sy", ".rs", ".md"}:
            continue
        if "target" in p.parts or "archive" in p.parts:
            continue
        text = p.read_text(encoding="utf-8", errors="replace")
        for m in old_import.finditer(text):
            hits.append(f"{p}:{text[:m.start()].count(chr(10))+1}")
if hits:
    die("old import lines remain:\n  " + "\n  ".join(hits[:25]))
else:
    print("old_import_lines=0")

if not (root / "packages/system/mod.sy").is_file():
    die("missing packages/system/mod.sy")
if not (root / "packages/networking/mod.sy").is_file():
    die("missing packages/networking/mod.sy")
print("system_networking_mods=1")

if "(\"System\"" not in emb.replace(" ", "") and '("System"' not in emb:
    # softer check
    if '"System"' not in emb:
        die("embedded_std missing System")
print("embedded_system=1")

print("sa3_module_remap_helper=1")
if fail:
    sys.exit(1)
print("OK")
PY
