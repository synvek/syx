#!/usr/bin/env bash
# C-1c immutable ObjectRef write-refuse census.
# Soft park: let/var→mutable wiring · Nil refuse · C-1d churn docs · C-2 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-1c immutable-write census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-1c surface missing"
from pathlib import Path
import re
vr = Path("crates/syx-runtime-core/src/value.rs").read_text(encoding="utf-8")
for needle in (
    "C-1c",
    "c1c_immutable_object_ref_set_field_refused",
    "c1c_weak_load_result_set_field_refused",
    "c1c_mutable_object_ref_set_field_still_writes",
):
    if needle not in vr:
        raise SystemExit(f"value.rs missing {needle}")
m = re.search(
    r"pub fn mut_ref_cell_set_field\([\s\S]*?\n    pub fn is_mut_ref_cell",
    vr,
)
if not m:
    raise SystemExit("could not locate mut_ref_cell_set_field body")
body = m.group(0)
if "mutable: _" in body:
    raise SystemExit("must not ignore mutable with mutable:_")
if "if !mutable" not in body:
    raise SystemExit("must refuse when !mutable")
# Success path must not force true for refuse objects; refuse returns false.
if "mutable: false" not in body:
    raise SystemExit("refuse path must return mutable: false")
print("c1c_surface=1")
print("c1c_refuse_gate=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-1c-immutable-write-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-1c-immutable-write-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-1c", "immutable", "≠", "C-1d", "mut_ref_cell_set_field"):
    if needle not in ct:
        raise SystemExit(f"c-1c census missing {needle}")
print("c1c_census_doc=1")
PY

cargo test -p syx-runtime-core --lib c1c_ -- --test-threads=1 \
  >/tmp/c1c_nails.txt 2>&1 || {
  tail -40 /tmp/c1c_nails.txt
  die "c1c runtime-core nails failed"
}
cargo test -p syx-runtime-core --lib phase125_mutref_field_tests -- --test-threads=1 \
  >>/tmp/c1c_nails.txt 2>&1 || {
  tail -40 /tmp/c1c_nails.txt
  die "phase125 mutref regression failed"
}
cargo test -p syx-runtime-core --lib weak_ref_tests -- --test-threads=1 \
  >>/tmp/c1c_nails.txt 2>&1 || {
  tail -40 /tmp/c1c_nails.txt
  die "C-1b weak regression failed"
}
ok "c1c_nails" "1"

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c1c_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c1c_book_out.txt
python3 - <<PY || die "sy,output must be 12/12 (en+zh)"
import json, sys
from pathlib import Path
d = json.loads(Path("$out_json").read_text())
ok_n = d.get("output_run_ok", 0)
tot = d.get("output_run_total", 0)
en = [r for r in d["results"] if "zh-cn" not in r["file"]]
zh = [r for r in d["results"] if "zh-cn" in r["file"]]
print(f"output_run_ok={ok_n}")
print(f"output_run_total={tot}")
print(f"output_en={len(en)} output_zh={len(zh)}")
sys.exit(0 if ok_n == 12 and tot == 12 and len(en) == 6 and len(zh) == 6 else 1)
PY
rm -f "$out_json"

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

python3 - <<'PY' || die "roadmap C-1c exit / next C-1d missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("C-1c" in l and "[x]" in l for l in text.splitlines()):
    raise SystemExit("C-1c not [x]")
if "默认下一 = **C-1d**" not in text:
    raise SystemExit("need 默认下一 = **C-1d**")
print("roadmap_c1c_exit=1")
print("roadmap_next_c1d=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c1c_immutable_write_census_ok=0"
  exit 1
fi
echo "c1c_immutable_write_census_ok=1"
echo "current=C-1c-immutable-write"
echo "next=C-1d"
