#!/usr/bin/env bash
# R-27 Collections public Object→Sendable census.
# KPIs: Array/Dict/Set element·key Object=0 · inventory ≤186/11 · simd ≥159.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-27 collections Object census =="

python3 - <<'PY'
import re, sys
from pathlib import Path

root = Path(".")
fail = 0

def die(msg: str) -> None:
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

def ok(key: str, val: object = 1) -> None:
    print(f"{key}={val}")

obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b|Array\s*<\s*Object\s*>)")
item_key_obj = re.compile(r"\b(?:item|key)\s*:\s*Object\b")

coll_files = [
    root / "packages/core/builtins/collections/array.sy",
    root / "packages/core/builtins/collections/dictionary.sy",
    root / "packages/core/builtins/collections/set.sy",
]
coll_obj = 0
for path in coll_files:
    if not path.is_file():
        die(f"missing {path}")
        continue
    code = "\n".join(
        ln
        for ln in path.read_text(encoding="utf-8").splitlines()
        if not ln.strip().startswith("//")
    )
    n = len(obj_re.findall(code))
    coll_obj += n
    if item_key_obj.search(code):
        die(f"{path.as_posix()}: still has item:|key: Object (R-27 requires Sendable)")
ok("collections_array_dict_set_object", coll_obj)
if coll_obj != 0:
    die(f"collections_array_dict_set_object expected 0, got {coll_obj}")

# Full public inventory (same scope as r13).
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
if obj_total > 186:
    die(f"object_sig_residual_count must be ≤186 (got {obj_total})")
if obj_files > 11:
    die(f"object_sig_residual_files must be ≤11 (got {obj_files})")
# Prefer exact post-slice pin when inventory is clean.
if obj_total != 186 or obj_files != 11:
    # Allow further honest reductions; still report.
    ok("object_sig_residual_exact_r27", 0)
else:
    ok("object_sig_residual_exact_r27", 1)

simd_path = root / "packages/core/simd.sy"
simd_code = "\n".join(
    ln
    for ln in simd_path.read_text(encoding="utf-8").splitlines()
    if not ln.strip().startswith("//")
)
simd_n = len(obj_re.findall(simd_code))
ok("simd_object_residual", simd_n)
if simd_n < 159:
    die(f"simd_object_residual must stay ≥159 this window (got {simd_n})")

# OrderedSet shell must still be present (out of R-27 scope).
os_path = root / "packages/core/builtins/collections/ordered_set.sy"
os_code = "\n".join(
    ln
    for ln in os_path.read_text(encoding="utf-8").splitlines()
    if not ln.strip().startswith("//")
)
os_n = len(obj_re.findall(os_code))
ok("ordered_set_object_shell_residual", os_n)
if os_n < 1:
    die("ordered_set Object-shell unexpectedly cleared (out of R-27 scope)")

print("current=R-27-collections-object")
print("next=R-28-plus-residual")
print("r27_exit_hinge=met" if fail == 0 else "r27_exit_hinge=open")
sys.exit(fail)
PY
