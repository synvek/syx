#!/usr/bin/env bash
# C-1a ManagedHeap default degrade census.
# Soft park: weak/immutable fix · generational algorithm rewrite · C-2 L006 · R-50.
# KPIs: wasm 46/46 · browser=0 · Object≤180/10 · simd≥159 · sy,output 12/12
# Hygiene: bounded probes only — never unbounded while.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

echo "== C-1a heap-default-degrade census =="

fail=0
ok() { echo "$1=$2"; }
die() { echo "FAIL: $*" >&2; fail=1; }

python3 - <<'PY' || die "C-1a surface missing"
from pathlib import Path
mh = Path("crates/syx-runtime-core/src/managed_heap.rs").read_text(encoding="utf-8")
for needle in (
    'unwrap_or(false)',
    "SYX_GC_CHURN_STDOUT",
    "gc_churn_stdout_enabled",
    "c1a_dangerous_layers_default_off",
    "c1a_default_path_128_alloc_collect_no_overflow",
):
    if needle not in mh:
        raise SystemExit(f"managed_heap missing {needle}")
# Must not keep dangerous default-on for v9/v13/compact
import re
# After C-1a, compact/v9/v13 use unwrap_or(false)
if "heap_v9_generational_enabled" not in mh:
    raise SystemExit("missing heap_v9")
if re.search(r"heap_v9_generational_enabled[\s\S]{0,200}?unwrap_or\(true\)", mh):
    raise SystemExit("v9 must not unwrap_or(true)")
if re.search(r"fn heap_v13_generational[\s\S]{0,200}?unwrap_or\(true\)", mh):
    raise SystemExit("v13 must not unwrap_or(true)")
if re.search(r"fn compact_enabled[\s\S]{0,200}?unwrap_or\(true\)", mh):
    raise SystemExit("compact must not unwrap_or(true)")
print("c1a_surface=1")
print("c1a_defaults_off=1")
print("c1a_churn_gate=1")
PY

python3 - <<'PY' || die "census doc hygiene failed"
from pathlib import Path
census = Path("docs/census/c-1a-heap-default-degrade-census.md")
if not census.is_file():
    raise SystemExit("missing docs/census/c-1a-heap-default-degrade-census.md")
ct = census.read_text(encoding="utf-8")
for needle in ("C-1a", "opt-in", "≠", "C-1b", "ManagedHeap"):
    if needle not in ct:
        raise SystemExit(f"c-1a census missing {needle}")
print("c1a_census_doc=1")
PY

cargo test -p syx-runtime-core --lib c1a_ -- --test-threads=1 \
  >/tmp/c1a_nails.txt 2>&1 || {
  tail -40 /tmp/c1a_nails.txt
  die "c1a runtime-core nails failed"
}
ok "c1a_nails" "1"

cargo test -p syx-driver --test runtime \
  p98_gc_v13_generational_default_off_c1a \
  -- --test-threads=1 >/tmp/c1a_driver_nails.txt 2>&1 || {
  tail -40 /tmp/c1a_driver_nails.txt
  die "c1a driver nails failed"
}
cargo test -p syx-driver --test runtime \
  p98_gc_churn_host_stdout_line \
  -- --test-threads=1 >>/tmp/c1a_driver_nails.txt 2>&1 || {
  tail -40 /tmp/c1a_driver_nails.txt
  die "c1a driver churn nail failed"
}
ok "c1a_driver_nails" "1"

out_json=$(mktemp)
SYX=./target/debug/syx
cargo build -p syx-cli --bin syx >/dev/null 2>&1 || die "build syx failed"
python3 scripts/book_pass_rate.py --syx "$SYX" --zh --only-output --strict --json "$out_json" \
  >/tmp/c1a_book_out.txt 2>&1 || die "opt-in output pins failed"
tail -15 /tmp/c1a_book_out.txt
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

python3 - <<'PY' || die "roadmap C-1a exit / next C-1b missing"
from pathlib import Path
text = Path("plan/02-improvement-roadmap.md").read_text(encoding="utf-8")
if not any("C-1a" in l and "[x]" in l for l in text.splitlines()):
    raise SystemExit("C-1a not [x]")
if "默认下一 = **C-1b**" not in text:
    raise SystemExit("need 默认下一 = **C-1b**")
print("roadmap_c1a_exit=1")
print("roadmap_next_c1b=1")
PY

if [[ "$fail" -ne 0 ]]; then
  echo "c1a_heap_default_degrade_census_ok=0"
  exit 1
fi
echo "c1a_heap_default_degrade_census_ok=1"
echo "current=C-1a-heap-default-degrade"
echo "next=C-1b"
