#!/usr/bin/env bash
# R-45 user Comparable sort census.
# Hard gates: struct_with_compare_witness_sorted order-strict · comparable filter green
# Soft park: JIT Suite-1. KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== R-45 user Comparable sort census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "R-45 StoreLocal / ObjectRef witness wiring missing"
from pathlib import Path
init = Path("crates/syx-mir/src/mir_lower_class_init.rs").read_text(encoding="utf-8")
if "R-45" not in init or "StoreLocal(self_slot)" not in init:
    raise SystemExit("mir_lower_class_init missing R-45 StoreLocal after ObjectSet")
# Both witness and vtable paths must StoreLocal
if init.count("StoreLocal(self_slot)") < 2:
    raise SystemExit("expected StoreLocal on both witness and vtable ObjectSet paths")
vm = Path("crates/syx-vm/src/lib.rs").read_text(encoding="utf-8")
if "mut_ref_cell_get_field(\"__witness_Comparable_compare\")" not in vm:
    raise SystemExit("VM comparable_witness_fn_index missing ObjectRef path")
if "Value::I64(idx)" not in vm.split("fn comparable_witness_fn_index")[1][:800]:
    raise SystemExit("VM comparable witness must accept I64")
mir_run = Path("crates/syx-mir-run/src/runtime.rs").read_text(encoding="utf-8")
if "mut_ref_cell_get_field(\"__witness_Comparable_compare\")" not in mir_run:
    raise SystemExit("mir-run array_sorted missing ObjectRef Comparable witness")
nail = Path("crates/syx-driver/tests/lang/comparable.rs").read_text(encoding="utf-8")
if 'vec!["1", "2", "3"]' not in nail and "vec![\"1\", \"2\", \"3\"]" not in nail:
    # allow either quoting
    if '["1", "2", "3"]' not in nail.replace(" ", ""):
        raise SystemExit("struct_with_compare_witness_sorted must be order-strict")
print("storelocal_witness_vtable=1")
print("objectref_comparable_witness=1")
print("order_strict_nail=1")
PY

lang_out=$(mktemp)
cargo test -p syx-driver --test lang struct_with_compare_witness_sorted -- --test-threads=1 >"$lang_out" 2>&1 || true
result_line=$(grep -E "^test result:" "$lang_out" | tail -1 || true)
ok "comparable_nail_result_line" "$result_line"
nail_failed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
nail_passed=$(echo "$result_line" | sed -n 's/.* \([0-9][0-9]*\) passed.*/\1/p')
nail_failed=${nail_failed:-999}
nail_passed=${nail_passed:-0}
ok "comparable_nail_failed" "$nail_failed"
ok "comparable_nail_passed" "$nail_passed"
if [[ "$nail_failed" -ne 0 || "$nail_passed" -lt 1 ]]; then
  die "struct_with_compare_witness_sorted must pass (≥1)"
fi
rm -f "$lang_out"

comp_out=$(mktemp)
cargo test -p syx-driver --test lang comparable -- --test-threads=8 >"$comp_out" 2>&1 || true
c_line=$(grep -E "^test result:" "$comp_out" | tail -1 || true)
ok "comparable_suite_result_line" "$c_line"
c_failed=$(echo "$c_line" | sed -n 's/.* \([0-9][0-9]*\) failed.*/\1/p')
c_failed=${c_failed:-999}
ok "comparable_suite_failed" "$c_failed"
if [[ "$c_failed" -ne 0 ]]; then
  die "comparable suite must be green"
fi
rm -f "$comp_out"

python3 - <<'PY' || die "census / still-red hygiene failed"
from pathlib import Path
census = Path("docs/census/r-45-comparable-sort-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/r-45-comparable-sort-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("R-45", "Comparable", "1/2/3", "ObjectRef", "StoreLocal", "JIT", "≠"):
    if needle not in ct:
        raise SystemExit(f"r-45 census missing {needle}")
r29 = Path("docs/census/r-29-lang-super-census.md").read_text(encoding="utf-8")
if "Comparable" in r29 and "R-45" not in r29:
    raise SystemExit("R-29 still-red must point Comparable → R-45")
if "JIT" not in r29:
    raise SystemExit("R-29 must retain JIT park")
print("r45_census_doc=1")
print("r29_still_red_updated=1")
PY

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

python3 - <<'PY' || die "object/simd KPI failed"
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
if total > 180 or files > 10:
    sys.exit(1)
simd = (root / "packages/core/simd.sy").read_text(encoding="utf-8")
code = "\n".join(ln for ln in simd.splitlines() if not ln.strip().startswith("//"))
simd_n = len(obj_re.findall(code))
print(f"simd_object_residual={simd_n}")
sys.exit(0 if simd_n >= 159 else 1)
PY

python3 - <<'PY' || die "roadmap R-45 exit / R-46 default missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
rows = [l for l in text.splitlines() if "| **R-45** |" in l]
if not rows or not any("Comparable" in r and ("`[x]`" in r or "| [x] |" in r) for r in rows):
    raise SystemExit(f"R-45 Comparable not [x]: {rows}")
# Former books window must be R-46
if "| **R-46** |" not in text or "书籍" not in text:
    raise SystemExit("roadmap must renumber books surface to R-46")
tail = text.split("默认下一窗", 1)
if len(tail) < 2 or "R-46" not in tail[1][:200]:
    raise SystemExit("roadmap must default next window to R-46")
print("roadmap_r45_exit=1")
print("roadmap_next_r46=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "r45_comparable_sort_census_ok=0"
  exit 1
fi
echo "r45_comparable_sort_census_ok=1"
echo "current=R-45-comparable-sort"
echo "next=R-46-books-swift"
