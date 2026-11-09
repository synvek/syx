#!/usr/bin/env bash
# SA-6-01 exit census rollup.
# KPIs: prelude/importable SSOT · old import lines = 0 · public __syx_* = 0 ·
#       Object inventory · System/IO File.stat/readAsync/withOpen Object-cleared.
# Object residual count may be >0 (scoped exit; ≠ full SA-4-03 clear).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== SA-6 exit census =="

python3 - <<'PY'
import json, re, subprocess, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg: str) -> None:
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

def ok(key: str, val: object = 1) -> None:
    print(f"{key}={val}")

# --- prelude / importable SSOT (thin sa_0 hinge) ---
manifest = json.loads((root / "packages/internal/manifest.json").read_text())
man_prelude = list(manifest["prelude_modules"])
man_imp = list(manifest["importable_modules"])
cfg = (root / "crates/syx-config/src/lib.rs").read_text()
m = re.search(r"pub const CORE_PRELUDE_MODULES: &\[&str\] = &\[(.*?)\];", cfg, re.S)
core_prelude = re.findall(r'"([^"]+)"', m.group(1)) if m else []
if core_prelude != man_prelude:
    die(f"prelude mismatch core={core_prelude} man={man_prelude}")
else:
    ok("prelude_match", 1)

emb = (root / "crates/syx-driver/src/embedded_std.rs").read_text()
im = re.search(r"const IMPORTABLE: &\[\(&str, &str\)\] = &\[(.*?)\];", emb, re.S)
emb_names = re.findall(r'\(\s*"([^"]+)"\s*,', im.group(1)) if im else []
if emb_names != man_imp:
    die(f"importable mismatch emb={emb_names} man={man_imp}")
else:
    ok("importable_match", 1)
ok("prelude_module_count", len(man_prelude))
ok("importable_module_count", len(man_imp))

# --- old import lines (sa_3 pattern; packages/examples/books) ---
old_import = re.compile(
    r"^\s*import\s+(Os|Process|Env|Path|IO|Time|DateTime|Net|Sync|Sql|Log|Trace|Profiler|Di|Linalg)\s*$",
    re.M,
)
hits = []
for base in ("packages", "examples", "books"):
    b = root / base
    if not b.exists():
        continue
    for p in b.rglob("*.sy"):
        if "target" in p.parts:
            continue
        text = p.read_text(encoding="utf-8", errors="replace")
        for m in old_import.finditer(text):
            hits.append(f"{p}:{text[:m.start()].count(chr(10))+1}")
if hits:
    die("old import lines remain:\n  " + "\n  ".join(hits[:20]))
ok("old_import_lines", 0)

# --- public __syx_* (delegate sa4 script) ---
r = subprocess.run(
    [sys.executable, str(root / "scripts/census_sa4_host_surface.py")],
    cwd=root,
    capture_output=True,
    text=True,
)
if r.returncode != 0:
    die(f"__syx_ census failed: {r.stdout}{r.stderr}")
ok("public_syx_hostfn_spellings", 0)

# --- Object inventory (exclude packages/internal) ---
obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b)")
obj_hits: list[tuple[str, int]] = []
obj_total = 0
for path in sorted((root / "packages").rglob("*.sy")):
    rel = path.relative_to(root).as_posix()
    if rel.startswith("packages/internal/"):
        continue
    text = path.read_text(encoding="utf-8")
    n = len(obj_re.findall(text))
    if n:
        obj_hits.append((rel, n))
        obj_total += n
ok("object_sig_residual_count", obj_total)
ok("object_sig_residual_files", len(obj_hits))

# --- System/IO Object slice cleared (SA-6-01b hinge) ---
# Hard green: File.stat / File.readAsync must not return Object.
# withOpen(body: Object) remains post-SA residual (closure/host unify).
slice_files = [root / "packages/io/mod.sy"]
slice_bad = []
for p in slice_files:
    text = p.read_text(encoding="utf-8")
    for i, ln in enumerate(text.splitlines(), 1):
        if re.search(r"func\s+(stat|readAsync)\b", ln) and re.search(r"\bObject\b", ln):
            slice_bad.append(f"{p.relative_to(root)}:{i}:{ln.strip()}")

if slice_bad:
    die(
        "System/IO Object slice not cleared (stat/readAsync):\n  "
        + "\n  ".join(slice_bad)
    )
ok("system_io_object_slice_cleared", 1)
# Honesty: withOpen still Object-bodied
with_open_obj = 0
fh = (root / "packages/io/file_handle.sy").read_text()
for ln in fh.splitlines():
    if re.search(r"func\s+withOpen\b", ln) and "Object" in ln:
        with_open_obj = 1
        break
ok("filehandle_withopen_object_residual", with_open_obj)

# Top residual files (informational)
for rel, n in sorted(obj_hits, key=lambda x: -x[1])[:8]:
    print(f"object_residual_top\t{n}\t{rel}")

print(f"current=SA-6-exited")
print(f"next=post-SA-Residual")
print(f"sa6_exit_hinge=met")
sys.exit(fail)
PY
