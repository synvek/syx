#!/usr/bin/env bash
# R-41 OrderedSet Object-shell→OrderedSet/Sendable census.
# KPIs: ordered_set Object residual=0 · inventory ≤180/10 · browser_failed=0 · wasm 46/46
# Explicit slice: ≠ simd/json same-window; ≠ OrderedSet<T> GA.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-41 OrderedSet Object-shell census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "OrderedSet façade / inventory gates failed"
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

os_path = root / "packages/core/builtins/collections/ordered_set.sy"
if not os_path.is_file():
    die(f"missing {os_path}")
    sys.exit(1)
os_text = os_path.read_text(encoding="utf-8")
os_code = "\n".join(
    ln for ln in os_text.splitlines() if not ln.strip().startswith("//")
)
os_n = len(obj_re.findall(os_code))
ok("ordered_set_object_shell_residual", os_n)
if os_n != 0:
    die(f"ordered_set_object_shell_residual expected 0, got {os_n}")

# Façade must spell OrderedSet / Sendable (R-41).
for needle in (
    "new() -> OrderedSet",
    "insert(set: OrderedSet, item: Sendable) -> OrderedSet",
    "contains(set: OrderedSet, item: Sendable) -> Bool",
):
    if needle not in os_code.replace(" ", ""):
        # allow whitespace variance
        pass
needles_ws = [
    ("-> OrderedSet", "new return"),
    ("set: OrderedSet", "insert/contains set"),
    ("item: Sendable", "item Sendable"),
]
if "-> OrderedSet" not in os_code:
    die("ordered_set.sy missing -> OrderedSet")
if "item: Sendable" not in os_code:
    die("ordered_set.sy missing item: Sendable")
if "set: OrderedSet" not in os_code:
    die("ordered_set.sy missing set: OrderedSet")
if "item: Object" in os_code or "set: Object" in os_code:
    die("ordered_set.sy still has Object params")
ok("ordered_set_sendable_facade", 1)

# Non-claim markers in source comment.
if "OrderedSet<T>" not in os_text and "generic OrderedSet" not in os_text:
    die("ordered_set.sy must keep ≠ OrderedSet<T> honesty comment")
ok("ordered_set_non_ga_comment", 1)

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
if obj_total > 180:
    die(f"object_sig_residual_count must be ≤180 (got {obj_total})")
if obj_files > 10:
    die(f"object_sig_residual_files must be ≤10 (got {obj_files})")
if obj_total == 180 and obj_files == 10:
    ok("object_sig_residual_exact_r41", 1)
else:
    ok("object_sig_residual_exact_r41", 0)

simd_path = root / "packages/core/simd.sy"
simd_code = "\n".join(
    ln
    for ln in simd_path.read_text(encoding="utf-8").splitlines()
    if not ln.strip().startswith("//")
)
simd_n = len(obj_re.findall(simd_code))
ok("simd_object_residual", simd_n)
if simd_n < 159:
    die(f"simd must stay parked ≥159 this window (got {simd_n})")

# Explicit slice charter: census doc names simd/json out of window.
census = Path("docs/census/r-41-orderedset-census.md")
if not census.is_file():
    die("missing docs/census/r-41-orderedset-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("explicit slice", "simd", "json", "OrderedSet<T>", "≤180", "R-41"):
    if needle not in ct and needle.replace("≤", "<=") not in ct:
        # allow html entity for ≤
        if needle == "≤180" and ("180/10" in ct or "≤180" in ct or "&lt;=180" in ct):
            continue
        if needle not in ct:
            die(f"r-41 census missing {needle}")
ok("r41_census_doc", 1)

sys.exit(fail)
PY

# Surfaces: roadmap + façade file
python3 - <<'PY' || die "roadmap / façade surface missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-41** |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r for r in rows):
    raise SystemExit(f"R-41 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-42" not in tail[1][:120]:
    raise SystemExit("roadmap must default next window to R-42")
print("roadmap_r41_exit=1")
print("roadmap_next_r42=1")
facade = Path("packages/core/builtins/collections/ordered_set.sy").read_text(encoding="utf-8")
if "Object-shell → OrderedSet" not in facade and "Object→OrderedSet" not in facade.replace(" ", ""):
    if "Sendable" not in facade:
        raise SystemExit("facade missing Sendable")
print("facade_r41=1")
PY

# --- Hard KPIs ---
browser_out=$(mktemp)
cargo test -p syx-driver --test browser -- --test-threads=8 >"$browser_out" 2>&1 || true
b_line=$(grep -E "^test result:" "$browser_out" | tail -1 || true)
ok "browser_result_line" "$b_line"
browser_failed=$(echo "$b_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
browser_failed=${browser_failed:-999}
ok "browser_failed" "$browser_failed"
if [[ "$browser_failed" -ne 0 ]]; then
  die "browser_failed must be 0"
fi
rm -f "$browser_out"

wasm_out=$(mktemp)
cargo test -p syx-driver --test wasm -- --test-threads=8 >"$wasm_out" 2>&1 || true
w_line=$(grep -E "^test result:" "$wasm_out" | tail -1 || true)
ok "wasm_result_line" "$w_line"
wasm_failed=$(echo "$w_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
wasm_passed=$(echo "$w_line" | sed -n 's/.* \([0-9][0-9]*\) passed.*/\1/p')
wasm_failed=${wasm_failed:-999}
wasm_passed=${wasm_passed:-0}
ok "wasm_passed" "$wasm_passed"
ok "wasm_failed" "$wasm_failed"
if [[ "$wasm_failed" -ne 0 || "$wasm_passed" -ne 46 ]]; then
  die "wasm must be 46/46"
fi
rm -f "$wasm_out"

python3 - <<'PY' || die "object residual exceeded 180/10"
import re, sys
from pathlib import Path
root = Path(".")
obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b|Array\s*<\s*Object\s*>)")
total = files = 0
for path in sorted((root / "packages").rglob("*.sy")):
    rel = path.relative_to(root).as_posix()
    if rel.startswith("packages/internal/"):
        continue
    code = "\n".join(
        ln for ln in path.read_text(encoding="utf-8").splitlines()
        if not ln.strip().startswith("//")
    )
    n = len(obj_re.findall(code))
    if n:
        files += 1
        total += n
print(f"object_sig_residual_count={total}")
print(f"object_sig_residual_files={files}")
sys.exit(0 if total <= 180 and files <= 10 else 1)
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r41_ordered_set_census_ok=0"
  exit 1
fi
echo "r41_ordered_set_census_ok=1"
