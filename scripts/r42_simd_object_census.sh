#!/usr/bin/env bash
# R-42 simd Object 159 honesty census (charter window).
# KPIs: simd residual ≥159 (no wipe) · inventory ≤180/10 · browser_failed=0 · wasm 46/46
# Non-goals: façade Object→Sendable rewrite this window.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-42 simd Object honesty census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "simd cluster / charter gates failed"
import re, sys
from pathlib import Path
from collections import defaultdict

root = Path(".")
fail = 0

def die(msg: str) -> None:
    global fail
    print(f"FAIL: {msg}", file=sys.stderr)
    fail = 1

def ok(key: str, val: object = 1) -> None:
    print(f"{key}={val}")

obj_re = re.compile(r"(?::\s*Object\b|->\s*Object\b|Array\s*<\s*Object\s*>)")

simd_path = root / "packages/core/simd.sy"
if not simd_path.is_file():
    die(f"missing {simd_path}")
    sys.exit(1)
simd_text = simd_path.read_text(encoding="utf-8")
code = "\n".join(
    ln for ln in simd_text.splitlines() if not ln.strip().startswith("//")
)
simd_n = len(obj_re.findall(code))
ok("simd_object_residual", simd_n)
if simd_n < 159:
    die(f"simd_object_residual must stay ≥159 this charter window (got {simd_n}) — wipe forbidden")

def family(name: str) -> str:
    if name == "fromArray":
        return "fromArray"
    if name in ("loadFromArray", "storeToArray"):
        return "load_store"
    if name.startswith("reduce"):
        return "reduce"
    if name.startswith("shuffle") or "Shuffle" in name:
        return "shuffle"
    if name.startswith(("extract", "replace", "splat")):
        return "lane"
    if name.startswith(("add", "sub", "mul", "div")):
        return "arith"
    if name.startswith(("and", "or", "xor", "shl", "shr", "not")):
        return "bitwise"
    if any(
        name.startswith(p)
        for p in (
            "min",
            "max",
            "abs",
            "eq",
            "ne",
            "lt",
            "le",
            "gt",
            "ge",
            "allTrue",
            "anyTrue",
            "bitselect",
            "sat",
            "avgr",
        )
    ) or "Sat" in name or "Min" in name or "Max" in name:
        return "cmp_minmax"
    return "other"

class_spans = [(m.start(), m.group(1)) for m in re.finditer(r"public class (SIMD\d+)", code)]
func_spans = [(m.start(), m.group(1)) for m in re.finditer(r"public static func (\w+)\(", code)]

def enclosing(spans, pos):
    cur = None
    for s, n in spans:
        if s <= pos:
            cur = n
        else:
            break
    return cur

by_class = defaultdict(int)
by_fam = defaultdict(int)
for m in obj_re.finditer(code):
    c = enclosing(class_spans, m.start()) or "?"
    f = family(enclosing(func_spans, m.start()) or "?")
    by_class[c] += 1
    by_fam[f] += 1

for c in ("SIMD2", "SIMD4", "SIMD8", "SIMD16"):
    ok(f"simd_object_{c.lower()}", by_class.get(c, 0))
for f in (
    "fromArray",
    "arith",
    "load_store",
    "reduce",
    "shuffle",
    "bitwise",
    "cmp_minmax",
    "lane",
    "other",
):
    ok(f"simd_object_fam_{f}", by_fam.get(f, 0))
if sum(by_class.values()) != simd_n:
    die("class cluster sum mismatch")

# Public inventory ceiling (R-41).
obj_total = obj_files = 0
for path in sorted((root / "packages").rglob("*.sy")):
    rel = path.relative_to(root).as_posix()
    if rel.startswith("packages/internal/"):
        continue
    c = "\n".join(
        ln
        for ln in path.read_text(encoding="utf-8").splitlines()
        if not ln.strip().startswith("//")
    )
    n = len(obj_re.findall(c))
    if n:
        obj_files += 1
        obj_total += n
ok("object_sig_residual_count", obj_total)
ok("object_sig_residual_files", obj_files)
if obj_total > 180:
    die(f"object_sig_residual_count must be ≤180 (got {obj_total})")
if obj_files > 10:
    die(f"object_sig_residual_files must be ≤10 (got {obj_files})")

# Host gap: simd_* still returns Object; args often Float64/String placeholders.
host = (root / "packages/internal/system_host.sy").read_text(encoding="utf-8")
host_code = "\n".join(ln for ln in host.splitlines() if not ln.strip().startswith("//"))
host_ret_obj = host_funcs = 0
for fm in re.finditer(
    r"static func (simd_\w+)\((.*?)\)(?:\s*->\s*([^\n{]+))?", host_code, re.S
):
    host_funcs += 1
    ret = (fm.group(3) or "").strip()
    if re.search(r"\bObject\b", ret):
        host_ret_obj += 1
ok("host_simd_funcs", host_funcs)
ok("host_simd_return_object", host_ret_obj)
if host_ret_obj < 1:
    die("expected SyxHost.simd_* to still return Object (gap for charter)")

# TypeId::Simd exists in semantic (runtime type) but façade not aligned.
sem = (root / "crates/syx-semantic/src/semantic_types.rs").read_text(encoding="utf-8")
if "Simd {" not in sem and "Simd{" not in sem:
    die("TypeId::Simd missing from semantic_types — unexpected")
ok("typeid_simd_present", 1)
ok("facade_typeid_gap", 1)  # documented: public Object ≠ TypeId::Simd

# Charter docs
census = root / "docs/census/r-42-simd-object-census.md"
adr = root / "docs/adr/r-42-simd-object-charter.md"
if not census.is_file():
    die("missing docs/census/r-42-simd-object-census.md")
if not adr.is_file():
    die("missing docs/adr/r-42-simd-object-charter.md")
ct = census.read_text(encoding="utf-8")
at = adr.read_text(encoding="utf-8")
for needle in ("159", "SIMD2", "wipe", "charter", "TypeId::Simd", "R-42"):
    if needle not in ct:
        die(f"r-42 census missing {needle}")
for needle in ("wipe", "动刀", "SIMD2", "TypeId::Simd", "Non-claims", "R-42"):
    if needle not in at:
        die(f"r-42 ADR missing {needle}")
ok("r42_census_doc", 1)
ok("r42_adr_charter", 1)

# No wipe: simd.sy must still contain Object signatures (charter window).
if "-> Object" not in code and ": Object" not in code:
    die("simd.sy lost Object signatures — wipe not allowed this window")
ok("simd_object_signatures_retained", 1)

sys.exit(fail)
PY

# Roadmap exit pointer
python3 - <<'PY' || die "roadmap R-42 exit / R-43 default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-42** |" in l]
if not rows or not any("`[x]`" in r or "| [x] |" in r for r in rows):
    raise SystemExit(f"R-42 not [x]: {rows}")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-43" not in tail[1][:120]:
    raise SystemExit("roadmap must default next window to R-43")
print("roadmap_r42_exit=1")
print("roadmap_next_r43=1")
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
  echo "r42_simd_object_census_ok=0"
  exit 1
fi
echo "r42_simd_object_census_ok=1"
